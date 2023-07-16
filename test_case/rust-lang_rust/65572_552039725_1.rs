rust
    let x = match x {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
