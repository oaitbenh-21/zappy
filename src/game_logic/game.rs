use std::collections::HashMap;

use crate::game_logic::game;

use super::player::Player;
use super::stone::Item;
use uuid::Uuid;

pub struct Game {
    pub map: Vec<Vec<u8>>,
    pub players: HashMap<Uuid, Player>,
    pub families: Vec<Uuid>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            map: Vec::new(),
            players: HashMap::new(),
            families: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Uuid) {
        self.players.insert(player, Player::new());
    }
    pub fn kill_player(&mut self, player: Uuid) {
        self.players.remove(&player);
    }
}

pub struct Tile {
    pub objects: Vec<Item>,
    pub players: Vec<Uuid>,
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(row: usize, col: usize) -> Tile {
        Tile {
            objects: Vec::new(),
            players: Vec::new(),
            x: col,
            y: row,
        }
    }
}
