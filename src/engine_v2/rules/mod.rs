use super::Sudoku;

mod cage;
mod palindrome;
mod set;

pub use cage::CageRule;
pub use palindrome::PalindromeRule;
pub use set::{r#box::BoxRule, cage::CageRule as SetCageRule, col::ColRule, row::RowRule};

pub trait Rule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()>;
}
