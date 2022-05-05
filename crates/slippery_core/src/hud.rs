use gdnative::prelude::*;

use crate::game::set_pause_mode;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Hud {
    active: bool,
}

#[methods]
impl Hud {
    fn new(_owner: &Node) -> Self {
        Hud { active: false }
    }

    pub fn toggle(&mut self, owner: &Node) {
        if self.active {
            self.hide(owner);
        } else {
            self.show(owner);
        }
    }

    pub fn show(&mut self, owner: &Node) {
        self.active = true;

        // Pause game.
        set_pause_mode(owner, true);

        let button = unsafe { owner.get_node_as::<Button>("ContinueParty").unwrap() };
        button.show();
        let button = unsafe { owner.get_node_as::<Button>("QuitParty").unwrap() };
        button.show();
    }

    pub fn hide(&mut self, owner: &Node) {
        self.active = false;

        let button = unsafe { owner.get_node_as::<Button>("ContinueParty").unwrap() };
        button.hide();
        let button = unsafe { owner.get_node_as::<Button>("QuitParty").unwrap() };
        button.hide();

        // Unpause game.
        set_pause_mode(owner, false);
    }

    /// Hide HUD to allow user to continue his party.
    ///
    /// Godot signals triggered by the "Continue" button.
    #[export]
    fn on_continueparty_button_pressed(&mut self, owner: &Node) {
        self.hide(owner);
    }

    /// Exit the party and load the title screen scene.
    ///
    /// Godot signals triggered by the "Quit Party" button.
    #[export]
    fn on_quitparty_button_pressed(&self, owner: &Node) {
        // Load the TitleScreen scene.
        let tree = owner.get_tree().unwrap();
        let tree = unsafe { tree.assume_safe() };
        tree.change_scene("res://scenes/TitleScreen.tscn")
            .expect("TitleScreen could not be loaded");

        // Unpause game.
        set_pause_mode(owner, false);
    }
}
