rust
    fn f(v: Vec<RefCell<u8>>) {
        let _t = &mut *RefCell::borrow_mut(&v[0]);
    }
    