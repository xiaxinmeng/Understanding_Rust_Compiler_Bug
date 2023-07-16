 rust
fn main() {
    let _x = |x: usize, y: usize| {
        |t: bool| if t { x } else { y }
    };
