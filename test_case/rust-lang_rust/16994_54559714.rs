 rust
        self.get_entry(key, |k| key == k).map(|e| {
            let &(_, ref v) = e;
            v
        })
