// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Synthesizer sample file reader.

use hound;
use std::error::Error;
use std::io::{self, ErrorKind};

/// Read a sample file and return it as a buffer of
/// normalized (float) samples.
pub fn get_sample(name: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    // Open and check the file.
    let mut wavfile = hound::WavReader::open(name)?;
    let ws = wavfile.spec();
    if ws.channels != 1 || ws.bits_per_sample != 16 || ws.sample_rate != crate::SAMPLE_RATE {
        return Err(Box::new(io::Error::from(ErrorKind::InvalidData)));
    }

    // Get the signal.
    let signal = wavfile
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0f32)
        .collect();
    Ok(signal)
}
