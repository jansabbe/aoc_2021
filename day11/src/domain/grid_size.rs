use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Position {
    pub row: i32,
    pub column: i32,
}

impl Position {
    pub fn new(row: i32, column: i32) -> Self {
        Position { row, column }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, Position { row, column }: Self) -> Self::Output {
        Position::new(self.row + row, self.column + column)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GridSize {
    pub max_rows: i32,
    pub max_columns: i32,
}

impl GridSize {
    pub fn contains(&self, &Position { row, column }: &Position) -> bool {
        row >= 0 && column >= 0 && row < self.max_rows && column < self.max_columns
    }


    pub fn adjacent_positions(&self, position: Position) -> Vec<Position> {
        [
            Position::new(-1, -1),
            Position::new(-1, 0),
            Position::new(-1, 1),
            Position::new(0, -1),
            Position::new(0, 1),
            Position::new(1, -1),
            Position::new(1, 0),
            Position::new(1, 1),
        ]
            .iter()
            .map(|delta| *delta + position)
            .filter(|pos| self.contains(pos))
            .collect()
    }


    pub fn all_positions(&self) -> impl Iterator<Item=Position> + '_ {
        (0..self.max_rows)
            .flat_map(move |row| (0..self.max_columns)
                .map(move |column| Position::new(row, column)))
    }
}
