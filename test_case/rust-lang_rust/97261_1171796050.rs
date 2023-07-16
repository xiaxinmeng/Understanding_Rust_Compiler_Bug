rust
#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_start(1)]
struct NonZero(usize);

let x: NonZero = NonZero(5);
x.0 = 0;
dbg!(x.0);
x.0 = 5
