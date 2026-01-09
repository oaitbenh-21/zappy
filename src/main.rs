mod game_logic;
use game_logic::*;

fn main() {
    let game_map: Vec<Vec<u8>> = [[0u8; 10]; 10].iter().map(|row| row.to_vec()).collect();
    println!("Hello, zappy!");
    for row in game_map.iter() {
        println!("{:?}", row);
    }
    println!("stones: {:?}", stone::Stone::all());
    // game loop (non-blocking)
    // check if there are commands to execute (after the player action timeout has ended)
    // check if 127/t time has passed to decrease the player's food
    // check if there is data received on the socket (using epoll)
}
