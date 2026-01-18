mod rules;

use std::{
    fmt::Display,
    num::NonZeroU8,
    ops::{Index, IndexMut},
};

//   0 for number, 1 for pencilmark
//   |
// 0b1000 0000 0000 0000
//           |      |
//           |      digits 1-9
//           |
//           1-9 pencilmarks for digits 1-9
type Value = u16;

/// A bitmask representing possible pencilmarks for digits 1-9.
#[derive(PartialEq, Debug, Clone, Copy)]
struct Mask(u16);

impl Mask {
    const ALL: Mask = Mask(0b111111111);

    fn set_bit(&mut self, index: usize, value: bool) {
        if index >= 9 {
            panic!("Index out of bounds");
        }

        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }

    fn set_digit(&mut self, digit: NonZeroU8, value: bool) {
        self.set_bit(digit.get() as usize - 1, value);
    }
}

impl Index<usize> for Mask {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 9 {
            panic!("Index out of bounds");
        }

        if self.0 & (1 << index) != 0 { &true } else { &false }
    }
}

impl Index<NonZeroU8> for Mask {
    type Output = bool;

    fn index(&self, index: NonZeroU8) -> &Self::Output {
        &self[index.get() as usize - 1]
    }
}

impl From<Mask> for Value {
    fn from(value: Mask) -> Self {
        value.0 | 0x8000
    }
}

// Represents the content of a cell: either empty, a digit, or a pencilmark mask.
// Used to simplify working with raw cell values.
#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Entry {
    #[default]
    Empty,
    Digit(NonZeroU8),
    Pencil(Mask),
}

impl From<Value> for Entry {
    fn from(value: Value) -> Self {
        if value == 0 {
            return Entry::Empty;
        }

        if (value & 0x8000) == 0 {
            Entry::Digit(NonZeroU8::new((value & 0xF) as u8).unwrap())
        } else {
            Entry::Pencil(Mask(value & 0x1FF))
        }
    }
}

type Pos = (usize, usize);
type CellEntry = (Pos, Entry);
type CellMask = (Pos, Mask);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Sudoku([[Value; 9]; 9]);

impl Sudoku {
    fn cell_entry(&self, pos: Pos) -> CellEntry {
        (pos, self[pos].into())
    }
}

impl Index<Pos> for Sudoku {
    type Output = Value;

    fn index(&self, (row, col): Pos) -> &Self::Output {
        &self.0[row][col]
    }
}

