rust
macro_rules! erase {
    ($($tt:tt)*) => {}
}

fn main() {
    erase! {
        '\u{FFFFFF}'
    }
}
