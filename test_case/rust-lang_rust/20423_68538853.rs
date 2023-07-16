 rust
fn foo(a: *mut int, b: *mut int) {}

foo(&mut x, &mut x); // error
foor(&mut x, &mut x as *mut int); // error
foor(&mut x as *mut int, &mut x); // ok
