use gdnative::api::*;
use gdnative::prelude::*;

use crate::ball_resource::BallResource;
use crate::ball_state::BallState;
use crate::ball_states::Fixed;

use std::f32::consts::PI;
use std::ops::Mul;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Ball {
    #[property(default = 5.0)]
    rotate_speed: f32,
    #[property]
    ball_resource: Option<Instance<BallResource>>,

    state: Box<dyn BallState + Sync + Send>,
}

#[methods]
impl Ball {
    fn new(_owner: &RigidBody) -> Self {
        Ball {
            rotate_speed: 5.0,
            ball_resource: None,
            state: Box::new(Fixed::new()),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        // Activate physics process method.
        owner.set_physics_process(true);

        if let Some(ball_resource) = self.ball_resource.as_ref() {
            let ball_resource = unsafe { ball_resource.assume_safe() };
            ball_resource.map_mut(|s, _o| self.state.enter(owner, s)).unwrap();
        }
    }

    #[export]
    fn _process(&mut self, owner: &RigidBody, delta: f32) {
        if let Some(ball_resource) = self.ball_resource.as_ref() {
            let ball_resource = unsafe { ball_resource.assume_safe() };
            let new_state = ball_resource.map_mut(|s, _o| self.state.update(owner, s, delta)).unwrap();
            if let Some(new_state) = new_state {
                godot_print!("Go to {:?} state!\n", new_state);
                self.state = new_state;
            }
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f32) {
        let up = Vector3::UP;
        let mut dir = Vector3::ZERO;

        let cam_xform = unsafe {
            owner
                .get_node_as::<Camera>("CamBall")
                .unwrap()
                .global_transform()
        };

        // TODO : use `fn _input(&self, owner: &Node, event: Ref<InputEvent>);` instead.
        let input = Input::godot_singleton();
        dir += -cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_up", false) as f32);
        dir += cam_xform.basis.c().mul(Input::get_action_strength(input, "ui_down", false) as f32);
        dir += -cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_left", false) as f32);
        dir += cam_xform.basis.a().mul(Input::get_action_strength(input, "ui_right", false) as f32);

        let target_dir = dir - up * dir.dot(up);
        let target_axis = target_dir.rotated(up, PI / 2.0);

        owner.add_torque(target_axis.mul(self.rotate_speed));

        let gamepad = input.get_vector("ui_down", "ui_up", "ui_left", "ui_right", -1.0);
        if let Some(ball_resource) = self.ball_resource.as_ref() {
            let ball_resource = unsafe { ball_resource.assume_safe() };

            let new_state = ball_resource.map_mut(|s, _o| {
                s.input_fly = input.is_action_pressed("ui_accept", false);
                s.input_vertical = gamepad.x;
                s.input_horizontal = gamepad.y;

                // Control our direction slightly when falling
                let _x_mov = s.input_horizontal;
                let _z_mov = s.input_vertical;

                // Get our direction of input based on camera position
                let screen_movement_forward = -cam_xform.basis.c();
                let screen_movement_right = cam_xform.basis.a();
                // let screen_movement_up = cam_xform.basis.b();

                let h = screen_movement_right * _x_mov;
                let v = screen_movement_forward * _z_mov;

                s.move_direction = (v + h).normalized();
                self.state.physics_update(owner, s, delta)
            }).unwrap();

            if let Some(new_state) = new_state {
                godot_print!("Go to {:?} state!\n", new_state);
                self.state = new_state;
            }
        }
    }

    #[export]
    fn _integrate_forces(&self, owner: &RigidBody, state: Ref<PhysicsDirectBodyState>) {
        if let Some(ball_resource) = self.ball_resource.as_ref() {
            let ball_resource = unsafe { ball_resource.assume_safe() };
            let state = unsafe { state.assume_safe() };
            ball_resource.map_mut(|s, _o| self.state.integrate_forces(owner, s, state.step() as f32)).unwrap();
        }
    }

    // fn AnimCtrl() {
    //     // Setup the location of any velocity based animations from our hip position 
    //     let RelPos = self.transform;

    //     // Find animations based on our hip position (for flying velocity animations
    //     if HipsPos {
    //         RelPos = HipsPos;
    //     }

    //     // Get movement amounts in each direction
    //     Vector3 RelVel = RelPos.transform.InverseTransformDirection(Rigid.velocity);
    //     Anim.SetFloat("forwardMove", RelVel.z);
    //     Anim.SetFloat("sideMove", RelVel.x);
    //     Anim.SetFloat("upwardsMove", RelVel.y);
        
    //     // Our rigidbody y amount (for upwards or downwards velocity animations
    //     Anim.SetFloat("YVel", Rigid.velocity.y);

    //     // Set movement animator
    //     self.run_timer = Vector3::new(Rigid.velocity.x, 0, Rigid.velocity.z).magnitude;
    //     Anim.SetFloat("Moving", RunTimer);

    //     // Set our grounded and flying animations
    //     Anim.SetBool("OnGround", OnGround);
    //     Anim.SetBool("Flying", InputHand.Fly);
    // }

    // /// Animations involving a timer
    // fn FixedAnimCtrl(&self, D: f32) {
    //     // Setup the xinput animation for tilting our wings left and right
    //     let LAMT = InputHand.Horizontal;
    //     XAnimFloat = Mathf.Lerp(XAnimFloat, LAMT, D * 4.0);
    //     Anim.SetFloat("XInput", XAnimFloat);
    // } 
}
