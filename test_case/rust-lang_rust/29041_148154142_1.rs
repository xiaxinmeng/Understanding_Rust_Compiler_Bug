 rust
mod tests {
    use AtomicCell;
    use std::thread;
    use std::rc::Rc;

    static CELL: AtomicCell<Rc<u32>> = AtomicCell::new();

    #[test]
    fn it_works() {
        let rc = Rc::new(0); // my `Rc` is here
        CELL.set(rc.clone());
        for _ in 0..10 {
            thread::spawn(move || {
                let rc = CELL.spin_lock(|rc| rc.clone()); // ... and here (ok, this one is protected)
                rc.clone(); // ... and here (this one isn't)
            });
        }
        let _ = rc.clone(); // ... and here (nor is this one)
    }
}
