use super::player::Player;
use super::stone::Item;
use uuid::Uuid;

pub struct Game {
    pub map: Vec<Vec<u8>>,
    pub players: Vec<Player>,
    pub families: Vec<Uuid>,
}

pub struct Tile {
    pub objects: Vec<Item>,
    pub players: Vec<Uuid>,
    pub x: usize,
    pub y: usize,
}
