use gdnative::api::RigidBody;
use gdnative::prelude::*;

use crate::sm::*;
use crate::sm_godot::GodotState;
use crate::system::fly::resource::FlyResource;

#[derive(Debug)]
pub struct Landing;

impl State for Landing {}

impl GodotState for Landing {
    type Owner = RigidBody;
    type Resource = FlyResource;

    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {
    }

    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {
        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn StateTraits>> {
        None
    }

    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {
    }
}
