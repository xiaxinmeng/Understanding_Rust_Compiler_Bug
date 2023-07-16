rust
#![feature(no_core, lang_items, intrinsics)]
#![no_core]

#[lang="sized"]
pub trait Sized {}

#[lang="freeze"]
pub trait Freeze {}

#[lang="copy"]
pub trait Copy {}

impl Copy for usize {}
impl Copy for isize {}
impl Copy for *mut u8 {}
impl Copy for *const u8 {}

#[lang = "partial_ord"]
pub trait PartialOrd<Rhs: ?Sized = Self> {
    fn ge(&self, other: &Rhs) -> bool;
}

#[lang = "mul"]
pub trait Mul<RHS=Self> {
    type Output;
    fn mul(self, rhs: RHS) -> Self::Output;
}

#[lang = "panic"]
pub fn panic(elc: &(&'static str, &'static str, u32, u32)) -> ! {
    loop {}
}

impl Mul for isize {
    type Output = isize;
    fn mul(self, other: isize) -> isize { self * other }
}

impl PartialOrd for usize {
    fn ge(&self, other: &usize) -> bool { *self >= *other }
}

extern "rust-intrinsic" {
    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;
}

pub unsafe fn hardcoded_sieve(start: *mut u8, prime: usize) {
    let loop_end = offset(start, 42);
    let p = offset(start, 0);
    let prime_ = prime as isize;

    loop {
        'label0: loop {
            break 'label0;
        }
        while (loop_end as usize) >= (p as usize) {
            *(offset(p, prime_ * 0) as *mut u8) = 254;
        }
    }
}
