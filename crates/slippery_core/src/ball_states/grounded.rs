use gdnative::api::*;
use gdnative::prelude::*;

use crate::ball_resource::BallResource;
use crate::ball_state::BallState;

use lerp::Lerp;

#[derive(Debug)]
pub struct Grounded {}

impl Grounded {
    pub fn new() -> Self {
        Grounded {}
    }
}

impl BallState for Grounded {
    fn enter(&self, owner: &RigidBody, ball_resource: &mut BallResource) {
        // Visuals.Landing();

        // Reset wind animation
        // Visuals.SetFallingEffects(1.6);

        // Reset flying animation 
        ball_resource.flying_timer = 0.0;

        // Reset flying adjustment
        ball_resource.flying_adjustement_lerp = 0.0;

        // Reset physics and jumps
        ball_resource.downward_direction = Vector3::UP;

        // Camera reset flying state
        // CamFol.SetFlyingState(0);

        // Turn on gravity
        owner.set_gravity_scale(1.0);
    }

    fn update(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        None
    }

    fn physics_update(&self, _owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Turn off wind audio
        // if Visuals.WindLerpAmt > 0 {
        //     Visuals.WindAudioSetting(delta * 3.0, 0.0);
        // }

        let mut l_speed = ball_resource.max_speed;
        let mut acceleration = ball_resource.acceleration;

        // Reduce floor timer
        if ball_resource.floor_timer > 0.0 {
            ball_resource.floor_timer -= delta;
        }

        if ball_resource.input_horizontal == 0.0 && ball_resource.input_vertical == 0.0 {
            // We are not moving, lerp to a walk speed
            l_speed = 0.0;
            acceleration = ball_resource.slow_down_acceleration;
        }
        
        // Lerp our current speed
        if ball_resource.act_speed > l_speed - 0.5 || ball_resource.act_speed < l_speed + 0.5 {
            ball_resource.lerp_speed(delta, l_speed, acceleration);
        }
        
        // Move our character
        // self.control(delta, ball_resource.act_speed, move_accel, move_direction);
        None
    }

    fn integrate_forces(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) {
        let transform = owner.global_transform();
        let velocity = owner.linear_velocity();
        
        let mut target_dir = ball_resource.move_direction;
        if ball_resource.move_direction == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        if target_dir == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        // Turn speed after flown is reduced
        if ball_resource.flown_adjustment_lerp < 1.0 {
            ball_resource.flown_adjustment_lerp += delta * 2.0;
        }

        // Set our turn speed
        let turn_spd = (ball_resource.turn_speed + (ball_resource.act_speed * 0.1)) * ball_resource.flown_adjustment_lerp;
        let turn_spd = turn_spd.clamp(0.0, 6.0);

        // Lerp mesh slower when not on ground
        ball_resource.rotate_self(owner, ball_resource.downward_direction, delta, 8.0);
        ball_resource.rotate_mesh(owner, delta, target_dir, turn_spd);

        // Move character
        let mut spd = ball_resource.act_speed;
        let move_dir = transform.basis.c();

        // If we are not pressing a move input we move towards velocity, or are crouching
        if ball_resource.move_direction == Vector3::ZERO {
            // Less speed is applied to our character
            spd = ball_resource.act_speed * 0.8; 
        }

        let target_velocity = move_dir * spd;
        
        // Accelerate our character
        ball_resource.act_accel = ball_resource.act_accel.lerp(ball_resource.movement_acceleration, ball_resource.handle_return_speed * delta);
        
        // Lerp our movement direction
        let mut dir = velocity.linear_interpolate(target_velocity, delta * ball_resource.act_accel);   
        dir.y = velocity.y;
        
        // Set our rigibody direction
        owner.set_linear_velocity(dir);
    }
}
