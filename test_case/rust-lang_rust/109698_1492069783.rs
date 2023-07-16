rust
// Similar to `str::split_at` and `[T]::split_at`
// Panics if `mid` is not on a UTF-8 code point boundary.
fn split_at(&self, mid: usize) -> (&OsStr, &OsStr);
