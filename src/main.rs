/// Limited cage pair combinations solver.
mod engine_v1;

/// Experimental full sudoku solver.
mod engine_v2;

use engine_v2::{SolveResult, Sudoku, solve_sudoku};
use std::io::Result;

fn main() -> Result<()> {
    if true { engine_v1::generate() } else { engine_v2() }
}

fn engine_v2() -> Result<()> {
    let sudoku = Sudoku::default();
    let result = solve_sudoku(sudoku, true);

    match result {
        SolveResult::Solved(_c, _s) => todo!(),
        SolveResult::Unsolvable => todo!(),
        SolveResult::Stuck(_c) => todo!(),
        SolveResult::LimitReached(_c, _s) => todo!(),
    }
}
