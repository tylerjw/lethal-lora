// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use core::fmt;
use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Coord, Game};

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
        }
    }
}

impl Move {
    fn to_coord(&self, you: &Battlesnake) -> Option<Coord> {
        let head = &you.body[0];
        match (self, head.x, head.y) {
            (Move::Down, _, 1..) => Some(Coord {
                x: head.x,
                y: head.y - 1,
            }),
            (Move::Up, _, _) => Some(Coord {
                x: head.x,
                y: head.y + 1,
            }),
            (Move::Right, _, _) => Some(Coord {
                x: head.x + 1,
                y: head.y,
            }),
            (Move::Left, 1.., _) => Some(Coord {
                x: head.x - 1,
                y: head.y,
            }),
            _ => None,
        }
    }

    fn from_coord(you: &Battlesnake, coord: &Coord) -> Option<Self> {
        let head = &you.body[0];
        if head.x == coord.x {
            if head.y + 1 == coord.y {
                Some(Move::Up)
            } else if coord.y + 1 == head.y {
                Some(Move::Down)
            } else {
                None
            }
        } else if head.y == coord.y {
            if head.x + 1 == coord.x {
                Some(Move::Right)
            } else if coord.x + 1 == head.x {
                Some(Move::Left)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn all() -> Vec<Self> {
        vec![Self::Left, Self::Right, Self::Up, Self::Down]
    }
}

fn in_bounds(board: &Board, coord: &Coord) -> bool {
    coord.x < board.width && coord.y < board.height
}

fn manhattan_distance(a: &Coord, b: &Coord) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(
            manhattan_distance(&Coord { x: 0, y: 0 }, &Coord { x: 0, y: 0 }),
            0
        );
        assert_eq!(
            manhattan_distance(&Coord { x: 0, y: 0 }, &Coord { x: 10, y: 10 }),
            20
        );

        assert_eq!(
            manhattan_distance(&Coord { x: 5, y: 5 }, &Coord { x: 0, y: 0 }),
            10
        );
        assert_eq!(
            manhattan_distance(&Coord { x: 3, y: 1 }, &Coord { x: 4, y: 9 }),
            9
        );
    }

    #[test]
    fn move_to_coord() {
        let you = Battlesnake {
            body: vec![Coord { x: 0, y: 0 }],
            ..Default::default()
        };
        assert_eq!(Move::Left.to_coord(&you), None, "move would go off board");
        assert_eq!(Move::Right.to_coord(&you), Some(Coord { x: 1, y: 0 }));
        assert_eq!(Move::Up.to_coord(&you), Some(Coord { x: 0, y: 1 }));
        assert_eq!(Move::Down.to_coord(&you), None, "move would go off board");

        let you = Battlesnake {
            body: vec![Coord { x: 1, y: 1 }],
            ..Default::default()
        };
        assert_eq!(Move::Left.to_coord(&you), Some(Coord { x: 0, y: 1 }));
        assert_eq!(Move::Right.to_coord(&you), Some(Coord { x: 2, y: 1 }));
        assert_eq!(Move::Up.to_coord(&you), Some(Coord { x: 1, y: 2 }));
        assert_eq!(Move::Down.to_coord(&you), Some(Coord { x: 1, y: 0 }));
    }

    #[test]
    fn coord_to_move() {
        let you = Battlesnake {
            body: vec![Coord { x: 0, y: 0 }],
            ..Default::default()
        };
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 0, y: 0 }),
            None,
            "Can't move from head to same position"
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 1, y: 0 }),
            Some(Move::Right),
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 0, y: 1 }),
            Some(Move::Up),
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 1, y: 1 }),
            None,
            "Can't move diagonally"
        );
        let you = Battlesnake {
            body: vec![Coord { x: 1, y: 1 }],
            ..Default::default()
        };
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 1, y: 0 }),
            Some(Move::Down),
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 0, y: 1 }),
            Some(Move::Left),
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 1, y: 2 }),
            Some(Move::Up),
        );
        assert_eq!(
            Move::from_coord(&you, &Coord { x: 2, y: 1 }),
            Some(Move::Right),
        );
    }

    #[test]
    fn coord_move_round_trip() {
        let you = Battlesnake {
            body: vec![Coord { x: 1, y: 1 }],
            ..Default::default()
        };
        let coord = Move::Right.to_coord(&you).unwrap();
        assert_eq!(Move::from_coord(&you, &coord), Some(Move::Right));
        let coord = Move::Left.to_coord(&you).unwrap();
        assert_eq!(Move::from_coord(&you, &coord), Some(Move::Left));
        let coord = Move::Up.to_coord(&you).unwrap();
        assert_eq!(Move::from_coord(&you, &coord), Some(Move::Up));
        let coord = Move::Down.to_coord(&you).unwrap();
        assert_eq!(Move::from_coord(&you, &coord), Some(Move::Down));
    }

    #[test]
    fn out_of_bounds() {
        let board = Board {
            height: 11,
            width: 11,
            ..Default::default()
        };

        assert!(in_bounds(&board, &Coord { x: 0, y: 0 }));
        assert!(in_bounds(&board, &Coord { x: 10, y: 10 }));
        assert!(!in_bounds(&board, &Coord { x: 11, y: 10 }));
        assert!(!in_bounds(&board, &Coord { x: 10, y: 11 }));
    }

    #[test]
    fn all_moves() {
        let all = Move::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&Move::Up));
        assert!(all.contains(&Move::Down));
        assert!(all.contains(&Move::Left));
        assert!(all.contains(&Move::Right));
    }
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "Lethal Lora",
        "color": "#8ceb34",
        "head": "fang",
        "tail": "pixel", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let snake_coords: Vec<_> = board.snakes.iter().flat_map(|s| &s.body).collect();
    let safe_moves = Move::all()
        .iter()
        .flat_map(|m| m.to_coord(&you))
        .filter(|m| in_bounds(&board, m))
        .filter(|c| !snake_coords.contains(&c))
        .collect::<Vec<Coord>>();

    if safe_moves.is_empty() {
        info!("There are no safe moves, we are dead :(");
        return json!({"move": Move::Left.to_string()});
    }

    // select random move
    let mut chosen =
        Move::from_coord(&you, safe_moves.choose(&mut rand::thread_rng()).unwrap()).unwrap();

    let closest_food = board
        .food
        .iter()
        .map(|food| (food, manhattan_distance(&you.body[0], food)))
        .min_by(|(_, ad), (_, bd)| ad.cmp(bd))
        .map(|(coord, _)| coord);

    // if there is food, move towards nearest food
    if let Some(food) = closest_food {
        chosen = Move::from_coord(
            &you,
            safe_moves
                .iter()
                .map(|c| (c, manhattan_distance(&c, food)))
                .min_by(|(_, ad), (_, bd)| ad.cmp(bd))
                .map(|(coord, _)| coord)
                .unwrap(),
        )
        .unwrap();
    }

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen.to_string() });
}
