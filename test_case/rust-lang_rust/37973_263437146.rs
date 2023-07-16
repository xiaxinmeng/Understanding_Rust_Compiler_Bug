rust
#![crate_type = "cdylib"]

#[link(name = "native", kind="static")]
extern "C" {
    pub fn static_func2(x: i32) -> i32;
    pub static static_global2: i32;
}

#[no_mangle]
pub extern "C" fn local() {}
