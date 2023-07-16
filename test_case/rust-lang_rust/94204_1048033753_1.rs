rust
use static_la::Matmul;
fn main() {
    let l = static_la::MatrixSxS::<f32,2,2>::from([[0.,1.],[2.,3.]]);
    let lt = l.transpose();
    // The errors arises from `l.matmul(&lt);` and a few other operations which change the return type.
    let _ = l.matmul(&lt);
}
