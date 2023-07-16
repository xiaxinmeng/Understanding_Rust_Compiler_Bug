rust
    fn pop_back(&mut self) {
        let rawp : *const ListElem = &mut self.tail as *const ListElem;
        unsafe {
            ListElem::remove((*rawp).prev);
        }
    }
