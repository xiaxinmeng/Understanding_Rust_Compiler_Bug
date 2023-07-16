rust
let x = 'a: {
    Ok(match foo {
        Ok(v) => v,
        Err(e) => break 'a Err(e.into()),
    })
};
