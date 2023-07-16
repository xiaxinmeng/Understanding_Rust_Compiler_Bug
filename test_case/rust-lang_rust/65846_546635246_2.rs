rust
macro_rules! m {
    ($e:expr) => { dbg!(0); };
    (box $e:expr, foo2) => { dbg!(1); };
}

fn main() {
    m!(box 1, foo);
}
