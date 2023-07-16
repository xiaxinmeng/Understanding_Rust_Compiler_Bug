rust
match res {
    Ok(val) => val,
    Err(err) => return Some(Err(err)),
}
