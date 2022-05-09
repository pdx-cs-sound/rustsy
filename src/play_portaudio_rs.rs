// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Synthesizer audio player.

use std::error::Error;
use std::sync::{Arc, Mutex};

use portaudio_rs as pa;

use crate::*;

/// Gather samples and post for playback.
pub fn play(mixer: Arc<Mutex<Mixer<'static>>>) -> Result<Player<pa::stream::Stream<'static, f32, f32>>, Box<dyn Error>>
{
    let callback = move |_: &[f32], out: &mut[f32], _, _| {
        let mut samples = mixer.lock().unwrap();
        let mut result = pa::stream::StreamCallbackResult::Continue;
        let nout = out.len();
        for i in 0..nout {
            match samples.next() {
                Some(s) => out[i] = s,
                None => {
                    for s in &mut out[i..] {
                        *s = 0.0;
                    }
                    result = pa::stream::StreamCallbackResult::Complete;
                    break;
                }
            }
        }
        result
    };

    // Create and initialize audio output.
    pa::initialize()?;
    let stream = pa::stream::Stream::open_default(
        0, // 0 input channels.
        1, // 1 output channel.
        SAMPLE_RATE as f64,
        WANT_BUFSIZE as u64,
        Some(Box::new(callback)),
    )?;
    stream.start()?;
    Ok(Player(stream))
}
