use crate::effect::Effect;

use gdnative::prelude::*;

/// A simple distortion effect.
#[derive(Debug)]
pub struct Distortion {
    /// Overdrive that adds hard clipping.
    crunch: f32,
    /// Overdrive with soft clipping.
    drive: f32,
}

impl Distortion {
    /// Setup the effect.
    pub fn new(crunch: f32, drive: f32) -> Self {
        let crunch = 1.0 - crunch.max(0.01);

        Self { crunch, drive }
    }
}

impl Effect for Distortion {
    /// Apply the effect on the buffer.
    ///
    /// Algorithm from: https://github.com/amsynth/amsynth
    fn apply(&mut self, buffer: &mut [Vector2], _offset: usize) {
        buffer.iter_mut().for_each(|tone| {
            let a = *tone * self.drive;
            let x_sign = a.x.signum();
            let y_sign = a.y.signum();
            // Make negative numbers positive, apply the power function and make them negative
            // again
            *tone = Vector2::new(
                (a.x * x_sign).powf(self.crunch).min(1.0) * x_sign,
                (a.y * y_sign).powf(self.crunch).min(1.0) * y_sign,
            );
        });
    }
}
