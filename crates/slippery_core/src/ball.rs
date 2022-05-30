use gdnative::api::*;
use gdnative::prelude::*;

use crate::sm::Initializer;
use crate::system::fly::{
    states::OnSlide,
    machine::FlyMachine,
    resource::FlyResource
};

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Ball {
    #[property(default = 5.0)]
    rotate_speed: f32,

    #[property]
    ball_resource: Instance<FlyResource>,

    state_machine: FlyMachine,
}

#[methods]
impl Ball {
    fn new(_owner: &RigidBody) -> Self {
        Ball {
            rotate_speed: 5.0,
            ball_resource: Instance::<FlyResource, Unique>::new().into_shared(),
            // Set default state to OnSlide
            state_machine: FlyMachine::new(OnSlide),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        // Activate physics process method
        owner.set_physics_process(true);

        // Activate contact monitor to receive collision signals
        owner.set_contact_monitor(true);

        // Initialize first state 
        let ball_resource = unsafe { self.ball_resource.assume_safe() };
        ball_resource.map_mut(|r, _o| self.state_machine.state_mut().init(owner, r, 0.0)).unwrap();
    }

    #[export]
    fn _process(&mut self, owner: &RigidBody, delta: f32) {
        let ball_resource = unsafe { self.ball_resource.assume_safe() };
        ball_resource.map_mut(|r, _o| self.state_machine.state_mut().update(owner, r, delta)).unwrap();
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f32) {
        let cam_xform = unsafe {
            owner
                .get_node_as::<Camera>("CamBall")
                .unwrap()
                .global_transform()
        };
        
        let input = Input::godot_singleton();
        let gamepad = input.get_vector("ui_down", "ui_up", "ui_left", "ui_right", -1.0);
        
        let ball_resource = unsafe { self.ball_resource.assume_safe() };
        ball_resource.map_mut(|r, _o| {
            r.input_fly = input.is_action_pressed("ui_accept", false);
            r.input_vertical = gamepad.x;
            r.input_horizontal = gamepad.y;

            // Control our direction slightly when falling
            let _x_mov = r.input_horizontal;
            let _z_mov = r.input_vertical;

            // Get our direction of input based on camera position
            let screen_movement_forward = -cam_xform.basis.c();
            let screen_movement_right = cam_xform.basis.a();
            // let screen_movement_up = cam_xform.basis.b();

            let h = screen_movement_right * _x_mov;
            let v = screen_movement_forward * _z_mov;

            r.move_direction = (v + h).normalized();
            self.state_machine.state_mut().physics_update(owner, r, delta)
        }).unwrap();
    }

    #[export]
    fn _integrate_forces(&mut self, owner: &RigidBody, physics_state: Ref<PhysicsDirectBodyState>) {
        let physics_state = unsafe { physics_state.assume_safe() };
        let ball_resource = unsafe { self.ball_resource.assume_safe() };

        ball_resource.map_mut(|r, _o| self.state_machine.state_mut().integrate_forces(owner, r, physics_state.step() as f32)).unwrap();
    }

    #[export]
    fn on_ball_body_entered(&self, _owner: &RigidBody, body: Ref<Node>) {
        let ball_resource = unsafe { self.ball_resource.assume_safe() };
        ball_resource.map_mut(|s, _o| s.body_collision = Some(body)).unwrap();
    }

    #[export]
    fn on_ball_body_existed(&self, _owner: &RigidBody, _body: Ref<Node>) {
        let ball_resource = unsafe { self.ball_resource.assume_safe() };
        ball_resource.map_mut(|s, _o| s.body_collision = None).unwrap();
    }
}
