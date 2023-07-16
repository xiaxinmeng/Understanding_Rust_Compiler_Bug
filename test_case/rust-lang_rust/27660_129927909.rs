 rust
macro_rules! expr_identity {
    ($e:expr) => { $e }
}
macro_rules! apply_op {
    ( $op:tt ) => { expr_identity!(4 $op 5)};
}

fn main() {
    let _ = apply_op!(/);
}
