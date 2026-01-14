use std::collections::HashMap;
use uuid::Uuid;

use crate::game_logic::stone::Item;

use super::stone::Stone;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub food: u8,
    pub level: u8,
    pub stones: HashMap<Stone, u32>,
    pub family: Uuid,
    pub resolving: bool,
    pub queue: Vec<String>,
}

impl Player {
    pub fn new(col: usize, row: usize) -> Player {
        return Player {
            x: col,
            y: row,
            food: 0,
            level: 1,
            stones: HashMap::new(),
            family: Uuid::new_v4(),
            resolving: false,
            queue: Vec::new(),
        };
    }
    // add player actions (move, see, fork, etc...)
    pub fn fork(family: Uuid, row: usize, col: usize) -> Player {
        return Player {
            x: col,
            y: row,
            food: 0,
            level: 1,
            stones: HashMap::new(),
            family: family,
            queue: Vec::new(),
            resolving: false,
        };
    }
    pub fn pick(&mut self, item: Item) {
        if let Item::Stone(stone) = item {
            // handle logic of stone eat and calculate level of player
            let count = self.stones.entry(stone).or_insert(0);
            *count += 1;
        } else {
            // handle logic of eat food(time coin)
            self.food += 1;
            return;
        }
    }
}
