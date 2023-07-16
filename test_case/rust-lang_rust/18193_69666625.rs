 rust
#![feature(asm)]
#![allow(unstable)]

extern crate test;

use test::{Bencher};

#[inline(never)]
fn gen() -> Vec<u8> {
    (0..1024*65).map(|_| 0).collect()
}

#[bench]
fn position(b: &mut Bencher) {
    let v = gen();
    b.iter(|| {
        test::black_box(v.as_slice().iter().position(|&c| c == 1));
    });
}

#[bench]
fn iter(b: &mut Bencher) {
    let v = gen();
    b.iter(|| {
        let mut res = None;
        let mut i = 0us;
        for &b in v.as_slice().iter() {
            if b == 1 {
                res = Some(i);
                break;
            }
            i += 1;
        }
        test::black_box(res);
    });
}

#[bench]
fn enumerate(b: &mut Bencher) {
    let v = gen();
    b.iter(|| {
        let mut res = None;
        for (i, &b) in v.as_slice().iter().enumerate() {
            if b == 1 {
                res = Some(i);
                break;
            }
        }
        test::black_box(res);
    });
}

#[bench]
fn _range(b: &mut Bencher) {
    let v = gen();
    b.iter(|| {
        let mut res = None;
        for i in (0..v.len()) {
            if v[i] == 1 {
                res = Some(i);
                break;
            }
        }
        test::black_box(res);
    });
}

#[bench]
fn assembly(b: &mut Bencher) {
    let v = gen();
    b.iter(|| {
        unsafe {
            let mut start = v.as_ptr();
            let end = start.offset(v.len() as isize);
            asm!("
                dec $0
                .align 16, 0x90
            AGAIN:
                inc $0
                cmp $0, $1
                je EXIT
                cmpb $$1, ($0)
                jne AGAIN
            EXIT:
                " : "+r"(start) : "r"(end));
            if start < end {
                test::black_box(Some(start as usize - v.as_ptr() as usize));
            } else {
                test::black_box(None::<u8>);
            }
        }
    });
}
