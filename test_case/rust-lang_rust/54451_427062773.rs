Rust
#![crate_type = "rlib"]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static FOO: i32 = 1;
pub fn foo() -> usize { &FOO as *const _ as usize }
