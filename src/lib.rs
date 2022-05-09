// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Educational music synthesizer.

mod midi;
mod mixer;
mod play;
mod sampler;
mod wavio;

pub use midi::*;
pub use mixer::*;
pub use play::*;
pub use sampler::*;
pub use wavio::*;

/// The audio sample rate is currently fixed at 48000
/// samples per second. This constant will be made a
/// parameter somehow in some future crate version.
pub const SAMPLE_RATE: u32 = 48_000;

/// All voices run as iterators producing `f32`. This trait
/// allows a voice to generically produce an iterator for
/// a given note.
pub trait Voice<'a> {
    fn iter_freq(&'a self, freq: f32) -> Box<dyn Iterator<Item=f32> + Send + 'a>;
}
