rust
fn unclear_compiler_error<'a>(f: &dyn Fn(&'a i32) -> Option<&i32>) -> i32 {
    242
}
