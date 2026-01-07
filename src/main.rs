mod game_logic;
use game_logic::*;

fn main() {
    let game_map: Vec<Vec<u8>> = [[0u8; 10]; 10].iter().map(|row| row.to_vec()).collect();
    println!("Hello, zappy!");
    for row in game_map.iter() {
        println!("{:?}", row);
    }
    println!("stones: {:?}", stone::Stone::all());
}
