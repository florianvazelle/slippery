use gdnative::prelude::*;

use std::fmt::Debug;

/// Generic interface for effects.
pub trait Effect: Debug + Send {
    /// Apply the effect on the buffer.
    fn apply(&mut self, buffer: &mut [Vector2], offset: usize);
}
