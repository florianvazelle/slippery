use gdnative::api::*;
use gdnative::prelude::*;

use crate::ball_states::Grounded;
use crate::ball_resource::BallResource;
use crate::ball_state::BallState;

#[derive(Debug)]
pub struct Stunned {
    push_direction: Vector3
}

impl Stunned {
    pub fn new(push_direction: Vector3) -> Self {
        Stunned { push_direction }
    }
}

impl BallState for Stunned {
    fn enter(&self, owner: &RigidBody, ball_resource: &mut BallResource) {
        // if Anim {
        //     Anim.SetBool("Stunned", true);
        // }

        ball_resource.stun_timer = ball_resource.stunned_time;

        // Set physics
        ball_resource.act_speed = 0.0;
        ball_resource.downward_direction = Vector3::UP;

        owner.set_linear_velocity(Vector3::ZERO);
        owner.add_central_force(self.push_direction * ball_resource.stun_push_back);

        // Turn on gravity
        owner.set_gravity_scale(1.0);
    }

    fn update(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        None
    }

    fn physics_update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Reduce stun timer
        if ball_resource.stun_timer > 0.0 {
            ball_resource.stun_timer -= delta;

            if ball_resource.stun_timer > ball_resource.stunned_time * 0.5 {
                return None;
            }
        }

        // Reduce ground timer
        if ball_resource.check_ground(owner) {
            // if Anim {
            //     Anim.SetBool("Stunned", false);
            // }

            // Get up from ground
            if ball_resource.stun_timer <= 0.0 {
                return Some(Box::new(Grounded::new()));
            }
        }

        let transform = owner.global_transform();
        
        // Lerp mesh slower when not on ground
        ball_resource.rotate_self(owner, ball_resource.downward_direction, delta, 8.0);
        ball_resource.rotate_mesh(owner, delta, -transform.basis.c(), ball_resource.turn_speed);

        // Push backwards while we fall
        let mut velocity = owner.linear_velocity();
        let mut fall_dir = transform.basis.c() * 4.0;

        fall_dir.y = velocity.y;
        velocity = velocity.linear_interpolate(fall_dir, delta * 2.0);
        owner.set_linear_velocity(velocity);

        // Falling audio
        // Visuals.WindAudioSetting(delta, velocity.length());
        None
    }

    fn integrate_forces(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) {}
}
