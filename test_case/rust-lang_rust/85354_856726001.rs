rust
use std::time::Instant;

const N1: usize = 4_096;
const N1B: u32 = N1 as _;
const N2: usize = 5_000;
const K: u64 = 1_234;
const LIM: usize = 10_000_000;

#[derive(Clone)]
pub struct Aux {
    pub sieve: [u64; N1],
    pub primes: [u32; N2],
}

#[inline]
//pub fn foo_1a(data1: &mut [u64], data2: &[u32]) {
pub fn foo_1a(data1: &mut [u64; N1], data2: &[u32; N2]) {
    for i in 0 .. LIM {
        let mut j = 0;
        while j < N1B {
            data1[j as usize] |= K;
            j += data2[i % N2];
        }
    }
}

#[inline]
pub fn foo_1b(aux: &mut Aux) { // SLOW
    for i in 0 .. LIM {
        let mut j = 0;
        while j < N1B {
            aux.sieve[j as usize] |= K;
            j += aux.primes[i % N2];
        }
    }
}

pub fn main() {
    let sieve = [0_u64; N1];
    let primes = [10_u32; N2];

    let mut aux1 = Aux { sieve, primes };
    let mut aux2 = aux1.clone();

    let start = Instant::now();
    foo_1a(&mut aux1.sieve, &aux1.primes);
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("1a {:?}\n", duration);

    //////////// 2b
    let start = Instant::now();
    foo_1b(&mut aux2);
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("1b {:?}\n", duration);
}