impl IndexMut<Pos> for Sudoku {
    fn index_mut(&mut self, (row, col): Pos) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " _________________")?;
        for row in 0..9 {
            write!(f, "|")?;
            let empty = if (row + 1) % 3 == 0 { '_' } else { ' ' };

            for col in 0..9 {
                match self[(row, col)].into() {
                    Entry::Empty => write!(f, "{}", empty)?,
                    Entry::Digit(d) => write!(f, "{}", d)?,
                    Entry::Pencil(_) => write!(f, ".")?,
                }

                if (col + 1) % 3 == 0 {
                    write!(f, "|")?;
                } else {
                    write!(f, "{}", empty)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub enum SolveResult {
    Solved(usize, Sudoku),
    Unsolvable,
    Stuck(usize),
    LimitReached(usize, Sudoku),
}

fn solve(mut sudoku: Sudoku, rules: &mut [&mut dyn rules::Rule], counter: &mut usize) -> SolveResult {
    loop {
        let old_sudoku = sudoku.clone();

        for _ in 0..9 {
            for rule in &mut *rules {
                *counter += 1;
                if rule.update_cells(&mut sudoku).is_err() {
                    return SolveResult::Unsolvable;
                }
            }
        }

        if old_sudoku == sudoku {
            let mut progress = false;
            for i in 0..9 {
                for j in 0..9 {
                    let mut cell_value = sudoku[(i, j)];
                    if let Entry::Pencil(pm) = cell_value.into() {
                        for d in 0..9 {
                            if !pm[d] {
                                continue;
                            }

                            let mut test_sudoku = sudoku.clone();
                            test_sudoku[(i, j)] = d as Value + 1;

                            if let SolveResult::Unsolvable = solve(test_sudoku, rules, counter) {
                                progress = true;
                                cell_value &= !(1 << d);
                                sudoku[(i, j)] = cell_value;
                            }
                        }
                    }
                }
            }

            if progress {
                continue;
            } else {
                return SolveResult::Stuck(*counter);
            }
        }

        let done = sudoku.0.iter().all(|row| row.iter().all(|&cell| matches!(cell.into(), Entry::Digit(_))));

        if done {
            return SolveResult::Solved(*counter, sudoku);
        }

        if *counter >= 2000000 {
            return SolveResult::LimitReached(*counter, sudoku);
        }
    }
}

pub fn solve_sudoku(sudoku: Sudoku, sum_sequence: bool) -> SolveResult {
    use rules::*;

    let mut row_rule = RowRule::default();
    let mut col_rule = ColRule::default();
    let mut box_rule = BoxRule::default();
    let mut rules: Vec<&mut dyn Rule> = vec![&mut box_rule, &mut col_rule, &mut row_rule];

    let mut counter = 0;

    if sum_sequence {
        let mut cage_rule = CageRule::default();
        let mut palindrome_rule = PalindromeRule::default();
        let mut set_cage_rule = SetCageRule::default();

        rules.push(&mut cage_rule);
        rules.push(&mut palindrome_rule);
        rules.push(&mut set_cage_rule);
        return solve(sudoku, &mut rules, &mut counter);
    }

    solve(sudoku, &mut rules, &mut counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker() {
        let sudoku = Sudoku([
            [0, 6, 0, 8, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 0, 5, 0, 8, 0],
            [0, 3, 7, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 4, 0, 9, 7, 3, 0, 0],
            [0, 0, 0, 0, 5, 3, 0, 0, 0],
            [7, 0, 0, 0, 0, 1, 4, 6, 0],
            [5, 9, 0, 0, 0, 4, 7, 3, 0],
            [0, 0, 1, 0, 7, 0, 0, 0, 5],
        ]);

        let solution = Sudoku([
            [9, 6, 5, 8, 4, 2, 1, 7, 3],
            [4, 1, 2, 7, 3, 5, 9, 8, 6],
            [8, 3, 7, 9, 1, 6, 5, 4, 2],
            [3, 7, 9, 1, 2, 8, 6, 5, 4],
            [2, 5, 4, 6, 9, 7, 3, 1, 8],
            [1, 8, 6, 4, 5, 3, 2, 9, 7],
            [7, 2, 3, 5, 8, 1, 4, 6, 9],
            [5, 9, 8, 2, 6, 4, 7, 3, 1],
            [6, 4, 1, 3, 7, 9, 8, 2, 5],
        ]);

        let start = std::time::Instant::now();

        let result = solve_sudoku(sudoku, false);

        let duration = start.elapsed();
        println!("Time elapsed in solve_sudoku() is: {:?}", duration);

        match result {
            SolveResult::Solved(counter, sudoku) => {
                println!("Solved in {} iterations", counter);
                println!("Resulting Sudoku:\n{}", sudoku);
                assert_eq!(sudoku, solution);
            }
            SolveResult::Unsolvable => {
                println!("Sudoku is unsolvable");
                panic!("Should have been solvable");
            }
            SolveResult::Stuck(counter) => {
                println!("No progress made after {} iterations", counter);
                panic!("Should have been solvable");
            }
            SolveResult::LimitReached(counter, sudoku) => {
                println!("Reached iteration limit: {}", counter);
                println!("Resulting Sudoku:\n{}", sudoku);
                panic!("Should have been solved before reaching limit");
            }
        };
    }

    #[test]
    fn test_extract_digit_or_pencilmark_mask() {
        let digit = 0b0000_0000_0000_0101;
        let marks = 0b1000_0001_0101_0110;

        assert_eq!(Entry::from(digit), Entry::Digit(NonZeroU8::new(5).unwrap()));
        assert_eq!(Entry::from(marks), Entry::Pencil(Mask(0b1_0101_0110)));
    }
}
