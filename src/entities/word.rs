use crate::entities::direction::Direction;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Word {
    pub direction: Direction,
    length: usize,
    pub coords: (usize, usize),
    pub word: String,
}

impl Word {
    pub fn new(value: String, direction: Direction, coords: (usize, usize)) -> Self {
        Word {
            coords, direction, length: value.len(), word: value,
        }
    }
}