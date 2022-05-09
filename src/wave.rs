use std::f32::consts::TAU;

use crate::*;

pub struct Sine;

struct SineWave {
    t: f32,
    dt: f32,
}

impl SineWave {
    fn new(freq: f32) -> Self {
        Self {
            t: 0.0,
            dt: TAU * freq / SAMPLE_RATE as f32,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.t += self.dt;
        if self.t >= TAU {
            self.t -= TAU;
        }
        Some(f32::sin(self.t))
    }
}

impl<'a> Voice<'a> for Sine {
    fn iter_freq(&'a self, freq: f32) -> Box<dyn Iterator<Item=f32> + Send + 'a> {
        Box::new(SineWave::new(freq))
    }
}
