mod sum_sequence;

use std::{
    fmt::Display,
    num::NonZeroU8,
    ops::{Index, IndexMut},
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
fn extract_cell(value: u16) -> Value {
    if value == 0 {
        return Value::Empty;
    }

    if (value & 0x8000) == 0 {
        Value::Digit(NonZeroU8::new((value & 0xF) as u8).unwrap())
    } else {
        Value::Pencil(Mask(value & 0x1FF))
    }
}

type Pos = (usize, usize);
type CellValue = (Pos, Value);
type CellMask = (Pos, Mask);

trait SetRule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Option<()> {
        let mut allowed_digits = Mask(0b111111111);
        let mut pencilmarks: Vec<CellMask> = Vec::new();
        let mut empty_cells: Vec<Pos> = Vec::new();

        let set = self.next_set(sudoku);

        for &(pos, value) in set.iter() {
            match value {
                Value::Empty => empty_cells.push(pos),
                Value::Digit(d) => allowed_digits.set((d.get() - 1) as usize, false),
                Value::Pencil(pm) => pencilmarks.push((pos, pm)),
            }
        }

        if pencilmarks.is_empty() && empty_cells.is_empty() {
            return Some(());
        }

        for pm in &mut pencilmarks {
            for i in 0..9 {
                if !allowed_digits[i] {
                    pm.1.set(i, false);
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
                    pm.set(i, false);
                }
            }
        }

        for (pos, pm) in pencilmarks.iter_mut() {
            let count = pm.0.count_ones();

            let new_value = match count {
                0 => return None,
                1 => pm.0.trailing_zeros() as u16 + 1,
                _ => {
                    let mut pm_value = *pm;
                    pm_value.set(15, true);

                    if pm_value != sudoku[*pos] {
                        pm_value.0
                    } else {
                        continue;
                    }
                }
            };

            sudoku[*pos] = new_value;
        }

        Some(())
    }

    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9];
}

#[derive(Default)]
struct RowRule {
    counter: usize,
}

impl SetRule for RowRule {
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let mut result = [CellValue::default(); 9];

        for (i, res) in result.iter_mut().enumerate() {
            *res = sudoku.cell_value((self.counter, i));
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
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let mut result = [CellValue::default(); 9];

        for (i, res) in result.iter_mut().enumerate() {
            *res = sudoku.cell_value((i, self.counter));
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
    fn next_set(&mut self, sudoku: &Sudoku) -> [CellValue; 9] {
        let box_row = (self.counter / 3) * 3;
        let box_col = (self.counter % 3) * 3;

        let mut result = [CellValue::default(); 9];

        for i in 0..3 {
            for j in 0..3 {
                result[i * 3 + j] = sudoku.cell_value((box_row + i, box_col + j));
            }
        }

        self.counter = (self.counter + 1) % 9;
        result
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

fn get_single_digit(value: u16) -> Option<u16> {
    if value != 0 && value.is_power_of_two() { Some(value.trailing_zeros() as u16 + 1) } else { None }
}

#[derive(Debug, PartialEq, Clone)]
struct Sudoku([[u16; 9]; 9]);

impl Sudoku {
    fn cell_value(&self, pos: Pos) -> CellValue {
        (pos, extract_cell(self[pos]))
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
        for row in &self.0 {
            for &cell in row {
                match extract_cell(cell) {
                    Value::Empty => write!(f, "_ ")?,
                    Value::Digit(d) => write!(f, "{} ", d)?,
                    Value::Pencil(_) => write!(f, ". ")?,
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

fn solve_sudoku(mut sudoku: Sudoku, rules: &mut [&mut dyn SetRule], counter: &mut usize) -> SolveResult {
    loop {
        let old_sudoku = sudoku.clone();

        for _ in 0..9 {
            for rule in &mut *rules {
                *counter += 1;
                if rule.update_cells(&mut sudoku).is_none() {
                    return SolveResult::Unsolvable;
                }
            }
        }

        if old_sudoku == sudoku {
            let mut progress = false;
            for i in 0..9 {
                for j in 0..9 {
                    let mut cell_value = sudoku[(i, j)];
                    if let Value::Pencil(pm) = extract_cell(cell_value) {
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

        let done = sudoku.0.iter().all(|row| row.iter().all(|&cell| matches!(extract_cell(cell), Value::Digit(_))));

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
        let mut rules: Vec<&mut dyn SetRule> = vec![&mut box_rule, &mut col_rule, &mut row_rule];

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

        assert_eq!(extract_cell(value_digit), Value::Digit(NonZeroU8::new(5).unwrap()));
        assert_eq!(extract_cell(value_marks), Value::Pencil(Mask(0b1_0101_0110)));
    }

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
