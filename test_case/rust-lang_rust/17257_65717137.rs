 rust
fn build_car<'a>() -> Box<Vehicle + 'a> {
    box Car
}
