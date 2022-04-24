use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TitleScreen;

#[methods]
impl TitleScreen {
    fn new(_owner: &Node) -> Self {
        TitleScreen
    }

    /// Load the Game scene.
    #[export]
    fn on_newgame_button_pressed(&self, owner: &Node) {
        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://scenes/Game.tscn")
                .expect("Game could not be loaded");
        }
    }

    /// Exit game.
    #[export]
    fn on_quitgame_button_pressed(&self, owner: &Node) {
        let tree = owner.get_tree().expect("Couldn't find scene tree!");
        let tree = unsafe { tree.assume_safe() };
        tree.quit(-1);
    }
}
