 rust
#![feature(test)]

extern crate test;

use test::Bencher as BH;
macro_rules! mk_benches {
    ( $($name:ident, $n:expr;)* ) => {

        $(
            #[bench]
            fn $name(bh: &mut BH) {
                bh.iter(|| for _ in 0u64..$n { test::black_box(1); } );
            }
            )*
    }
}

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
