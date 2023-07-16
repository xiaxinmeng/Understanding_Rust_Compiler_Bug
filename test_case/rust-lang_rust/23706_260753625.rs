
use std::os::raw::c_void;

const MMAP_FAILURE: *mut c_void = -1 as *mut c_void;

fn whatever() -> *mut c_void {
    match -1 as *mut c_void {
        MMAP_FAILURE => 0 as *mut c_void,
        p => p,
    }
}
