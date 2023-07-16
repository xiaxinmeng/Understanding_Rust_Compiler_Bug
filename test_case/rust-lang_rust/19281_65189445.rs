 rust
    pub fn reserve(&mut self, additional: uint) {
        if self.cap - self.len < additional {
            match self.len.checked_add(additional) {
                None => panic!("Vec::reserve: `uint` overflow"),
                // if the checked_add
                Some(new_cap) => {
                    let amort_cap = new_cap.next_power_of_two();
                    // next_power_of_two will overflow to exactly 0 for really big capacities
                    if amort_cap == 0 {
                        self.grow_capacity(new_cap);
                    } else {
                        self.grow_capacity(amort_cap);
                    }
                }
            }
        }
    }
