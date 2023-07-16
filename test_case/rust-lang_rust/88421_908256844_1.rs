rust
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use std::ops::Index;

pub struct CellPossibilities;

pub enum CellState<const SQUARE_SIZE: usize> {
    Empty(Option<CellPossibilities>),
}

pub struct Sudoku<const SQUARE_SIZE: usize>;

impl<const SQUARE_SIZE: usize> Sudoku<SQUARE_SIZE>where
    [CellState<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    pub fn random() {
        let CellState::Empty(_) = Self[()];
    }
}

impl<const SQUARE_SIZE: usize> Index<()> for Sudoku<SQUARE_SIZE>
where
    [CellState<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    type Output = CellState<SQUARE_SIZE>;

    fn index(&self, _: ()) -> &Self::Output {
        todo!()
    }
}
