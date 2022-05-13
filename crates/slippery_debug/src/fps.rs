use gdnative::api::Engine;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]
pub struct Fps;

#[methods]
impl Fps {
    fn new(_owner: &Label) -> Self {
        Fps
    }

    #[export]
    fn _process(&self, owner: &Label, _delta: f64) {
        let engine = Engine::godot_singleton();

        let template = GodotString::from("FPS {0}");
        let data = VariantArray::new();
        data.push(engine.get_frames_per_second());

        let formatted = template.format(&data.into_shared().to_variant());
        owner.set_text(formatted);
    }
}
