use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::sm::*;
use crate::sm_godot::GodotState;
use crate::system::fly::resource::FlyResource;

#[derive(Debug)]
pub struct Stunned {
    pub push_direction: Vector3
}

impl State for Stunned {}

impl GodotState for Stunned {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {
        resource.stun_timer = resource.stunned_time;

        // Set physics
        resource.act_speed = 0.0;
        resource.downward_direction = Vector3::UP;

        owner.set_linear_velocity(Vector3::ZERO);
        owner.add_central_force(self.push_direction * resource.stun_push_back);

        // Turn on gravity
        owner.set_gravity_scale(1.0);
    }

    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {
        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {
        // Reduce stun timer
        if resource.stun_timer > 0.0 {
            resource.stun_timer -= delta;

            if resource.stun_timer > resource.stunned_time * 0.5 {
                return None;
            }
        }

        let transform = owner.global_transform();
        
        // Lerp mesh slower when not on ground
        resource.rotate_self(owner, resource.downward_direction, delta, 8.0);
        resource.rotate_mesh(owner, delta, -transform.basis.c(), resource.turn_speed);

        // Push backwards while we fall
        let mut velocity = owner.linear_velocity();
        let mut fall_dir = transform.basis.c() * 4.0;

        fall_dir.y = velocity.y;
        velocity = velocity.linear_interpolate(fall_dir, delta * 2.0);
        owner.set_linear_velocity(velocity);

        None
    }

    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {}
}
