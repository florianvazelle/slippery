use gdnative::api::*;
use gdnative::prelude::*;

use lerp::Lerp;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct FlyResource {
    /// How quickly our handle on our character is returned to normal after a force is added (such as jumping
    #[property(default = 2.0)]
    pub handle_return_speed: f32, 
    /// The actual gravity that is applied to our character
    pub act_grav_amt: f32, 
    /// How long we are on the floor
    pub floor_timer: f32, 
    /// The air timer counting our current actions performed in air
    pub action_air_timer: f32,

    /// Max speed for basic movement
    #[property(default = 15.0)]
    pub max_speed: f32,
    /// Max possible speed
    #[property(default = 50.0)]
    pub speed_clamp: f32, 
    /// Our actual acceleration
    pub act_accel: f32, 
    /// How quickly we build speed
    #[property(default = 4.0)]
    pub acceleration: f32, 
    /// How quickly we adjust to new speeds
    #[property(default = 20.0)]
    pub movement_acceleration: f32,   
    /// How quickly we slow down
    #[property(default = 2.0)]
    pub slow_down_acceleration: f32, 
    /// How quickly we turn on the ground
    #[property(default = 2.0)]
    pub turn_speed: f32, 
    /// If we have flown this will be reset at 0, and effect turn speed on the ground
    pub flown_adjustment_lerp: f32, 
    /// Our actual speed
    pub act_speed: f32, 

    // Where to move to
    pub move_direction: Vector3,
    pub downward_direction: Vector3, 

    /// How quickly we adjust to new speeds when in air
    #[property(default = 5.0, path = "falling/air/acceleration")]
    pub air_acceleration: f32,  
    /// 
    #[property(default = 2.0, path = "falling/air/turn_speed")]
    pub turn_speed_in_air: f32,
    /// How quickly we will return to a normal direction
    #[property(default = 0.5, path = "falling/direction_speed")]
    pub falling_direction_speed: f32,

    /// How much influence our direction relative to the camera will influence our flying
    #[property(default = 2.0, path = "flying/direction_speed")]
    pub flying_direction_speed: f32,  
    /// How fast we turn in air overall
    #[property(default = 6.0, path = "flying/rotation_speed")]
    pub flying_rotation_speed: f32, 
    /// How fast we rotate up and down
    #[property(default = 0.1, path = "flying/up_down_speed")]
    pub flying_up_down_speed: f32, 
    /// How fast we rotate left and right
    #[property(default = 0.1, path = "flying/left_right_speed")]
    pub flying_left_right_speed: f32,  
    /// How fast we roll
    #[property(default = 0.1, path = "flying/roll_speed")]
    pub flying_roll_speed: f32, 

    /// How much we accelerate to max speed
    #[property(default = 4.0, path = "flying/acceleration")]
    pub flying_acceleration: f32, 
    /// How quickly we slow down when flying
    #[property(default = 1.0, path = "flying/decelleration")]
    pub flying_decelleration: f32, 
    /// Our max flying speed
    #[property(default = 0.0, path = "flying/speed")]
    pub flying_speed: f32,
    /// Our flying slow down speed
    #[property(default = 0.0, path = "flying/min_speed")]
    pub flying_min_speed: f32, 

    /// How quickly our velocity adjusts to the flying speed
    #[property(default = 0.0, path = "flying/adjustement_speed")]
    pub flying_adjustement_speed: f32,
    /// The lerp for our adjustment amount 
    #[property(default = 0.0, path = "flying/adjustement_lerp")]
    pub flying_adjustement_lerp: f32,

    /// How much gravity will pull us down when flying
    #[property(default = 2.0, path = "flying/physics/gravity_amt")]
    pub flying_gravity_amt: f32, 
    /// How much gravity affects us when just gliding
    #[property(default = 4.0, path = "flying/physics/glide_gravity_amt")]
    pub glide_gravity_amt: f32, 
    /// How much our gravity is lerped when stopping flying
    #[property(default = 3.0, path = "flying/physics/gravity_build_speed")]
    pub flying_gravity_build_speed: f32, 

    /// How much velocity we gain for flying downwards
    #[property(default = 2.0, path = "flying/physics/velocity_gain")]
    pub flying_velocity_gain: f32, 
    /// How much velocity we lose for flying upwards
    #[property(default = 1.0, path = "flying/physics/velocity_loss")]
    pub flying_velocity_loss: f32, 
    /// How much we fly down before a boost
    #[property(default = -6.0, path = "flying/physics/lower_limit")]
    pub flying_lower_limit: f32, 
    /// How much we fly up before a boost;
    #[property(default = 4.0, path = "flying/physics/upper_limit")]
    pub flying_upper_limit: f32,
    /// How long we glide for when not flying before we start to fall
    #[property(default = 10.0, path = "flying/physics/glide_time")]
    pub glide_time: f32, 

    /// How fast we have to be going to crash
    #[property(default = 18.0, path = "impact/speed_limit_before_crash")]
    pub speed_limit_before_crash: f32, 
    /// How much we are pushed back
    #[property(default = 50.0, path = "impact/stun_push_back")]
    pub stun_push_back: f32,  
    /// How long we are stunned for
    #[property(default = 0.25, path = "impact/stunned_time")]
    pub stunned_time: f32,
    /// The in use stun timer
    pub stun_timer: f32,
    
    /// How long we have to be on the floor before an action can be made
    #[property(default = 0.2)]
    pub grounded_timer_before_jump: f32, 
    /// The time before the animation stops flying
    pub flying_timer: f32, 

    pub input_fly: bool,
    pub input_up: bool,
    pub input_down: bool,
    pub input_right: bool,
    pub input_left: bool,
    pub input_horizontal: f32,
    pub input_vertical: f32,
    pub body_collision: Option<Ref<Node>>
} 

