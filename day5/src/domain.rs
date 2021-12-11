use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone, Copy)]
pub struct Point(pub usize, pub usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (x, y) = str.split_once(',').ok_or(())?;
        let x: usize = x.parse().or(Err(()))?;
        let y: usize = y.parse().or(Err(()))?;
        return Ok(Point(x, y));
    }
}

fn range(a: usize, b: usize) -> Box<dyn Iterator<Item=usize>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Line {
        Line { from, to }
    }

    fn points(&self) -> Vec<Point> {
        let Point(x1, y1) = self.from;
        let Point(x2, y2) = self.to;

        if y1 == y2 {
            range(x1, x2).map(move |x| Point(x, y1)).collect()
        } else if x1 == x2 {
            range(y1, y2).map(move |y| Point(x1, y)).collect()
        } else {
            range(x1, x2)
                .zip(range(y1, y2))
                .map(|(x, y)| Point(x, y))
                .collect()
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(line_str: &str) -> Result<Self, Self::Err> {
        let (from, to) = line_str.split_once(" -> ").ok_or(())?;
        let from: Point = from.parse().or(Err(()))?;
        let to: Point = to.parse().or(Err(()))?;
        return Ok(Line::new(from, to));
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
            Point(9, 7),
            Point(8, 7),
            Point(7, 7),
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
    fn test_points_in_45_line() {
        let line = Line::new(Point(1, 1), Point(3, 3));
        let result = vec![
            Point(1, 1),
            Point(2, 2),
            Point(3, 3),
        ];
        assert_eq!(result, line.points());
    }


    #[test]
    fn test_points_in_45_line_direction() {
        let line = Line::new(Point(3, 1), Point(1, 3));
        let result = vec![
            Point(3, 1),
            Point(2, 2),
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