rust
#[inline]
pub const fn is_ascii_punctuation(&self) -> bool {
   matches!(*self, b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~')
}
