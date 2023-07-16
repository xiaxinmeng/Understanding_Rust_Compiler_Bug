rust
    fn f(v: Vec<RefCell<u8>>) {
        use std::ops::Index;
        let _t = &mut *v.index(0).borrow_mut();
    }
    