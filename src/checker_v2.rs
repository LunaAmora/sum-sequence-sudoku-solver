#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

struct Position(usize, usize);

#[derive(PartialEq, Debug)]
enum Cell {
    Digit(u8),
    Pencilmark([u8; 9]),
}

use Cell::*;

enum CellState {
    Digit,
    Pencilmark,
    Null,
}

//   0 for number, 1 for pencilmark
//   |
// 0b1000 0000 0000 0000
//           |      |
//           |      digits 1-9
//           |
//           1-9 pencilmarks for digits 1-9
fn extract_cell(value: u16) -> Cell {
    let is_digit = (value & 0x8000) == 0;

    if is_digit {
        Digit((value & 0xF) as u8)
    } else {
        let mut pm = [0u8; 9];
        for (i, p) in pm.iter_mut().enumerate() {
            if (value & (1 << i)) != 0 {
                *p = 1;
            }
        }
        Pencilmark(pm)
    }
}

trait CelRule {
    fn update_cell(sudoku: &mut Sudoku, Position(row, col): Position) -> CellState {
        let mut allowed_digits: [u8; 9] = match extract_cell(sudoku[row][col]) {
            Digit(0) => [1; 9],
            Digit(_) => return CellState::Digit,
            Pencilmark(pm) => pm,
        };

        let group = Self::get_group(sudoku, Position(row, col));
        let mut pencilmarks: Vec<[u8; 9]> = Vec::new();

        for (i, &value) in group.iter().enumerate() {
            if Self::skip_cell(Position(row, col), i) {
                continue;
            }

            match extract_cell(value) {
                Digit(0) => (),
                Digit(d) => allowed_digits[(d - 1) as usize] = 0,
                Pencilmark(pm) => pencilmarks.push(pm),
            }
        }

        let disalowed = find_naked_sets(&pencilmarks);

        for i in 0..9 {
            if disalowed[i] == 1 {
                allowed_digits[i] = 0;
            }
        }

        if allowed_digits.contains(&1) {
            let mut new_value = 0;
            for (i, &digit) in allowed_digits.iter().enumerate() {
                if digit == 1 {
                    new_value |= 1 << i;
                }
            }

            if let Some(digit) = get_single_digit(new_value) {
                sudoku[row][col] = digit;
                return CellState::Digit;
            }

            new_value |= 0x8000;
            sudoku[row][col] = new_value;
            CellState::Pencilmark
        } else {
            CellState::Null
        }
    }

    fn get_group(sudoku: &Sudoku, pos: Position) -> [u16; 9];
    fn skip_cell(pos: Position, i: usize) -> bool;
}

struct RowRule;
impl CelRule for RowRule {
    fn get_group(sudoku: &Sudoku, Position(row, _col): Position) -> [u16; 9] {
        sudoku[row]
    }

    fn skip_cell(Position(_row, col): Position, i: usize) -> bool {
        i == col
    }
}

struct ColRule;
impl CelRule for ColRule {
    fn get_group(sudoku: &Sudoku, Position(_row, col): Position) -> [u16; 9] {
        (0..9).map(|r| sudoku[r][col]).collect::<Vec<u16>>().try_into().unwrap()
    }

    fn skip_cell(Position(row, _col): Position, i: usize) -> bool {
        i == row
    }
}

struct BoxRule;
impl CelRule for BoxRule {
    fn get_group(sudoku: &Sudoku, Position(row, col): Position) -> [u16; 9] {
        let box_start_row = (row / 3) * 3;
        let box_start_col = (col / 3) * 3;
        let mut cells = [0; 9];
        let mut index = 0;
        for r in box_start_row..box_start_row + 3 {
            for c in box_start_col..box_start_col + 3 {
                cells[index] = sudoku[r][c];
                index += 1;
            }
        }
        cells
    }

    fn skip_cell(Position(row, col): Position, i: usize) -> bool {
        i == (row % 3) * 3 + (col % 3)
    }
}

