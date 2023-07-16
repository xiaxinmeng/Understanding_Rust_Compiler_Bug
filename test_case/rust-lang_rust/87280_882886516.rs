plain
   Compiling clippy_lints v0.1.55 (/checkout/src/tools/clippy/clippy_lints)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/tools/clippy/clippy_lints/src/self_named_constructor.rs:66:17
   |
66 |             if !contains_adt_constructor(ret_ty, self_adt) {
   |                 |
   |                 expected 3 arguments
   |
note: function defined here
note: function defined here

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/tools/clippy/clippy_lints/src/self_named_constructor.rs:69:20
   |
69 |         } else if !contains_ty(ret_ty, self_ty) {
   |                    |
   |                    expected 3 arguments
   |
note: function defined here
---
   Compiling jsonrpc-pubsub v17.0.0
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/tools/clippy/clippy_lints/src/self_named_constructor.rs:66:17
   |
66 |             if !contains_adt_constructor(ret_ty, self_adt) {
   |                 |
   |                 expected 3 arguments
   |
note: function defined here
note: function defined here

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> src/tools/clippy/clippy_lints/src/self_named_constructor.rs:69:20
   |
69 |         } else if !contains_ty(ret_ty, self_ty) {
   |                    |
   |                    expected 3 arguments
   |
note: function defined here
---
 finished in 12.057 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1139:14
 finished in 14.681 seconds
Build completed unsuccessfully in 0:21:12
