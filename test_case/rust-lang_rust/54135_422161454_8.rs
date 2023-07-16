 rust
// bar/src/lib.rs
#[no_mangle]
fn foo() {}

#[used]
static KEEP: fn() = foo;
