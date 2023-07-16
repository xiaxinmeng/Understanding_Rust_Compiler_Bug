plain
   Compiling quine-mc_cluskey v0.2.4
   Compiling clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
   Compiling pest v2.1.3
   Compiling clippy v0.1.53 (/checkout/src/tools/clippy)
error[E0277]: `&Statements<'_>` is not an iterator
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:74:21
   |
74 |         for stmt in &bb.statements {
   |                     ^^^^^^^^^^^^^^ `&Statements<'_>` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `&Statements<'_>`
   = note: required because of the requirements on the impl of `IntoIterator` for `&Statements<'_>`


error[E0609]: no field `source_info` on type `&rustc_middle::mir::Statement<'tcx>`
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:208:26
    |
208 |     let span = statement.source_info.span;
    |
    = note: available fields are: `kind`

error: aborting due to 2 previous errors
---
   Compiling lsp-types v0.60.0
   Compiling cargo_metadata v0.12.0
   Compiling futures-macro v0.3.12
   Compiling miow v0.3.6
error[E0277]: `&Statements<'_>` is not an iterator
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:74:21
   |
74 |         for stmt in &bb.statements {
   |                     ^^^^^^^^^^^^^^ `&Statements<'_>` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `&Statements<'_>`
   = note: required because of the requirements on the impl of `IntoIterator` for `&Statements<'_>`


error[E0609]: no field `source_info` on type `&rustc_middle::mir::Statement<'tcx>`
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:208:26
    |
208 |     let span = statement.source_info.span;
    |
    = note: available fields are: `kind`

error: aborting due to 2 previous errors
---
 finished in 13.651 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:17:53
