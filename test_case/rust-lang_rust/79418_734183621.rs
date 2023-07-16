 rust
pub extern "C" fn atomic_usize_func(ptr: &AtomicUsize) -> (bool, usize) {
    let mut old = ptr.load();

    loop {
        let new = calc_new_from_old();
        if !check_something(old, new) {
            break (false, old);
        }

        atomic {
            let _old = ptr.load();
            if old == _old {
                ptr.store(new);
                break (true, old);
            } else {
                old = _old;
            }
        }
    }
}
