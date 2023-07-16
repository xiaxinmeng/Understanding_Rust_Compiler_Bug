rust
let ok = match not_so_good_success() {
    Ok(err) => return Err(err),
    Err(ok) = ok,
};
