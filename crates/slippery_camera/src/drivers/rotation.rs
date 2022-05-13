use crate::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::prelude::*;

/// Directly sets the rotation of the camera
#[derive(Default, Debug)]
pub struct Rotation {
    pub rotation: Basis,
}

impl Rotation {
    pub fn new(rotation: Basis) -> Self {
        Self { rotation }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: params.parent.origin,
            basis: self.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
