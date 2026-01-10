use std::collections::HashMap;

use crate::game_logic::config::WorldConfig;

use super::player::Player;
use super::stone::Item;
use rand::{Rng, thread_rng};
use uuid::Uuid;

pub struct GameWorld {
    pub current_time: u32,
    pub world_map: Vec<Vec<Square>>,
    pub families: Vec<String>,
    pub players: HashMap<Uuid, Player>,
    pub world_config: WorldConfig,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            current_time: 0,
            world_map: Vec::new(),
            players: HashMap::new(),
            families: Vec::new(),
            world_config: WorldConfig::new(
                Vec::from([String::from("Yellow"), String::from("Green")]),
                1,
                1,
            ),
        }
    }

    pub fn initialize_map(&mut self) -> Vec<Vec<Square>> {
        let mut world_map: Vec<Vec<Square>> = Vec::new();
        for _row in 1..=self.world_config.world_height {
            let mut map_row: Vec<Square> = Vec::new();
            for _col in 1..=self.world_config.world_widht {
                let mut current_square: Square = Square::new();
                for _turn in 0..3 {
                    match Item::get_random() {
                        Some(getted_item) => {
                            // rule based adding of items
                            current_square.objects.push(getted_item);
                        }
                        None => {}
                    }
                }
                println!("{:?}", current_square);
                map_row.push(current_square);
            }
            world_map.push(map_row);
        }
        return world_map;
    }

    pub fn join_player(&mut self, player: Uuid) {
        let mut rng = thread_rng();
        let col = rng.gen_range(0..=self.world_config.world_widht);
        let row = rng.gen_range(0..=self.world_config.world_height);
        self.players.insert(player, Player::new(col, row));
    }
    pub fn resolve_action(action: Action) {
        // take action from here as Action::Pick(Item)
        // check is Player can perform this action
        // like is this item still valid in the square(position)
    }
}

#[derive(Debug)]
pub struct Square {
    pub objects: Vec<Item>,
    pub players: Vec<Uuid>,
}

impl Square {
    pub fn new() -> Square {
        Square {
            objects: Vec::new(),
            players: Vec::new(),
        }
    }
    pub fn randomize() -> Square {
        let mut objects: Vec<Item> = Vec::new();
        for _resources_count in 0..3 {}
        return Square {
            objects: Vec::new(),
            players: Vec::new(),
        };
    }
}
