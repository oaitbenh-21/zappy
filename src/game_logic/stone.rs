#[derive(Debug)]
pub enum Stone {
    Linemate,
    Deraumere,
    Sibur,
    Mendiane,
    Phiras,
    Thystame,
}

impl Stone {
    pub fn all() -> [Stone; 6] {
        [
            Stone::Linemate,
            Stone::Deraumere,
            Stone::Sibur,
            Stone::Mendiane,
            Stone::Phiras,
            Stone::Thystame,
        ]
    }
}

pub enum Item {
    Food,
    Stone(Stone),
}
