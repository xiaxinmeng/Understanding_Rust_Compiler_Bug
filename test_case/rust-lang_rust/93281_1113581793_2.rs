rust
    let u: Vec<_> = vec![1, 2].iter().map(|x| S(*x)).collect();
    let u: Vec<_> = vec![S(1), S(2)];
