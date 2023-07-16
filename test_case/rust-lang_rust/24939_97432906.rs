 rust
let mut chars = s.chars();
match (chars.next(), chars.next()) {
    (None, None) => // empty
    (Some(c), None) => Ok(c),
    _  => // Too many
}
