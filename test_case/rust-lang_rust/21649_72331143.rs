 rust
pub fn main() {
    let mut x = [[0f64; 3]; 3];

    for i in 0..3 {
        x[i][0] = 1.0; // the type of this value must be known in this context
    }
}
