 rust
fn main() {
    for
        &1i8
//~^  old-ERROR refutable pattern in `for` loop binding
//~^^ new-ERROR non-exhaustive patterns: `Some(&_)` not covered
        in [1i8].iter() {}
}
