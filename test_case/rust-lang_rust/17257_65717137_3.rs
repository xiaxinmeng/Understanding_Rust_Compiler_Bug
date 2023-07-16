 rust
fn build_car<'a>() -> Box<Vehicle + 'a> {
    return box Car as Box<Vehicle>;
}
