rust
    fn name_from(&self, start: BytePos) -> ast::Name {
        debug!("taking an ident from {:?} to {:?}", start, self.pos);
        Symbol::intern(self.str_from(start))
    }
