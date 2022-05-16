use gdnative::api::*;
use gdnative::prelude::*;

use crate::ball_states::{Stunned, Grounded};
use crate::ball_resource::BallResource;
use crate::ball_state::BallState;

use lerp::Lerp;

#[derive(Debug)]
pub struct Flying {}

impl Flying {
    pub fn new() -> Self {
        Flying {}
    }
}

impl BallState for Flying {
    fn enter(&self, owner: &RigidBody, ball_resource: &mut BallResource) {
        // Set animation 
        ball_resource.flying_timer = ball_resource.glide_time;
        
        // Our gravity is returned to the flying amount
        ball_resource.act_grav_amt = 0.0; 

        ball_resource.flown_adjustment_lerp = -1.0;

        // Camera set flying state
        // CamFol.SetFlyingState(1);

        // Turn off gravity
        owner.set_gravity_scale(0.0);
    }

    fn update(&self, owner: &RigidBody, ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        let transform = owner.global_transform();

        // Reduce air timer 
        if ball_resource.action_air_timer > 0.0 { 
            return None;
        }

        // Check wall collision for a crash, if this unit can crash
        // If we have hit a wall
        if ball_resource.check_wall(owner) {
            // If we are going fast enough to crash into a wall
            if ball_resource.act_speed > ball_resource.speed_limit_before_crash {
                // Stun character
                return Some(Box::new(Stunned::new(transform.basis.c())));
            }
        }

        // Check for ground if we are not holding the flying button
        if !ball_resource.input_fly {
            if ball_resource.check_ground(owner) {
                return Some(Box::new(Grounded::new()));
            }
        }

        None
    }

    fn physics_update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Setup gliding
        if !ball_resource.input_fly {
            // Reduce flying timer 
            if ball_resource.flying_timer > 0.0 {
                ball_resource.flying_timer -= delta;
            }
        } else if ball_resource.flying_timer < ball_resource.glide_time {
            // Flapping animation
            if ball_resource.flying_timer < ball_resource.glide_time * 0.8 {
                // Anim.SetTrigger("Flap");
            }

            ball_resource.flying_timer = ball_resource.glide_time;
        }

        // Reduce air timer 
        if ball_resource.action_air_timer > 0.0 {
            ball_resource.action_air_timer -= delta;
        }

        // Falling effect
        // Visuals.FallEffectCheck(delta);

        // Falling audio
        // Visuals.WindAudioSetting(delta, owner.linear_velocity().length());

        // Lerp controls
        if ball_resource.flying_adjustement_lerp < 1.1 {
            ball_resource.flying_adjustement_lerp += delta * ball_resource.flying_adjustement_speed;
        }

        // Lerp speed
        let y_amt = owner.linear_velocity().y;
        let mut fly_accel = ball_resource.flying_acceleration * ball_resource.flying_adjustement_lerp;
        let mut spd = ball_resource.flying_speed;
        
        // We are not holding fly, slow down
        if !ball_resource.input_fly {
            spd = ball_resource.flying_min_speed; 
            if ball_resource.act_speed > ball_resource.flying_min_speed {
                fly_accel = ball_resource.flying_decelleration * ball_resource.flying_adjustement_lerp;
            }
        } else {
            // Flying effects 
            // Visuals.FlyingFxTimer(delta);
        }

        ball_resource.handle_velocity(delta, spd, fly_accel, y_amt);

        // Flying controls
        // self.control(delta, ball_resource.act_speed, _xMov, _zMov);
        None
    }

    fn integrate_forces(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) {
        let transform = owner.global_transform();
        
        // Input direction 
        let invert_x = -1.0;
        let invert_y = -1.0;

        // Horizontal inputs
        let x_move = ball_resource.input_horizontal * invert_x;
        // Vertical inputs 
        let z_move = ball_resource.input_vertical * invert_y;

        // Get direction to move character
        ball_resource.downward_direction = ball_resource.flying_downward_direction(owner, delta, z_move);
        let slide_dir = ball_resource.flying_side_direction(owner, delta, x_move);

        // Get our rotation and adjustment speeds
        let rot_spd = ball_resource.flying_rotation_speed;
        let fly_lerp_spd = ball_resource.flying_adjustement_speed * ball_resource.flying_adjustement_lerp;

        // Lerp mesh slower when not on ground
        ball_resource.rotate_self(owner, ball_resource.downward_direction, delta, rot_spd);
        ball_resource.rotate_mesh(owner, delta, slide_dir, rot_spd);

        // Lerp to velocity if not flying
        if ball_resource.flying_timer < ball_resource.glide_time * 0.7 {
            ball_resource.rotate_to_velocity(owner, delta, rot_spd * 0.05);
        }

        let target_velocity = -transform.basis.c() * ball_resource.act_speed;

        // Push down more when not pressing fly
        if ball_resource.input_fly {
            ball_resource.act_grav_amt = ball_resource.act_grav_amt.lerp(ball_resource.flying_gravity_amt, ball_resource.flying_gravity_build_speed * 4.0 * delta);
        } else {
            ball_resource.act_grav_amt = ball_resource.act_grav_amt.lerp(ball_resource.glide_gravity_amt, ball_resource.flying_gravity_build_speed * 0.5 * delta);
        }

        let target_velocity = target_velocity - (Vector3::UP * ball_resource.act_grav_amt);
        
        // Lerp velocity
        owner.set_linear_velocity(
            owner.linear_velocity()
                .linear_interpolate(target_velocity, delta * fly_lerp_spd)
        );
    }
}
