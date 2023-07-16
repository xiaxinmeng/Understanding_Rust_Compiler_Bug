 rust
macro_rules! foo {
    ($y:expr) => ({
        $y = 2;
    })
}

fn main() {
    let x = 1;
    foo!(x);
}
