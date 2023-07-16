
[01:02:33]    --> tools/clippy/clippy_lints/src/consts.rs:439:50
[01:02:33]     |
[01:02:33] 439 |         ConstValue::ScalarPair(Scalar::Ptr(ptr), Scalar::Bits { bits: n, .. }) => match result.ty.sty {
[01:02:33]     |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc::mir::interpret::value::ScalarMaybeUndef`, found enum `rustc::mir::interpret::value::Scalar`
[01:02:33]     |
[01:02:33]     = note: expected type `rustc::mir::interpret::value::ScalarMaybeUndef`
[01:02:33]                found type `rustc::mir::interpret::value::Scalar`
