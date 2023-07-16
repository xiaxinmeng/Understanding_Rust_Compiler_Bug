rust
    let mut v = vec![0, 1, 2];
    let shared = &v;
    let inner = shared.len();
    v.push(inner);
    dbg!(v);
