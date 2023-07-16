rust
    let mut hasher = RandomState::new().build_hasher();
    Path::new("a").hash(&mut hasher);
    dbg!(hasher.finish());
