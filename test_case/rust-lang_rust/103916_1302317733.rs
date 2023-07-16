
fn push(&mut self, value: T) -> bool {
        let mut head  = unsafe {
            std::ptr::read_volatile(&self.idx_head) + 1
        };
        let tail = unsafe {
            std::ptr::read_volatile(&self.idx_tail)
        };
        if head == Size {
            head = 0;
        }
        if head == tail {
            return false;
        }
        self.m_data[self.idx_head] = value;
        unsafe {
            std::ptr::write_volatile(&mut self.idx_head, head);
        }
        return true;
    }

    fn pop(&mut self) -> Option<&T> {
        let mut tail = unsafe {
            std::ptr::read_volatile(&self.idx_tail)
        };
        let head  = unsafe {
            std::ptr::read_volatile(&self.idx_head)
        };
        if head == tail {
            return None;
        }
        let res = &self.m_data[tail];
        tail += 1;
        if tail == Size {
            tail = 0;
        }
        unsafe {
            std::ptr::write_volatile(&mut self.idx_tail, tail);
        }
        return Some(res);
    }
