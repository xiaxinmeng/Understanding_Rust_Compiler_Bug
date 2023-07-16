rust
#![feature(bench_black_box)]
use std::mem::MaybeUninit;
use std::ops::Range;
use std::hint::black_box;

const N: usize = 400;
const RANGE: Range<u32> = 100..300;

pub fn foo() -> u32 {
    unsafe {
        let mut array = MaybeUninit::<[MaybeUninit<u32>; N]>::uninit().assume_init();
        let mut len = 0;

        for value in black_box(RANGE) {
            array.get_unchecked_mut(len).write(black_box(value));
            len += 1;
        }

        (0..len).map(|i| array.get_unchecked(i).assume_init_read()).sum()
    }
}

struct S {
    array: [MaybeUninit<u32>; N],
    len: usize,
}

pub fn bar() -> u32 {
    unsafe {
        let mut s = S {
            array: MaybeUninit::uninit().assume_init(),
            len: 0,
        };

        for value in black_box(RANGE) {
            s.array.get_unchecked_mut(s.len).write(black_box(value));
            s.len += 1;
        }

        (0..s.len).map(|i| s.array.get_unchecked(i).assume_init_read()).sum()
    }
}
