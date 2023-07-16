 Rust
#![feature(test)]
extern crate test;
use test::Bencher;

pub type T = f32;

const MR: usize = 4;
const NR: usize = 4;

macro_rules! loop4 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
        let $i = 2; $e;
        let $i = 3; $e;
    }}
}

/// 4x4 matrix multiplication kernel
///
/// This does the matrix multiplication:
///
/// C ← α A B
///
/// + k: length of data in a, b
/// + a, b are packed
/// + c has general strides
/// + rsc: row stride of c
/// + csc: col stride of c
#[inline(never)]
pub unsafe fn kernel(k: usize, alpha: T, a: *const T, b: *const T,
                     c: *mut T, rsc: isize, csc: isize)
{
    let mut ab = [[0.; NR]; MR];
    let mut a = a;
    let mut b = b;

    // Compute matrix multiplication into ab[i][j]
    for _ in 0..k {
        let v0: [_; MR] = [at(a, 0), at(a, 1), at(a, 2), at(a, 3)];
        let v1: [_; NR] = [at(b, 0), at(b, 1), at(b, 2), at(b, 3)];
        loop4!(i, loop4!(j, ab[i][j] += v0[i] * v1[j]));

        a = a.offset(MR as isize);
        b = b.offset(NR as isize);
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (*c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // set C = α A B
    for i in 0..MR {
        for j in 0..NR {
            c![i, j] = alpha * ab[i][j];
        }
    }
}

#[inline(always)]
unsafe fn at(ptr: *const T, i: usize) -> T {
    *ptr.offset(i as isize)
}

#[test]
fn test_gemm_kernel() {
    let k = 4;
    let mut a = [1.; 16];
    let mut b = [0.; 16];
    for (i, x) in a.iter_mut().enumerate() {
        *x = i as f32;
    }

    for i in 0..4 {
        b[i + i * 4] = 1.;
    }
    let mut c = [0.; 16];
    unsafe {
        kernel(k, 1., &a[0], &b[0], &mut c[0], 1, 4);
        // col major C
    }
    assert_eq!(&a, &c);
}

#[bench]
fn bench_gemm(bench: &mut Bencher) {
    const K: usize = 32;
    let mut a = [1.; MR * K];
    let mut b = [0.; NR * K];
    for (i, x) in a.iter_mut().enumerate() {
        *x = i as f32;
    }

    for i in 0..NR {
        b[i + i * K] = 1.;
    }
    let mut c = [0.; NR * MR];
    bench.iter(|| {
        unsafe {
            kernel(K, 1., &a[0], &b[0], &mut c[0], 1, 4);
        }
        c
    });
}
