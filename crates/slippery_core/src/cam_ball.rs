use gdnative::api::{Camera, PhysicsServer, RigidBody};
use gdnative::prelude::*;

use slippery_camera::prelude::*;
use slippery_macro::deg2rad;

use crate::utils::is;

#[derive(NativeClass)]
#[inherit(Camera)]
pub struct CameraBall {
    collision_exception: VariantArray,
    #[property(default = 0.5)]
    min_distance: f32,
    #[property(default = 4.0)]
    max_distance: f32,
    #[property(default = 0.0)]
    angle_v_adjust: f32,
    #[property(default = 25.0)]
    autoturn_ray_aperture: f32,
    #[property(default = 50.0)]
    autoturn_speed: f32,
    max_height: f32,
    min_height: f32,

    rig: CameraRig,
}

#[methods]
impl CameraBall {
    fn new(_owner: &Camera) -> Self {
        CameraBall {
            collision_exception: VariantArray::new_shared(),
            min_distance: 0.5,
            max_distance: 4.0,
            angle_v_adjust: 0.0,
            autoturn_ray_aperture: 25.0,
            autoturn_speed: 50.0,
            max_height: 2.0,
            min_height: 1.0,
            rig: CameraRig::builder()
                .with(Position::new(Vector3::ZERO))
                // .with(Rotation::new(Basis::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(Vector3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(2.5))
                .with(YawPitch::new())
                .with(
                    LookAt::new(Vector3::ZERO + Vector3::UP)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Camera) {
        // We retrieve all RigidBody parents of the camera,
        // to ignore them during the calculation of intersection.
        let collision_exception = VariantArray::new();
        let mut parent = owner.get_parent();
        while parent.is_some() {
            let parent_node: TRef<Node> = unsafe { parent.unwrap().assume_safe() };
            if is::<RigidBody, _>(&parent_node) {
                collision_exception.push(parent_node.cast::<RigidBody>().unwrap().get_rid());
                break;
            } else {
                parent = parent_node.get_parent();
            }
        }
        self.collision_exception = collision_exception.into_shared();

        owner.set_physics_process(true);
        owner.set_as_toplevel(true);
    }

    #[export]
    fn _physics_process(&mut self, owner: &Camera, dt: f32) {
        // DEBUG
        // owner.print_tree_pretty();
        // godot_print!("{}", owner.get_path().to_string());

        // TODO : use get_parent_spatial instead.
        if let Some(ball) = owner.get_parent_spatial() {
            let ball_transform = unsafe { ball.assume_safe().global_transform() };

            let target = ball_transform.origin;
            let pos = owner.global_transform().origin;
            let up = Vector3::UP;

            let mut delta = pos - target;

            if delta.length() < self.min_distance {
                delta = delta.normalized() * self.min_distance;
            } else if delta.length() > self.max_distance {
                delta = delta.normalized() * self.max_distance;
            }

            if delta.y > self.max_height {
                delta.y = self.max_height;
            }
            if delta.y < self.min_height {
                delta.y = self.min_height;
            }

            if let Some(world) = owner.get_world() {
                let world_space = unsafe { world.assume_safe().space() };

                let physics_server = unsafe { PhysicsServer::godot_singleton() };
                if let Some(ds) = unsafe { physics_server.space_get_direct_state(world_space) } {
                    let ds = unsafe { ds.assume_safe() };

                    let col_left = ds.intersect_ray(
                        target,
                        target
                            + Basis::from_axis_angle(up, deg2rad!(self.autoturn_ray_aperture))
                                .xform(delta),
                        self.collision_exception.new_ref(),
                        2147483647,
                        true,
                        false,
                    );
                    let col = ds.intersect_ray(
                        target,
                        target + delta,
                        self.collision_exception.new_ref(),
                        2147483647,
                        true,
                        false,
                    );
                    let col_right = ds.intersect_ray(
                        target,
                        target
                            + Basis::from_axis_angle(up, deg2rad!(-self.autoturn_ray_aperture))
                                .xform(delta),
                        self.collision_exception.new_ref(),
                        2147483647,
                        true,
                        false,
                    );

                    if !col.is_empty() {
                        if let Some(position) = col.get("position") {
                            let position = Vector3::from_variant(&position).unwrap();
                            delta = position - target
                        }
                    } else if !col_left.is_empty() && col_right.is_empty() {
                        delta = Basis::from_axis_angle(up, deg2rad!(-dt * self.autoturn_speed))
                            .xform(delta);
                    } else if col_left.is_empty() && !col_right.is_empty() {
                        delta = Basis::from_axis_angle(up, deg2rad!(dt * self.autoturn_speed))
                            .xform(delta);
                    }

                    if delta == Vector3::ZERO {
                        delta = (pos - target).normalized() * 0.0001;
                    }

                    self.rig.driver_mut::<Position>().position = target + delta;
                    self.rig
                        .driver_mut::<YawPitch>()
                        .rotate_yaw_pitch(self.angle_v_adjust, 0.0);
                    self.rig.driver_mut::<LookAt>().target = target;

                    owner.set_transform(self.rig.update(dt));
                }
            }
        }
    }
}