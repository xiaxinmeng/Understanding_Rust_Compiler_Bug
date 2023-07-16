rust
for _ in 0..1 << 7 {
    for len in 21..20_000 {
        let lut = [0, 0, 0, 0, 0, 1, 2, 3];
        input[..len].fill_with(|| *lut.choose(rng).unwrap());    
        sum += quicksort_badpivots(&mut input[..len], |a, b| a < b);