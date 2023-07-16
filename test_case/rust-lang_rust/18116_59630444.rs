 rust
fn foo<T>(f: &mut T) where T: <for 'a> FnMut(&'a mut (Reader + 'static)) { ... }
