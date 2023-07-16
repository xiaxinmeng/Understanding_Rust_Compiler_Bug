rust
    let mut v = vec![0, 1, 2];
    let shared = &v;
    v.push(shared.len());
    dbg!(v);
