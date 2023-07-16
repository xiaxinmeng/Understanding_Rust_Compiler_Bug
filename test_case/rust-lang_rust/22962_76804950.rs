 Rust
        (1..100).map(|x| some_arr(x)).collect::<Vec<_>>().iter()
        .flat_map(|x| x.iter().flat_map(|&x| (0..x))
        .zip( ... ).filter( ... ).count();
