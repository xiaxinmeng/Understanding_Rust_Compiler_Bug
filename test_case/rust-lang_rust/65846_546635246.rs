rust
macro_rules! m {
    ($e:expr) => { dbg!(0); };
    (lab as $e:expr, foo) => { dbg!(1); };
}

fn main() {
    m!(lab as 1, foo);
}
