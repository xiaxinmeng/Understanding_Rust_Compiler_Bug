rust
#![feature(decl_macro)]
#![feature(asm)]

pub unsafe fn aarch64(a: f64, b: f64) {
    let c;
    asm!("add {:d}, {:d}, d0", out(vreg) c, in(vreg) a, in("d0") {
        || {
            macro m() {}
        };
        b
    });
    c
}
