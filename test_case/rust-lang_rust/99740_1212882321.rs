rust
#![feature(linkage)]
use std::ffi::c_void;
extern "C" {
    #[linkage = "extern_weak"]
    static posix_spawn_file_actions_addchdir_np: *const c_void;
}
fn main() {
    println!("{}", unsafe {
        posix_spawn_file_actions_addchdir_np.is_null()
    });
}
