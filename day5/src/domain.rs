use std::cmp::{min, max};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy)]
pub struct Point(pub usize, pub usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let Some(point) = str.split_once(',') {
            let x: usize = point.0.parse().map_err(|_| ())?;
            let y: usize = point.1.parse().map_err(|_| ())?;
            return Ok(Point(x, y));
        }
        Err(())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Line {
            from: min(a, b),
            to: max(a, b),
        }
    }

    fn points(&self) -> Vec<Point> {
        let Point(x1, y1) = self.from;
        let Point(x2, y2) = self.to;

        if y1 == y2 {
            (x1..=x2).map(|x| Point(x, y1)).collect()
        } else if x1 == x2 {
            (y1..=y2).map(|y| Point(x1, y)).collect()
        } else {
            Vec::new()
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(line_str: &str) -> Result<Self, Self::Err> {
        if let Some(line) = line_str.split_once(" -> ") {
            let point1: Point = line.0.parse().map_err(|_| ())?;
            let point2: Point = line.1.parse().map_err(|_| ())?;
            return Ok(Line::new(point1, point2));
        }
        Err(())
    }
}

#[derive(Debug)]
pub struct Board {
    dots: Vec<Vec<u32>>,
}

impl Board {
    pub fn new() -> Self {
        Board { dots: vec![vec![0; 1000]; 1000] }
    }

    pub fn draw(&mut self, line: Line) {
        for point in line.points() {
            let Point(x, y) = point;
            self.dots[x][y] += 1;
        }
    }
    pub fn count_intersections(&self) -> usize {
        self.dots.iter().flatten().filter(|&&f| f > 1).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_in_horizontal_line() {
        let line = Line::new(Point(9, 7), Point(7, 7));
        let result = vec![
            Point(7, 7),
            Point(8, 7),
            Point(9, 7),
        ];
        assert_eq!(result, line.points());
    }

    #[test]
    fn test_points_in_vertical_line() {
        let line = Line::new(Point(1, 1), Point(1, 3));
        let result = vec![
            Point(1, 1),
            Point(1, 2),
            Point(1, 3),
        ];
        assert_eq!(result, line.points());
    }


    #[test]
    fn test_drawing() {
        let mut board = Board::new();
        board.draw(Line::new(Point(1, 1), Point(1, 3)));
        board.draw(Line::new(Point(1, 2), Point(1, 4)));
        assert_eq!(board.dots[1][1], 1);
        assert_eq!(board.dots[1][2], 2);
    }
}