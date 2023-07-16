rust
match write!(f, "{}", s) {
    Ok(val) => val,
    Err(e) => return e.into(),
}
