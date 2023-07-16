 rust
let seed_rng = XorShiftRng::from_seed([1, 2, 3, 4]);
let your_real_rng: IsaacRng = seed_rng.gen();
