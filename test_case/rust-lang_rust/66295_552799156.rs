rust
macro_rules! foo {
    ($($foo:expr),+) => {
        ($($foo),*)
    }
}

fn main() {
    let _a = foo!(3, 4);
    let _b = foo!(3);
}
