 rust
#![crate_type = "lib"]

#[no_mangle]
fn foo() -> u32 {
    42
}

#[used]
static KEEP: fn() -> u32 = foo;
