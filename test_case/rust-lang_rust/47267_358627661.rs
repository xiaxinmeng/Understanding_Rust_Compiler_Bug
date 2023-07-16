
#![feature(nll)]

use std::mem::size_of;

macro_rules! isqrt { ($e:expr, $t:ident) => ((($e) as f64).sqrt() as $t)  }

struct SieveBits { bits: Vec<usize> }

impl SieveBits {
    const BPC: usize = size_of::<usize>() * 8;

    fn is_bit_set(bits: &[usize], i: usize) -> bool {
        debug_assert!(i < bits.len() * Self::BPC);
        let offset = i / Self::BPC;
        let mask = 1 << (i % Self::BPC);
        unsafe { (*bits.get_unchecked(offset) & mask) != 0 }
    }

    fn reset_bit(bits: &mut [usize], i: usize) {
        debug_assert!(i < bits.len() * Self::BPC);
        let offset = i / Self::BPC;
        let mask = 1 << (i % Self::BPC);
        unsafe { *bits.get_unchecked_mut(offset) &= !mask; }
    }

    fn new(m: usize) -> Self {
        if m < 2 {
            return Self { bits: vec![] };
        }
        let mut primes = vec![std::usize::MAX; (m / 3) / Self::BPC + 1];
        Self::reset_bit(&mut primes, 0);

        let lim = isqrt!(m, usize) + 1;
        let mut i = 5;
        let mut step = 2;
        while i < lim {
            if Self::is_bit_set(&primes, i / 3) {
                let mut j = i * i;
                let mut step2 = step;
                while j < m {
                    Self::reset_bit(&mut primes, j / 3);
                    j += step2 * i;
                    step2 = 6 - step2;
                }
            }
            i += step;
            step = 6 - step;
        }

        Self { bits: primes }
    }

    fn is_prime(&self, i: usize) -> bool {
        if i == 2 || i == 3 { true }
        else if i % 2 == 0 || i % 3 == 0 { false }
        else { Self::is_bit_set(&self.bits, i / 3) }
    }
}

fn eratosthenes_sieve_u32(limit: usize) -> Vec<u32> {
    assert!(limit <= std::u32::MAX as usize);

    #[inline(always)]
    fn fill_u8(data: &mut [u8], value: u8) {
        unsafe {
            std::ptr::write_bytes(data.as_mut_ptr(), value, data.len());
        }
    }

    const L1D_CACHE_SIZE:u32 = 32_768;

    let mut result = vec![];
    if limit < 2 {
        return result;
    } else {
        result.push(2);
    }

    let lsqrt = isqrt!(limit, u32);

    let mut is_prime = vec![1u8; lsqrt as usize + 1];
    let mut i = 2;
    while i * i <= lsqrt {
        unsafe {
            if *is_prime.get_unchecked(i as usize) != 0 {
                let mut j = i * i;
                while j <= lsqrt {
                    *is_prime.get_unchecked_mut(j as usize) = 0;
                    j += i;
                }
            }
        }
        i += 1;
    }

    let segment_size = lsqrt.max(L1D_CACHE_SIZE);
    let mut s: usize = 3;
    let mut n: usize = 3;

    let mut sieve = vec![1u8; segment_size as usize];
    let mut primes: Vec<u32> = vec![];
    let mut next: Vec<u32> = vec![];
    let mut low: usize = 0;

    while low <= limit {
        fill_u8(&mut sieve, 1);

        let high = limit.min(low + segment_size as usize - 1);

        unsafe {
            while s * s <= high {
                if *is_prime.get_unchecked(s) != 0 {
                    primes.push(s as u32);
                    next.push((s * s - low) as u32);
                }
                s += 2;
            }

            for (i, &p) in primes.iter().enumerate() {
                let k = p * 2;
                let mut j = *next.get_unchecked(i);
                while j < segment_size {
                    *sieve.get_unchecked_mut(j as usize) = 0;
                    j += k;
                }
                *next.get_unchecked_mut(i) = j - segment_size;
            }

            while n <= high {
                if *sieve.get_unchecked(n - low) != 0 {
                    result.push(n as u32);
                }
                n += 2;
            }
        }

        low += segment_size as usize;
    }

    result
}

fn main() {
    fn e60() -> u32 {
        struct DPrime { n: u32, fac: u32 }

        impl DPrime {
            fn new(p: u32) -> Self {
                fn next_pow10(n: u32) -> u32 {
                    let mut p = 10;
                    while n > p { p *= 10; }
                    p
                }

                Self { n: p, fac: next_pow10(p) }
            }
        }

        const LIM: u32 = 10_000;

        let mut primes = vec![DPrime::new(3)];
        primes.extend(eratosthenes_sieve_u32(LIM as usize).iter().skip(3).map(|&p| DPrime::new(p)));

        let sieve = SieveBits::new((LIM * LIM) as usize);

        let are_coprimes = |a: &DPrime, b: &DPrime|
            b.n > a.n &&
            sieve.is_prime((a.n + a.fac * b.n) as usize) &&
            sieve.is_prime((b.n + b.fac * a.n) as usize);

        let mut best_sum = std::u32::MAX;

        for p1 in &primes {
            if p1.n >= best_sum { break; }
            let p2v: Vec<_> = primes.iter().filter(|p2| are_coprimes(p1, p2)).collect();
            for p2 in &p2v {
                if p1.n + p2.n >= best_sum { break; }
                let p3v: Vec<_> = p2v.iter().filter(|p3| are_coprimes(p2, p3)).collect();
                for p3 in &p3v {
                    if p1.n + p2.n + p3.n >= best_sum { break; }
                    let p4v: Vec<_> = p3v.iter().filter(|p4| are_coprimes(p3, p4)).collect();
                    for p4 in &p4v {
                        if p1.n + p2.n + p3.n + p4.n >= best_sum { break; }
                        if let Some(p5) = p4v.iter().find(|p5| are_coprimes(p4, p5)) {
                            let sum = p1.n + p2.n + p3.n + p4.n + p5.n;
                            if sum < best_sum {
                                best_sum = sum;
                            }
                        }
                    }
                }
            }
        }
        best_sum
    }
    assert_eq!(e60(), 26_033);
}


Without nll feature:
time: 0.000; rss: 69MB    MIR borrow checking

With nll feature:
time: 0.203; rss: 70MB    MIR borrow checking
