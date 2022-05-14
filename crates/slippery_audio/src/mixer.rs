use crate::effects::Distortion;
use crate::envelope::Envelope;
use crate::generator::Generator;
use crate::oscillator::Oscillator;
use crate::oscillator::{DutyCycle, OscillatorType};
use crate::sample::Sample;

use gdnative::prelude::*;

use std::{cell::RefCell, collections::HashMap};

/// Manage samples and mix the volume output of each.
///
/// ```rust
/// // Instantiate a new mixer with a sample rate of 44100
/// let mut mixer = usfx::Mixer::new(44_100);
///
/// // Create a default sample as the sinewave
/// let sample = usfx::Sample::default();
/// // Create another sample with a trianglewave
/// let mut other_sample = usfx::Sample::default();
/// other_sample.osc_type(usfx::OscillatorType::Triangle);
///
/// // Play two oscillators at the same time
/// mixer.play(sample);
/// mixer.play(other_sample);
///
/// // This buffer should be passed by the audio library.
/// let mut buffer = [0.0; 44_100];
/// // Fill the buffer with procedurally generated sound.
/// mixer.generate(&mut buffer);
/// ```
#[derive(Debug)]
pub struct Mixer {
    /// List of generators.
    generators: Vec<Generator>,
    /// Store the sample rate so we can keep oscillator buffers.
    sample_rate: usize,
    /// A lookup table of oscillator buffers.
    oscillator_lookup: HashMap<(usize, DutyCycle, OscillatorType), RefCell<Vec<Vector2>>>,
}

impl Mixer {
    /// Create a new mixer object.
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,
            ..Self::default()
        }
    }

    /// Play a sample.
    pub fn play(&mut self, sample: Sample) {
        // Create the ADSR envelope generator
        let envelope = Envelope::new(
            self.sample_rate as f32,
            sample.env_attack,
            sample.env_decay,
            sample.env_sustain,
            sample.env_release,
        );

        // Get the cached buffer (or automatically create a new one)
        let buffer =
            self.oscillator_buffer(sample.osc_frequency, sample.osc_duty_cycle, sample.osc_type);

        // Create the oscillator
        let oscillator = Oscillator::new(buffer, self.sample_rate);

        // Create the distortion if applicable
        let distortion = match (sample.dis_crunch, sample.dis_drive) {
            (Some(crunch), Some(drive)) => Some(Distortion::new(crunch, drive)),
            (Some(crunch), None) => Some(Distortion::new(crunch, 1.0)),
            (None, Some(drive)) => Some(Distortion::new(0.0, drive)),
            (None, None) => None,
        };

        // Combine them in a generator
        let generator = Generator {
            finished: false,
            offset: 0,
            volume: sample.volume,

            oscillator,
            envelope,

            distortion,
        };

        // Use the generator
        self.generators.push(generator);
    }

    /// Generate a frame for the sample.
    ///
    /// The output buffer can be smaller but not bigger than the sample size.
    ///
    /// ```rust
    /// // Instantiate a new mixer
    /// let mut mixer = usfx::Mixer::default();
    ///
    /// // Create a default sample as the sinewave
    /// mixer.play(usfx::Sample::default());
    ///
    /// // This buffer should be passed by the audio library
    /// let mut buffer = [0.0; 44_100];
    /// // Fill the buffer with procedurally generated sound
    /// mixer.generate(&mut buffer);
    /// ```
    pub fn generate(&mut self, output: &mut [Vector2]) {
        // Set the buffer to zero
        output.iter_mut().for_each(|tone| *tone = Vector2::new(0.0, 0.0));

        // If there are no generators just return the empty buffer
        let generators_len = self.generators.len();
        if generators_len == 0 {
            return;
        }

        // Run the generators
        self.generators
            .iter_mut()
            .for_each(|generator| generator.run(output));

        // Remove the ones that are finished
        self.generators.retain(|generator| !generator.finished);

        // Calculate the inverse so we can multiply instead of divide which is more efficient
        let buffer_len_inv = 1.0 / generators_len as f32;

        // Divide the generators by the current samples
        output.iter_mut().for_each(|tone| *tone *= buffer_len_inv);
    }

    /// Retrieve an oscillator buffer or create it when it doesn't exist yet.
    fn oscillator_buffer(
        &mut self,
        frequency: usize,
        duty_cycle: DutyCycle,
        oscillator_type: OscillatorType,
    ) -> RefCell<Vec<Vector2>> {
        match self
            .oscillator_lookup
            .get(&(frequency, duty_cycle, oscillator_type))
        {
            // A buffer was already cached, return it
            Some(buffer) => RefCell::clone(buffer),
            // Nothing is found, cache a new buffer of frequencies
            None => {
                // Build a lookup table and wrap it in a refcell so there can be multiple immutable
                // references to it
                let lut = RefCell::new(oscillator_type.build_lut(
                    frequency,
                    duty_cycle,
                    self.sample_rate,
                ));

                // Clone it so it can be returned after the original object is inserted
                let cloned_ref = RefCell::clone(&lut);

                // Add the new lookup table to the cache
                self.oscillator_lookup
                    .insert((frequency, duty_cycle, oscillator_type), lut);

                cloned_ref
            }
        }
    }
}

impl Default for Mixer {
    /// The default sample rate is 44100.
    fn default() -> Self {
        Self {
            sample_rate: 44_100,
            generators: vec![],
            oscillator_lookup: HashMap::new(),
        }
    }
}
