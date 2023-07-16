rust
   Compiling playground v0.0.1 (/playground)
error[E0283]: type annotations required: cannot resolve `MyEnum: std::convert::Into<_>`
  --> src/lib.rs:16:38
   |
16 |     assert_eq!(0u16, MyEnum::Variant.into());
   |                                      ^^^^

error: aborting due to previous error
