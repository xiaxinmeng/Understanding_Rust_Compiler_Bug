rust
match Carrier::translate(res) {
    Ok(val) => val,
    Err(err) => return Carrier::from_error(From::from(err)),
}
