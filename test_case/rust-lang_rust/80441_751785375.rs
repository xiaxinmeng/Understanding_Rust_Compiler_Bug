plain
   Compiling semver-parser v0.10.1
   Compiling semver v0.11.0
   Compiling cargo_metadata v0.12.0
   Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no method named `eq_unspanned` found for reference `&rustc_ast::token::Token` in the current scope
   --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:544:41
    |
544 |         (Eq(_, lts), Eq(_, rts)) => lts.eq_unspanned(rts),
    |                                         ^^^^^^^^^^^^ method not found in `&rustc_ast::token::Token`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
---
   Compiling rustc-ap-rustc_span v691.0.0
   Compiling tokio-reactor v0.1.12
   Compiling jsonrpc-derive v14.2.1
   Compiling tokio-tcp v0.1.4
error[E0599]: no method named `eq_unspanned` found for reference `&rustc_ast::token::Token` in the current scope
   --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:544:41
    |
544 |         (Eq(_, lts), Eq(_, rts)) => lts.eq_unspanned(rts),
    |                                         ^^^^^^^^^^^^ method not found in `&rustc_ast::token::Token`
   Compiling tokio-uds v0.2.7
   Compiling tokio-udp v0.1.6
   Compiling tokio v0.1.22
   Compiling tokio-named-pipes v0.1.0
---
 finished in 17.585 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/llvm-finished-building`
Dist llvm-tools-nightly-x86_64-unknown-linux-gnu
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1121:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:23:44
