use crate::effects::Distortion;
use crate::effect::Effect;
use crate::envelope::{Envelope, State};
use crate::oscillator::Oscillator;

use gdnative::prelude::*;

/// Convert samples with PCM.
///
/// This struct is created by [`Sample`].
/// You can use this generator directly or plug it into a [`Mixer`] object.
///
/// [`Sample`]: struct.Sample.html
/// [`Mixer`]: struct.Mixer.html
#[derive(Debug)]
pub struct Generator {
    /// Whether we are finished running the sample.
    pub(crate) finished: bool,
    /// The total offset.
    pub(crate) offset: usize,
    /// Multiplier of the result.
    pub(crate) volume: Option<f32>,

    /// The oscillator, because it's a trait it has to be boxed.
    pub(crate) oscillator: Oscillator,
    /// The ADSR envelope.
    pub(crate) envelope: Envelope,

    /// Distortion effect.
    pub(crate) distortion: Option<Distortion>,
}

impl Generator {
    /// Generate the sound for the sample.
    pub fn run(&mut self, mut output: &mut [Vector2]) {
        // Run the oscillator
        self.oscillator.generate(&mut output, self.offset);

        // Apply the ADSR and set the state if we're finished or not
        if self.envelope.apply(&mut output, self.offset) == State::Done {
            self.finished = true;
        }

        // Apply the distortion
        if let Some(distortion) = &mut self.distortion {
            distortion.apply(&mut output, self.offset);
        }

        // Apply the volume
        if let Some(volume) = self.volume {
            output.iter_mut().for_each(|tone| *tone *= volume);
        }

        self.offset += output.len();
    }
}