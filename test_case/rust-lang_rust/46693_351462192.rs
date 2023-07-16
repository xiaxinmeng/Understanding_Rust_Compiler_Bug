rust
#![feature(test)]
extern crate libc;
extern crate test;

use libc::c_char;
use std::ffi::{CString};
use test::{black_box, Bencher};

const NEEDLE: &str = "there";
const HAYSTACK: &str = "this is a string with a decent number of ascii characters and \n there is a new line in the middle which it should find";

fn strstr(haystack: *const c_char, needle: *const c_char) -> Option<usize> {
    let found = unsafe { libc::strstr(haystack, needle) };

    if found.is_null() { None }
    else { Some(found as usize - haystack as usize) }
}

#[bench]
fn bench_find(b: &mut Bencher) {
    b.iter(|| {
        let haystack = test::black_box(HAYSTACK);
        let needle = test::black_box(NEEDLE);
        haystack.find(needle)
    });
}

#[bench]
fn bench_strstr(b: &mut Bencher) {
    let haystack = CString::new(HAYSTACK).unwrap();
    let haystack = haystack.as_ptr();
    let needle = CString::new(NEEDLE).unwrap();
    let needle = needle.as_ptr();
    b.iter(|| {
        let haystack = test::black_box(haystack);
        let needle = test::black_box(needle);
        strstr(haystack, needle)
    });
}
