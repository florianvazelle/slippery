use crate::{
    driver::RigDriver,
    rig::RigUpdateParams,
    utils::{ExpSmoothed, ExpSmoothingParams},
};

use gdnative::prelude::*;

/// Smooths the parent transformation.
#[derive(Debug)]
pub struct Smooth {
    /// Exponential smoothing factor for the position
    pub origin_smoothness: f32,

    /// Exponential smoothing factor for the rotation
    pub basis_smoothness: f32,

    // The scale with which smoothing should be applied
    output_offset_scale: f32,

    smoothed_origin: ExpSmoothed<Vector3>,
    smoothed_basis: ExpSmoothed<Basis>,
}

impl Default for Smooth {
    fn default() -> Self {
        Self {
            origin_smoothness: 1.0,
            basis_smoothness: 1.0,
            output_offset_scale: 1.0,
            smoothed_origin: Default::default(),
            smoothed_basis: Default::default(),
        }
    }
}

impl Smooth {
    /// Only smooth position
    pub fn new_position(origin_smoothness: f32) -> Self {
        Self {
            origin_smoothness,
            basis_smoothness: 0.0,
            ..Default::default()
        }
    }

    /// Only smooth rotation
    pub fn new_rotation(basis_smoothness: f32) -> Self {
        Self {
            basis_smoothness,
            origin_smoothness: 0.0,
            ..Default::default()
        }
    }

    /// Smooth both position and rotation
    pub fn new_position_rotation(origin_smoothness: f32, basis_smoothness: f32) -> Self {
        Self {
            origin_smoothness,
            basis_smoothness,
            ..Default::default()
        }
    }

    /// Reverse the smoothing, causing the camera to look ahead of the parent transform
    ///
    /// This can be useful on top of [`Position`], and before another `Smooth`
    /// in the chain to create a soft yet responsive follower camera.
    ///
    /// [`Position`]: struct.Position.html
    /// [`Smooth`]: struct.Smooth.html
    pub fn predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive { -1.0 } else { 1.0 };
        self
    }
}

impl RigDriver for Smooth {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let origin = self.smoothed_origin.exp_smooth_towards(
            &params.parent.origin,
            ExpSmoothingParams {
                smoothness: self.origin_smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        let basis = self.smoothed_basis.exp_smooth_towards(
            &params.parent.basis,
            ExpSmoothingParams {
                smoothness: self.basis_smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        Transform { origin, basis }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
