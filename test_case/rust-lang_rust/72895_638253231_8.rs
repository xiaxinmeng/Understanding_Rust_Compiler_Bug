rust
#![feature(test)]
#![feature(decl_macro)]

extern crate test;


// "Hamlet, Prince of Denmark", adapted from https://www.gutenberg.org/files/27761/27761-0.txt
const SOURCE_TEXT: &'static str = include_str!("hamlet.txt");

macro bench_impl($bench:expr, $condition:expr) {
    $bench.iter(|| {
        let mut total = 0;
        for byte in SOURCE_TEXT.bytes() {
            if $condition(&byte) { total += 1; }
        }

        assert_eq!(total, 10566);
    });
}

#[bench]
fn is_ascii_punctuation_branch(bench: &mut test::Bencher) {
    bench_impl!(bench, |byte: &u8| {
        byte.is_ascii_punctuation()
    });
}

#[bench]
fn is_ascii_punctuation_branch_adapted(bench: &mut test::Bencher) {
    bench_impl!(bench, |byte: &u8| {
        !byte.is_ascii_alphanumeric() && byte.is_ascii_graphic()
    });
}

#[bench]
fn is_ascii_punctuation_lookup(bench: &mut test::Bencher) {
    const LOOKUP : [bool; 256] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];

    bench_impl!(bench, |byte: &u8| {
        LOOKUP[*byte as usize]
    });
}

#[bench]
fn is_ascii_punctuation_hybrid(bench: &mut test::Bencher) {
    const LOOKUP : [bool; 128] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false];

    bench_impl!(bench, |byte: &u8| {
        byte.is_ascii() && LOOKUP[*byte as usize]
    });
}

#[bench]
fn is_ascii_punctuation_linear_search(bench: &mut test::Bencher) {
    const LOOKUP : [u8; 32] = [46, 95, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 96, 123, 124, 125, 126];

    bench_impl!(bench, |byte: &u8| {
        LOOKUP.contains(byte)
    });
}

#[bench]
fn is_ascii_punctuation_binary_search(bench: &mut test::Bencher) {
    const LOOKUP : [u8; 32] = [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126];

    bench_impl!(bench, |byte: &u8| {
        LOOKUP.binary_search(byte).is_ok()
    });
}

#[bench]
fn is_ascii_punctuation_lookup_bitset(bench: &mut test::Bencher) {
    const LOOKUP : [u8; 32] = [0, 0, 0, 0, 254, 255, 0, 252, 1, 0, 0, 248, 1, 0, 0, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    bench_impl!(bench, |byte: &u8| {
        LOOKUP[(*byte / 8) as usize] >> (*byte % 8) & 1u8 == 1
    });
}

#[bench]
fn is_ascii_punctuation_hybrid_bitset(bench: &mut test::Bencher) {
    const LOOKUP : [u8; 16] = [0, 0, 0, 0, 254, 255, 0, 252, 1, 0, 0, 248, 1, 0, 0, 120];

    bench_impl!(bench, |byte: &u8| {
        byte.is_ascii() && LOOKUP[(*byte / 8) as usize] >> (*byte % 8) & 1u8 == 1
    });
}
