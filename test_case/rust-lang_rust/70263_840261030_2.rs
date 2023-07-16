
fn foo<F>(f: F) where F: for<'a> FnOnce(&'a i32) -> &'a i32 {}
