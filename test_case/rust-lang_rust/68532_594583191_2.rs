rust
    pub fn foo(&self) where T: Sized {
        let b = [0; Self::N];
        println!("{}", b.len());
    }
