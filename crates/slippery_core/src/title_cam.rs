use gdnative::api::*;
use gdnative::prelude::*;

use slippery_camera::prelude::*;

const RADIUS: f32 = 50.0;

#[derive(NativeClass)]
#[inherit(Camera)]
pub struct TitleCamera {
    #[property(default = 0.5)]
    rotation_speed: f32,
    current_angle: f32,
    rig: CameraRig,
}

#[methods]
impl TitleCamera {
    fn new(_owner: &Camera) -> Self {
        TitleCamera {
            rotation_speed: 0.5,
            current_angle: 0.0,
            rig: CameraRig::builder()
                .with(Position::new(Vector3::new(RADIUS, 15.0, RADIUS)))
                .with(LookAt::new(Vector3::new(0.0, 0.0, 0.0)))
                .build(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Camera) {
        owner.set_physics_process(true);
        owner.set_as_toplevel(true);
    }

    #[export]
    fn _physics_process(&mut self, owner: &Camera, dt: f32) {
        self.current_angle = (self.current_angle + dt * self.rotation_speed) % 180.0;
        self.rig.driver_mut::<Position>().position = Vector3::new(RADIUS * self.current_angle.cos(), 15.0, RADIUS * self.current_angle.sin());
        owner.set_transform(self.rig.update(dt));
    }
}
