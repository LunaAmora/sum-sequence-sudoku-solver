use super::Sudoku;

mod cage;
mod palindrome;
mod set;

pub use set::{r#box::BoxRule, col::ColRule, row::RowRule};

pub trait Rule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()>;
}
