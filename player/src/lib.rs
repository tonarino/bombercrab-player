use bomber_lib::{
    self,
    world::{Enemy, Object, Tile, Direction},
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
        // TODO - Observe your surroundings and return an Action to do this turn
        Action::Move(Direction::West)
    }

    fn name(&self) -> String {
        "noob".to_owned()
    }

    fn team_name() -> String {
        "noob team".to_owned()
    }
}
