Rust
enum Bug {
    V = [Vec::new; { [0].len()  ].len() as isize,
}
