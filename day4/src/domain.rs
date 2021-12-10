use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct BingoBox {
    value: u32,
    mark: Mark,
}

impl BingoBox {
    pub fn new(value: u32) -> Self {
        BingoBox { value, mark: Mark::Unmarked }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
    pub fn mark(&self) -> &Mark {
        &self.mark
    }
}

pub struct BingoBoard {
    pub boxes: [[BingoBox; 5]; 5],
}

impl BingoBoard {
    pub fn new(values: [[u32; 5]; 5]) -> Self {
        let boxes = values.map(|row| row.map(BingoBox::new));
        BingoBoard { boxes }
    }

    pub fn mark(&mut self, number: u32) {

        for row_index in 0..5 {
            for column_index in 0..5 {
                if self.boxes[row_index][column_index].value == number {
                    self.boxes[row_index][column_index].mark = Mark::Marked
                }
            }
        }
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        self.boxes
            .iter()
            .flatten()
            .filter_map(|bingo_box| match bingo_box.mark() {
                Mark::Unmarked => Some(bingo_box.value()),
                Mark::Marked => None
            })
            .sum()
    }

    pub fn is_bingo(&self) -> bool {
        for row_index in 0..5 {
            if self.row(row_index).iter().all(|bingo_box| bingo_box.mark == Mark::Marked) {
                return true;
            }
        }
        for column_index in 0..5 {
            if self.column(column_index).iter().all(|bingo_box| bingo_box.mark == Mark::Marked) {
                return true;
            }
        }
        false
    }

    fn row(&self, row_index: usize) -> Vec<BingoBox> {
        Vec::from(self.boxes[row_index])
    }

    fn column(&self, column_index: usize) -> Vec<BingoBox> {
        self.boxes.iter().map(|&row| row[column_index]).collect()
    }
}

impl FromStr for BingoBoard {
    type Err = ();
    //    "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19",
    fn from_str(board_str: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = board_str.split_whitespace().filter_map(|b| b.parse::<u32>().ok()).collect();
        Ok(BingoBoard::new([
            numbers[0..5].try_into().unwrap(),
            numbers[5..10].try_into().unwrap(),
            numbers[10..15].try_into().unwrap(),
            numbers[15..20].try_into().unwrap(),
            numbers[20..25].try_into().unwrap(),
        ]))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mark {
    Marked,
    Unmarked,
}