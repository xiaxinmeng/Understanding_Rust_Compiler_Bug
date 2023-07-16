rust
let res: [u32; 2] = [1u32, 2, 3, 4]
    .into_iter_fixed()
    .zip([4, 3, 2, 1])
    .map(|(a, b)| a + b)
    .skip::<1>()
    .take::<2>()
    .collect();
