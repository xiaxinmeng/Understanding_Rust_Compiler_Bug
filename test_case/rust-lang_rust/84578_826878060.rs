plain
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  4273  100  4273    0     0  12987      0 --:--:-- --:--:-- --:--:-- 12987
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib /tmp/ctfe-stress-4.rs
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
   |
   |
67 | expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 16 16 16 16 16]);
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bounds)]` to the crate attributes to enable

