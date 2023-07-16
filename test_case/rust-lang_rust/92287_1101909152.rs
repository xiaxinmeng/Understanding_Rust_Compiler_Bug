rust
    pub fn as_slice(&self) -> Option<&'a [T]> {
        if self.finished { None } else { Some(&self.v) }
    }
