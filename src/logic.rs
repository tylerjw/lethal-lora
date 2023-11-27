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

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game, Coord};

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
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    
    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let board_width = board.width;
    let board_height = board.height;
    info!("width: {}, height: {}", board_width, board_height);
    let my_body = &you.body;
    if my_head.y > 0 {
        let down = Coord{x: my_head.x, y: my_head.y - 1};
        for space in my_body.iter().skip(1) {
            if *space == down {
                is_move_safe.insert("down", false);
                break;
            }
        }
    } else {
        is_move_safe.insert("down", false);
    }

    if my_head.y < board_height - 1 {
        let up = Coord{x: my_head.x, y: my_head.y + 1};
        for space in my_body.iter().skip(1) {
            if *space == up {
                is_move_safe.insert("up", false);
                break;
            }
        }
    } else {
        is_move_safe.insert("up", false);
    }

    if my_head.x > 0 {
        let left = Coord{x: my_head.x - 1, y: my_head.y};
        for space in my_body.iter().skip(1) {
            if *space == left {
                is_move_safe.insert("left", false);
                break;
            }
        }
    } else {
        is_move_safe.insert("left", false);
    }

    if my_head.x < board_width - 1 {
        let right = Coord{x: my_head.x + 1, y: my_head.y};
        for space in my_body.iter().skip(1) {
            if *space == right {
                is_move_safe.insert("right", false);
                break;
            }
        }
    } else {
        is_move_safe.insert("right", false);
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    // Choose a random move from the safe ones
    let chosen = 
        if safe_moves.is_empty() {
            info!("no safe moves -- we die now :(");
            "left"
        } else {
            safe_moves.choose(&mut rand::thread_rng()).unwrap()
        };

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
