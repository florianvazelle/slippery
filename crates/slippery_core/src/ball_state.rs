use gdnative::api::*;

use crate::ball_resource::BallResource;

/// Virtual base class for all states.
pub trait BallState: std::any::Any + std::fmt::Debug {
    /// Virtual function. Called by the state machine upon changing the active state.
    fn enter(&self, owner: &RigidBody, ball_resource: &mut BallResource);

    /// Virtual function. Corresponds to the `_process()` callback.
    fn update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>>;
    
    /// Virtual function. Corresponds to the `_physics_process()` callback.
    fn physics_update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> ;
    
    /// Virtual function. Corresponds to the `_integrate_forces()` callback.
    fn integrate_forces(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32);
}

// fn pre_update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) -> Option<Box<dyn BallState + Sync + Send>> {
//     // Control the animator
//     AnimCtrl();

//     transform.position = Rigid.position;
// }

// fn pre_physics_update(&self, owner: &RigidBody, ball_resource: &mut BallResource, delta: f32) {
//     // Tick any fixed camera controls
//     // FixedAnimCtrl(delta);

//     // Control our direction slightly when falling
//     let _xMov = InputHand.Horizontal;
//     let _zMov = InputHand.Vertical;

//     // Get our direction of input based on camera position
//     let screenMovementForward = CamY.transform.forward;
//     let screenMovementRight = CamY.transform.right;
//     let screenMovementUp = CamY.transform.up;

//     let h = screenMovementRight * _xMov;
//     let v = screenMovementForward * _zMov;

//     ball_resource.move_direction = (v + h).normalized();
// }
