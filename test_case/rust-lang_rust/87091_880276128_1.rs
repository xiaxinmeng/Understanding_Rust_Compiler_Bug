rust
    (0..100000).skip(100).cycle().skip(100)
      .zip((0..100000).cycle().skip(10))
      .map(|(a,b)| a+b)
      .skip(100000)
      .take(1000000)
      .sum()
