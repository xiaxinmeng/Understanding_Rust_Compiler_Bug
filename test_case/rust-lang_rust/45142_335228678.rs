rust
fn a_function<'a, R: io::Read>(value: &'a mut EventReader<R>) -> Result<Other, ()> {
    Other::try_from(value)
}
