 rust
impl Enumerable for Shape {
    fn values() -> &'static [Shape] {
        static VALUES: &'static [Shape] = [Circle, Square, Triangle];
        VALUES
    }
}
