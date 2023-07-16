rust
pub fn boundcheck(mut lit: &[u8]) -> Option<&[u8]> {
    if let Some(&b'_') = lit.first() {
        lit = &lit[1..];
    }
    Some(lit)
}
