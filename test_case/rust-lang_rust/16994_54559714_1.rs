 rust
        self.get_entry(key, |k| key == k).map(|&(_, ref v)| v)
