use crate::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::prelude::*;

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    ///
    pub offset: Vector3,
}

impl Arm {
    ///
    pub fn new(offset: Vector3) -> Self {
        Self { offset }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: params.parent.origin + params.parent.basis * self.offset,
            basis: params.parent.basis,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
