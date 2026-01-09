mod game_logic;
use std::io;

use game_logic::*;
use uuid::Uuid;

use crate::game_logic::game::GameWorld;

fn main() {
    // game loop (non-blocking)
    // check if there are commands to execute (after the player action timeout has ended)
    // check if 127/t time has passed to decrease the player's food
    // check if there is data received on the socket (using epoll)

    // initialize just one player and play the game in the command line
    let mut game: GameWorld = GameWorld::new();
    game.initialize_map();
    for row in game.world_map.iter() {
        println!("{:?}", row.len());
    }
    let my_player_id: Uuid = Uuid::new_v4();
    game.join_player(my_player_id);
    // while game.current_time <= 10 {
    //     print!("$ ");
    //     io::Write::flush(&mut io::stdout()).unwrap();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("error");
    //     println!("{}", input.trim());
    // }
}
