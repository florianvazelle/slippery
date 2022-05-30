use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::state_machine::*;
use crate::godot_state::*;
use crate::system::fly::{
    states::{Grounded, Stunned},
    context::FlyContext
};

use lerp::Lerp;

#[derive(Debug)]
pub struct Flying;

impl State for Flying {}

impl GodotState for Flying {
    type Context = FlyContext;

    fn init(&self, context: &mut Self::Context) {
        // Set animation 
        context.resource.flying_timer = context.resource.glide_time;
        
        // Our gravity is returned to the flying amount
        context.resource.act_grav_amt = 0.0; 

        context.resource.flown_adjustment_lerp = -1.0;

        // Camera set flying state
        // CamFol.SetFlyingState(1);

        // Turn off gravity
        context.owner.set_gravity_scale(0.0);
    }

    fn update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        let transform = context.owner.global_transform();

        // Reduce air timer 
        if context.resource.action_air_timer > 0.0 { 
            return None;
        }

        // Check wall collision for a crash, if this unit can crash
        // If we have hit a wall
        if context.resource.check_wall(context.owner) {
            // If we are going fast enough to crash into a wall
            if context.resource.act_speed > context.resource.speed_limit_before_crash {
                // Stun character
                return Some(Box::new(Stunned::new(transform.basis.c())));
            }
        }

        // Check for ground if we are not holding the flying button
        if !context.resource.input_fly {
            if context.resource.check_ground(context.owner) {
                return Some(Box::new(Grounded::new()));
            }
        }

        None
    }

    fn physics_update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        // Setup gliding
        if !context.resource.input_fly {
            // Reduce flying timer 
            if context.resource.flying_timer > 0.0 {
                context.resource.flying_timer -= context.delta;
            }
        } else if context.resource.flying_timer < context.resource.glide_time {
            // Flapping animation
            if context.resource.flying_timer < context.resource.glide_time * 0.8 {
                // Anim.SetTrigger("Flap");
            }

            context.resource.flying_timer = context.resource.glide_time;
        }

        // Reduce air timer 
        if context.resource.action_air_timer > 0.0 {
            context.resource.action_air_timer -= context.delta;
        }

        // Falling effect
        // Visuals.FallEffectCheck(context.delta);

        // Falling audio
        // Visuals.WindAudioSetting(context.delta, context.owner.linear_velocity().length());

        // Lerp controls
        if context.resource.flying_adjustement_lerp < 1.1 {
            context.resource.flying_adjustement_lerp += context.delta * context.resource.flying_adjustement_speed;
        }

        // Lerp speed
        let y_amt = context.owner.linear_velocity().y;
        let mut fly_accel = context.resource.flying_acceleration * context.resource.flying_adjustement_lerp;
        let mut spd = context.resource.flying_speed;
        
        // We are not holding fly, slow down
        if !context.resource.input_fly {
            spd = context.resource.flying_min_speed; 
            if context.resource.act_speed > context.resource.flying_min_speed {
                fly_accel = context.resource.flying_decelleration * context.resource.flying_adjustement_lerp;
            }
        } else {
            // Flying effects 
            // Visuals.FlyingFxTimer(context.delta);
        }

        context.resource.handle_velocity(context.delta, spd, fly_accel, y_amt);

        // Flying controls
        // self.control(context.delta, context.resource.act_speed, _xMov, _zMov);
        None
    }

    fn integrate_forces(&self, context: &mut Self::Context) {
        let transform = context.owner.global_transform();
        
        // Input direction 
        let invert_x = -1.0;
        let invert_y = -1.0;

        // Horizontal inputs
        let x_move = context.resource.input_horizontal * invert_x;
        // Vertical inputs 
        let z_move = context.resource.input_vertical * invert_y;

        // Get direction to move character
        context.resource.downward_direction = context.resource.flying_downward_direction(context.owner, context.delta, z_move);
        let slide_dir = context.resource.flying_side_direction(context.owner, context.delta, x_move);

        // Get our rotation and adjustment speeds
        let rot_spd = context.resource.flying_rotation_speed;
        let fly_lerp_spd = context.resource.flying_adjustement_speed * context.resource.flying_adjustement_lerp;

        // Lerp mesh slower when not on ground
        context.resource.rotate_self(context.owner, context.resource.downward_direction, context.delta, rot_spd);
        context.resource.rotate_mesh(context.owner, context.delta, slide_dir, rot_spd);

        // Lerp to velocity if not flying
        if context.resource.flying_timer < context.resource.glide_time * 0.7 {
            context.resource.rotate_to_velocity(context.owner, context.delta, rot_spd * 0.05);
        }

        let target_velocity = -transform.basis.c() * context.resource.act_speed;

        // Push down more when not pressing fly
        if context.resource.input_fly {
            context.resource.act_grav_amt = context.resource.act_grav_amt.lerp(context.resource.flying_gravity_amt, context.resource.flying_gravity_build_speed * 4.0 * context.delta);
        } else {
            context.resource.act_grav_amt = context.resource.act_grav_amt.lerp(context.resource.glide_gravity_amt, context.resource.flying_gravity_build_speed * 0.5 * context.delta);
        }

        let target_velocity = target_velocity - (Vector3::UP * context.resource.act_grav_amt);
        
        // Lerp velocity
        context.owner.set_linear_velocity(
            context.owner.linear_velocity()
                .linear_interpolate(target_velocity, context.delta * fly_lerp_spd)
        );
    }
}
