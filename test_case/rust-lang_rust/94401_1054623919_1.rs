rust
fn get_avg<R: TinyRng + Rng, F: Fn(&mut R, u64) -> u64>(func: F) -> f64 {
    let range = MAX_INSERTION + 1..1 << 28;
    let range_len = range.end - range.start;
    let mut sum = 0.0;
    for len in range {
        let mut rng = R::from_seed(len as u32);
        for _ in 0..N_SWAPS {
            let sample = func(&mut rng, len as u64);
            // make sure this is symmetric
            sum += (sample + 1) as f64 / (len + 1) as f64;
        }
    }
    sum / (range_len * N_SWAPS) as f64
}
