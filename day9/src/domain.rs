use std::cmp::Ordering;
use std::ops::{Add, Index};
use std::str::FromStr;

pub struct HeightMap {
    value: Vec<Vec<Height>>,
}

impl HeightMap {
    pub fn new(nums: Vec<Vec<i32>>) -> Self {
        HeightMap {
            value: nums
                .iter()
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

    pub fn low_points(&self) -> impl Iterator<Item=Height> + '_ {
        self.iter_positions()
            .filter(|&pos| {
                self.adjacent(pos).iter().all(|&ad| ad > self[pos])
            })
            .map(|pos| self[pos])
    }


    fn size(&self) -> MapSize {
        MapSize {
            max_rows: self.value.len() as i32,
            max_columns: self.value[0].len() as i32,
        }
    }


    fn adjacent(&self, position: Position) -> Vec<Height> {
        let map_size = self.size();
        [
            Position(-1, 0),
            Position(0, -1),
            Position(0, 1),
            Position(1, 0)
        ]
            .iter()
            .map(|delta| *delta + position)
            .filter(|pos| map_size.contains(pos))
            .map(|pos| self[pos])
            .collect()
    }

    fn iter_positions(&self) -> impl Iterator<Item=Position> {
        let MapSize { max_rows, max_columns } = self.size();
        (0..max_rows).flat_map(move |row| (0..max_columns).map(move |column| Position(row, column)))
    }

    pub fn flood(&self, position: Position) -> Vec<Height> {
        let mut result = Vec::new();

        let mut points_to_check = vec![self[position]];

        while !points_to_check.is_empty() {
            let current = points_to_check.pop().unwrap();
            let Height(position, height) = current;
            if height < 9 && !result.contains(&current) {
                result.push(current);
                points_to_check.append(&mut self.adjacent(position))
            }
        }

        return result
    }
}

impl Index<Position> for HeightMap {
    type Output = Height;

    fn index(&self, index: Position) -> &Self::Output {
        let Position(row, column) = index;
        return &self.value[row as usize][column as usize];
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let result: Vec<Vec<i32>> = input
            .lines()
            .map(|line| line
                .split("")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect())
            .collect();
        Ok(HeightMap::new(result))
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
    fn partial_cmp(&self, Height(_pos, height): &Self) -> Option<Ordering> {
        self.1.partial_cmp(height)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MapSize {
    max_rows: i32,
    max_columns: i32,
}

impl MapSize {
    fn contains(&self, &Position(row, column): &Position) -> bool {
        row >= 0 && column >= 0 && row < self.max_rows && column < self.max_columns
    }
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

        let map = HeightMap::new(val);
        assert_eq!(map[Position(1, 1)], Height(Position(1, 1), 5))
    }

    #[test]
    fn test_get_adjacent_points() {
        let val = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let map = HeightMap::new(val);
        let points = map.adjacent(Position(1, 1));
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
        let map = HeightMap::new(val);
        let points = map.adjacent(Position(2, 2));
        assert_eq!(points, vec![
            Height(Position(1, 2), 6),
            Height(Position(2, 1), 8),
        ])
    }

    #[test]
    fn test_fill_flood() {
        let map: HeightMap = "\
            2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678\n"
            .parse()
            .unwrap();

        let result = map.flood(Position(0, 9));

        assert_eq!(result, vec![
            Height(Position(0,9), 0),
            Height(Position(1,9), 1),
            Height(Position(2,9), 2),
            Height(Position(1,8), 2),
            Height(Position(0,8), 1),

            Height(Position(0,7), 2),
            Height(Position(0,6), 3),
            Height(Position(1,6), 4),
            Height(Position(0,5), 4),
        ])
    }
}