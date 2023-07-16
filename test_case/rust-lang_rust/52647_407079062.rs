plain
[00:07:17]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:41] error[E0308]: mismatched types
[00:07:41]     --> librustc/traits/error_reporting.rs:1060:17
[00:07:41]      |
[00:07:41] 1060 | /                 err.span_suggestion(
[00:07:41] 1061 | |                     found_span,
[00:07:41] 1062 | |                     "consider changing this to",
[00:07:41] 1063 | |                     format!("|{}|", underscores),
[00:07:41] 1064 | |                 )
[00:07:41]      | |                 ^- help: try adding a semicolon: `;`
[00:07:41]      | |_________________|
[00:07:41]      |                   expected (), found mutable reference
[00:07:41]      = note: expected type `()`
[00:07:41]                 found type `&mut errors::DiagnosticBuilder<'_>`
[00:07:41] 
[00:07:41] 
3cb288550.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-67c8aaeb97e8d843.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-d405c21c3677962b.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a185b6f37f2c1490.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1436fcd917ac8cb6.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7f18c5c76091a313.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-e808ab63ca762e81.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayoninux-gnu/stage0/lib/rustlib
251948 ./src/llvm/test
241192 ./src/llvm-emscripten
192644 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
177828 ./obj/build/bootstrap/debug/deps
177828 ./obj/build/bootstrap/debug/deps
170388 ./obj/build/cache
170384 ./obj/build/cache/2018-07-13
156480 ./.git/modules
156476 ./.git/modules/src
149128 ./src/llvm-emscripten/test
145372 ./obj/build/bootstrap/debug/incremental
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130556 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f36ana5ze9-h7aakh-3om13ebi8plfd
97528 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
78976 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
77372 ./.git/modules/src/tools
71508 ./src/llvm/lib
