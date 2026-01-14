mod game_logic;
use std::{io, process};

use crate::game_logic::game::GameWorld;
use game_logic::*;
use uuid::Uuid;

fn main() {
    // game loop (non-blocking)
    // check if there are commands to execute (after the player action timeout has ended)
    // check if 127/t time has passed to decrease the player's food
    // check if there is data received on the socket (using epoll)

    // initialize just one player and play the game in the command line
    let mut game: GameWorld = GameWorld::new();
    game.initialize_map();
    let my_player_id: Uuid = Uuid::new_v4();
    game.join_player(my_player_id);
    game.dump_map();
    while game.current_time <= 10 {
        print!("$ ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        match input.trim() {
            "q" | "quit" => {
                // action not permitted for the client
                println!("Quitting");
                process::exit(0);
            }
            "dump" => {
                // action not permitted for the client
                println!("Dumping map");
                game.dump_map();
            }
            "help" => {
                // action not permitted for the client
                println!("Commands: dump, quit, mr");
            }
            "mr" => {
                game.move_player(my_player_id, actions::Direction::Right);
                game.dump_map();
            }
            "ml" => {
                game.move_player(my_player_id, actions::Direction::Left);
                game.dump_map();
            }
            "mf" => {
                game.move_player(my_player_id, actions::Direction::Forward);
                game.dump_map();
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
