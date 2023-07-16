rust
fn lang_start<T: Termination + 'static>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
