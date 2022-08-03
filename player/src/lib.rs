use bomber_lib::{
    self,
    world::{Enemy, Object, Tile},
    Action, Player,
};
use bomber_macro::wasm_export;

#[derive(Default)]
struct Fool;

#[wasm_export]
impl Player for Fool {
    fn act(
        &mut self,
        _surroundings: Vec<(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)>,
    ) -> Action {
        todo!("Act on surroundings and return an Action to do")
    }

    fn name(&self) -> String {
        todo!("Return your name, mind character limit")
    }

    fn team_name() -> String {
        todo!("Return your team name, mind character limit")
    }
}
