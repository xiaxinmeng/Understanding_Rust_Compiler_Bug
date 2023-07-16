rust
fn IsReparseTagNameSurrogate(reparse_tag: ULONG) -> bool {
    (reparse_tag & 0x20000000) == 0x20000000
}
pub fn is_symlink(&self) -> bool {
    self.is_reparse_point() && IsReparseTagNameSurrogate(self.reparse_tag)
}
