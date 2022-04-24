mod ball;
mod cam_ball;
mod game;
mod game_state;
mod title_screen;
mod utils;

use gdnative::prelude::{godot_init, InitHandle};

/// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<ball::Ball>();
    handle.add_class::<cam_ball::CameraBall>();
    handle.add_class::<game::Game>();
    handle.add_class::<game_state::GameState>();
    handle.add_class::<title_screen::TitleScreen>();
}

// Macros that create the entry-points of the dynamic library.
godot_init!(init);
