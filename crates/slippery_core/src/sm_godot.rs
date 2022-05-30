use gdnative::prelude::*;

use crate::sm::StateTraits;

pub trait GodotStateContext {}

/// Virtual base class for all states.
pub trait GodotState: StateTraits {
    type Owner: GodotObject;
    type Resource: NativeClass;

    /// Virtual function. Called by the state machine upon changing the active state.
    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32);

    /// Virtual function. Corresponds to the `_process()` callback.
    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>>;
    
    /// Virtual function. Corresponds to the `_physics_process()` callback.
    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>>;
    
    /// Virtual function. Corresponds to the `_integrate_forces()` callback.
    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32);
}
