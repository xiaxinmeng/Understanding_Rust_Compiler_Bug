 rust
#![crate_type="lib"]
#![feature(test)]

extern crate test;

#[inline(always)]
fn decrement(val: u32) -> Option<u32> {
    if val != 0 {
        Some(val - 1)
    } else {
        None
    }
}

pub fn testfun(mut a: u32) {
    loop {
        if a == 0 { return; }
        a = test::black_box(a - 1);
    }
}

pub fn testfun2(mut a: u32) {
    loop {
        a = test::black_box(match decrement(a) {
            Some(aa) => { aa },
            None => { return; },
        })
    }
}
