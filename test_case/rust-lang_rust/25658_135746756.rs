 rust
macro_rules! e {
    ($($e:expr)*) => ($($e)*);
}

fn main() {
    e!(() "");
    e!("" 4);
}
