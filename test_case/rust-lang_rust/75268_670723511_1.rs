rust
pub fn boundcheck(mut lit: &[u8]) -> Option<&[u8]> {
    if let Some((&b'_', rem)) = lit.split_first() {
        lit = rem;
    }
    Some(lit)
}
