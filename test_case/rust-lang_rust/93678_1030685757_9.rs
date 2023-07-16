rs
fn top_level_used() {
    unsafe {
        unsf();
        unsafe { unsf() }
        unsafe { unsf() }
        unsafe { unsf() }
    }
}
