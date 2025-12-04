use std::{
    fmt::Display,
    num::NonZeroU8,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, PartialEq)]
struct Pos(usize, usize);

#[derive(PartialEq, Debug)]
enum Cell {
    Empty,
    Digit(NonZeroU8),
    Pencil([bool; 9]),
}

//   0 for number, 1 for pencilmark
//   |
// 0b1000 0000 0000 0000
//           |      |
//           |      digits 1-9
//           |
//           1-9 pencilmarks for digits 1-9
fn extract_cell(value: u16) -> Cell {
    if value == 0 {
        return Cell::Empty;
    }

    let is_digit = (value & 0x8000) == 0;

    if is_digit {
        Cell::Digit(NonZeroU8::new((value & 0xF) as u8).unwrap())
    } else {
        let mut pm = [false; 9];
        for (i, p) in pm.iter_mut().enumerate() {
            if (value & (1 << i)) != 0 {
                *p = true;
            }
        }
        Cell::Pencil(pm)
    }
}

trait SetRule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Option<()> {
        let mut allowed_digits: [bool; 9] = [true; 9];
        let mut pencilmarks: Vec<(Pos, [bool; 9])> = Vec::new();
        let mut empty_cells: Vec<Pos> = Vec::new();

        let set = self.next_set(sudoku);

        for &(cell, value) in set.iter() {
            match extract_cell(value) {
                Cell::Empty => empty_cells.push(cell),
                Cell::Digit(d) => allowed_digits[(d.get() - 1) as usize] = false,
                Cell::Pencil(pm) => pencilmarks.push((cell, pm)),
            }
        }

        if pencilmarks.is_empty() && empty_cells.is_empty() {
            return Some(());
        }

        for pm in &mut pencilmarks {
            for (i, allowed) in allowed_digits.iter().enumerate() {
                if !allowed {
                    pm.1[i] = false;
                }
            }
        }

        for cell in empty_cells.iter() {
            pencilmarks.push((*cell, allowed_digits));
        }

        let naked_sets = find_naked_sets(&pencilmarks);

        for (i, naked_cells) in naked_sets.into_iter().enumerate().filter_map(|(i, n)| n.map(|n| (i, n))) {
            for (cell, pm) in pencilmarks.iter_mut() {
                if naked_cells.contains(cell) {
                    continue;
                }

                if pm[i] {
                    pm[i] = false;
                }
            }
        }

        for (cell, pm) in pencilmarks.iter_mut() {
            let count = pm.iter().filter(|&&x| x).count();

            let new_value = match count {
                0 => return None,
                1 => {
                    let digit = pm.iter().position(|&x| x).unwrap() + 1;
                    digit as u16
                }
                _ => {
                    let mut pm_value = 0x8000;
                    for (i, &marked) in pm.iter().enumerate() {
                        if marked {
                            pm_value |= 1 << i;
                        }
                    }
                    if pm_value != sudoku[cell.0][cell.1] {
                        pm_value
                    } else {
                        continue;
                    }
                }
            };

            sudoku[cell.0][cell.1] = new_value;
        }

        Some(())
    }

    fn next_set(&mut self, sudoku: &Sudoku) -> [(Pos, u16); 9];
}

#[derive(Default)]
struct RowRule {
    counter: usize,
}

impl SetRule for RowRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [(Pos, u16); 9] {
        let mut result = [(Pos(0, 0), 0u16); 9];

        for i in 0..9 {
            let pos = Pos(self.counter, i);
            let value = sudoku[self.counter][i];
            result[i] = (pos, value);
        }

        self.counter = (self.counter + 1) % 9;
        result
    }
}

#[derive(Default)]
struct ColRule {
    counter: usize,
}

impl SetRule for ColRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [(Pos, u16); 9] {
        let mut result = [(Pos(0, 0), 0u16); 9];

        for i in 0..9 {
            let pos = Pos(i, self.counter);
            let value = sudoku[i][self.counter];
            result[i] = (pos, value);
        }

        self.counter = (self.counter + 1) % 9;
        result
    }
}

#[derive(Default)]
struct BoxRule {
    counter: usize,
}

impl SetRule for BoxRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [(Pos, u16); 9] {
        let box_row = (self.counter / 3) * 3;
        let box_col = (self.counter % 3) * 3;

        let mut result = [(Pos(0, 0), 0u16); 9];

        for i in 0..3 {
            for j in 0..3 {
                let pos = Pos(box_row + i, box_col + j);
                let value = sudoku[box_row + i][box_col + j];
                result[i * 3 + j] = (pos, value);
            }
        }

        self.counter = (self.counter + 1) % 9;
        result
    }
}

fn find_naked_sets(pencilmarks: &[(Pos, [bool; 9])]) -> [Option<Vec<Pos>>; 9] {
    let mut result = [const { None }; 9];

    // Naked pairs
    for i in 0..pencilmarks.len() {
        for j in (i + 1)..pencilmarks.len() {
            let mut union = [false; 9];
            let mut count = 0;

            let ((ip, pi), (jp, pj)) = (&pencilmarks[i], &pencilmarks[j]);

            for a in 0..9 {
                if pi[a] || pj[a] {
                    union[a] = true;
                    count += 1;
                }
            }
            if count == 2 {
                for a in 0..9 {
                    if union[a] {
                        result[a] = Some(vec![*ip, *jp]);
                    }
                }
            }
        }
    }

    // Naked triplets
    for i in 0..pencilmarks.len() {
        for j in (i + 1)..pencilmarks.len() {
            for k in (j + 1)..pencilmarks.len() {
                let mut union = [false; 9];
                let mut count = 0;

                let ((ip, pi), (jp, pj), (kp, pk)) = (&pencilmarks[i], &pencilmarks[j], &pencilmarks[k]);

                for a in 0..9 {
                    if pi[a] || pj[a] || pk[a] {
                        union[a] = true;
                        count += 1;
                    }
                }

                if count == 3 {
                    for a in 0..9 {
                        if union[a] {
                            result[a] = Some(vec![*ip, *jp, *kp]);
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
                    Cell::Empty => write!(f, "_ ")?,
                    Cell::Digit(d) => write!(f, "{} ", d)?,
                    Cell::Pencil(_) => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_sudoku(mut sudoku: Sudoku) -> Option<Sudoku> {
    let mut row_rule = RowRule::default();
    let mut col_rule = ColRule::default();
    let mut box_rule = BoxRule::default();
    let mut rules: Vec<&mut dyn SetRule> = vec![&mut box_rule, &mut row_rule, &mut col_rule];

    let mut counter = 0;
    loop {
        let old_sudoku = sudoku.clone();

        for _ in 0..9 {
            for rule in &mut rules {
                counter += 1;
                rule.update_cells(&mut sudoku)?;
            }
        }

        if old_sudoku == sudoku {
            println!("No progress made after {} iterations", counter);
            break;
        }

        let done = sudoku.iter().all(|row| row.iter().all(|&cell| matches!(extract_cell(cell), Cell::Digit(_))));

        if done {
            println!("Solved in {} iterations", counter);
            break;
        }

        if counter >= 100000 {
            println!("Reached iteration limit: {}", counter);
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

        assert_eq!(extract_cell(value_digit), Cell::Digit(NonZeroU8::new(5).unwrap()));
        assert_eq!(
            extract_cell(value_pencilmark),
            Cell::Pencil([false, true, true, false, true, false, true, false, true])
        );
    }
}
