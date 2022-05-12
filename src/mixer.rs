// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// *Historical note:* the use of a `HashMap` instead of a
// `Vec` as a notemap was originally a workaround for
// `Vec::retain()` passing `&T` instead of `&mut T`.
// Once [[Vec::retain_mut()]] is stabilized (issue #90829),
// one could undo this decision, but it seems fine for now.
use std::collections::HashMap;

/// A sample "mixer" that adds values from streams of
/// samples (currently always associated with a key) and
/// scales appropriately to get output samples.  Implemented
/// as an unbounded iterator: will return `Some(0.0)` when
/// no sample streams are available.
pub struct Mixer<N> {
    /// Held key indexes and generators.
    pub held: HashMap<usize, N>,
    /// Current mixer gain value.
    gain: f32,
}

/// Max voices before AGC kicks in.
const AGC_VOICES: usize = 8;
/// Mixer gain before AGC kicks in.
const LINEAR_GAIN: f32 = 0.1;

impl<N> Mixer<N> {
    /// Remove a stream from the mixer by key.
    pub fn remove_key(&mut self, key: usize) {
        self.held.remove(&key);
    }

    pub fn get_key_mut(&mut self, key: usize) -> Option<&mut N> {
        self.held.get_mut(&key)
    }

    /// Remove all streams from the mixer.
    pub fn clear(&mut self) {
        self.held.clear();
    }

    /// Adjust the gain to avoid clipping while preserving
    /// some linearity. Essentially a compressor.
    fn agc(&mut self) {
        let nstreams = self.held.len();
        self.gain = if nstreams <= AGC_VOICES {
            LINEAR_GAIN
        } else {
            LINEAR_GAIN * AGC_VOICES as f32 / nstreams as f32
        };
    }

    /// Add a stream to the mixer.
    pub fn add_key(&mut self, key: usize, note: N) {
        let was_held = self.held.insert(key, note);
        assert!(was_held.is_none());
        self.agc();
    }
}

impl<N> Default for Mixer<N> {
    fn default() -> Self {
        Self {
            held: HashMap::with_capacity(128),
            gain: LINEAR_GAIN,
        }
    }
}

/// Iterator over simultaneous streams of samples that adds
/// them to get a result.
impl<N> Iterator for Mixer<N>
where
    N: Iterator<Item = f32>,
{
    type Item = f32;

    // Get the next mixed sample. We do not assume that the
    // input streams are infinite, but the output stream is.
    fn next(&mut self) -> Option<f32> {
        let mut result = 0.0;
        let mut finished = Vec::new();
        for (k, st) in self.held.iter_mut() {
            let s = st.next();
            match s {
                Some(s) => result += s,
                None => finished.push(*k),
            }
        }
        for k in finished {
            self.remove_key(k);
        }
        self.agc();
        Some(result * self.gain)
    }
}
