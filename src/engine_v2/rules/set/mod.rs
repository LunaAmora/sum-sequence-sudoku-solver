pub mod r#box;
pub mod cage;
pub mod col;
pub mod row;

use super::Rule;
use crate::engine_v2::{CellEntry, CellMask, Entry, Mask, Pos, Sudoku, Value};

trait SetRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellEntry; 9];
}

impl<T: SetRule> Rule for T {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()> {
        let mut allowed_digits = Mask::ALL;
        let mut pencilmarks: Vec<CellMask> = Vec::new();
        let mut empty_cells: Vec<Pos> = Vec::new();

        let set = self.next_set(sudoku);

        for &(pos, entry) in set.iter() {
            match entry {
                Entry::Empty => empty_cells.push(pos),
                Entry::Digit(digit) => allowed_digits.set_digit(digit, false),
                Entry::Pencil(pm) => pencilmarks.push((pos, pm)),
            }
        }

        if pencilmarks.is_empty() && empty_cells.is_empty() {
            return Ok(());
        }

        for pm in &mut pencilmarks {
            for i in 0..9 {
                if !allowed_digits[i] {
                    pm.1.set_bit(i, false);
                }
            }
        }

        for cell in empty_cells.iter() {
            pencilmarks.push((*cell, allowed_digits));
        }

        let naked_sets = find_naked_sets(&pencilmarks);

        for (i, naked_cells) in naked_sets.into_iter().enumerate() {
            let Some(naked_cells) = naked_cells else {
                continue;
            };

            for (pos, pm) in pencilmarks.iter_mut() {
                if naked_cells.contains(pos) {
                    continue;
                }

                if pm[i] {
                    pm.set_bit(i, false);
                }
            }
        }

        for (pos, pm) in pencilmarks.iter_mut() {
            let count = pm.0.count_ones();

            let new_value = match count {
                0 => return Err(()),
                1 => pm.0.trailing_zeros() as Value + 1,
                _ => {
                    let Mask(mut pm_value) = *pm;
                    pm_value |= 1 << 15;

                    if pm_value != sudoku[*pos] {
                        pm_value
                    } else {
                        continue;
                    }
                }
            };

            sudoku[*pos] = new_value;
        }

        Ok(())
    }
}

fn find_naked_sets(marks: &[CellMask]) -> [Option<Vec<Pos>>; 9] {
    let mut result = [const { None }; 9];

    fn check_combination(marks: &[CellMask], indices: &[usize], result: &mut [Option<Vec<Pos>>; 9]) {
        let mut digits = [false; 9];
        let mut count = 0;

        for &idx in indices {
            for (i, digit) in digits.iter_mut().enumerate() {
                if !*digit && marks[idx].1[i] {
                    *digit = true;
                    count += 1;
                }
            }
        }

        if count == indices.len() {
            let positions: Vec<Pos> = indices.iter().map(|&idx| marks[idx].0).collect();
            for i in 0..9 {
                if digits[i] {
                    result[i] = Some(positions.clone());
                }
            }
        }
    }

    let len = marks.len();

    // Naked quads
    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                for l in (k + 1)..len {
                    check_combination(marks, &[i, j, k, l], &mut result);
                }
            }
        }
    }

    // Naked triplets
    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                check_combination(marks, &[i, j, k], &mut result);
            }
        }
    }

    // Naked pairs
    for i in 0..len {
        for j in (i + 1)..len {
            check_combination(marks, &[i, j], &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_naked_sets() {
        let pencilmarks =
            vec![((0, 0), Mask(0b0111)), ((0, 1), Mask(0b0011)), ((0, 2), Mask(0b0101)), ((0, 3), Mask(0b1001))];

        let result = find_naked_sets(&pencilmarks);

        let a: [Option<Vec<Pos>>; 9] = [
            Some(vec![(0, 0), (0, 1), (0, 2)]),
            Some(vec![(0, 0), (0, 1), (0, 2)]),
            Some(vec![(0, 0), (0, 1), (0, 2)]),
            Some(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            None,
            None,
            None,
            None,
            None,
        ];

        assert_eq!(result, a);
    }
}
