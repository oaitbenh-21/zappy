use rand::{Rng, thread_rng};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
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
    pub fn get_random() -> Option<Stone> {
        let mut rng = thread_rng();
        return match rng.gen_range(0..=10) {
            0 => Some(Stone::Linemate),
            1 => Some(Stone::Deraumere),
            2 => Some(Stone::Sibur),
            3 => Some(Stone::Mendiane),
            4 => Some(Stone::Phiras),
            5 => Some(Stone::Thystame),
            _ => None,
        };
    }
}

#[derive(Debug)]
pub enum Item {
    Food,
    Stone(Stone),
}

impl Item {
    pub fn get_random() -> Option<Item> {
        let mut rng = thread_rng();
        return match rng.gen_range(0..=5) {
            0 => Some(Item::Food),
            1 => match Stone::get_random() {
                Some(stone) => Some(Item::Stone(stone)),
                None => None,
            },
            _ => None,
        };
    }
}
