 rust
#![crate_type = "lib"]

pub unsafe fn a(p: *const usize, g: fn()) -> usize {
    let a = *p;
    g();
    let b = *p;
    a ^ b
}

pub unsafe fn b(p: &usize, g: fn()) -> usize {
    let a = *p;
    g();
    let b = *p;
    a ^ b
}

pub unsafe fn c(p: *const usize, g: fn()) -> usize {
    let p = &*p;

    let a = *p;
    g();
    let b = *p;
    a ^ b
}

pub unsafe fn d(p: *const usize, g: fn()) -> usize {
    let p = &*p;

    b(p, g)
}
