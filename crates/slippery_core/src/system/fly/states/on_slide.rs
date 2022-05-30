use gdnative::api::{Camera, RigidBody};
use gdnative::prelude::*;

use std::f32::consts::PI;
use std::ops::Mul;

use crate::sm::*;
use crate::sm_godot::GodotState;
use crate::system::fly::resource::FlyResource;
use crate::system::fly::states::Flying;

// Define the `OnSlide` unit-like struct. We also derive several standard traits,
// since those are required on any struct we want to use as a machine state.
/// State for when the ball is in the air/falling.
#[derive(Debug)]
pub struct OnSlide;

// Implement the `State` marker trait for our new struct, which allows us to use
// the struct as a valid state in our state machine.
impl State for OnSlide {}

// We also implement the `InitialState` marker trait for the `OnSlide` state.
// This allows us to use this state when initialising a new machine.
impl InitialState for OnSlide {}

impl GodotState for OnSlide {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {}

    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {       
        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {
        let up = Vector3::UP;
        let mut dir = Vector3::ZERO;

        let cam_xform = unsafe {
            owner
                .get_node_as::<Camera>("CamBall")
                .unwrap()
                .global_transform()
        };

        // TODO : use `fn _input(&self, owner: &Node, event: Ref<InputEvent>);` instead.
        let input = Input::godot_singleton();
        dir += -cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_up", false) as f32);
        dir += cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_down", false) as f32);
        dir += -cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_left", false) as f32);
        dir += cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_right", false) as f32);

        let target_dir = dir - up * dir.dot(up);
        let target_axis = target_dir.rotated(up, PI / 2.0);

        owner.add_torque(target_axis.mul(resource.turn_speed));

        // Switch to flying
        if resource.input_fly {  
            return Some(Box::new(Flying));
        }
        
        None
    }

    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {}
}
