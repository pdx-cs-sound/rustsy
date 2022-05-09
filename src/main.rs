// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Educational synthesizer.

mod argparse;

use std::borrow::BorrowMut;
use std::sync::Mutex;
use std::thread;

use once_cell::sync::OnceCell;
use wmidi::MidiMessage::*;

use rustsy::*;

static MIXER: OnceCell<Mutex<Mixer<'static>>> = OnceCell::new();
static SLOOP: OnceCell<Loop> = OnceCell::new();

fn main() {
    // Parse arguments.
    let args = argparse::args();
    let kbd = args.keyboard;
    let wav = args.wave.unwrap();

    // Get a signal from a WAV file, make a loop,
    // set up the mixer.
    let sound = get_sample(&wav).unwrap();
    SLOOP.set(Loop::new(&sound)).unwrap();
    MIXER.set(Mutex::new(Mixer::new())).unwrap();

    // Start the keyreader to get input.
    let keystream = read_keys(&kbd).unwrap();
    // Start outputting samples.
    let player = thread::spawn(|| {
        play(MIXER.get().unwrap()).unwrap();
    });
    for kev in keystream {
        match kev {
            NoteOn(_c, note, _vel) => {
                let gsloop = SLOOP.get().unwrap();
                let mut gmixer = MIXER.get().unwrap().lock().unwrap();
                let samples = gsloop.iter_freq(note.to_freq_f32());
                let key = usize::from(note as u8);
                gmixer.borrow_mut().add_key(key, samples);
                drop(gmixer);
            }
            NoteOff(_c, note, _vel) => {
                let mut gmixer = MIXER.get().unwrap().lock().unwrap();
                let key = usize::from(note as u8);
                gmixer.borrow_mut().remove_key(key);
                drop(gmixer);
            }
            _ => (),
        }
    }
    player.join().unwrap();
}
