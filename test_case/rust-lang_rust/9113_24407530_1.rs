 rust
    pub fn read3<'a>(&'a mut self, key: int) -> Option<&'a Data> {
        self.cache.find_or_fallback_with(
            key,
            (),

            // found:
            |_key, data, _a| { let data : &'a Data = data; Some(data) },

            // fallback:
            |cache, _key, _a| {
                match self.db.find(&key) {
                    Some(data) => {
                        let result: &Data = cache.find_or_insert(key, data.clone());
                        Some(result)
                    },
                    None => None
                }
            })
    }

