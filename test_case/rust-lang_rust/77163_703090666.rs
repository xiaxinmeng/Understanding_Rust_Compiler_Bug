rust
#[derive(Default)]
pub struct Matrix {
    rows: Vec<Option<u32>>,
}

impl Matrix {
    fn get(&self, row: usize) -> Option<&u32> {
        if let Some(Some(row)) = self.rows.get(row) { Some(row) } else { None }
    }
}

fn main() {
    let m  = Matrix::default();
    println!("{:?}", m.get(0));
}
