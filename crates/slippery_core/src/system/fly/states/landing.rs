use gdnative::api::RigidBody;
use gdnative::prelude::*;

use sm_gd::*;

use crate::system::fly::resource::FlyResource;

#[derive(Debug)]
pub struct Landing;

impl GodotState for Landing {
    type Owner = RigidBody;
    type Resource = FlyResource;
}
