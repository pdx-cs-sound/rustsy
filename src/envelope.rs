use crate::*;

/// Attack-Decay-Sustain-Release envelope parameters.
// XXX For better performance, we could avoid a bunch
// of expensive divides later by also storing the
// multiplicative inverses of the times.
pub struct ADSR {
    /// Attack time in seconds.
    attack: f32,
    /// Decay time in seconds.
    decay: f32,
    /// Sustain *level*.
    sustain: f32,
    /// Release time in seconds.
    release: f32,
}

impl ADSR {
    /// Make a new ADSR envelope. `attack`, `decay` and `release` are
    /// times in seconds, `sustain` is a level.
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self { attack, decay, sustain, release }
    }
}

pub struct Envelope<'a> {
    /// Time in seconds since start of envelope.
    t: f32,
    /// Time in seconds at which note release happened.
    release_time: Option<f32>,
    /// Last level output pre-release, treated as sustain
    /// level during release.
    sus: f32,
    /// ADSR envelope parameters.
    env: &'a ADSR,
}

impl<'a> Envelope<'a> {
    pub fn new(env: &'a ADSR) -> Self {
        Self {
            t: 0.0,
            release_time: None,
            sus: env.sustain,
            env,
        }
    }

    /// Enter the release phase.
    pub fn release(&mut self) {
        self.release_time = Some(self.t);
    }
}

impl Iterator for Envelope<'_> {
    type Item = f32;

    // Return the envelope coefficient at the current time.
    // Returns None when note should be dropped.
    //
    // Envelope generation is a pain in the neck, especially
    // given multiple samples. It is possible for the
    // current segment of envelope to span attack, decay and
    // sustain, or to span release and continue "off the
    // end". It is also possible to have "early release", at
    // which point release happens either from the current
    // level or the nominal sustain level: we choose the
    // former.
    //
    // The envelope has a built-in timer that starts at 0
    // samples and auto-increments.
    fn next(&mut self) -> Option<Self::Item> {
        // Read and bump the timer.
        let t = self.t;
        self.t += 1.0 / SAMPLE_RATE as f32;

        // Handle release phase.
        if let Some(rt) = self.release_time {
            // Release is special. We a sample on the slope
            // from the effective sustain (level where the
            // key was released) to 0.

            // The signal had level sus at time rt.  We are
            // now at some later time t. If t is off the end
            // we are done.
            let rl = self.env.release;
            if t >= rt + rl {
                return None;
            }
            // Turns out t is in range. Compute and return the envelope
            return Some(self.sus * (1.0 - (t - rt) / rl));
        }

        // Try phases in reverse order until we find one that applies.
        let ta = self.env.attack;
        let td = self.env.decay;
        let sus = self.env.sustain;
        let e = if t >= ta + td {
            // Sustain phase.
            sus
        } else if t >= ta {
            // Decay phase.
            1.0 + (sus - 1.0) * (t - ta) / td
        } else {
            // Attack phase.
            t / ta
        };

        // Make sure to remember the returned level in case we are
        // about to get a release.
        self.sus = e;
        Some(e)
    }
}

pub struct EnvelopedVoice<'a> {
    voice: Box<dyn Voice<'a>>,
    adsr: &'a ADSR,
}

impl<'a> EnvelopedVoice<'a> {
    pub fn new(voice: Box<dyn Voice<'a>>, adsr: &'a ADSR) -> Self {
        Self { voice, adsr }
    }
}

impl<'a> Voice<'a> for EnvelopedVoice<'a> {
    fn iter_freq(&'a self, freq: f32) -> Box<Signal<'a>> {
        let mut signal = self.voice.iter_freq(freq);
        let mut envelope = Envelope::new(self.adsr);
        let enveloped_signal = std::iter::from_fn(move || {
            let e = envelope.next()?;
            let s = signal.next()?;
            Some(e * s)
        });
        Box::new(enveloped_signal)
    }
}
