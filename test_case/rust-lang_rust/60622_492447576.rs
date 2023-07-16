rust
struct Borked {}
impl Borked {
    fn a(&self) {}
}

fn run_wild<T>(b: &Borked) {
    b.a::<'_, T>();
}
