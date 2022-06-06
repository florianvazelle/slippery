use gdnative::api::{Camera, RigidBody};
use gdnative::prelude::*;

use std::f32::consts::PI;
use std::ops::Mul;

use lerp::Lerp;
use sm_gd::*;

use crate::system::fly::resource::FlyResource;
use crate::system::fly::states::Flying;

#[derive(Debug)]
pub struct OnSlide;

impl GodotInitialState for OnSlide {}

impl GodotState for OnSlide {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn input(&self, owner: &Self::Owner, resource: &mut Self::Resource, event: Ref<InputEvent>) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        let event = unsafe { event.assume_safe() };

        // Go to the Flying state when user click on Enter
        if event.is_action_pressed("ui_accept", false, false) {
            godot_print!("Go to Flying state");
            return Some(Box::new(Flying));
        }
        
        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        let up = Vector3::UP;
        let mut dir = Vector3::ZERO;

        let cam_xform = unsafe {
            owner
                .get_node_as::<Camera>("CamBall")
                .unwrap()
                .global_transform()
        };

        let input = Input::godot_singleton();
        dir += -cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_up", false) as f32);
        dir += cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_down", false) as f32);
        dir += -cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_left", false) as f32);
        dir += cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_right", false) as f32);

        resource.move_direction = dir; // TODO - remove

        let target_dir = dir - up * dir.dot(up);
        let target_axis = target_dir.rotated(up, PI / 2.0);

        owner.add_torque(target_axis.mul(resource.turn_speed));
        
        None
    }
}
