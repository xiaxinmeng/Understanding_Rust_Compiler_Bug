rust
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {}
}

fn main() {
    log!(Level::Error ,);
}
