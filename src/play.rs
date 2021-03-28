// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Synthesizer audio player.

use std::error::Error;
use std::io::{self, ErrorKind};

use cpal::traits::*;

use crate::*;

pub struct Player {
    _stream: cpal::Stream,
}

/// Gather samples and post for playback.
pub fn play(mut samples: Box<(dyn Iterator<Item = f32> + Send + 'static)>) -> Result<Player, Box<dyn Error + '_>> {

    // Get the device.
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or_else(|| Box::new(io::Error::from(
            ErrorKind::ConnectionRefused)))?;

    // Force-build a config.
    let target_rate = cpal::SampleRate(SAMPLE_RATE as u32);
    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: target_rate,
        buffer_size: cpal::BufferSize::Fixed(24),
    };

    // Build player callback.
    let data_callback = move |out: &mut cpal::Data, _info: &cpal::OutputCallbackInfo| {
        let out = out.as_slice_mut().unwrap();
        let nout = out.len();
        // println!("run {}", nout);
        for i in 0..nout {
            match samples.next() {
                Some(s) => {
                    out[i] = f32::floor(s * 32767.0) as i16;
                },
                None => {
                    #[allow(clippy::needless_range_loop)]
                    for j in i..nout {
                        out[j] = 0;
                    }
                    // XXX Handle takedown somehow.
                    break;
                },
            }
        }
    };

    // Build player error callback.
    let error_callback = |err| {
        eprintln!("an error occurred on the output audio stream: {}", err);
        std::process::exit(1);
    };

    // Set up the stream.
    let stream = device.build_output_stream_raw(
        &config,
        cpal::SampleFormat::I16,
        data_callback,
        error_callback,
    )?;

    Ok(Player { _stream: stream })
}
