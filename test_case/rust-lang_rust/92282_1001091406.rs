rust
    pub fn new(rules: HashMap<T, Vec<(T, u64)>>) -> Self {
        let rule = Box::new(move |src: &_| rules.get(src).map(|o| o.to_owned()));
        //                             ^^--------
        Self { rule }
    }
