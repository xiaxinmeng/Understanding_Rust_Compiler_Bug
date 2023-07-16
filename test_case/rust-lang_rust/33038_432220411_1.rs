rust
    println!("{}", (0..1u32<<30).filter(|&i| i.count_ones() == 15).fold(0u64, |count, _| count + 1));
    println!("{}", (0..1u32<<30).filter(|&i| i.count_ones() == 15).count());
