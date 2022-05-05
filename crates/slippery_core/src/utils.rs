use std::any::Any;
use std::any::TypeId;

pub fn is<T: ?Sized + Any, K: ?Sized + Any>(_s: &K) -> bool {
    TypeId::of::<T>() == TypeId::of::<K>()
}