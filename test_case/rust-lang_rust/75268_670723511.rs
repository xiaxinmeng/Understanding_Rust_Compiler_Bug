rust
pub fn boundcheck(mut lit: &[u8]) -> Option<&[u8]> {
    if let [b'_', rem @ ..] = lit {
        lit = rem;
    }
    Some(lit)
}
