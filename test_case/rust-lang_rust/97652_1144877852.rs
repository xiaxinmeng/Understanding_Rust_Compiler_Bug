plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 108 tests
ii...i.iF.....i......i.ii..i......i......iii.....................i........i..i.......... 88/108
failures:

---- src/builtin.rs - builtin::CENUM_IMPL_DROP_CAST (line 2616) stdout ----
---- src/builtin.rs - builtin::CENUM_IMPL_DROP_CAST (line 2616) stdout ----
error: cannot cast enum `E` into integer `u32` because it implements `Drop`
   |
15 |     let i = e as u32;
   |             ^^^^^^^^
   |
   |
   = note: `#[deny(cenum_impl_drop_cast)]` on by default
   = note: for more information, see issue #73333 <https://github.com/rust-lang/rust/issues/73333>

error: aborting due to previous error

