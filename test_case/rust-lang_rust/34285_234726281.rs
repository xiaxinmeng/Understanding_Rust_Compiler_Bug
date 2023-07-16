 rust
        let mut key = (key, 0);
        loop {
            match self.values.entry(key) {
                Entry::Vacant(e) => {
                    let idx = e.key().1;
                    e.insert(value);
                    return idx;
                },
                Entry::Occupied(e) => {
                    key = e.??? /* What here? */
                    key.1 += 1;
                }
            }
        }
