
fn truncate_at_boundary<'a>(s: &'a str, len: usize) -> &'a str {
    let mut len = min(len, s.len());
    while !s.is_char_boundary(len) { len -= 1; }
    &s[0..len]
}
