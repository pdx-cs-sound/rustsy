// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Educational synthesizer.

mod argparse;

use std::borrow::BorrowMut;
use std::sync::Mutex;

// This should be replaced with `std::thread::Scope`
// when that feature is stabilized.
use crossbeam::thread::scope;
use wmidi::MidiMessage::*;

use rustsy::*;

fn main() {
    // Parse arguments.
    let args = argparse::args();
    let kbd = args.keyboard;

    let voice: Box<dyn Voice<'_>> = if let Some(ref sample) = args.sampler {
        // Get a signal from a WAV file, make a loop.
        let sound = get_sample(&sample).unwrap();
        Box::new(Loop::new(&sound))
    } else if let Some(ref wave) = args.wave {
        match wave.as_str() {
            "sin" | "sine" => Box::new(Sine),
            _ => panic!("invalid wave shape: use sine"),
        }
    } else {
        panic!("no valid voice: use --sampler or --wave");
    };
    let voice: &'static dyn Voice<'_> = Box::leak(voice);

    // Start the synth.
    let mixer = Mutex::new(Mixer::default());

    scope(|s| {
        let h = s.spawn(|_| play(&mixer).unwrap());

        let keystream = read_keys(&kbd).unwrap();
        for kev in keystream {
            match kev {
                NoteOn(_c, note, _vel) => {
                    let mut gmixer = mixer.lock().unwrap();
                    let samples = voice.iter_freq(note.to_freq_f32());
                    let key = usize::from(note as u8);
                    gmixer.borrow_mut().add_key(key, samples);
                    drop(gmixer);
                }
                NoteOff(_c, note, _vel) => {
                    let mut gmixer = mixer.lock().unwrap();
                    let key = usize::from(note as u8);
                    gmixer.borrow_mut().remove_key(key);
                    drop(gmixer);
                }
                _ => (),
            }
        }

        h.join().unwrap();
    })
    .unwrap();
}
