rust
#![feature(test)]
extern crate test;

use ascii::*;
use test::black_box;
use test::Bencher;

macro_rules! repeat {
    ($s: expr) => {
        concat!($s, $s, $s, $s, $s, $s, $s, $s, $s, $s)
    };
}

const SHORT: &'static str = "Alice's";
const MEDIUM: &'static str = "Alice's Adventures in Wonderland";
const LONG: &'static str = repeat!(
    r#"
    La Guida di Bragia, a Ballad Opera for the Marionette Theatre (around 1850)
    Alice's Adventures in Wonderland (1865)
    Phantasmagoria and Other Poems (1869)
    Through the Looking-Glass, and What Alice Found There
        (includes "Jabberwocky" and "The Walrus and the Carpenter") (1871)
    The Hunting of the Snark (1876)
    Rhyme? And Reason? (1883) – shares some contents with the 1869 collection,
        including the long poem "Phantasmagoria"
    A Tangled Tale (1885)
    Sylvie and Bruno (1889)
    Sylvie and Bruno Concluded (1893)
    Pillow Problems (1893)
    What the Tortoise Said to Achilles (1895)
    Three Sunsets and Other Poems (1898)
    The Manlet (1903)[106]
"#
);

macro_rules! benches {
    ($( fn $name: ident($arg: ident: &[u8]) $body: block )+) => {
        benches!(mod short SHORT[..] $($name $arg $body)+);
        benches!(mod medium MEDIUM[..] $($name $arg $body)+);
        benches!(mod long LONG[..] $($name $arg $body)+);
        // Ensure we benchmark cases where the functions are called with strings
        // that are not perfectly aligned or have a length which is not a
        // multiple of size_of::<usize>() (or both)
        benches!(mod unaligned_head MEDIUM[1..] $($name $arg $body)+);
        benches!(mod unaligned_tail MEDIUM[..(MEDIUM.len() - 1)] $($name $arg $body)+);
        benches!(mod unaligned_both MEDIUM[1..(MEDIUM.len() - 1)] $($name $arg $body)+);
    };

    (mod $mod_name: ident $input: ident [$range: expr] $($name: ident $arg: ident $body: block)+) => {
        mod $mod_name {
            use super::*;
            $(
                #[bench]
                fn $name(bencher: &mut Bencher) {
                    bencher.bytes = $input[$range].len() as u64;
                    let mut vec = $input.as_bytes().to_vec();
                    bencher.iter(|| {
                        let $arg: &[u8] = &black_box(&mut vec)[$range];
                        black_box($body)
                    })
                }
            )+
        }
    };
}

benches! {
    fn case00_libcore(bytes: &[u8]) {
        is_ascii(bytes)
    }

    fn case00_libcore_new(bytes: &[u8]) {
        is_ascii_new(bytes)
    }

    fn case01_iter_all(bytes: &[u8]) {
        bytes.iter().all(|b| b.is_ascii())
    }

    fn case02_align_to(bytes: &[u8]) {
        is_ascii_align_to(bytes)
    }

    fn case03_align_to_unrolled(bytes: &[u8]) {
        is_ascii_align_to_unrolled(bytes)
    }
}

// These are separate since it's easier to debug errors if they don't go through
// macro expansion first.
fn is_ascii_align_to(bytes: &[u8]) -> bool {
    if bytes.len() < core::mem::size_of::<usize>() {
        return bytes.iter().all(|b| b.is_ascii());
    }
    // SAFETY: transmuting a sequence of `u8` to `usize` is always fine
    let (head, body, tail) = unsafe { bytes.align_to::<usize>() };
    head.iter().all(|b| b.is_ascii())
        && body.iter().all(|w| !contains_nonascii(*w))
        && tail.iter().all(|b| b.is_ascii())
}

fn is_ascii_align_to_unrolled(bytes: &[u8]) -> bool {
    if bytes.len() < core::mem::size_of::<usize>() {
        return bytes.iter().all(|b| b.is_ascii());
    }
    // SAFETY: transmuting a sequence of `u8` to `[usize; 2]` is always fine
    let (head, body, tail) = unsafe { bytes.align_to::<[usize; 2]>() };
    head.iter().all(|b| b.is_ascii())
        && body.iter().all(|w| !contains_nonascii(w[0] | w[1]))
        && tail.iter().all(|b| b.is_ascii())
}

#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x80808080_80808080u64 as usize;
    (NONASCII_MASK & v) != 0
}
