plain
   Compiling semver-parser v0.10.1
error[E0658]: custom inner attributes are unstable
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.10.1/src/generated.rs:4:4
  |
4 | #![rustfmt::skip]
  |
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = help: add `#![feature(custom_inner_attributes)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `semver-parser`
---
   Compiling rls v1.41.0 (/checkout/src/tools/rls)
error[E0658]: custom inner attributes are unstable
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.10.1/src/generated.rs:4:4
  |
4 | #![rustfmt::skip]
  |
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = help: add `#![feature(custom_inner_attributes)]` to the crate attributes to enable
   Compiling crossbeam-queue v0.2.3
   Compiling crossbeam-channel v0.5.0
error: aborting due to previous error

---
 finished in 15.104 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:18:33
