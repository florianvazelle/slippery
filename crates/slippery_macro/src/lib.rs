#[macro_export]
macro_rules! deg2rad {
    ($l:expr) => { $l * 180.0 / std::f32::consts::PI };
}

#[macro_export]
macro_rules! rad2deg {
    ($l:expr) => { $l * std::f32::consts::PI / 180.0 };
}