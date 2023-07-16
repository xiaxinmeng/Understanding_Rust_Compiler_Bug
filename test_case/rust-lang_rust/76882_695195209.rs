rust
fn erase<I: Trait>(x: I) -> impl Trait + 'static {
    x
}
