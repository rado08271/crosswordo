#[derive(Clone, Copy, PartialOrd, PartialEq, Hash, Eq, Ord)]
pub enum Direction {
    __NORTHWEST{row: i32, col: i32},
    __NORTH{row: i32, col: i32},
    __NORTHEAST{row: i32, col: i32},
    __WEST{row: i32, col: i32},
    __CENTER{row: i32, col: i32},
    __EAST{row: i32, col: i32},
    __SOUTHWEST{row: i32, col: i32},
    __SOUTH{row: i32, col: i32},
    __SOUTHEAST{row: i32, col: i32},
}

impl Direction {
    pub fn NORTHWEST() -> Self { Direction::__NORTHWEST     {row: -1, col: -1} }
    pub fn NORTH() -> Self { Direction::__NORTH             {row: -1, col:  0} }
    pub fn NORTHEAST() -> Self { Direction::__NORTHEAST     {row: -1, col:  1} }
    pub fn WEST() -> Self { Direction::__WEST               {row:  0, col: -1} }
    pub fn CENTER() -> Self { Direction::__CENTER           {row:  0, col:  0} }
    pub fn EAST() -> Self { Direction::__EAST               {row:  0, col:  1} }
    pub fn SOUTHWEST() -> Self { Direction::__SOUTHWEST     {row:  1, col: -1} }
    pub fn SOUTH() -> Self { Direction::__SOUTH             {row:  1, col:  0} }
    pub fn SOUTHEAST() -> Self { Direction::__SOUTHEAST     {row:  1, col:  1} }
    pub fn DIRECTION_MATRIX() -> [Direction; 9]{
        return [
            Self::NORTHWEST(), Self::NORTH(), Self::NORTHEAST(),
            Self::WEST(), Self::CENTER(), Self::EAST(),
            Self::SOUTHWEST(), Self::SOUTH(), Self::SOUTHEAST(),
        ]
    }

    pub fn getRow(&self) -> i32 {
        return match self {
            Direction::__NORTHWEST { row, col } => { *row }
            Direction::__NORTH { row, col } => { *row }
            Direction::__NORTHEAST { row, col } => { *row }
            Direction::__WEST { row, col } => { *row }
            Direction::__CENTER { row, col } => { *row }
            Direction::__EAST { row, col } => { *row }
            Direction::__SOUTHWEST { row, col } => { *row }
            Direction::__SOUTH { row, col } => { *row }
            Direction::__SOUTHEAST { row, col } => { *row }
        }
    }

    pub fn getCol(&self) -> i32 {
        return match self {
            Direction::__NORTHWEST { row, col } => { *col }
            Direction::__NORTH { row, col } => { *col }
            Direction::__NORTHEAST { row, col } => { *col }
            Direction::__WEST { row, col } => { *col }
            Direction::__CENTER { row, col } => { *col }
            Direction::__EAST { row, col } => { *col }
            Direction::__SOUTHWEST { row, col } => { *col }
            Direction::__SOUTH { row, col } => { *col }
            Direction::__SOUTHEAST { row, col } => { *col }
        }
    }

    pub fn getIndex(&self) -> usize {
        return match self {
            Direction::__NORTHWEST { .. } =>    {0}
            Direction::__NORTH { .. } =>        {1}
            Direction::__NORTHEAST { .. } =>    {2}
            Direction::__WEST { .. } =>         {3}
            Direction::__CENTER { .. } =>       {4}
            Direction::__EAST { .. } =>         {5}
            Direction::__SOUTHWEST { .. } =>    {6}
            Direction::__SOUTH { .. } =>        {7}
            Direction::__SOUTHEAST { .. } =>    {8}
        }
    }

}