rust
/// function docs
fn asdf<
    /// typaram docs
    Asdf: Display
>(
    /// function arg docs
    // ~^ ERROR: expected pattern, found `/// function arg docs`
    thing: Asdf
) -> String {
    format!("{}", thing)
}
