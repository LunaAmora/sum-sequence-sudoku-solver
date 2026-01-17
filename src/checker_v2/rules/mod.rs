use super::Sudoku;

pub mod set;

pub trait Rule {
    fn update_cells(&mut self, sudoku: &mut Sudoku) -> Result<(), ()>;
}
