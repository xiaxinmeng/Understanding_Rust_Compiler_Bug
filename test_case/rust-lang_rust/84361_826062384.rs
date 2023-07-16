rust
    pub fn next_multiple(&mut self, n: usize) -> Option<&str> {
        let opt = self.next();
        match opt {
            None => return None,
            Some(txt) if txt.len() % n == 0 => return Some(txt),
            Some(_) => (),
        }
        std::mem::drop(opt);
        self.next()
    }
