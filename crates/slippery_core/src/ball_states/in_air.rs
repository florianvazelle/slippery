use gdnative::api::*;
use gdnative::prelude::*;

use crate::ball_states::{Flying, Grounded};
use crate::ball_resource::BallResource;
use crate::ball_state::BallState;

use lerp::Lerp;

/// State for when the ball is in the air/falling.
#[derive(Debug)]
pub struct InAir {}

impl InAir {
    ///
    pub fn new() -> Self {
        InAir {}
    }
}

impl BallState for InAir {
    fn enter(&self, owner: &RigidBody, ball_resource: &mut BallResource) {
        // ball_resource.on_ground = false;
        ball_resource.floor_timer = ball_resource.grounded_timer_before_jump;
        ball_resource.action_air_timer = 0.2;

        // Camera reset flying state
        // CamFol.SetFlyingState(0);

        // Turn on gravity
        owner.set_gravity_scale(1.0);
    }

    fn update(&self, owner: &RigidBody, ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Reduce air timer 
        if ball_resource.action_air_timer > 0.0 {
            return None;
        }
        
        // Cannot switch to flying until jump is done
        // if ball_resource.has_jumped { 
        //     return None;
        // }

        // Switch to flying
        if ball_resource.input_fly {  
            return Some(Box::new(Flying::new()));
        }

        // Check for ground
        if ball_resource.check_ground(owner) {
            return Some(Box::new(Grounded::new()));
        }

        None
    }

    fn physics_update(&self, _owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Reduce air timer 
        if ball_resource.action_air_timer > 0.0 {
            ball_resource.action_air_timer -= delta;
        }

        // Falling effect
        // Visuals.FallEffectCheck(delta);

        // Falling audio
        // Visuals.WindAudioSetting(delta, owner.linear_velocity().length());

        // Slow our flying control if we were not
        if ball_resource.flying_adjustement_lerp > -0.1 {
            ball_resource.flying_adjustement_lerp -= delta * (ball_resource.flying_adjustement_speed * 0.5);
        }

        // Control our character when falling
        // self.control(delta, ball_resource.act_speed, ball_resource.air_acceleration, move_direction);
        None
    }

    fn integrate_forces(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) {
        let transform = owner.global_transform();
        let velocity = owner.linear_velocity();

        let mut target_dir = ball_resource.move_direction;
        if ball_resource.move_direction == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        //rotate towards the rigid body velocity 
        let mut lerp_direction = ball_resource.downward_direction;
        let mut fall_dir_spd = ball_resource.falling_direction_speed;

        // We are going downwards
        if velocity.y < -6.0 {
            lerp_direction = Vector3::UP;
            fall_dir_spd *= -(velocity.y * 0.2);
        }         

        ball_resource.downward_direction = ball_resource.downward_direction.linear_interpolate(lerp_direction, fall_dir_spd * delta);

        // Lerp mesh slower when not on ground
        ball_resource.rotate_self(owner, ball_resource.downward_direction, delta, 8.0);
        ball_resource.rotate_mesh(owner, delta, -transform.basis.c(), ball_resource.turn_speed_in_air);

        // Move character
        let target_velocity = target_dir * ball_resource.act_speed;

        // Lerp our acceleration
        ball_resource.act_accel = ball_resource.act_accel.lerp(ball_resource.air_acceleration, ball_resource.handle_return_speed * delta);
        
        // Set rigid direction
        let mut dir = velocity.linear_interpolate(target_velocity, delta * ball_resource.act_accel);
        dir.y = velocity.y;

        owner.set_linear_velocity(dir);
    }
}
