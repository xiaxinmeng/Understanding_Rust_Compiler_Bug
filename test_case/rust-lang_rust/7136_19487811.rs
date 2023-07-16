
extern mod extra;

use std::vec;
use std::ptr;

#[bench]
fn bench_from_elem(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let v: ~[u8] = vec::from_elem(1024, 0u8);
    }
}

#[bench]
fn bench_set_memory(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let mut v: ~[u8] = vec::with_capacity(1024);
        unsafe {
            let vp = vec::raw::to_mut_ptr(v);
            ptr::set_memory(vp, 0, 1024);
            vec::raw::set_len(&mut v, 1024);
        }
    }
}

fn main() {}
