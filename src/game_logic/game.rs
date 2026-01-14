use std::collections::HashMap;

use crate::game_logic::{actions::Direction, config::WorldConfig, player};

use super::actions::Action;
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

    pub fn initialize_map(&mut self) {
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
                // print!("{}", current_square.objects.len());
                map_row.push(current_square);
            }
            world_map.push(map_row);
        }
        self.world_map = world_map;
    }

    pub fn dump_map(&self) {
        for row in self.world_map.iter() {
            for col in row.iter() {
                print!("_{}", col.players.len());
            }
            println!();
        }
    }

    pub fn join_player(&mut self, player: Uuid) {
        let mut rng = thread_rng();
        let col = rng.gen_range(1..=self.world_config.world_widht) - 1;
        let row = rng.gen_range(1..=self.world_config.world_height) - 1;
        self.players.insert(player, Player::new(col, row));
        self.world_map[row][col].players.push(player);
    }
    pub fn move_player(&mut self, pid: Uuid, dir: Direction) {
        let mut player_option = self.players.get_mut(&pid);
        match player_option {
            Some(player) => {
                self.world_map[player.y][player.x]
                    .players
                    .retain(|uuid| *uuid != pid);
                // let newx = 0;
                // let newy = 0;
                match dir {
                    // i should to implement logic of where players looking at
                    Direction::Forward => {
                        if (player.y != 0) {
                            player.y -= 1;
                        } else {
                            player.y = self.world_config.world_height - 1;
                        }
                        player.y -= 1;
                    }
                    Direction::Left => {
                        if (player.x != 0) {
                            player.x -= 1;
                        } else {
                            player.x = self.world_config.world_widht - 1;
                        }
                    }
                    Direction::Right => {
                        player.x += 1;
                    }
                }
                player.x %= self.world_config.world_widht;
                player.y %= self.world_config.world_height;
                println!("y: {}\nx: {}", player.y, player.x);
                self.world_map[player.y][player.x].players.push(pid);
            }
            None => {}
        }
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
