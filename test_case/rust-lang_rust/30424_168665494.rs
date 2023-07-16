 rust
static zz: i32 = 13;
fn foo(x: &i32, y: &i32) -> i32 { let z = &zz; *z + *x + *y }
