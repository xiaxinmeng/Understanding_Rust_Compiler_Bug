rust
fn foo<'s, F>(s: &'s i32, f: F) where for<'a> F: FnOnce(&'a i32) {
...
}
