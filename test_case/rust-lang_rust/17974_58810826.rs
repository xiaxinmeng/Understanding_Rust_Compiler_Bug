 rust
#[lang="fail_bounds_check"]
fn fail_bounds_check(file_line: &(&'static str, uint), index: uint, len: uint) -> ! {
    // This should probably do something more intelligent
    loop {}
}
