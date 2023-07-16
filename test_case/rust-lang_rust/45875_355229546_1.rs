rust
    pub fn read_raw_bytes(&mut self, s: &mut [u8]) -> Result<(), String> {
        let start = self.position;
        let end = self.position + s.len();
        s.copy_from_slice(&self.data[start .. end]); 
        self.position = end;
        Ok(())
    }
