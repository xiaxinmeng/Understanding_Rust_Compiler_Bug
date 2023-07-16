rs

#![deny(unused_unsafe)]

fn top_level_ignored() {
    #[allow(unused_unsafe)]
    unsafe {
        #[deny(unused_unsafe)]
        {
            unsafe { unsf() } //~ ERROR: unnecessary `unsafe` block
            unsafe { unsf() } //~ ERROR: unnecessary `unsafe` block
            unsafe { unsf() } //~ ERROR: unnecessary `unsafe` block
        }
    }
}
