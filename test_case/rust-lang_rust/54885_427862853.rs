plain
[00:16:42]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:16:47]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:16:47]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:16:55]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:55] error[E0425]: cannot find function `is_break` in this scope
[00:16:55]     |
[00:16:55]     |
[00:16:55] 283 |                     is_break(inner)) {
[00:16:55] 
[00:16:56] error[E0308]: mismatched types
[00:16:56]    --> librustc_lint/unused.rs:268:16
[00:16:56]     |
[00:16:56]     |
[00:16:56] 268 |         if let ast::ExprKind::Break(..) = e {
[00:16:56]     |                ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::Expr`, found enum `syntax::ast::ExprKind`
[00:16:56]     = note: expected type `syntax::ast::Expr`
[00:16:56]                found type `syntax::ast::ExprKind`
[00:16:56] 
[00:16:56] error: aborting due to 2 previous errors
---
151412 ./src/tools/clang
149116 ./src/llvm-emscripten/test
144988 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
135996 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
135992 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5j6z9dlow-11vvp8z-1akhp35ixzhyj
107656 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
103224 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
103220 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
