 rust
match f {
    Some(a) => {}
    _ if f.as_mut().is_some() {}
    _ => {}
}
