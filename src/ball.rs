use gdnative::api::*;
use gdnative::prelude::*;

use std::f32::consts::PI;
use std::ops::Mul;

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Ball {
    #[property]
    cam_ball: Option<Ref<Camera>>,
    #[property(default = 5.0)]
    rotate_speed: f32,
}

#[methods]
impl Ball {
    fn new(_owner: &RigidBody) -> Self {
        Ball {
            cam_ball: None,
            rotate_speed: 5.0,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &RigidBody) {
        // Find child camera.
        for child in owner.get_children().iter() {
            match child.try_to_object::<Camera>() {
                Ok(camera) => {
                    self.cam_ball = Some(camera);
                    godot_print!("Camera found");
                    break;
                }
                Err(_) => continue,
            }
        }

        // Activate physics process method.
        owner.set_physics_process(true);
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, _delta: f32) {
        let up = Vector3::new(0.0, 1.0, 0.0);
        let mut dir = Vector3::new(0.0, 0.0, 0.0);

        let camera = self.cam_ball.as_ref().expect("No camera");
        let cam_xform = unsafe {
            camera
                .assume_safe()
                .call("get_global_transform", &[])
                .try_to::<Transform>()
                .expect("Unable to retrieve camera transform")
        };

        // TODO : use `fn _input(&self, owner: &Node, event: Ref<InputEvent>);` instead.
        let input = Input::godot_singleton();
        dir += -cam_xform
            .basis
            .c()
            .mul(Input::get_action_strength(input, "ui_up", false) as f32);
        dir += cam_xform
            .basis
            .c()
            .mul(Input::get_action_strength(input, "ui_down", false) as f32);
        dir += -cam_xform
            .basis
            .a()
            .mul(Input::get_action_strength(input, "ui_left", false) as f32);
        dir += cam_xform
            .basis
            .a()
            .mul(Input::get_action_strength(input, "ui_right", false) as f32);

        let target_dir = dir - up * dir.dot(up);
        let target_axis = target_dir.rotated(up, PI / 2.0);

        owner.add_torque(target_axis.mul(self.rotate_speed));
    }
}
