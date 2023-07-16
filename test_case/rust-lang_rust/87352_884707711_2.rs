rust
    fn pop_back(&mut self) {
        let rawself = self as *mut List;
        unsafe {
            let rawp : *mut ListElem = &mut (*rawself).tail as *mut ListElem;
            ListElem::remove((*rawp).prev);
        }
    }
