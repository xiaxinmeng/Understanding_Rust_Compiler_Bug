
    pub fn into_inner(self) -> T {
        match self.cell.into_inner() {
            Some(x) => x,
            None => match self.init.take() {
                Some(f) => f(),
                None => panic!("`Lazy` instance has previously been poisoned"),
            },
        }
    }
