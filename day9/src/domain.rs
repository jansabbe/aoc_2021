use std::cmp::Ordering;
use std::ops::{Add, Index};

pub struct HeightMap {
    value: Vec<Vec<Height>>,
}

impl HeightMap {
    pub fn new<'a, T>(nums: T) -> Self where T: Iterator<Item=&'a Vec<i32>> + 'a {
        HeightMap {
            value: nums
                .enumerate()
                .map(|(row_index, value)| {
                    value
                        .iter()
                        .enumerate()
                        .map(move |(column_index, height)| {
                            Height(Position(row_index as i32, column_index as i32), *height)
                        })
                        .collect()
                }).collect()
        }
    }

    fn size(&self) -> MapSize {
        MapSize {
            max_rows: self.value.len() as i32,
            max_columns: self.value[0].len() as i32,
        }
    }

    pub fn adjacent_heights(&self, position: Position) -> Vec<Height> {
        let MapSize { max_rows, max_columns } = self.size();
        [
            Position(-1, 0),
            Position(0, -1),
            Position(0, 1),
            Position(1, 0)
        ]
            .iter()
            .map(|delta| *delta + position)
            .filter(|Position(row, column)| {
                *row >= 0 && *column >= 0 && *row < max_rows && *column < max_columns
            })
            .map(|pos| self[pos])
            .collect()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item=Position> {
        let MapSize { max_rows, max_columns } = self.size();
        (0..max_rows).flat_map(move |row| (0..max_columns).map(move |column| Position(row, column)))
    }
}

impl Index<Position> for HeightMap {
    type Output = Height;

    fn index(&self, Position(row, column): Position) -> &Self::Output {
        &self.value[row as usize][column as usize]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Position(pub i32, pub i32);

impl Add for Position {
    type Output = Position;

    fn add(self, Position(x, y): Self) -> Self::Output {
        Position(self.0 + x, self.1 + y)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Height(pub Position, pub i32);

impl PartialOrd for Height {
    fn partial_cmp(&self, Height(pos, height): &Self) -> Option<Ordering> {
        self.1.partial_cmp(height)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MapSize {
    max_rows: i32,
    max_columns: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points() {
        let val = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let map = HeightMap::new(val.iter());
        assert_eq!(map[Position(1, 1)], Height(Position(1, 1), 5))
    }

    #[test]
    fn test_get_adjacent_points() {
        let val = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let map = HeightMap::new(val.iter());
        let points = map.adjacent_heights(Position(1, 1));
        assert_eq!(points, vec![
            Height(Position(0, 1), 2),
            Height(Position(1, 0), 4),
            Height(Position(1, 2), 6),
            Height(Position(2, 1), 8),
        ])
    }


    #[test]
    fn test_get_adjacent_points_in_corner() {
        let val = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let map = HeightMap::new(val.iter());
        let points = map.adjacent_heights(Position(2, 2));
        assert_eq!(points, vec![
            Height(Position(1, 2), 6),
            Height(Position(2, 1), 8),
        ])
    }
}