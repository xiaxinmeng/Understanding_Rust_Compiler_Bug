 rust
fn main() {
    let _x = |x: usize, y: usize| {
        let x = x;
        let y = y;
        |t: bool| if t { x } else { y }
    };
