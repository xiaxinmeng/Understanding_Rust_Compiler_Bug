rust
let f = match file {
    Ok(f) => f,
    Err(_) => return Err(Error::ConfigLoadFail),
};
