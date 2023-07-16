rust
#[inline]
pub const fn is_ascii_punctuation(&self) -> bool {
   matches!(*self, b'!'..=b'@' | b'['..=b'`' | b'{'..=b'~') && !matches!(*self, b'0'..=b'9')
}
