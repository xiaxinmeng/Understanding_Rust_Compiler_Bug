rust
pub fn replace_large(dest: &mut Option<[u8; 127]>) -> Option<[u8; 127]> {
    replace(dest, None)
}
