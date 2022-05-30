use gdnative::api::RigidBody;

use crate::sm::*;
use crate::sm_godot::GodotState;
use crate::system::fly::resource::FlyResource;

// Create a new `FlyMachine` struct, which will be our state machine
// object. It keeps track of the `State`.
#[derive(Debug)]
pub struct FlyMachine {
    pub state: Box<dyn StateTraits>,
}

// Implement the `Machine` trait, allowing us to query the current `state() and
// the `trigger()` event that caused the current state of the machine.
impl Machine for FlyMachine {
    fn state(&self) -> &Box<dyn StateTraits> {
        &self.state
    }
}

// Implement the `Initializer` trait, to allow a new machine to be initialised
// using the `new` associated function, given a valid state marked with the
// `InitialState` marker trait.
impl Initializer for FlyMachine {
    fn new(state: impl InitialState) -> Self {
        FlyMachine { state: Box::new(state) }
    }
}

impl FlyMachine {
    pub fn state_mut(&mut self) -> &mut Box<dyn GodotState<Owner = RigidBody, Resource = FlyResource>> {
        self.try_state_mut().unwrap_or_else(|| {
            panic!("State not implement GodotState trait")
        })
    }

    pub fn try_state_mut(&mut self) -> Option<&mut Box<dyn GodotState<Owner = RigidBody, Resource = FlyResource>>> {
        self.state.as_mut().as_any_mut().downcast_mut::<Box<dyn GodotState<Owner = RigidBody, Resource = FlyResource>>>()
    }
}