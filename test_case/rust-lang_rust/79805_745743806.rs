
    let v = vec![1,2,3];
    let mut vi = v.iter();
    v.iter().fold(vi.next().cloned(), |a, i| a.map(|a| a + i));

    vec![1,2,3].iter().cloned().fold_first(|a, b| a + b);
