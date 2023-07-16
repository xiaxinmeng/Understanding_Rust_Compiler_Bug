
use std::time::Instant;

#[derive(Clone)]
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
pub fn foo_1a(aux_sieve: &mut [u64], _aux_primes: &[u32]) {
    for j in 0..(1<<18) {
      mark_2(aux_sieve, j as usize);
    }
}

#[inline]
pub fn foo_1b(aux: &mut Aux) {
    for j in 0..(1<<18) {
      mark_2(&mut aux.sieve, j as usize);
    }
}

#[inline]
pub fn foo_2a(aux_sieve: &mut [u64], aux_primes: &[u32]) {
    for i in 0..6536 {
        let mut j = aux_primes[i] * aux_primes[i];
        while j < (1 << 18) {
            mark_2(aux_sieve, j as usize);
            j += aux_primes[i];
        }
    }
}

#[inline]
pub fn foo_2b(aux: &mut Aux) { // SLOW
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
    for i in (3..256).step_by(2) {
        for j in (((i*i) >> 1)..32768).step_by(i) {
            mark_2(&mut aux_sieve, j as usize);
        }
    }
    
    let mut aux_primes: [u32;6536] = [1;6536];
    for j in 0..6536 {
      aux_primes[j] = 2 * (j as u32) + 1;
    }

    (aux_sieve, aux_primes)
}

pub fn main() {

    let (sieve, primes)  = init_aux_primes();
    let mut aux1 = Aux { sieve, primes };
    let mut aux2 = aux1.clone();
    let mut aux3 = aux1.clone();
    let mut aux4 = aux1.clone();
    
    //////////// 1a
    let start = Instant::now();
    for _ in 0..4096 {
        foo_1a(&mut aux1.sieve, &aux1.primes);
    }
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("1a {:?}\n", duration);
    
    //////////// 1b
    let start = Instant::now();
    for _ in 0..4096 {
        foo_1b(&mut aux2);
    }
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("1b {:?}\n", duration);

    //////////// 2a
    let start = Instant::now();
    for _ in 0..4096 {
        foo_2a(&mut aux3.sieve, &aux3.primes);
    }
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("2a {:?}\n", duration);

    //////////// 2b
    let start = Instant::now();
    for _ in 0..4096 {
        foo_2b(&mut aux4);
    }
    let end = Instant::now();
    let duration = end.duration_since(start);
    eprintln!("2b {:?}\n", duration);
    
}
