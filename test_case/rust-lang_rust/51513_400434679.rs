
fn foo<'a: 'static>(x: for<'a> fn(&'a u32)) { }
