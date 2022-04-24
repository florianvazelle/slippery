use gdnative::api::*;
use gdnative::prelude::*;

use crate::game_state;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Game {
    name: String,
}

#[methods]
impl Game {
    fn new(_owner: &Spatial) -> Self {
        Game {
            name: "Slippery".to_string(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: &Spatial) {
        // Reset game state.
        let game_state =
            game_state::load_game_state(owner).expect("Failed to get game state instance");

        game_state
            .map_mut(|gs, o| gs.reset(&o))
            .expect("Could not reset game state");

        godot_print!("{} is ready!", self.name);
    }

    #[export]
    unsafe fn _process(&self, _owner: &Spatial, delta: f64) {
        godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}
