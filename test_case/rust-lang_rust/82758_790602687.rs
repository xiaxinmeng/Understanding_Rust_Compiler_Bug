rust
use libc;

extern "C" {
    fn __wasilibc_find_relpath_alloc(
        path: *const libc::c_char,
        abs_prefix: *mut *const libc::c_char,
        relative_path: *mut *const libc::c_char,
        relative_path_len: libc::size_t,
        can_realloc: libc::c_int,
    ) -> libc::c_int;
}

fn main() {
    unsafe {
        let _ = __wasilibc_find_relpath_alloc(
            0 as *const libc::c_char,
            0 as *mut *const libc::c_char,
            0 as *mut *const libc::c_char,
            0,
            0,
        );
    }
    let _ = std::fs::File::open("/proxy-wasm-sandbox/regression.txt");
}
