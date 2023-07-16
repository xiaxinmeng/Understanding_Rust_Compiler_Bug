rs
#![deny(unused_unsafe)]

fn granularity() {
    unsafe { //~ ERROR: unnecessary `unsafe` block
        unsafe { unsf() }
        unsafe { unsf() }
        unsafe { unsf() }
    }
}
