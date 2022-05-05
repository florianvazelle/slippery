use crate::{
    driver::RigDriver,
    rig::RigUpdateParams,
    utils::{ExpSmoothed, ExpSmoothingParams},
};

use gdnative::prelude::*;

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, and made to look ahead of it.
#[derive(Debug)]
pub struct LookAt {
    /// Exponential smoothing factor
    pub smoothness: f32,

    /// The world-space position to look at
    pub target: Vector3,

    // The scale with which smoothing should be applied to the target position
    output_offset_scale: f32,

    smoothed_target: ExpSmoothed<Vector3>,
}

impl LookAt {
    ///
    pub fn new(target: Vector3) -> Self {
        Self {
            smoothness: 0.0,
            output_offset_scale: 1.0,
            target,
            smoothed_target: Default::default(),
        }
    }

    /// Set the exponential smoothing factor for target position tracking.
    pub fn tracking_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    /// Reverse target position smoothing, causing the camera to look ahead of it.
    /// This can then be chained with [`Smooth`], to create
    /// a camera that smoothly follows an object, but doesn't lag far behind it.
    ///
    /// [`Smooth`]: struct.Smooth.html
    pub fn tracking_predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive { -1.0 } else { 1.0 };
        self
    }
}

impl RigDriver for LookAt {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        let target = self.smoothed_target.exp_smooth_towards(
            &self.target,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds,
            },
        );

        let forward = (target - params.parent.origin).normalized();
        
        let right = forward.cross(Vector3::UP).normalized();
        let up = right.cross(forward);
        let basis = Basis::from_basis_vectors(right, up, -forward);

        Transform {
            origin: params.parent.origin,
            basis,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
