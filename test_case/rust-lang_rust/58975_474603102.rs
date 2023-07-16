rust
v.iter().map(|&x: &i32|
    if x < 0 { None }
    else { Some(x) }
).sum()
