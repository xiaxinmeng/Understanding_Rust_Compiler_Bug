rust
fn from(e: std::convert::Infallible) -> Box<dyn std::error::Error> {
    Box::new(e)
}
