 rust
enum X { A = 0 as isize }

enum Y { A = X::A as isize }

fn main() { }
