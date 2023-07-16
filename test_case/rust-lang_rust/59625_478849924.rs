Rust
#[inline(always)]
unsafe fn raw_copy(&self) -> Self {
    va_copy(self)
}
