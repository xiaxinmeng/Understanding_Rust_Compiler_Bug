 rust
        fn short_name(self, s: &str) ⟶  Flag {
            Flag {
                name: self.name,
                desc: self.desc,
                short_name: some(s),
                max_count: self.max_count,
                banner: self.banner,
                value: self.value
            }
        }
