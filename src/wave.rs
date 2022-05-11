use std::f32::consts::{PI, TAU};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveShape {
    Sine,
    Square,
    Saw,
    Tri,
}

struct Wave {
    t: f32,
    dt: f32,
    f: fn(f32)->f32,
}

fn square(t: f32) -> f32 {
    if t < PI {
        -1.0
    } else {
        1.0
    }
}

fn saw(t: f32) -> f32 {
    1.0 - (2.0 / TAU) * t
}

fn tri(t: f32) -> f32 {
    if t < PI {
        1.0 - (2.0 / PI) * t
    } else {
        -3.0 + (2.0 / PI) * t
    }
}

impl Wave {
    fn new(freq: f32, shape: WaveShape) -> Self {
        let shapefn = match shape {
            WaveShape::Sine => f32::sin,
            WaveShape::Square => square,
            WaveShape::Saw => saw,
            WaveShape::Tri => tri,
        };
        Self {
            t: 0.0,
            dt: TAU * freq / SAMPLE_RATE as f32,
            f: shapefn,
        }
    }
}

impl Iterator for Wave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.t += self.dt;
        while self.t >= TAU {
            self.t -= TAU;
        }
        Some((self.f)(self.t))
    }
}

pub struct WaveGen {
    shape: WaveShape,
}

impl WaveGen {
    pub fn new(shape: WaveShape) -> Self {
        Self { shape }
    }
}

impl<'a> Voice<'a> for WaveGen {
    fn iter_freq(&'a self, freq: f32) -> Box<dyn Iterator<Item = f32> + Send + 'a> {
        Box::new(Wave::new(freq, self.shape))
    }
}
