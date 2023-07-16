rust
#[link(name = "nonexist")]
extern "C" {}

#[no_mangle]
pub extern "C" fn hello() {}
