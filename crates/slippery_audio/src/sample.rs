use crate::oscillator::{DutyCycle, OscillatorType};

/// Audio sample that procedurally generates it's sound.
///
/// Plug this into the [`Mixer`] object to play the sound.
///
/// ```rust
/// // Generate a sine wave at 2khz
/// let mut sine_wave = usfx::Sample::default();
/// sine_wave.osc_frequency(2000);
/// sine_wave.osc_type(usfx::OscillatorType::Sine);
///
/// // Add it to the mixer
/// let mut mixer = usfx::Mixer::default();
/// mixer.play(sine_wave);
///
/// // Plug it into a audio library, see the examples for a cpal & SDL2 implementation
/// // ...
/// // Call the generator to get a buffer for the audio library
/// # let mut buffer = [0.0];
/// mixer.generate(&mut buffer);
/// ```
///
/// [`Generator`]: struct.Generator.html
#[derive(Debug, Copy, Clone)]
pub struct Sample {
    pub(crate) volume: Option<f32>,
    pub(crate) osc_frequency: usize,
    pub(crate) osc_type: OscillatorType,
    pub(crate) osc_duty_cycle: DutyCycle,
    pub(crate) env_attack: f32,
    pub(crate) env_decay: f32,
    pub(crate) env_release: f32,
    pub(crate) env_sustain: f32,
    pub(crate) dis_crunch: Option<f32>,
    pub(crate) dis_drive: Option<f32>,
}

impl Default for Sample {
    /// The default is a sinewave of 441 hz.
    fn default() -> Self {
        Self {
            volume: None,
            osc_frequency: 441,
            osc_type: OscillatorType::Sine,
            osc_duty_cycle: DutyCycle::default(),
            env_attack: 0.01,
            env_decay: 0.1,
            env_sustain: 0.5,
            env_release: 0.5,
            dis_crunch: None,
            dis_drive: None,
        }
    }
}

impl Sample {
    /// Set the volume which is a multiplier of the result.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn volume(&mut self, volume: f32) -> &mut Self {
        self.volume = Some(volume);

        self
    }

    /// Set the frequency of the oscillator in hertz.
    ///
    /// When using the noise oscillator type this will be the seed.
    /// A range from 1-20000 is allowed.
    pub fn osc_frequency(&mut self, frequency: usize) -> &mut Self {
        self.osc_frequency = frequency;

        self
    }

    /// Set the type of the oscillator.
    ///
    /// See the [`OscillatorType`] enum for supported wave types.
    ///
    /// [`OscillatorType`]: enum.OscillatorType.html
    pub fn osc_type(&mut self, oscillator: OscillatorType) -> &mut Self {
        self.osc_type = oscillator;

        self
    }

    /// Set the length of the pulse, this only applies when you use a square wave.
    ///
    /// Changing of the duty cycle mainly results in a different timbre.
    /// A range from 0.0-1.0 will have results, other values won't do anything.
    pub fn osc_duty_cycle(&mut self, duty_cycle: DutyCycle) -> &mut Self {
        self.osc_duty_cycle = duty_cycle;

        self
    }

    /// Set the time until the first envelope slope reaches it's maximum height.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn env_attack(&mut self, attack: f32) -> &mut Self {
        self.env_attack = attack;

        self
    }

    /// Set the time it takes from the maximum height to go into the main plateau.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn env_decay(&mut self, decay: f32) -> &mut Self {
        self.env_decay = decay;

        self
    }

    /// Set the height of the main plateau.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn env_sustain(&mut self, sustain: f32) -> &mut Self {
        self.env_sustain = sustain;

        self
    }

    /// Set the time it takes to go from the end of the plateau to zero.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn env_release(&mut self, release: f32) -> &mut Self {
        self.env_release = release;

        self
    }

    /// Overdrive that adds hard clipping.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn dis_crunch(&mut self, crunch: f32) -> &mut Self {
        self.dis_crunch = Some(crunch);

        self
    }

    /// Overdrive with soft clipping.
    ///
    /// A range from 0.0-1.0 will result in proper behavior, but you can experiment with other
    /// values.
    pub fn dis_drive(&mut self, drive: f32) -> &mut Self {
        self.dis_drive = Some(drive);

        self
    }
}