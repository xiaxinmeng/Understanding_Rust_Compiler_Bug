rust
fn bar(b: bool) -> impl std::fmt::Debug {
    if b {
        return 42
    }
    fn private_helper() {
        let x: u32 = bar(false); // Does this work now, since private_helper is "within" at least the lexical defining scope?
    }
    99
}
