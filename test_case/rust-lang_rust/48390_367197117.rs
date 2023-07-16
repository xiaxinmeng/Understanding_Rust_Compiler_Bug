rust
fn foo<R: Read>(r: R) { }

fn bar<R: Read + ?Sized>(r: &mut R) { foo(r) }  // ok
