rust
#[derive(Default)]
struct A {
    #[default("Instant::now")]
    last: Instant,
}
