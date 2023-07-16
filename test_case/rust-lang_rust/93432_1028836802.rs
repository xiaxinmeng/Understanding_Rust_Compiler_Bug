rust
fn write_isize(&mut self, i: isize) {
        ...
        let value = i as u64;
        if value < 0xFF {
             self.write_u8(value as u8);
        } else {
            self.write_u8(0xFF);
            hash_value(..., value.to_le());
       }
    }
