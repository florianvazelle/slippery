use gdnative::api::*;
use gdnative::prelude::*;

use crate::hud::Hud;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Game {}

#[methods]
impl Game {
    fn new(_owner: &Spatial) -> Self {
        Game {}
    }

    #[export]
    fn _input(&self, owner: &Spatial, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };

        // Toggle pause menu.
        if event.is_action_pressed("ui_cancel", false, false) {
            let hud = unsafe { owner.get_node_as_instance::<Hud>("Hud").unwrap() };
            hud.map_mut(|x, o| x.toggle(&*o))
                .expect("Unable to get hud");
        }
    }
}

/// Set the pause mode.
pub(crate) fn set_pause_mode(node: &Node, enable: bool) {
    let tree = node.get_tree().unwrap();
    let tree = unsafe { tree.assume_safe() };
    tree.set_pause(enable);
}
