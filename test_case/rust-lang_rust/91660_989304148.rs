
    fn hash<H: Hasher>(&self, h: &mut H) {
        (((self.krate.as_u32() as u64) << 32) | (self.index.as_u32() as u64)).hash(h)
    }
