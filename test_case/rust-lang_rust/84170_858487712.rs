rust
struct Wrapper {
    value: Box<dyn std::any::Any>,
}

const fn newv() -> Vec<Wrapper> {
    Vec::new()
}
