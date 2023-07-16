plain
   Compiling quine-mc_cluskey v0.2.4
   Compiling clippy_utils v0.1.52 (/checkout/src/tools/clippy/clippy_utils)
   Compiling pest v2.1.3
   Compiling clippy v0.1.52 (/checkout/src/tools/clippy)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:9
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:41
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils`
---
   Compiling miow v0.3.6
   Compiling tokio v0.2.24
   Compiling cargo_metadata v0.12.0
   Compiling rls-data v0.19.1
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:9
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:175:41
     |
175  |         Rvalue::BinaryOp(_, lhs, rhs) | Rvalue::CheckedBinaryOp(_, lhs, rhs) => {
     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils`
---
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
 finished in 13.806 seconds
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1129:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:18:25
