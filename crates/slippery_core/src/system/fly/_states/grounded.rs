use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::state_machine::*;
use crate::godot_state::*;
use crate::system::fly::{
    states::Flying,
    context::FlyContext
};

use lerp::Lerp;

#[derive(Debug)]
pub struct Grounded;

impl State for Grounded {}

impl GodotState for Grounded {
    type Context = FlyContext;

    fn init(&self, context: &mut Self::Context) {
        // Visuals.Landing();

        // Reset wind animation
        // Visuals.SetFallingEffects(1.6);

        // Reset flying animation 
        context.resource.flying_timer = 0.0;

        // Reset flying adjustment
        context.resource.flying_adjustement_lerp = 0.0;

        // Reset physics and jumps
        context.resource.downward_direction = Vector3::UP;

        // Camera reset flying state
        // CamFol.SetFlyingState(0);

        // Turn on gravity
        context.owner.set_gravity_scale(1.0);
    }

    fn update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        None
    }

    fn physics_update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        // Turn off wind audio
        // if Visuals.WindLerpAmt > 0 {
        //     Visuals.WindAudioSetting(context.delta * 3.0, 0.0);
        // }

        let mut l_speed = context.resource.max_speed;
        let mut acceleration = context.resource.acceleration;

        // Reduce floor timer
        if context.resource.floor_timer > 0.0 {
            context.resource.floor_timer -= context.delta;
        }

        if context.resource.input_horizontal == 0.0 && context.resource.input_vertical == 0.0 {
            // We are not moving, lerp to a walk speed
            l_speed = 0.0;
            acceleration = context.resource.slow_down_acceleration;
        }
        
        // Lerp our current speed
        if context.resource.act_speed > l_speed - 0.5 || context.resource.act_speed < l_speed + 0.5 {
            context.resource.lerp_speed(context.delta, l_speed, acceleration);
        }
        
        // Move our character
        // self.control(context.delta, context.resource.act_speed, move_accel, move_direction);
        None
    }

    fn integrate_forces(&self, context: &mut Self::Context) {
        let transform = context.owner.global_transform();
        let velocity = context.owner.linear_velocity();
        
        let mut target_dir = context.resource.move_direction;
        if context.resource.move_direction == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        if target_dir == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        // Turn speed after flown is reduced
        if context.resource.flown_adjustment_lerp < 1.0 {
            context.resource.flown_adjustment_lerp += context.delta * 2.0;
        }

        // Set our turn speed
        let turn_spd = (context.resource.turn_speed + (context.resource.act_speed * 0.1)) * context.resource.flown_adjustment_lerp;
        let turn_spd = turn_spd.clamp(0.0, 6.0);

        // Lerp mesh slower when not on ground
        context.resource.rotate_self(context.owner, context.resource.downward_direction, context.delta, 8.0);
        context.resource.rotate_mesh(context.owner, context.delta, target_dir, turn_spd);

        // Move character
        let mut spd = context.resource.act_speed;
        let move_dir = transform.basis.c();

        // If we are not pressing a move input we move towards velocity, or are crouching
        if context.resource.move_direction == Vector3::ZERO {
            // Less speed is applied to our character
            spd = context.resource.act_speed * 0.8; 
        }

        let target_velocity = move_dir * spd;
        
        // Accelerate our character
        context.resource.act_accel = context.resource.act_accel.lerp(context.resource.movement_acceleration, context.resource.handle_return_speed * context.delta);
        
        // Lerp our movement direction
        let mut dir = velocity.linear_interpolate(target_velocity, context.delta * context.resource.act_accel);   
        dir.y = velocity.y;
        
        // Set our rigibody direction
        context.owner.set_linear_velocity(dir);
    }
}
