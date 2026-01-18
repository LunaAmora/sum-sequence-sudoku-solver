use crate::engine_v2::rules::Rule;
use crate::engine_v2::{CellEntry, Entry, Mask, Pos, Sudoku};

/// A rule that returns the palindrome in the sum-sequence Sudoku.        
/// ```txt
///  _________________
/// |     |     |     |
/// |     |  1  |2    |
/// |_____|1___2|_____|
/// |    1|  3  |4    |
/// |  1  |3   4|  6  |
/// |____3|__5__|6____|
/// |  3  |5   6|     |
/// |     |  6  |     |
/// |_____|_____|_____|
/// ```
pub struct PalindromeRule {
    counter: usize,
    sets: [Vec<Pos>; 6],
}

impl Default for PalindromeRule {
    fn default() -> Self {
        let sets = [
            vec![(1, 4), (2, 3), (3, 2), (4, 1)],
            vec![(1, 6), (2, 5)],
            vec![(3, 4), (4, 3), (5, 2), (6, 1)],
            vec![(3, 6), (4, 5)],
            vec![(5, 4), (6, 3)],
            vec![(4, 7), (5, 6), (6, 5), (7, 4)],
        ];

        PalindromeRule { counter: 0, sets }
    }
}

impl Rule for PalindromeRule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()> {
        let palindrome = self.next(sudoku);

        for i in 0..(palindrome.len() / 2) {
            let (pos_l, entry_l) = palindrome[i];
            let (pos_r, entry_r) = palindrome[palindrome.len() - 1 - i];

            match (entry_l, entry_r) {
                (Entry::Empty, Entry::Empty) => {}

                (Entry::Digit(d1), Entry::Digit(d2)) => {
                    if d1 != d2 {
                        return Err(());
                    }
                }

                (Entry::Pencil(mask), Entry::Digit(digit)) | (Entry::Digit(digit), Entry::Pencil(mask))
                    if !mask[digit] =>
                {
                    return Err(());
                }

                (Entry::Pencil(_), Entry::Digit(d)) | (Entry::Empty, Entry::Digit(d)) => sudoku[pos_l] = d.get() as _,
                (Entry::Digit(d), Entry::Pencil(_)) | (Entry::Digit(d), Entry::Empty) => sudoku[pos_r] = d.get() as _,

                (Entry::Empty, Entry::Pencil(m)) => sudoku[pos_l] = m.into(),
                (Entry::Pencil(m), Entry::Empty) => sudoku[pos_r] = m.into(),

                (Entry::Pencil(mask_l), Entry::Pencil(mask_r)) => {
                    let intersection = Mask(mask_l.0 & mask_r.0).into();
                    sudoku[pos_l] = intersection;
                    sudoku[pos_r] = intersection;
                }
            }
        }

        Ok(())
    }
}

impl PalindromeRule {
    fn next(&mut self, sudoku: &Sudoku) -> Vec<CellEntry> {
        let result = self.sets[self.counter].iter().map(|&pos| sudoku.cell_entry(pos)).collect();

        self.counter = (self.counter + 1) % self.sets.len();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_to_pencilmark() {
        let mut sudoku = Sudoku::default();

        sudoku[(1, 4)] = 7;
        sudoku[(4, 1)] = Mask::ALL.into();

        let mut rule = PalindromeRule::default();
        let result = rule.update_cells(&mut sudoku);

        assert!(result.is_ok());
        assert_eq!(sudoku[(4, 1)], 7);
    }

    #[test]
    fn test_digit_invalid_pencilmark() {
        let mut sudoku = Sudoku::default();

        sudoku[(1, 4)] = 7;
        sudoku[(4, 1)] = Mask(0b000111111).into(); // Pencilmarks for 1-6

        let mut rule = PalindromeRule::default();
        let result = rule.update_cells(&mut sudoku);

        assert!(result.is_err());
    }

    #[test]
    fn test_pencilmark_intersection() {
        let mut sudoku = Sudoku::default();

        sudoku[(1, 4)] = Mask(0b000011111).into(); // 1-5
        sudoku[(4, 1)] = Mask(0b001111100).into(); // 3-7

        sudoku[(2, 3)] = Mask(0b101010101).into(); // 1,3,5,7,9
        sudoku[(3, 2)] = Mask(0b011111110).into(); // 2-8

        let mut rule = PalindromeRule::default();
        let result = rule.update_cells(&mut sudoku);

        assert!(result.is_ok());

        let expected = Mask(0b000011100).into(); // Pencilmarks for 3-5
        assert_eq!(sudoku[(1, 4)], expected);
        assert_eq!(sudoku[(4, 1)], expected);

        let expected = Mask(0b001010100).into(); // Pencilmarks for 3,5,7
        assert_eq!(sudoku[(2, 3)], expected);
        assert_eq!(sudoku[(3, 2)], expected);
    }
}
