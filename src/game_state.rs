use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameState {
    pub score: u16,
}

#[methods]
impl GameState {
    fn new(_owner: &Node) -> Self {
        GameState { score: 0 }
    }

    #[export]
    pub fn reset(&mut self, _owner: &Node) {
        self.score = 0;
    }

    #[export]
    fn score(&self, _owner: &Node) -> u16 {
        self.score
    }

    pub fn _increment_score(&mut self) {
        self.score += 1;
    }
}

pub fn load_game_state(node: &Node) -> Option<Instance<GameState, Unique>> {
    let tree = node.get_tree()?;
    let tree = unsafe { tree.assume_safe() };

    let root = tree.root()?;
    let root = unsafe { root.assume_safe() };

    let game_state_node = root.get_node("./GameState")?;
    let game_state_node = unsafe { game_state_node.assume_unique() };

    Instance::<GameState, _>::from_base(game_state_node)
}
