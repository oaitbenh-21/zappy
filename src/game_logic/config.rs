pub struct WorldConfig {
    pub world_height: usize,
    pub world_widht: usize,

    pub teams_count: usize,
    pub teams_name: Vec<String>,
    pub players_per_team: usize,

    pub initial_food: u8,
    pub initial_level: u8,

    pub time_per_sec: u8,

    pub food_per_square: u8,
    pub stones_per_square: u8,
    pub same_stone_per_square: u8,
}

impl WorldConfig {
    pub fn new(teams: Vec<String>, players_per_team: usize, time_unit_per_sec: u8) -> WorldConfig {
        WorldConfig {
            teams_name: teams.clone(),
            teams_count: teams.len(),
            time_per_sec: time_unit_per_sec,
            players_per_team: players_per_team,
            // those configs represent the rules of game
            world_widht: 80,
            world_height: 20,
            initial_food: 10, // defined in the game subject
            initial_level: 1, // there is no game starts at level 2
            food_per_square: 2,
            stones_per_square: 3,
            same_stone_per_square: 3,
        }
    }
}
