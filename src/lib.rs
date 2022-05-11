// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Educational music synthesizer.

mod envelope;
mod midi;
mod mixer;
mod sampler;
mod wave;
mod wavio;

#[cfg(feature = "cpal")]
mod play_cpal;
#[cfg(feature = "portaudio-rs")]
mod play_portaudio_rs;

#[cfg(feature = "cpal")]
use play_cpal as play;
#[cfg(feature = "portaudio-rs")]
use play_portaudio_rs as play;

pub use envelope::*;
pub use midi::*;
pub use mixer::*;
pub use play::*;
pub use sampler::*;
pub use wave::*;
pub use wavio::*;

/// The audio sample rate is currently fixed at 48000
/// samples per second. This constant will be made a
/// parameter somehow in some future crate version.
pub const SAMPLE_RATE: u32 = 48_000;

/// The number of samples we want buffered. Smaller is
/// better, until the underruns start.
pub const WANT_BUFSIZE: u32 = 256;

/// A signal is a stream of samples that can be sent across
/// threads.
type Signal<'a> = dyn Iterator<Item = f32> + Send + 'a;

/// All voices run as iterators producing `f32`. This trait
/// allows a voice to generically produce an iterator for
/// a given note.
pub trait Voice<'a> {
    fn iter_freq(&'a self, freq: f32) -> Box<Signal<'a>>;
}

/// Wrapper struct for player stream, to hold onto it until
/// done playing.
pub struct Player<S>(S);