fn find_naked_sets(pencilmarks: &[[u8; 9]]) -> [u8; 9] {
    let mut result = [0; 9];

    // Naked pairs
    for i in 0..pencilmarks.len() {
        for j in (i + 1)..pencilmarks.len() {
            let mut union = [0; 9];
            let mut count = 0;
            for a in 0..9 {
                if pencilmarks[i][a] == 1 || pencilmarks[j][a] == 1 {
                    union[a] = 1;
                    count += 1;
                }
            }
            if count == 2 {
                for a in 0..9 {
                    if union[a] == 1 {
                        result[a] = 1;
                    }
                }
            }
        }
    }

    // Naked triplets
    for i in 0..pencilmarks.len() {
        for j in (i + 1)..pencilmarks.len() {
            for k in (j + 1)..pencilmarks.len() {
                let mut union = [0; 9];
                let mut count = 0;
                for a in 0..9 {
                    if pencilmarks[i][a] == 1 || pencilmarks[j][a] == 1 || pencilmarks[k][a] == 1 {
                        union[a] = 1;
                        count += 1;
                    }
                }
                if count == 3 {
                    for a in 0..9 {
                        if union[a] == 1 {
                            result[a] = 1;
                        }
                    }
                }
            }
        }
    }

    // Naked quads
    for i in 0..pencilmarks.len() {
        for j in (i + 1)..pencilmarks.len() {
            for k in (j + 1)..pencilmarks.len() {
                for l in (k + 1)..pencilmarks.len() {
                    let mut union = [0; 9];
                    let mut count = 0;
                    for a in 0..9 {
                        if pencilmarks[i][a] == 1
                            || pencilmarks[j][a] == 1
                            || pencilmarks[k][a] == 1
                            || pencilmarks[l][a] == 1
                        {
                            union[a] = 1;
                            count += 1;
                        }
                    }
                    if count == 4 {
                        for a in 0..9 {
                            if union[a] == 1 {
                                result[a] = 1;
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

fn get_single_digit(value: u16) -> Option<u16> {
    if value != 0 && value.is_power_of_two() { Some(value.trailing_zeros() as u16 + 1) } else { None }
}

#[derive(Debug, PartialEq, Clone)]
struct Sudoku([[u16; 9]; 9]);

impl Deref for Sudoku {
    type Target = [[u16; 9]; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &cell in row {
                match extract_cell(cell) {
                    Digit(d) => write!(f, "{} ", d)?,
                    Pencilmark(_) => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_sudoku(mut sudoku: Sudoku) -> Option<Sudoku> {
    let mut counter = 0;
    loop {
        let mut done = true;
        let old_state = sudoku.clone();
        counter += 1;

        for i in 0..9 {
            for j in 0..9 {
                match RowRule::update_cell(&mut sudoku, Position(i, j)) {
                    CellState::Digit => (),
                    CellState::Pencilmark => done = false,
                    CellState::Null => return None,
                }
                match ColRule::update_cell(&mut sudoku, Position(i, j)) {
                    CellState::Digit => (),
                    CellState::Pencilmark => done = false,
                    CellState::Null => return None,
                }
                match BoxRule::update_cell(&mut sudoku, Position(i, j)) {
                    CellState::Digit => (),
                    CellState::Pencilmark => done = false,
                    CellState::Null => return None,
                }
            }
        }

        if counter >= 1000000 {
            print!("Reached iteration limit");
            break;
        }

        if done {
            println!("Solved in {} iterations", counter);
            break;
        }

        if old_state == sudoku {
            println!("No progress made after {} iterations", counter);
            break;
        }
    }

    Some(sudoku)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checker() {
        let sudoku = Sudoku([
            [1, 5, 4, 2, 0, 6, 0, 0, 0],
            [0, 2, 0, 7, 1, 0, 0, 0, 0],
            [6, 8, 0, 4, 5, 0, 0, 2, 0],
            [0, 0, 0, 8, 0, 0, 7, 0, 0],
            [4, 0, 8, 9, 0, 7, 2, 0, 0],
            [0, 0, 0, 0, 0, 2, 4, 0, 9],
            [2, 0, 5, 0, 0, 0, 0, 0, 0],
            [9, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 7, 6, 0, 0, 0, 0, 4, 0],
        ]);

        let solution = Sudoku([
            [1, 5, 4, 2, 9, 6, 3, 7, 8],
            [3, 2, 9, 7, 1, 8, 6, 5, 4],
            [6, 8, 7, 4, 5, 3, 9, 2, 1],
            [5, 9, 2, 8, 4, 1, 7, 3, 6],
            [4, 6, 8, 9, 3, 7, 2, 1, 5],
            [7, 1, 3, 5, 6, 2, 4, 8, 9],
            [2, 3, 5, 6, 8, 4, 1, 9, 7],
            [9, 4, 1, 3, 7, 5, 8, 6, 2],
            [8, 7, 6, 1, 2, 9, 5, 4, 3],
        ]);

        // time the execution of solve_sudoku(sudoku) and print the duration
        let start = std::time::Instant::now();
        let result = solve_sudoku(sudoku);
        let duration = start.elapsed();
        println!("Time elapsed in solve_sudoku() is: {:?}", duration);
        println!("Resulting Sudoku:\n{}", result.as_ref().unwrap());

        assert_eq!(result, Some(solution));
    }

    #[test]
    fn test_extract_digit_or_pencilmark_mask() {
        let value_digit = 0b0000_0000_0000_0101;
        let value_pencilmark = 0b1000_0001_0101_0110;

        assert_eq!(extract_cell(value_digit), Digit(5));
        assert_eq!(extract_cell(value_pencilmark), Pencilmark([0, 1, 1, 0, 1, 0, 1, 0, 1]));
    }
}
