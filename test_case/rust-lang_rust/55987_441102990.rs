rust
fn get(a: &AtomicWeak<i32>) -> Arc<i32> {
    loop {
        let weak = a.load();
        match weak.upgrade() {
            Some(arc) => return arc,
            None => {
                let arc = Arc::new(0);
                let prev = a.compare_and_swap(&weak, Arc::downgrade(&arc));
                if Weak::ptr_eq(&prev, &weak) {
                    return arc;
                }
            }
        }
    }
}
