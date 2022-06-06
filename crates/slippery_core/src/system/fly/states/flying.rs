use gdnative::api::{Camera, RigidBody};
use gdnative::prelude::*;

use std::ops::Mul;

use lerp::Lerp;
use sm_gd::*;

use crate::system::fly::resource::FlyResource;
use crate::system::fly::states::Stunned;

#[derive(Debug)]
pub struct Flying;

impl GodotState for Flying {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource) {
        // Set animation 
        resource.flying_timer = resource.glide_time;
        
        // Our gravity is returned to the flying amount
        resource.act_grav_amt = 0.0; 
        resource.flown_adjustment_lerp = -1.0;

        // Turn off gravity
        owner.set_gravity_scale(0.0);
    }

    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        let transform = owner.global_transform();

        // Reduce air timer 
        if resource.action_air_timer > 0.0 { 
            return None;
        }

        // // Check wall collision for a crash, if this unit can crash
        // // If we have hit a wall
        // if resource.check_wall(owner) {
        //     // If we are going fast enough to crash into a wall
        //     if resource.act_speed > resource.speed_limit_before_crash {
        //         // Stun character
        //         return Some(Box::new(Stunned { push_direction: transform.basis.c() }));
        //     }
        // }

        // // Check for ground if we are not holding the flying button
        // if !resource.input_fly {
        //     if resource.check_ground(owner) {
        //         return Some(Box::new(Stunned { push_direction: transform.basis.c() }));
        //     }
        // }

        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        // Setup gliding
        if !resource.input_fly {
            // Reduce flying timer 
            if resource.flying_timer > 0.0 {
                resource.flying_timer -= delta;
            }
        } else if resource.flying_timer < resource.glide_time {
            resource.flying_timer = resource.glide_time;
        }

        // Reduce air timer 
        if resource.action_air_timer > 0.0 {
            resource.action_air_timer -= delta;
        }

        // Lerp controls
        if resource.flying_adjustement_lerp < 1.1 {
            resource.flying_adjustement_lerp += delta * resource.flying_adjustement_speed;
        }

        // Lerp speed
        let y_amt = owner.linear_velocity().y;
        let mut fly_accel = resource.flying_acceleration * resource.flying_adjustement_lerp;
        let mut spd = resource.flying_speed;
        
        // We are not holding fly, slow down
        if !resource.input_fly {
            spd = resource.flying_min_speed; 
            if resource.act_speed > resource.flying_min_speed {
                fly_accel = resource.flying_decelleration * resource.flying_adjustement_lerp;
            }
        }

        resource.handle_velocity(delta, spd, fly_accel, y_amt);

        None
    }

    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {
        let transform = owner.global_transform();
        
        // Input direction 
        let invert_x = -1.0;
        let invert_y = -1.0;

        // Horizontal inputs
        let x_move = resource.input_horizontal * invert_x;
        // Vertical inputs 
        let z_move = resource.input_vertical * invert_y;

        // Get direction to move character
        resource.downward_direction = resource.flying_downward_direction(owner, delta, z_move);
        let slide_dir = resource.flying_side_direction(owner, delta, x_move);

        // Get our rotation and adjustment speeds
        let rot_spd = resource.flying_rotation_speed;
        let fly_lerp_spd = resource.flying_adjustement_speed * resource.flying_adjustement_lerp;

        // Lerp mesh slower when not on ground
        resource.rotate_self(owner, resource.downward_direction, delta, rot_spd);
        resource.rotate_mesh(owner, delta, slide_dir, rot_spd);

        // Lerp to velocity if not flying
        if resource.flying_timer < resource.glide_time * 0.7 {
            resource.rotate_to_velocity(owner, delta, rot_spd * 0.05);
        }

        let target_velocity = -transform.basis.c() * resource.act_speed;

        // Push down more when not pressing fly
        if resource.input_fly {
            resource.act_grav_amt = resource.act_grav_amt.lerp(resource.flying_gravity_amt, resource.flying_gravity_build_speed * 4.0 * delta);
        } else {
            resource.act_grav_amt = resource.act_grav_amt.lerp(resource.glide_gravity_amt, resource.flying_gravity_build_speed * 0.5 * delta);
        }

        let target_velocity = target_velocity - (Vector3::UP * resource.act_grav_amt);
        
        // Lerp velocity
        owner.set_linear_velocity(
            owner.linear_velocity()
                .linear_interpolate(target_velocity, delta * fly_lerp_spd)
        );
    }
}
