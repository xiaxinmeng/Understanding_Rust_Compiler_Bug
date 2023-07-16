rust
macro_rules! m {
    ($p: path) => {}
}

fn main() {
    m!(a<b>); // OK
    m!(a::<b>); // FAIL
}
