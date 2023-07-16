rust
    fn intern_from(&self, start: BytePos) -> Symbol {
        debug!("taking an ident from {:?} to {:?}", start, self.pos);
        Symbol::intern(self.str_from(start))
    }
