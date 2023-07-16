
#![feature(macro_rules, asm)]

extern crate test;

use BH = test::Bencher;
macro_rules! mk_benches(
    ( $($name:ident, $n:expr;)* ) => {

        $(
            #[bench]
            fn $name(bh: &mut BH) {
                bh.iter(|| for _ in range(0i, $n) { unsafe {asm!("")}; } );
            }
            )*
    })

mk_benches! {
    a,           1;
    b,          40; // to get a meaningful time
    c,         100;
    d,       1_000;
    e,      10_000;
    f,     100_000;
    g,   1_000_000;
    h,  10_000_000;
    i, 100_000_000;
}
