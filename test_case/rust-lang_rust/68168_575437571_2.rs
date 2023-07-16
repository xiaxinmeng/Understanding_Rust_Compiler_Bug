rust
if timeout.as_secs() == 0 && timeout.subsec_nanos() == 0 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "cannot set a 0 duration timeout",
    ));
}

if timeout.as_millis() > c_int::max_value() as u128{
    return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "duration timeout overflow",
    ));
}
