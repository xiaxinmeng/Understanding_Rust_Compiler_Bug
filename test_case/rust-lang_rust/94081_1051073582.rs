rust
fn bar(b: bool) -> impl std::fmt::Debug {
    if b {
        return 42
    }
    let x: u32 = bar(false); // this errors on stable
    99
}
