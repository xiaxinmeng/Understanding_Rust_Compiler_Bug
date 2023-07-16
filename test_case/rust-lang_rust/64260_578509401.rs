rust
...
impl Foo {
    fn new() -> Option<Self> {
        step_1_check().and_then(|| step_2_check().map(|| Self { /* ... */ }))
    }
}
