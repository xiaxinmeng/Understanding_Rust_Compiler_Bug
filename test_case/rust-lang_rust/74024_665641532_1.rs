rust
#![feature(test)]
extern crate test;

use test::black_box;
use test::Bencher;

use binary_search::*;

enum Cache {
    L1,
    L2,
    L3,
}

fn std_bench_binary_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
where
    F: Fn(usize) -> usize,
{
    let size = match cache {
        Cache::L1 => 1000,      // 8kb
        Cache::L2 => 10_000,    // 80kb
        Cache::L3 => 1_000_000, // 8Mb
    };
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        black_box(std_binary_search(&v, &i).is_ok());
    })
}

fn std_bench_binary_search_worst_case(b: &mut Bencher, cache: Cache) {
    let size = match cache {
        Cache::L1 => 1000,      // 8kb
        Cache::L2 => 10_000,    // 80kb
        Cache::L3 => 1_000_000, // 8Mb
    };
    let mut v = vec![0; size];
    let i = 1;
    v[size - 1] = i;
    b.iter(move || {
        black_box(std_binary_search(&v, &i).is_ok());
    })
}

#[bench]
fn std_binary_search_l1(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L1, |i| i * 2);
}

#[bench]
fn std_binary_search_l2(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L2, |i| i * 2);
}

#[bench]
fn std_binary_search_l3(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L3, |i| i * 2);
}

#[bench]
fn std_binary_search_l1_with_dups(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L1, |i| i / 16 * 16);
}

#[bench]
fn std_binary_search_l2_with_dups(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L2, |i| i / 16 * 16);
}

#[bench]
fn std_binary_search_l3_with_dups(b: &mut Bencher) {
    std_bench_binary_search(b, Cache::L3, |i| i / 16 * 16);
}

#[bench]
fn std_binary_search_l1_worst_case(b: &mut Bencher) {
    std_bench_binary_search_worst_case(b, Cache::L1);
}

#[bench]
fn std_binary_search_l2_worst_case(b: &mut Bencher) {
    std_bench_binary_search_worst_case(b, Cache::L2);
}

#[bench]
fn std_binary_search_l3_worst_case(b: &mut Bencher) {
    std_bench_binary_search_worst_case(b, Cache::L3);
}

fn stdnew_bench_binary_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
where
    F: Fn(usize) -> usize,
{
    let size = match cache {
        Cache::L1 => 1000,      // 8kb
        Cache::L2 => 10_000,    // 80kb
        Cache::L3 => 1_000_000, // 8Mb
    };
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        black_box(stdnew_binary_search(&v, &i).is_ok());
    })
}

fn stdnew_bench_binary_search_worst_case(b: &mut Bencher, cache: Cache) {
    let size = match cache {
        Cache::L1 => 1000,      // 8kb
        Cache::L2 => 10_000,    // 80kb
        Cache::L3 => 1_000_000, // 8Mb
    };
    let mut v = vec![0; size];
    let i = 1;
    v[size - 1] = i;
    b.iter(move || {
        black_box(stdnew_binary_search(&v, &i).is_ok());
    })
}

#[bench]
fn stdnew_binary_search_l1(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L1, |i| i * 2);
}

#[bench]
fn stdnew_binary_search_l2(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L2, |i| i * 2);
}

#[bench]
fn stdnew_binary_search_l3(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L3, |i| i * 2);
}

#[bench]
fn stdnew_binary_search_l1_with_dups(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L1, |i| i / 16 * 16);
}

#[bench]
fn stdnew_binary_search_l2_with_dups(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L2, |i| i / 16 * 16);
}

#[bench]
fn stdnew_binary_search_l3_with_dups(b: &mut Bencher) {
    stdnew_bench_binary_search(b, Cache::L3, |i| i / 16 * 16);
}

#[bench]
fn stdnew_binary_search_l1_worst_case(b: &mut Bencher) {
    stdnew_bench_binary_search_worst_case(b, Cache::L1);
}

#[bench]
fn stdnew_binary_search_l2_worst_case(b: &mut Bencher) {
    stdnew_bench_binary_search_worst_case(b, Cache::L2);
}

#[bench]
fn stdnew_binary_search_l3_worst_case(b: &mut Bencher) {
    stdnew_bench_binary_search_worst_case(b, Cache::L3);
}
