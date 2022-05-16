use gdnative::api::*;

use crate::ball_resource::BallResource;
use crate::ball_state::BallState;

#[derive(Debug)]
pub struct Fixed {}

impl Fixed {
    pub fn new() -> Self {
        Fixed {}
    }
}

impl BallState for Fixed {
    fn enter(&self, _owner: &RigidBody, _ball_resource: &mut BallResource) {}

    fn update(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        None
    }
    
    fn physics_update(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
        // Turn off air sound
        // Visuals.WindAudioSetting(delta * 3.0, 0.0);
        None
    }

    fn integrate_forces(&self, _owner: &RigidBody, _ball_resource: &mut BallResource, _delta: f32) {}
}
