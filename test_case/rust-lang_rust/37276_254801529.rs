 rust
#![feature(test)]
extern crate test;
use test::Bencher;

pub type T = f32;

const MR: usize = 8;
const NR: usize = 8;

macro_rules! loop4 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
        let $i = 2; $e;
        let $i = 3; $e;
    }}
}

macro_rules! loop8 {
    ($i:ident, $e:expr) => {{
        let $i = 0; $e;
        let $i = 1; $e;
        let $i = 2; $e;
        let $i = 3; $e;
        let $i = 4; $e;
        let $i = 5; $e;
        let $i = 6; $e;
        let $i = 7; $e;
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
pub unsafe fn kernel(k: usize, alpha: T, a: *const T, b: *const T,
                     c: *mut T, rsc: isize, csc: isize)
{
    let mut ab: [[T; NR]; MR];

    enum Method {
        ArrayLit,
        Loop,
        Assignments,
        Zeroed,
    }
    let method = Method::Assignments;

    match method {
        Method::ArrayLit => {
            ab = [[0.; NR]; MR];
        }
        Method::Loop => {
            ab = ::std::mem::uninitialized();
            for i in 0..MR {
                for j in 0..NR {
                    ab[i][j] = 0.;
                }
            }
        }
        Method::Assignments => {
            ab = ::std::mem::uninitialized();
            loop8!(i, loop8!(j, ab[i][j] = 0.));
        }
        Method::Zeroed => {
            ab = ::std::mem::zeroed();
        }
    }
    let mut a = a;
    let mut b = b;

    // Compute matrix multiplication into ab[i][j]
    for _ in 0..k {
        loop8!(i, loop8!(j, ab[i][j] += at(a, i) * at(b, j)));

        a = a.offset(MR as isize);
        b = b.offset(NR as isize);
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (*c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // set C = α A B
    loop8!(j, loop8!(i, c![i, j] = alpha * ab[i][j]));
}

#[inline(always)]
unsafe fn at(ptr: *const T, i: usize) -> T {
    *ptr.offset(i as isize)
}

#[bench]
fn bench_gemm(bench: &mut Bencher) {
    const K: usize = 128;
    let mut a = [1.; MR * K];
    let mut b = [0.; NR * K];
    for (i, x) in a.iter_mut().enumerate() {
        *x = i as T;
    }

    for i in 0..NR {
        b[i + i * K] = 1.;
    }
    let mut c = [0.; NR * MR];
    bench.iter(|| {
        unsafe {
            kernel(K, 1., &a[0], &b[0], &mut c[0], 1, MR as isize);
        }
        c
    });
}
