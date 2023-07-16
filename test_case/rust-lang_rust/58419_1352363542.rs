rust
#[derive(Debug)]
struct Matrix {
  elements: Vec<f32>,
  rows: usize,
  cols: usize,
}

impl Matrix {
  pub fn set(&mut self, row: usize, col: usize, value: f32) {
    self.elements[self.idx(row, col)] = value;  // ERR
  }
  
  fn idx(&self, row: usize, col: usize) -> usize {
    row * self.cols + col
  }
}

fn main() {
  let mut mat = Matrix { elements: vec![1.0,2.0,3.0,4.0,5.0,6.0], rows: 2, cols: 3 };
  mat.set(1,2, 3.0);
  println!("{:?}", mat);
}
