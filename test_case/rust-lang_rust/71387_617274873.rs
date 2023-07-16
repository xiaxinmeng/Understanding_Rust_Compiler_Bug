
let primes = &mut [0; 1000];
let primes = (2_u32 .. 1000)
    .filter(|&b| is_prime(b))
    .collect_slice(primes);
