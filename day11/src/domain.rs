use crate::domain::grid_size::{GridSize, Position};
use crate::domain::octopus::Octopus;
use crate::domain::octopus::Octopus::{Flashed, GainingEnergy, WillFlash};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

mod grid_size;
mod octopus;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    value: Vec<Vec<Octopus>>,
    nb_flashes: usize,
}

impl Grid {
    fn step(&mut self) {
        self.increase_all();
        self.flash_all();
        self.reset_all();
    }

    fn flash_all(&mut self) {
        while let Some(position) = self.octopus_that_will_flash() {
            self.flash(position);
        }
        self.nb_flashes += self
            .size()
            .all_positions()
            .filter(|&pos| self[pos] == Flashed)
            .count();
    }

    fn increase_all(&mut self) {
        self.size()
            .all_positions()
            .for_each(|pos| self[pos].increase())
    }

    fn reset_all(&mut self) {
        self.size()
            .all_positions()
            .for_each(|pos| self[pos].reset())
    }

    fn size(&self) -> GridSize {
        GridSize {
            max_rows: self.value.len() as i32,
            max_columns: self.value[0].len() as i32,
        }
    }

    pub fn flash(&mut self, position: Position) {
        let mut positions_to_increase: Vec<Position> = vec![position];
        while !positions_to_increase.is_empty() {
            let current = positions_to_increase.pop().unwrap();
            if self[current] != Flashed {
                self[current].increase();
                if self[current] == Flashed {
                    positions_to_increase.append(&mut self.size().adjacent_positions(current))
                }
            }
        }
    }
    fn octopus_that_will_flash(&self) -> Option<Position> {
        self.size()
            .all_positions()
            .find(|pos| self[*pos] == WillFlash)
    }

    fn is_all_zero(&self) -> bool {
        self.size()
            .all_positions()
            .all(|pos| self[pos] == GainingEnergy(0))
    }
}

impl Index<Position> for Grid {
    type Output = Octopus;

    fn index(&self, Position { row, column }: Position) -> &Self::Output {
        &self.value[row as usize][column as usize]
    }
}

impl IndexMut<Position> for Grid {
    fn index_mut(&mut self, Position { row, column }: Position) -> &mut Self::Output {
        &mut self.value[row as usize][column as usize]
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = Grid {
            nb_flashes: 0,
            value: s
                .lines()
                .map(|line| {
                    line.split("")
                        .filter_map(|num| num.parse::<Octopus>().ok())
                        .collect::<Vec<Octopus>>()
                })
                .collect(),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_can_step() {
        let mut grid: Grid = "\
            11111\n\
            19991\n\
            19191\n\
            19991\n\
            11111\n"
            .parse()
            .unwrap();
        grid.step();

        let mut expected: Grid = "\
            34543\n\
            40004\n\
            50005\n\
            40004\n\
            34543\n"
            .parse()
            .unwrap();
        expected.nb_flashes = 9;
        assert_eq!(grid, expected)
    }

    #[test]
    fn can_count_flashes() {
        let mut grid: Grid = "\
        5483143223\n\
        2745854711\n\
        5264556173\n\
        6141336146\n\
        6357385478\n\
        4167524645\n\
        2176841721\n\
        6882881134\n\
        4846848554\n\
        5283751526\n"
            .parse()
            .unwrap();
        (0..100).for_each(|_| grid.step());

        assert_eq!(grid.nb_flashes, 1656)
    }

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        let mut grid: Grid = input.parse().unwrap();
        (0..100).for_each(|_| grid.step());
        assert_eq!(grid.nb_flashes, 1691)
    }

    #[test]
    fn part2() {
        let input = include_str!("../input.txt");
        let mut grid: Grid = input.parse().unwrap();
        let mut step = 0;
        while !grid.is_all_zero() {
            grid.step();
            step += 1;
        }
        assert_eq!(step, 216)
    }
}
