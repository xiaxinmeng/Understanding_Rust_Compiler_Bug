rust
macro_rules! erase {
    ($($tt:tt)*) => {}
}

fn main() {
    erase!(999u8);
}
