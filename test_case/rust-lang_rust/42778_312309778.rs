
error[E0382]: use of moved value: `self`
    --> src/libcore/num/mod.rs:1412:28
     |
1412 |               (self << n) | (self >> (($BITS - n) % $BITS))
     |                ----          ^^^^ value used here after move
     |                |
     |                value moved here
...
2261 | /     uint_impl! { u16, u16, 16,
2262 | |         intrinsics::ctpop,
2263 | |         intrinsics::ctlz,
2264 | |         ctlz_nonzero,
...    |
2268 | |         intrinsics::sub_with_overflow,
2269 | |         intrinsics::mul_with_overflow }
     | |_______________________________________- in this macro invocation
     |
     = note: move occurs because `self` has type `u16`, which does not implement the `Copy` trait
