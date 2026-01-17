mod rules;

use rules::Rule;
use std::{
    fmt::Display,
    num::NonZeroU8,
    ops::{BitAnd, Index, IndexMut},
};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Mask(u16);

impl Index<usize> for Mask {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 16 {
            panic!("Index out of bounds");
        }

        if self.0 & (1 << index) != 0 { &true } else { &false }
    }
}

impl BitAnd for Mask {
    type Output = Mask;

    fn bitand(self, rhs: Self) -> Self::Output {
        Mask(self.0 & rhs.0)
    }
}

impl Mask {
    fn set(&mut self, index: usize, value: bool) {
        if index >= 16 {
            panic!("Index out of bounds");
        }

        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }
}

impl PartialEq<u16> for Mask {
    fn eq(&self, other: &u16) -> bool {
        &self.0 == other
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum Value {
    #[default]
    Empty,
    Digit(NonZeroU8),
    Pencil(Mask),
}

//   0 for number, 1 for pencilmark
//   |
// 0b1000 0000 0000 0000
//           |      |
//           |      digits 1-9
//           |
//           1-9 pencilmarks for digits 1-9
impl From<u16> for Value {
    fn from(value: u16) -> Self {
        if value == 0 {
            return Value::Empty;
        }

        if (value & 0x8000) == 0 {
            Value::Digit(NonZeroU8::new((value & 0xF) as u8).unwrap())
        } else {
            Value::Pencil(Mask(value & 0x1FF))
        }
    }
}

impl From<Value> for u16 {
    fn from(value: Value) -> Self {
        match value {
            Value::Empty => 0,
            Value::Digit(d) => d.get() as u16,
            Value::Pencil(pm) => 0x8000 | pm.0,
        }
    }
}

type Pos = (usize, usize);
type CellValue = (Pos, Value);
type CellMask = (Pos, Mask);

fn get_single_digit(value: u16) -> Option<u16> {
    if value != 0 && value.is_power_of_two() { Some(value.trailing_zeros() as u16 + 1) } else { None }
}

#[derive(Debug, PartialEq, Clone)]
struct Sudoku([[u16; 9]; 9]);

impl Sudoku {
    fn cell_value(&self, pos: Pos) -> CellValue {
        (pos, self[pos].into())
    }
}

impl Index<Pos> for Sudoku {
    type Output = u16;

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
                    Value::Empty => write!(f, "{}", empty)?,
                    Value::Digit(d) => write!(f, "{}", d)?,
                    Value::Pencil(_) => write!(f, ".")?,
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

enum SolveResult {
    Solved(Sudoku),
    Unsolvable,
    Stuck,
    LimitReached(Sudoku),
}

fn solve_sudoku(mut sudoku: Sudoku, rules: &mut [&mut dyn Rule], counter: &mut usize) -> SolveResult {
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
                    if let Value::Pencil(pm) = cell_value.into() {
                        for d in 0..9 {
                            if !pm[d] {
                                continue;
                            }

                            let mut test_sudoku = sudoku.clone();
                            test_sudoku[(i, j)] = d as u16 + 1;

                            if let SolveResult::Unsolvable = solve_sudoku(test_sudoku, rules, counter) {
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
                return SolveResult::Stuck;
            }
        }

        let done = sudoku.0.iter().all(|row| row.iter().all(|&cell| matches!(cell.into(), Value::Digit(_))));

        if done {
            return SolveResult::Solved(sudoku);
        }

        if *counter >= 2000000 {
            return SolveResult::LimitReached(sudoku);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rules::Rule;
    use super::rules::set::{r#box::BoxRule, col::ColRule, row::RowRule};
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

        let mut row_rule = RowRule::default();
        let mut col_rule = ColRule::default();
        let mut box_rule = BoxRule::default();
        let mut rules: Vec<&mut dyn Rule> = vec![&mut box_rule, &mut col_rule, &mut row_rule];

        let start = std::time::Instant::now();
        let mut counter = 0;
        let result = solve_sudoku(sudoku, &mut rules, &mut counter);

        let duration = start.elapsed();
        println!("Time elapsed in solve_sudoku() is: {:?}", duration);

        match result {
            SolveResult::Solved(sudoku) => {
                println!("Solved in {} iterations", counter);
                println!("Resulting Sudoku:\n{}", sudoku);
                assert_eq!(sudoku, solution);
            }
            SolveResult::Unsolvable => {
                println!("Sudoku is unsolvable");
                panic!("Should have been solvable");
            }
            SolveResult::Stuck => {
                println!("No progress made after {} iterations", counter);
                panic!("Should have been solvable");
            }
            SolveResult::LimitReached(sudoku) => {
                println!("Reached iteration limit: {}", counter);
                println!("Resulting Sudoku:\n{}", sudoku);
                panic!("Should have been solved before reaching limit");
            }
        };
    }

    #[test]
    fn test_extract_digit_or_pencilmark_mask() {
        let value_digit = 0b0000_0000_0000_0101;
        let value_marks = 0b1000_0001_0101_0110;

        assert_eq!(Value::from(value_digit), Value::Digit(NonZeroU8::new(5).unwrap()));
        assert_eq!(Value::from(value_marks), Value::Pencil(Mask(0b1_0101_0110)));
    }
}
