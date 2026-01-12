// the server will send this and the player uuid
// to perform any action I need to know which player is
// trying to do it and what action they want to perform
pub enum Action {
    Enchantment,
    Kick,
    Fork,
    See,
    Pick(Item),
    Drop(Item),
    Move(Direction),
    Broadcast(String),
}

impl Action {
    pub fn parse_action(cmd: String) -> Some(Action) {
        let trimmed_cmd = cmd.as_str().trim();
        // complete the logic of parsing commands
        // from "pick linemate(type of stones)"
        // to Action::Pick(Item::Stone(Linemate))
        // may i'll change return value from Option to Result
        return None;
    }
}


pub enum Direction {
    Left,
    Right,
    Forward,
}
