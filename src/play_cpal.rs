// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Synthesizer audio player.

use std::error::Error;
use std::io::{self, ErrorKind};
use std::sync::{Arc, Mutex};

use cpal::traits::*;

use crate::*;

/// Gather samples and post for playback.
pub fn play(mixer: Arc<Mutex<Mixer<'static>>>) -> Result<Player<cpal::Stream>, Box<dyn Error>> {

    // Get the device.
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or_else(|| Box::new(io::Error::from(
            ErrorKind::ConnectionRefused)))?;

    // Config matcher.
    let target_rate = cpal::SampleRate(SAMPLE_RATE as u32);
    let config_matcher = |device: &cpal::Device| {
        for config_range in device.supported_output_configs()? {
            if config_range.channels() != 1 {
                continue;
            }
            if config_range.sample_format() != cpal::SampleFormat::I16 {
                continue;
            }
            if config_range.min_sample_rate() > target_rate {
                continue;
            }
            if config_range.max_sample_rate() < target_rate {
                continue;
            }
            let buffer_size = match config_range.buffer_size() {
                cpal::SupportedBufferSize::Range {min, max} => {
                    eprintln!("buffer size {}..{}", min, max);
                    cpal::BufferSize::Fixed((*min).max(WANT_BUFSIZE.min(*max)))
                }
                cpal::SupportedBufferSize::Unknown => cpal::BufferSize::Default,
            };
            let config = cpal::StreamConfig {
                channels: 1,
                sample_rate: target_rate,
                buffer_size,
            };
            // let config = config_range.with_sample_rate(target_rate);
            // let config = config.config();
            eprintln!("config {:#?}", config);
            return Ok(config);
        }
        Err(cpal::SupportedStreamConfigsError::DeviceNotAvailable)
    };

    // Try to find a matching config.
    let config = config_matcher(&device)?;

    // Build player callback.
    let data_callback = move |out: &mut [i16], _info: &cpal::OutputCallbackInfo| {
        let mut samples = mixer.lock().unwrap();
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
    let data_callback = Box::leak(Box::new(data_callback));

    // Build player error callback.
    let error_callback = |err| {
        eprintln!("an error occurred on the output audio stream: {}", err);
        std::process::exit(1);
    };

    // Set up the stream.
    let stream = device.build_output_stream(
        &config,
        data_callback,
        error_callback,
    )?;
    stream.play()?;

    eprintln!("stream built");
    Ok(Player(stream))
}
