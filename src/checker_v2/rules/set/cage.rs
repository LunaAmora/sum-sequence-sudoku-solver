use super::SetRule;
use crate::checker_v2::{CellValue, Pos, Sudoku};

/// A rule that returns the sets constructed from the caged pairs in the sum-sequence Sudoku.        
/// ```txt
///  _________________
/// |    0|  0  |0    |
/// |    0|     |     |
/// |1_1__|_____|__1_1|
/// |     |     |     |
/// |1    |     |    1|
/// |_____|_____|_____|
/// |1    |     |  1 1|
/// |    0|     |0    |
/// |____0|__0__|0____|
/// ```
pub struct CageRule {
    counter: usize,
    sets: [[Pos; 9]; 2],
}

impl Default for CageRule {
    fn default() -> Self {
        let mut sets = [[Pos::default(); 9]; 2];

        let x = [0, 1, 0, 0, 7, 8, 8, 7, 8];
        let y = [2, 2, 4, 6, 2, 2, 4, 6, 6];

        for i in 0..9 {
            sets[0][i] = (x[i], y[i]);
        }

        for i in 0..9 {
            sets[1][i] = (y[i], x[i]);
        }

        CageRule { counter: 0, sets }
    }
}

impl SetRule for CageRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let mut result = [CellValue::default(); 9];

        for (i, res) in result.iter_mut().enumerate() {
            let pos = self.sets[self.counter][i];
            *res = sudoku.cell_value(pos);
        }

        self.counter = (self.counter + 1) % self.sets.len();
        result
    }
}
