use crate::checker_v2::rules::Rule;
use crate::checker_v2::{CellEntry, Entry, Pos, Sudoku};

/// A rule that returns the missing cages in the sum-sequence Sudoku.        
/// ```txt
///  _________________
/// |     |  1  |2    |
/// |     |  1  |2    |
/// |_____|_____|_____|
/// |     |     |     |
/// |3 3  |     |  4 4|
/// |_____|_____|_____|
/// |5 5  |     |     |
/// |     |  6  |     |
/// |_____|__6__|_____|
/// ```
pub struct CageRule {
    counter: usize,
    cages: [[Pos; 2]; 6],
}

impl Default for CageRule {
    fn default() -> Self {
        CageRule {
            counter: 0,
            cages: [
                [(0, 4), (1, 4)], //1
                [(0, 6), (1, 6)], //2
                [(4, 0), (4, 1)], //3
                [(4, 7), (4, 8)], //4
                [(6, 0), (6, 1)], //5
                [(7, 4), (8, 4)], //6
            ],
        }
    }
}

impl Rule for CageRule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()> {
        let cage = self.next(sudoku);

        let (pos_a, entry_a) = cage[0];
        let (pos_b, entry_b) = cage[1];

        match (entry_a, entry_b) {
            (Entry::Empty, _) | (_, Entry::Empty) => {}
            (Entry::Pencil(_), Entry::Pencil(_)) => {}

            (Entry::Digit(a), Entry::Digit(b)) => {
                if a == b {
                    return Err(());
                }
            }

            (Entry::Digit(digit), Entry::Pencil(mut mask)) => {
                if mask[digit] {
                    mask.set_digit(digit, false);
                    sudoku[pos_b] = mask.into();
                }
            }

            (Entry::Pencil(mut mask), Entry::Digit(digit)) => {
                if mask[digit] {
                    mask.set_digit(digit, false);
                    sudoku[pos_a] = mask.into();
                }
            }
        }

        Ok(())
    }
}

impl CageRule {
    fn next(&mut self, sudoku: &Sudoku) -> [CellEntry; 2] {
        let cage = &self.cages[self.counter];
        let result = cage.iter().map(|&pos| sudoku.cell_entry(pos)).collect::<Vec<_>>().try_into().unwrap();

        self.counter = (self.counter + 1) % self.cages.len();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker_v2::Mask;

    #[test]
    fn test_digit_and_pencilmark() {
        let mut sudoku = Sudoku([[0; 9]; 9]);

        sudoku[(0, 4)] = 7;
        sudoku[(1, 4)] = Mask::ALL.into();

        let mut rule = CageRule::default();
        let result = rule.update_cells(&mut sudoku);

        assert!(result.is_ok());
        assert_eq!(sudoku[(1, 4)], Mask(0b110111111).into());
    }
}
