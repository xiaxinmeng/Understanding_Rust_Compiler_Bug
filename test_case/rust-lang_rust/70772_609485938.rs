rust
let mut param_name = vec![];
s
    .by_ref()
    .take_while(|b| !matches!(b, b';' | b'='))
    .read_to_end(&mut param_name)?;
