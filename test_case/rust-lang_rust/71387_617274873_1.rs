
let primes = &mut [0; 1000];
let mut index = 0;
for b in 2_u32 .. 1000 {
    if is_prime(b) {
        primes[index] = b;
        index += 1;
    }
}
let primes = &[.. index];
