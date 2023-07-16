
pub struct Matrix {
    data: [[int, ..2], ..2]
}

impl Index<uint, [int, ..2]> for Matrix {
    fn index<'a>(&'a self, index: &uint) -> &'a [int, ..2] {
        &self.data[*index]
    }
}

fn main() {
    let m = Matrix {data: [[1, 0], [0, 1]]};
    let e = m[0][0];    
}
