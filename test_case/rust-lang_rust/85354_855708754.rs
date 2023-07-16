
use std::time::Instant;

#[derive(Clone)]
pub struct Aux {
    pub sieve: [u64;4096],
    pub primes: [u32;5000],
}

#[inline]
pub fn foo_1a(aux_sieve: &mut [u64], aux_primes: &[u32]) {
    for i in 0..10_000_000 {
        let mut j = 0;
        while j < 4096 {
            aux_sieve[j as usize] |= 1234;
            j += aux_primes[i%5000];
        }
    }
}

#[inline]
pub fn foo_1b(aux: &mut Aux) { // SLOW
    for i in 0..10_000_000 {
        let mut j = 0;
        while j < 4096 {
            aux.sieve[j as usize] |= 1234;
            j += aux.primes[i%5000];
        }
    }
}

pub fn main() { 
    let sieve:  [u64;4096] = [0;4096];
    let primes: [u32;5000] = [10;5000];

    let mut aux1 = Aux { sieve, primes };
    let mut aux2 = aux1.clone();
    
    //////////// 2a
    let start = Instant::now();
    foo_1a(&mut aux1.sieve, &aux1.primes);
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("1a {:?}\n", duration);

    //////////// 2b
    let start = Instant::now();
    foo_1b(&mut aux2);
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("2b {:?}\n", duration);
}

