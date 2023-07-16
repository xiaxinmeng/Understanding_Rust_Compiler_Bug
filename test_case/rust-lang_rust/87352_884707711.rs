rust
    fn pop_back(&mut self) {
        let rawp : *const *mut ListElem = &mut self.tail.prev as *const *mut ListElem;
        unsafe {
            ListElem::remove(*rawp);
        }
    }
