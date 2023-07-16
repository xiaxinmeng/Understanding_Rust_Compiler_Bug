rust
if value == Zero::zero() {
    return Err(Error::invalid_value("expected a non-zero value"))
}
unsafe {
    Ok(NonZero::new(value))
}
