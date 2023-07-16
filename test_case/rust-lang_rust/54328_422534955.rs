Rust
fn dyn_trait() -> Box<dyn Send + Send> {
    Box::new(())
}

fn impl_trait() -> impl Send + Send  { }

fn main() {
    dyn_trait();
    impl_trait();
}
