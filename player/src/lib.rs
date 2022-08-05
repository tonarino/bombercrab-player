use bomber_lib::{
    self,
    world::{Enemy, Object, Tile},
    Action, Player,
};
use bomber_macro::wasm_export;

/// Player struct. Can contain any arbitrary data, which will carry over between turns.
#[derive(Default)]
struct MyPlayer;

#[wasm_export]
impl Player for MyPlayer {
    fn act(
        &mut self,
        _surroundings: Vec<(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)>,
    ) -> Action {
        todo!("Observe your surroundings and return an Action to do this turn")
    }

    fn name(&self) -> String {
        todo!("Return your name within the 10 character limit")
    }

    fn team_name() -> String {
        todo!("Return your team name within the 20 character limit")
    }
}
