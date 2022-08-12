use std::collections::{HashMap, HashSet};

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

            if tile.0 == Tile::Wall {
                return false;
            }
            if matches!(tile.1, Some(Object::Bomb { .. } | Object::Crate)) {
                return false;
            }
            if matches!(&tile.2, Some(_enemy)) {
                return false;
            }
            true
        });

        // Remove bomb directions
        allowed_directions.retain(|direction| !bomb_in_direction(*direction, &surroundings));

        // TODO: more precise and less repetitive formula, also don't run into bombs one-tile offset but further away.
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

        let mut preferred_directions: HashMap<Direction, f32> =
            Direction::all().into_iter().map(|d| (d, 0.0)).collect();
        for &(tile, object, _, offset) in surroundings.iter() {
            // TODO: only add bonus for actually reachable (path exists) tiles.
            let mut score = 0.0;
            if tile == Tile::Hill {
                score += 100.0;
            }
            if matches!(object, Some(Object::PowerUp(_))) {
                score += 10.0;
            }
            if matches!(object, Some(Object::Crate)) {
                score += 1.0;
            }
            if score == 0.0 {
                continue;
            }
            score /= offset.taxicab_distance() as f32;

            let TileOffset(x, y) = offset;
            if x > 0 {
                *preferred_directions.get_mut(&East).unwrap() += score;
            }
            if x < 0 {
                *preferred_directions.get_mut(&West).unwrap() += score;
            }
            if y > 0 {
                *preferred_directions.get_mut(&North).unwrap() += score;
            }
            if y < 0 {
                *preferred_directions.get_mut(&South).unwrap() += score;
            }
        }

        let mut preferred_directions: Vec<_> =
            preferred_directions.into_iter().map(|(d, score)| (score, d)).collect();
        preferred_directions.sort_by_key(|&(score, _)| (-score * 1_000_000.0) as i64);

        let mut direction = None;
        for (_, d) in preferred_directions.into_iter().filter(|&(s, _)| s > 0.0) {
            if allowed_directions.contains(&d) {
                direction = Some(d);
                break;
            }
        }
        if direction.is_none() {
            direction = allowed_directions.into_iter().next()
        }

        // Drops a bomb every once in a while.
        let drop_bomb = crate_or_player_close(&surroundings);

        if (self.ticks as f64).log(2.0).fract().abs() < 0.001 {
            self.rotation += 1;
        }

        // Finalization
        self.ticks += 1;

        match (drop_bomb, direction) {
            (true, Some(direction)) => Action::DropBombAndMove(direction),
            (true, None) => Action::DropBomb,
            (false, Some(direction)) => Action::Move(direction),
            (false, None) => Action::StayStill,
        }
    }

    fn name(&self) -> String {
        "MatBomber".into()
    }

    fn team_name() -> String {
        "Prague Bombers".into()
    }
}

fn bomb_in_direction(
    direction: Direction,
    surroundings: &[(Tile, Option<Object>, Option<Enemy>, bomber_lib::world::TileOffset)],
) -> bool {
    // TODO: only count bombs that are not behind walls, count their range..
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
        .any(|(_, object, _, _)| matches!(object, Some(Object::Crate)))
        || surroundings
            .iter()
            .filter(|&(_, _, _, offset)| {
                (offset.0 == 0 || offset.1 == 1) && offset.taxicab_distance() <= 2
            })
            .any(|(_, _, enemy, _)| enemy.is_some())
}
