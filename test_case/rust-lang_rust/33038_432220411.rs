rust
    println!("{}", (0..1u32<<30).fold(0, |paths, i| if i.count_ones() == 15 {paths + 1} else {paths}));
    println!("{}", (0..1u32<<30).filter(|&i| i.count_ones() == 15).fold(0, |count, _| count + 1));
