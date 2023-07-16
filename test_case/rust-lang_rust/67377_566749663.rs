Rust
enum Bug {
    V = [Vec::new; { [].len()  ].len() as isize,
}
