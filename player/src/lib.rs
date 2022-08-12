use std::collections::HashSet;

use bomber_lib::{
    self,
    world::{Direction, Enemy, Object, Tile, TileOffset},
    Action, Player,
};
use bomber_macro::wasm_export;

#[derive(Default)]
struct MatejBot {
    rotation: usize,
    ticks: u32,
}

/// The `Player` implementation block must be decorated with `wasm_export`
/// in order to export the right shims to interface with the bevy `wasm` runtime
#[wasm_export]
impl Player for MatejBot {
    fn act(
        &mut self,
        surroundings: Vec<(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)>,
    ) -> Action {
        use Direction::*;
        let mut allowed_directions = Direction::all();
        allowed_directions.rotate_right(self.rotation % 4);
        let mut allowed_directions: HashSet<Direction> = allowed_directions.into_iter().collect();

        // Remove non-floors
        allowed_directions.retain(|direction| {
            let tile = surroundings
                .iter()
                .find(|s| s.3 == direction.extend(1))
                .expect("direction in surroundings");

            if tile.0 != Tile::Floor {
                return false;
            }
            if matches!(tile.1, Some(Object::Bomb { .. } | Object::Crate)) {
                return false;
            }
            true
        });

        // Remove bomb directions
        allowed_directions.retain(|direction| !bomb_in_direction(*direction, &surroundings));

        if bomb_at_offset(1, 1, &surroundings) {
            allowed_directions.remove(&North);
            allowed_directions.remove(&East);
        }
        if bomb_at_offset(1, -1, &surroundings) {
            allowed_directions.remove(&South);
            allowed_directions.remove(&East);
        }
        if bomb_at_offset(-1, 1, &surroundings) {
            allowed_directions.remove(&North);
            allowed_directions.remove(&West);
        }
        if bomb_at_offset(-1, -1, &surroundings) {
            allowed_directions.remove(&South);
            allowed_directions.remove(&West);
        }

        // for (_, object, _, TileOffset(x_offset, y_offset)) in surroundings.iter() {
        //     if !matches!(object, Some(Object::Bomb { .. })) {
        //         continue;
        //     }

        //     match (*x_offset, *y_offset) {
        //         (-1 | 0 | 1, y_offset) if y_offset >= 0 => allowed_directions.remove(&North),
        //         (-1 | 0 | 1, y_offset) if y_offset < 0 => allowed_directions.remove(&South),
        //         (x_offset, -1 | 0 | 1) if x_offset >= 0 => allowed_directions.remove(&East),
        //         (x_offset, -1 | 0 | 1) if x_offset < 0 => allowed_directions.remove(&West),
        //         // Nothing to do
        //         (_, _) => false,
        //     };
        // }

        // Drops a bomb every once in a while.
        let drop_bomb = crate_or_player_close(&surroundings);

        if (self.ticks as f64).log(2.0).fract().abs() < 0.001 {
            self.rotation += 1;
        }

        // Finalization
        self.ticks += 1;

        let direction = allowed_directions.into_iter().next();
        match (drop_bomb, direction) {
            (true, Some(direction)) => Action::DropBombAndMove(direction),
            (true, None) => Action::DropBomb,
            (false, Some(direction)) => Action::Move(direction),
            (false, None) => Action::StayStill,
        }
    }

    fn name(&self) -> String {
        "MatBomber3".into()
    }

    fn team_name() -> String {
        "Prague Bombers".into()
    }
}

fn bomb_in_direction(
    direction: Direction,
    surroundings: &[(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)],
) -> bool {
    for &(_, object, _, offset) in surroundings.iter() {
        if !matches!(object, Some(Object::Bomb { .. })) {
            continue;
        }

        if let Ok(offset_direction) = Direction::try_from(offset) {
            if offset_direction == direction {
                return true;
            }
        }
    }
    false
}

fn bomb_at_offset(
    x: i32,
    y: i32,
    surroundings: &[(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)],
) -> bool {
    let tile = surroundings
        .iter()
        .find(|&(_, _, _, offset)| offset == &TileOffset(x, y))
        .expect("tile exists");

    let &(_, object, _, _) = tile;
    matches!(object, Some(Object::Bomb { .. }))
}

fn crate_or_player_close(
    surroundings: &[(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)],
) -> bool {
    surroundings
        .iter()
        .filter(|&(_, _, _, offset)| offset.is_orthogonally_adjacent())
        .any(|(_, object, enemy, _)| matches!(object, Some(Object::Crate)) || enemy.is_some())
}
