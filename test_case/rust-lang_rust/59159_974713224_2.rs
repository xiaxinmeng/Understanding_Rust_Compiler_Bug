rust
self.password.chars().for_each(|c| {
    let count = *(counts.get(&c).unwrap_or(&0));

    let _ = counts.insert(c, count + 1);
});
