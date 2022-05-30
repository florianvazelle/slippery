use slippery_core::*;
use slippery_debug::*;

use gdnative::prelude::{godot_init, InitHandle};

/// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<fps::Fps>();
    handle.add_class::<system::fly::resource::FlyResource>();
    handle.add_class::<ball::Ball>();
    handle.add_class::<game::Game>();
    handle.add_class::<game_state::GameState>();
    handle.add_class::<ui::hud::Hud>();
    handle.add_class::<ui::title_screen::TitleScreen>();
}

// Macros that create the entry-points of the dynamic library.
godot_init!(init);
