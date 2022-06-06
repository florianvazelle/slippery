use gdnative::api::RigidBody;

use sm_gd::*;
use crate::system::fly::resource::FlyResource;

// Create a new `FlyMachine` struct, which will be our state machine
// object. It keeps track of the `State`.
#[derive(Debug)]
pub struct FlyMachine {
    pub state: Box<dyn GodotStateTraits<Owner = RigidBody, Resource = FlyResource>>,
}

// Implement the `Machine` trait, allowing us to query the current `state() and
// the `trigger()` event that caused the current state of the machine.
impl GodotMachine for FlyMachine {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn state(&mut self, state: Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>) {
        self.state = state
    }
}

// Implement the `Initializer` trait, to allow a new machine to be initialised
// using the `new` associated function, given a valid state marked with the
// `InitialState` marker trait.
impl GodotInitializer for FlyMachine {
    type Owner = RigidBody;
    type Resource = FlyResource;
    
    fn new(state: impl GodotInitialState<Owner = RigidBody, Resource = FlyResource>) -> Self {
        FlyMachine { state: Box::new(state) }
    }
}