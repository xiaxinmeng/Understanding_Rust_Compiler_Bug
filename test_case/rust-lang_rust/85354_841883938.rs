Rust
use std::time::Instant;

pub struct Aux {
    pub sieve: [u64;1 << 12],
    pub primes: [u32;6536],
}

pub const MARK_MASK: [u64;64] = {
    let mut res = [0; 64];
    let mut i = 0;
    while i < 64 {
        res[i] = 1 << i;
        i += 1;
    }
    res
};

#[inline(always)]
fn mark_2(arr: &mut [u64], index: usize) { arr[index >> 6] |= MARK_MASK[index & 63usize]; }
#[inline]
const fn test_2(arr: &[u64], index: usize) -> bool { arr[index >> 6] & MARK_MASK[index & 63usize] == 0 }

#[inline]
pub fn update_aux_sieve(aux_sieve: &mut [u64], aux_primes: &[u32]) {
    for i in 0..6536 {
        let mut j = aux_primes[i] * aux_primes[i];
        while j < (1 << 18) { 
            mark_2(aux_sieve, j as usize);
            j += aux_primes[i];
        }
    }
}

#[inline]
pub fn update_aux_struct_sieve(aux: &mut Aux) {
    for i in 0..6536 {
        let mut j = aux.primes[i] * aux.primes[i];
        while j < (1 << 18) { 
            mark_2(&mut aux.sieve, j as usize);
            j += aux.primes[i];
        }
    }
}


pub fn init_aux_primes() -> ([u64;1 << 12], [u32;6536]) {
    let mut aux_sieve:  [u64;1 << 12] = [0;1 << 12];
    let mut aux_primes: [u32;6536] = [1;6536];
    let start = Instant::now();
    println!("Initializing aux primes");
    for i in (3..256).step_by(2) {
        if test_2(&mut aux_sieve, i >> 1usize) {
            for j in (((i*i) >> 1)..32768).step_by(i) {
                mark_2(&mut aux_sieve, j as usize);
            }
        }
    }
    let mut j = 0;
    for i in 8..32768 {
        if test_2(&mut aux_sieve, i) {
            aux_primes[j] = 2 * (i as u32) + 1;
            j += 1;
        }
    }

    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("init_aux_primes took, {:?}\n", duration);
    (aux_sieve, aux_primes)
}

pub fn main() {

    let (sieve, primes)  = init_aux_primes();
    let mut aux = Aux { sieve, primes };

    let start = Instant::now();
    for _ in 0..8192 {
        //update_aux_struct_sieve(&mut aux); // slow
        update_aux_sieve(&mut aux.sieve, &aux.primes); // fast
    }
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("duration {:?}\n", duration);
}

