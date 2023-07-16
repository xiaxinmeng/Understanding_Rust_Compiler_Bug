 rust
#![feature(start)]

use std::ptr;

extern {
    fn abort() -> !;
}

struct S {
    ptr: &'static mut u8,
    cap: usize,
    len: usize
}

struct OS {
    ptr: *mut u8,
    cap: usize,
    len: usize
}

struct R {
    is_err: bool,
    buf: OS
}

#[inline(never)]
unsafe fn begin() -> R {
    R {
        is_err: true,
        buf: OS { ptr: ptr::null_mut(), len: 0, cap: 0 }
    }
}

unsafe fn end(r: &mut R) {
    if r.is_err {
        if r.buf.ptr != ptr::null_mut() {
            *r.buf.ptr = 0;
        }
    } else {
        *r.buf.ptr = 0;
    }
}

unsafe fn ok(r: [usize; 3]) -> (*const u8, usize) {
    let r: &(bool, (*const u8, usize)) = &*(&r as *const _ as *const _);
    if r.0 {
        (ptr::null(), std::mem::uninitialized())
    } else {
        r.1
    }
}

macro_rules! twice {
    ($x:expr) => {$x; $x}
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    twice!(unsafe {
        let mut r = begin();
        let a = if r.is_err {
            [1usize, &r.buf as *const _ as usize, 0]
        } else {
            let s: &S = &*(&r.buf as *const _ as *const S);
            [0usize, s.ptr as *const _ as usize, s.len]
        };

        let b = ok(a);
        if b.0 != ptr::null() && b.1 == 1 && *b.0 == 1 {
            abort()
        }

        end(&mut r);
    });
    0
}
