use std::any::Any;
use std::any::TypeId;
use std::f32::consts::PI;

pub fn is<T: ?Sized + Any, K: ?Sized + Any>(_s: &K) -> bool {
    TypeId::of::<T>() == TypeId::of::<K>()
}

pub fn deg2rad(angle: f32) -> f32 {
    angle * 180.0 / PI
}
