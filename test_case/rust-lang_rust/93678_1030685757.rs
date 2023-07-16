rs
unsafe fn unsf() {}

fn inner_ignored() {
    unsafe {
        #[allow(unused_unsafe)]
        unsafe {
            unsf()
        }
    }
}
