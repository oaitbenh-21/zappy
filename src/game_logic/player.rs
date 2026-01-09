use std::collections::HashMap;
use uuid::Uuid;

use crate::game_logic::stone::Item;

use super::stone::Stone;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub food: u32,
    pub level: u8,
    pub stones: HashMap<Stone, u32>,
    pub family: Uuid,
}

impl Player {
    pub fn new() -> Player {
        return Player {
            x: 0,
            y: 0,
            food: 0,
            level: 1,
            stones: HashMap::new(),
            family: Uuid::new_v4(),
        };
    }
    pub fn fork(family: Uuid, row: usize, col: usize) -> Player {
        return Player {
            x: col,
            y: row,
            food: 0,
            level: 1,
            stones: HashMap::new(),
            family: family,
        };
    }
    pub fn pick(&mut self, item: Item) {
        if (matches!(item, Item::Food)) {
            // handle logic of eat food(time coin)
            return;
        }
        // handle logic of stone eat and calculate level of player
    }
}
