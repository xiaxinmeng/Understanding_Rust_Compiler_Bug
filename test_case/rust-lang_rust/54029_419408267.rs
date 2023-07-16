plain
[00:23:01]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:23:25] error: variable does not need to be mutable
[00:23:25]     --> libsyntax/parse/parser.rs:1202:21
[00:23:25]      |
[00:23:25] 1202 |                 Err(mut e) => {
[00:23:25]      |                     |
[00:23:25]      |                     help: remove this `mut`
[00:23:25]      |
[00:23:25]      = note: `-D unused-mut` implied by `-D warnings`
---
151412 ./src/tools/clang
149124 ./src/llvm-emscripten/test
148976 ./obj/build/bootstrap/debug/incremental
134544 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134540 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4kuleqzvx-11jniqd-1qbwsq7osnpb0
104700 ./src/tools/lldb
98980 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