#[methods]
impl FlyResource {
    fn new(_owner: &Resource) -> Self {
        FlyResource {
            handle_return_speed: 0.0, 
            act_grav_amt: 0.0, 
            floor_timer: 0.0, 
            action_air_timer: 0.0,
            max_speed: 15.0,
            speed_clamp: 50.0, 
            act_accel: 0.0, 
            acceleration: 4.0, 
            movement_acceleration: 20.0,   
            slow_down_acceleration: 2.0, 
            turn_speed: 2.0, 
            flown_adjustment_lerp: 0.0, 
            act_speed: 0.0, 
            move_direction: Vector3::new(0.0, 0.0, 0.0),
            downward_direction: Vector3::new(0.0, 0.0, 0.0),
            air_acceleration: 5.0,  
            turn_speed_in_air: 2.0,
            falling_direction_speed: 0.5,
            flying_direction_speed: 2.0,  
            flying_rotation_speed: 6.0, 
            flying_up_down_speed: 0.1, 
            flying_left_right_speed: 0.1,  
            flying_roll_speed: 0.1, 
            flying_acceleration: 4.0, 
            flying_decelleration: 4.0, 
            flying_speed: 0.0,
            flying_min_speed: 0.0, 
            flying_adjustement_speed: 0.0,
            flying_adjustement_lerp: 0.0,
            flying_gravity_amt: 2.0, 
            glide_gravity_amt: 4.0, 
            flying_gravity_build_speed: 3.0, 
            flying_velocity_gain: 2.0, 
            flying_velocity_loss: 1.0, 
            flying_lower_limit: -6.0, 
            flying_upper_limit: 4.0,
            glide_time: 10.0, 
            speed_limit_before_crash: 0.0, 
            stun_push_back: 0.0,  
            stunned_time: 0.0,
            stun_timer: 0.0,
            grounded_timer_before_jump: 0.2,
            flying_timer: 0.0,
            input_fly: false,
            input_up: false,
            input_down: false,
            input_right: false,
            input_left: false,
            input_horizontal: 0.0,
            input_vertical: 0.0,
            body_collision: None,
        }
    }

    pub(crate) fn check_ground(&self, owner: &RigidBody) -> bool {
        let transform = owner.global_transform();
        transform.origin.y <= 0.0
    }

    /// Check if there is a wall to crash into
    pub(crate) fn check_wall(&self, _owner: &RigidBody) -> bool {
        false
    }

    /// Lerp our speed over time
    pub(crate) fn lerp_speed(&mut self, delta: f32, target_speed: f32, acceleration: f32) {
        // If our speed is larger than our max speed, reduce it slowly 
        if self.act_speed > self.max_speed {
            self.act_speed = self.act_speed.lerp(target_speed, delta * acceleration * 0.5);
        } else {
            if target_speed > 0.5 {
                // Influence by x and y input 
                let degree = Vector3::new(self.input_horizontal, self.input_vertical, 0.0).normalized().length();
                self.act_speed = self.act_speed.lerp(target_speed, (delta * acceleration) * degree);
            } else {
                self.act_speed = self.act_speed.lerp(target_speed, delta * acceleration);
            }
        }
        // Clamp our speed
        self.act_speed = self.act_speed.clamp(0.0, self.speed_clamp);
    }

