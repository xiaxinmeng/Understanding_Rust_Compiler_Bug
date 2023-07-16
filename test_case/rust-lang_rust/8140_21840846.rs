
fn lock(...) {
    if self.l.is_null() { // need barrier here?
        let attempted_thing = rust_create_little_lock();
        compare_and_swap(&mut self.l, attempted_thing, ptr::null()) // which barrier?
    }
    // actual lock routine follows
}
