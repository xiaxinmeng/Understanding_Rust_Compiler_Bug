rust
match expr { // Removed `Try::into_result()` here.
    Ok(v) => v,
    Err(e) => return Try::from_error(From::from(e)),
}
