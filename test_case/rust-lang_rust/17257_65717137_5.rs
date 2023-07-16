 rust
fn build_car<'a>() -> Box<Vehicle + 'a> {
    loop {
        return box Car as Box<Vehicle>;
    }
}
