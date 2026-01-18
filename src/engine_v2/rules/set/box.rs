use super::SetRule;
use crate::engine_v2::{CellEntry, Sudoku};

#[derive(Default)]
pub struct BoxRule {
    counter: usize,
}

impl SetRule for BoxRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellEntry; 9] {
        let box_row = (self.counter / 3) * 3;
        let box_col = (self.counter % 3) * 3;

        let mut result = [CellEntry::default(); 9];

        for i in 0..3 {
            for j in 0..3 {
                result[i * 3 + j] = sudoku.cell_entry((box_row + i, box_col + j));
            }
        }

        self.counter = (self.counter + 1) % 9;
        result
    }
}
