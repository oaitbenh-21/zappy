use std::collections::HashMap;
use uuid::Uuid;

use super::stone::Stone;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub food: u32,
    pub stones: HashMap<Stone, u32>,
    pub family: Uuid,
}
