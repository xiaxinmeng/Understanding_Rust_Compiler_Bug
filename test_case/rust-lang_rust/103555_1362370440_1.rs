rs
#![allow(dead_code)]
#![feature(array_zip, maybe_uninit_array_assume_init, maybe_uninit_uninit_array)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::OsRng, Rng};
use std::mem::MaybeUninit;

macro_rules! defbenches {
    ($c:ident, $fn:path, $intype:ty, [$($len:literal),+$(,)?]) => {{
        $(defbenches!(@ops $c, $fn, $intype, $len);)+
    }};
    (@ops $c:ident, $fn:path, $intype:ty, $len:literal) => {{
        let a: [$intype; $len] = OsRng.gen();
        let b: [$intype; $len] = OsRng.gen();
        defbenches!(@final $c, $fn, $intype, $len, +, a, b);
        defbenches!(@final $c, $fn, $intype, $len, -, a, b);
        defbenches!(@final $c, $fn, $intype, $len, *, a, b);
        defbenches!(@final $c, $fn, $intype, $len, &, a, b);
        defbenches!(@final $c, $fn, $intype, $len, |, a, b);
        defbenches!(@final $c, $fn, $intype, $len, ^, a, b);
    }};
    (@final $c:ident, $fn:path, $intype:ty, $len:literal, $op:tt, $a:ident, $b:ident) => {{
        $c.bench_function(
            concat!(stringify!($fn), " | [", stringify!($intype), "; ", stringify!($len), "] | ", stringify!($op)),
            |b| {
                b.iter(|| black_box($fn($a, $b, |a, b| a $op b)));
            },
        );
    }};
}

fn benchmark_primitives(c: &mut Criterion) {
    defbenches!(c, zip_with, u8, [8, 16, 32, 64, 128]);
    defbenches!(c, zip_map_std, u8, [8, 16, 32, 64, 128]);
    defbenches!(c, zip_map_fl, u8, [8, 16, 32, 64, 128]);

    defbenches!(c, zip_with, u16, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_std, u16, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_fl, u16, [8, 16, 32, 64, 128, 256, 512]);

    defbenches!(c, zip_with, u32, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_std, u32, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_fl, u32, [8, 16, 32, 64, 128, 256, 512]);

    defbenches!(c, zip_with, u64, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_std, u64, [8, 16, 32, 64, 128, 256, 512]);
    defbenches!(c, zip_map_fl, u64, [8, 16, 32, 64, 128, 256, 512]);
}

struct ZipWithGuard<T, U, const N: usize> {
    a: *mut [T; N],
    b: *mut [U; N],
    moved: usize,
}

impl<T, U, const N: usize> Drop for ZipWithGuard<T, U, N> {
    fn drop(&mut self) {
        if needs_drop::<T>() || needs_drop::<U>() {
            for i in self.moved + 1..N {
                unsafe {
                    drop(self.a.cast::<T>().add(i).read());
                    drop(self.b.cast::<U>().add(i).read());
                }
            }
        }
    }
}

pub fn zip_with<T, U, V, F: FnMut(T, U) -> V, const N: usize>(
    a: [T; N],
    b: [U; N],
    mut f: F,
) -> [V; N] {
    let aref = &a as *const _ as *mut [ManuallyDrop<T>; N];
    forget(a);
    let bref = &b as *const _ as *mut [ManuallyDrop<U>; N];
    forget(b);
    let mut guard = ZipWithGuard {
        a: aref,
        b: bref,
        moved: 0,
    };
    let mut out: MaybeUninit<[V; N]> = unsafe { MaybeUninit::uninit().assume_init() };
    let out_ptr = out.as_mut_ptr().cast::<MaybeUninit<V>>();
    while guard.moved < N {
        let i = guard.moved;
        unsafe {
            *out_ptr.add(i) = MaybeUninit::new(f(
                guard.a.cast::<T>().add(i).read(),
                guard.b.cast::<U>().add(i).read(),
            ));
        }
        guard.moved += 1;
    }
    unsafe { out.assume_init() }
}

fn zip_map_fl<T: Copy, U: Copy, V, F: FnMut(T, U) -> V, const N: usize>(a: [T; N], b: [U; N], mut f: F) -> [V; N] {
    let mut out: [MaybeUninit<V>; N] = MaybeUninit::uninit_array();
    for i in 0..N {
        out[i] = MaybeUninit::new(f(a[i], b[i]));
    }
    unsafe {
        MaybeUninit::array_assume_init(out)
    }
}

fn zip_map_std<T, U, V, F: Fn(T, U) -> V, const N: usize>(a: [T; N], b: [U; N], f: F) -> [V; N] {
    a.zip(b).map(|(a, b)| f(a, b))
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmark_primitives
}

criterion_main! {
    benches
}
