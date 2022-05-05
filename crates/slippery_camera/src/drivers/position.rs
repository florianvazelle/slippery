use crate::{driver::RigDriver, rig::RigUpdateParams};

use gdnative::prelude::*;

/// Directly sets the position of the camera
#[derive(Default, Debug)]
pub struct Position {
    pub position: Vector3,
}

impl Position {
    ///
    pub fn new(position: Vector3) -> Self {
        Self { position }
    }

    /// Add the specified vector to the position of this component
    pub fn translate(&mut self, move_vec: Vector3) {
        self.position += move_vec;
    }
}

impl RigDriver for Position {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            origin: self.position,
            basis: params.parent.basis,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
