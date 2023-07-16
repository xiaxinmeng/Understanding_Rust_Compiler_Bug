rust
#![feature(test)]

extern crate test;
use test::Bencher;

#[inline(always)]
fn custom_ascii_uppercase_without_ascii_check(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        _ => false,
    }
}

#[inline(always)]
fn custom_ascii_uppercase_with_ascii_check_builtin_is_ascii(c: char) -> bool {
    c.is_ascii() && match c {
        'A'..='Z' => true,
        _ => false,
    }
}

#[inline(always)]
fn custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u8(c: char) -> bool {
    c as u8 <= 0x7fu8 && match c {
        'A'..='Z' => true,
        _ => false,
    }
}

#[inline(always)]
fn custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u32(c: char) -> bool {
    c as u32 <= 0x7fu32 && match c {
        'A'..='Z' => true,
        _ => false,
    }
}

#[bench]
fn bench_custom_ascii_uppercase_without_ascii_check(b: &mut Bencher) {
    let x = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    b.iter(|| {
        for c in x.chars() {
            test::black_box(custom_ascii_uppercase_without_ascii_check(c));
        }
    });
}

#[bench]
fn bench_custom_ascii_uppercase_with_ascii_check_builtin_is_ascii(b: &mut Bencher) {
    let x = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    b.iter(|| {
        for c in x.chars() {
            test::black_box(custom_ascii_uppercase_with_ascii_check_builtin_is_ascii(c));
        }
    });
}

#[bench]
fn bench_custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u8(b: &mut Bencher) {
    let x = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    b.iter(|| {
        for c in x.chars() {
            test::black_box(custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u8(c));
        }
    });
}

#[bench]
fn bench_custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u32(b: &mut Bencher) {
    let x = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    b.iter(|| {
        for c in x.chars() {
            test::black_box(custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u32(c));
        }
    });
}

#[bench]
fn bench_builtin_ascii_uppercase(b: &mut Bencher) {
    let x = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    b.iter(|| {
        for c in x.chars() {
            test::black_box(c.is_ascii_uppercase());
        }
    });
}