    /// Handle how our speed is increased or decreased when flying
    pub(crate) fn handle_velocity(&mut self, delta: f32, target_speed: f32, acceleration: f32, y_amt: f32) {
        let mut target_speed = target_speed;
        let mut acceleration = acceleration;
        
        // We are over out max speed, slow down slower
        if self.act_speed > self.flying_speed {
            acceleration = acceleration * 0.8;
        }

        // We are flying down!
        if y_amt < self.flying_lower_limit {
            // Boost speed
            target_speed = target_speed + (self.flying_velocity_gain * (y_amt * -0.5));
        } else if y_amt > self.flying_upper_limit {
            // Reduce speed
            target_speed = target_speed - (self.flying_velocity_loss * y_amt);
            self.act_speed -= (self.flying_velocity_loss * y_amt) * delta;
        }

        // Clamp speed
        target_speed = target_speed.clamp(-self.speed_clamp, self.speed_clamp);
        
        // Lerp speed
        self.act_speed = self.act_speed.lerp(target_speed, acceleration * delta);
    }

    pub(crate) fn flying_downward_direction(&self, owner: &RigidBody, delta: f32, z_move: f32) -> Vector3 {
        let transform = owner.global_transform();
        let mut direction = -transform.basis.b();

        // up and down input = moving up and down (this effects our downward direction
        if z_move > 0.1 {
            // Upward tilt
            direction = direction.linear_interpolate(transform.basis.c(), delta * (self.flying_up_down_speed * z_move));
        } else if z_move < -0.1 {
            // Downward tilt
            direction = direction.linear_interpolate(-transform.basis.c(), delta * (self.flying_up_down_speed * (z_move * -1.0)));
        }

        // LB and RB input = roll (this effects our downward direction
        if self.input_left {
            // Lleft roll
            direction = direction.linear_interpolate(-transform.basis.a(), delta * self.flying_roll_speed);
        } else if self.input_right {
            // Right roll
            direction = direction.linear_interpolate(transform.basis.a(), delta * self.flying_roll_speed);
        }

        direction
    }

    pub(crate) fn flying_side_direction(&self, owner: &RigidBody, delta: f32, x_move: f32) -> Vector3 {
        let transform = owner.global_transform();
        let mut roll_direction = -transform.basis.c();

        // rb lb = move left and right (this effects our target direction)
        // Left right input
        if x_move > 0.1 {
            roll_direction = roll_direction.linear_interpolate(-transform.basis.a(), delta * (self.flying_left_right_speed * x_move));
        } else if x_move < -0.1 {
            roll_direction = roll_direction.linear_interpolate(transform.basis.a(), delta * (self.flying_left_right_speed * (x_move * -1.0)));
        }

        // Bumper input
        if self.input_left {
            roll_direction = roll_direction.linear_interpolate(-transform.basis.a(), delta * self.flying_left_right_speed * 0.2);
        } else if self.input_right {
            roll_direction = roll_direction.linear_interpolate(transform.basis.a(), delta * self.flying_left_right_speed * 0.2);
        }

        roll_direction
    }

    /// Rotate our upwards direction
    pub(crate) fn rotate_self(&self, owner: &RigidBody, direction: Vector3, delta: f32, gravity_spd: f32) {
        let mut transform = owner.global_transform();
        let _lerp_dir = transform.basis.b().linear_interpolate(direction, delta * gravity_spd);
        transform.basis = transform.basis.rotated(transform.basis.b(), delta); //lerp_dir);
        owner.set_transform(transform);
    }
    
    /// Rotate our left right direction
    pub(crate) fn rotate_mesh(&self, owner: &RigidBody, delta: f32, look_dir: Vector3, spd: f32) {
        let mut transform = owner.global_transform();
        let slerp_rot = transform.looking_at(look_dir, transform.basis.b()).basis.to_quat();
        transform.basis = Basis::from_quat(transform.basis.to_quat().slerp(slerp_rot, spd * delta));
        owner.set_transform(transform);
    }
    
    /// Rotate towards the velocity direction
    pub(crate) fn rotate_to_velocity(&self, owner: &RigidBody, delta: f32, spd: f32) {
        let mut transform = owner.global_transform();
        let slerp_rot = transform.looking_at(owner.linear_velocity().normalized(), Vector3::UP).basis.to_quat();
        transform.basis = Basis::from_quat(transform.basis.to_quat().slerp(slerp_rot, spd * delta));
        owner.set_transform(transform);
    }
}