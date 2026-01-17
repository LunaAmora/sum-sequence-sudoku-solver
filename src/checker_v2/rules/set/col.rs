use super::SetRule;
use crate::checker_v2::{CellValue, Sudoku};

#[derive(Default)]
pub struct ColRule {
    counter: usize,
}

impl SetRule for ColRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let mut result = [CellValue::default(); 9];

        for (i, res) in result.iter_mut().enumerate() {
            *res = sudoku.cell_value((i, self.counter));
        }

        self.counter = (self.counter + 1) % 9;
        result
    }
}
