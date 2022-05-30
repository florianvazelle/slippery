use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::state_machine::*;
use crate::godot_state::*;
use crate::system::fly::{
    states::{Grounded},
    context::FlyContext
};

#[derive(Debug)]
pub struct Stunned {
    push_direction: Vector3
}

impl State for Stunned {}

impl GodotState for Stunned {
    type Context = FlyContext;

    fn init(&self, context: &mut Self::Context) {
        // if Anim {
        //     Anim.SetBool("Stunned", true);
        // }

        context.resource.stun_timer = context.resource.stunned_time;

        // Set physics
        context.resource.act_speed = 0.0;
        context.resource.downward_direction = Vector3::UP;

        owner.set_linear_velocity(Vector3::ZERO);
        owner.add_central_force(self.push_direction * context.resource.stun_push_back);

        // Turn on gravity
        owner.set_gravity_scale(1.0);
    }

    fn update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        None
    }

    fn physics_update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        // Reduce stun timer
        if context.resource.stun_timer > 0.0 {
            context.resource.stun_timer -= context.delta;

            if context.resource.stun_timer > context.resource.stunned_time * 0.5 {
                return None;
            }
        }

        // Reduce ground timer
        if context.resource.check_ground(owner) {
            // if Anim {
            //     Anim.SetBool("Stunned", false);
            // }

            // Get up from ground
            if context.resource.stun_timer <= 0.0 {
                return Some(Box::new(Grounded::new()));
            }
        }

        let transform = owner.global_transform();
        
        // Lerp mesh slower when not on ground
        context.resource.rotate_self(owner, context.resource.downward_direction, context.delta, 8.0);
        context.resource.rotate_mesh(owner, context.delta, -transform.basis.c(), context.resource.turn_speed);

        // Push backwards while we fall
        let mut velocity = owner.linear_velocity();
        let mut fall_dir = transform.basis.c() * 4.0;

        fall_dir.y = velocity.y;
        velocity = velocity.linear_interpolate(fall_dir, context.delta * 2.0);
        owner.set_linear_velocity(velocity);

        // Falling audio
        // Visuals.WindAudioSetting(context.delta, velocity.length());
        None
    }

    fn integrate_forces(&self, context: &mut Self::Context) {}
}
