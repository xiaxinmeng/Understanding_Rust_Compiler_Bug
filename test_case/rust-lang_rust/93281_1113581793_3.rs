rust
    let u: Vec<_> = vec![1, 2].iter().map(|x| (1, S(*x))).collect();
    let u: Vec<_> = vec![1, 2].iter().map(|x| (x, S(1))).collect();
    let u: Vec<_> = vec![1, 2].iter().map(|x| *x).collect();
    let u: Vec<_> = vec![1, 2].iter().map(|x| (*x, *x)).collect();
    let u: Vec<_> = vec![(&1, S(1)), (&2, S(2))];
