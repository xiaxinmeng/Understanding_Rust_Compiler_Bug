rust
fn foo<F: for<'a> FnOnce<(&'a u8,), Output: Future<Output = u8> + 'a>>(f: F)
