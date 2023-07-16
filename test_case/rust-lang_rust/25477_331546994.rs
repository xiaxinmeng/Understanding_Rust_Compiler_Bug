
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|x| x % 2 == 0);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.drain_filter(|x| *x % 2 != 0).count();
    println!("{:?}", v);
