rust
let mut counts: HashMap<char, u32> = HashMap::new();

self.password
    .chars()
    .for_each(|c| match counts.get(&c) {
        None => {
            counts.insert(c, 1);
        }
        Some(t) => {
            let _ = counts.insert(c, *t + 1);
        }
    });

