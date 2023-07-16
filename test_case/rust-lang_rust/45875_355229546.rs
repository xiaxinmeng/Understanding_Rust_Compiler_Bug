rust
    pub fn read_raw_bytes(&mut self, s: &mut [u8]) -> Result<(), String> {
        let len = s.len();

        self.position += len;

        s.copy_from_slice(&self.data[0..len]); 
        // ^^^ always reads from the beginning, not from `position`

        Ok(())
    }
