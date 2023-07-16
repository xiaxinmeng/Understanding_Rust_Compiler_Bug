rust
    pub fn get_or_insert_owned<Q: ?Sized>(&mut self, value: &Q) -> &T
        where T: Borrow<Q>,
              Q: Hash + Eq + ToOwned<Owned = T>,
    {
        self.map.raw_entry_mut().from_key(value)
            .or_insert_with(|| (value.to_owned(), ())).0
    }
