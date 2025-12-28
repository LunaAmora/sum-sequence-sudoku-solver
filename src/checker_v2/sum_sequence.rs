use super::{CellValue, SetRule, Sudoku};

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
#[derive(Default)]
struct CageSet {
    counter: usize,
}

impl SetRule for CageSet {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let mut result = [CellValue::default(); 9];

        let x = [0, 1, 0, 0, 7, 8, 8, 7, 8];
        let y = [2, 2, 4, 6, 2, 2, 4, 6, 6];

        for i in 0..9 {
            let pos = if self.counter == 0 { (x[i], y[i]) } else { (y[i], x[i]) };
            result[i] = sudoku.cell_value(pos);
        }

        self.counter = (self.counter + 1) % 2;
        result
    }
}
