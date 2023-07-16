rust
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Matrix {
    elements: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self[(row, col)] = value;
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, (row, col): (usize, usize)) -> &f32 {
        &self.elements[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f32 {
        &mut self.elements[row * self.cols + col]
    }
}

fn main() {
    let mut mat = Matrix {
        elements: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        rows: 2,
        cols: 3,
    };
    mat.set(1, 2, 3.0);
    println!("{:?}", mat);
}
