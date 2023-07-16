
seg [ master][?][📦 v0.1.0][🦀 v1.66.1][⏱ 9s]
❯ cargo +nightly-2021-09-24 check
    Checking seg v0.1.0 (/home/username/dev/temp/seg)
error[E0658]: generic associated types are unstable
 --> src/main.rs:3:5
  |
3 |     type With<T>: Functor;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
  = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

error[E0658]: generic associated types are unstable
 --> src/main.rs:8:5
  |
8 |     type With<T2> = IdFunctor<T2>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
  = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

error[E0658]: generic associated types are unstable
  --> src/main.rs:12:5
   |
12 |     type With<T2> = Vec<T2>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

error[E0658]: generic associated types are unstable
  --> src/main.rs:16:5
   |
16 |     type With<T2> = Box<T2>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

error[E0658]: generic associated types are unstable
  --> src/main.rs:29:5
   |
29 |     type With<T2> = F1::With<F2::With<T2>>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable

/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/lib64/libc.so.6(+0x3cb20)[0x7fd44885fb20]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2Stable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(+0x1f9db51)[0x7fd44ad9db51]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(_ZN142_$LT$rustc_middle..ty..TyS$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(+0x1f9db51)[0x7fd44ad9db51]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(_ZN142_$LT$rustc_middle..ty..TyS$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(+0x1f9db51)[0x7fd44ad9db51]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(_ZN142_$LT$rustc_middle..ty..TyS$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(+0x1f9db51)[0x7fd44ad9db51]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(_ZN142_$LT$rustc_middle..ty..TyS$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(+0x1f9db51)[0x7fd44ad9db51]
/home/username/.rustup/toolchains/nightly-2021-09-24-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d2cc96ed75437e33.so(_ZN142_$LT$rustc_middle..ty..TyS$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$GT$11hash_stable17h2e55908815b11a3cE+0xa7)[0x7fd44ae1c987]
error: could not compile `seg` due to 5 previous errors

Caused by:
  process didn't exit successfully: `rustc --crate-name seg --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=8eaa634c885642ac -C extra-filename=-8eaa634c885642ac --out-dir /home/username/dev/temp/seg/target/debug/deps -C linker=/usr/bin/clang -C incremental=/home/username/dev/temp/seg/target/debug/incremental -L dependency=/home/username/dev/temp/seg/target/debug/deps -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--no-rosegment` (signal: 11, SIGSEGV: invalid memory reference)

