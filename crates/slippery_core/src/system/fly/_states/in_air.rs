use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::state_machine::*;
use crate::godot_state::*;
use crate::system::fly::{
    states::{Flying, Grounded},
    context::FlyContext
};

use lerp::Lerp;

// Define the `InAir` unit-like struct. We also derive several standard traits,
// since those are required on any struct we want to use as a machine state.
/// State for when the ball is in the air/falling.
#[derive(Debug)]
pub struct InAir;

// Implement the `State` marker trait for our new struct, which allows us to use
// the struct as a valid state in our state machine.
impl State for InAir {}

// We also implement the `InitialState` marker trait for the `InAir` state.
// This allows us to use this state when initialising a new machine.
impl InitialState for InAir {}

impl GodotState for InAir {
    type Context = FlyContext;

    fn init(&self, context: &mut Self::Context) {
        // context.resource.on_ground = false;
        context.resource.floor_timer = context.resource.grounded_timer_before_jump;
        context.resource.action_air_timer = 0.2;

        // Camera reset flying state
        // CamFol.SetFlyingState(0);

        // Turn on gravity
        context.owner.set_gravity_scale(1.0);
    }

    fn update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        // Reduce air timer 
        if context.resource.action_air_timer > 0.0 {
            return None;
        }
        
        // Cannot switch to flying until jump is done
        // if context.resource.has_jumped { 
        //     return None;
        // }

        // Switch to flying
        if context.resource.input_fly {  
            return Some(Box::new(Flying::new()));
        }

        // Check for ground
        if context.resource.check_ground(context.owner) {
            return Some(Box::new(Grounded::new()));
        }

        None
    }

    fn physics_update(&self, context: &mut Self::Context) -> Option<Box<dyn State>> {
        // Reduce air timer 
        if context.resource.action_air_timer > 0.0 {
            context.resource.action_air_timer -= context.delta;
        }

        // Falling effect
        // Visuals.FallEffectCheck(context.delta);

        // Falling audio
        // Visuals.WindAudioSetting(context.delta, context.owner.linear_velocity().length());

        // Slow our flying control if we were not
        if context.resource.flying_adjustement_lerp > -0.1 {
            context.resource.flying_adjustement_lerp -= context.delta * (context.resource.flying_adjustement_speed * 0.5);
        }

        // Control our character when falling
        // self.control(context.delta, context.resource.act_speed, context.resource.air_acceleration, move_direction);
        None
    }

    fn integrate_forces(&self, context: &mut Self::Context) {
        let transform = context.owner.global_transform();
        let velocity = context.owner.linear_velocity();

        let mut target_dir = context.resource.move_direction;
        if context.resource.move_direction == Vector3::ZERO {
            target_dir = -transform.basis.c();
        }

        //rotate towards the rigid body velocity 
        let mut lerp_direction = context.resource.downward_direction;
        let mut fall_dir_spd = context.resource.falling_direction_speed;

        // We are going downwards
        if velocity.y < -6.0 {
            lerp_direction = Vector3::UP;
            fall_dir_spd *= -(velocity.y * 0.2);
        }         

        context.resource.downward_direction = context.resource.downward_direction.linear_interpolate(lerp_direction, fall_dir_spd * context.delta);

        // Lerp mesh slower when not on ground
        context.resource.rotate_self(context.owner, context.resource.downward_direction, context.delta, 8.0);
        context.resource.rotate_mesh(context.owner, context.delta, -transform.basis.c(), context.resource.turn_speed_in_air);

        // Move character
        let target_velocity = target_dir * context.resource.act_speed;

        // Lerp our acceleration
        context.resource.act_accel = context.resource.act_accel.lerp(context.resource.air_acceleration, context.resource.handle_return_speed * context.delta);
        
        // Set rigid direction
        let mut dir = velocity.linear_interpolate(target_velocity, context.delta * context.resource.act_accel);
        dir.y = velocity.y;

        context.owner.set_linear_velocity(dir);
    }
}
