 rust
macro_rules! is_match {
    ($value:expr, $($pattern:pat)|+) => {
        match $value {
            $($pattern)|+ => true,
            _ => false
        }
    }
}
