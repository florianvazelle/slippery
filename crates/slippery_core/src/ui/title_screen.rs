use gdnative::prelude::*;

use crate::game_state;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TitleScreen;

#[methods]
impl TitleScreen {
    fn new(_owner: &Node) -> Self {
        TitleScreen
    }

    /// Start a new game.
    ///
    /// Godot signals triggered by the "New Game" button.
    #[export]
    fn on_newgame_button_pressed(&self, owner: &Node) {
        // Reset game state.
        let game_state =
            game_state::load_game_state(owner).expect("Failed to get game state instance");

        game_state
            .map_mut(|gs, o| gs.reset(&o))
            .expect("Could not reset game state");

        // Load the Game scene.
        let tree = owner.get_tree().unwrap();
        let tree = unsafe { tree.assume_safe() };
        tree.change_scene("res://scenes/Game.tscn")
            .expect("Game could not be loaded");
    }

    /// Exit game.
    ///
    /// Godot signals triggered by the "Quit Game" button.
    #[export]
    fn on_quitgame_button_pressed(&self, owner: &Node) {
        let tree = owner.get_tree().expect("Couldn't find scene tree!");
        let tree = unsafe { tree.assume_safe() };
        tree.quit(-1);
    }
}
