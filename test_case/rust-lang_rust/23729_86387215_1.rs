 rust
        impl Iterator for Recurrence {
            type Item = u64;

            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = (self.mem[0] + self.mem[1]);
                    self.mem[0] = self.mem[1];
                    self.mem[1] = next_val;
                    Some(next_val)
                }
            }
        }
