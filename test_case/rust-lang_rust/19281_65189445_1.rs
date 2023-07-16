 rust
    pub fn reserve(&mut self, additional: uint) {
        if self.cap - self.len < additional {
            match self.len.checked_add(additional) {
                None => panic!("Vec::reserve: `uint` overflow"),
                Some(new_cap) => {
                    let amort_cap = new_cap.next_power_of_two();
                    // next_power_of_two will overflow to exactly 0 for really big capacities
                    let capacity = if amort_cap == 0 { new_cap } else { amort_cap };
                    self.grow_capacity(capacity)
                }
            }
        }
    }
