plain
[00:37:47]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:37:47]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:37:47]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:37:47]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:37:54] thread '<unnamed>' panicked at 'not yet implemented: existential types are not yet implemented', librustdoc/visit_ast.rs:480:17
[00:37:54] 
[00:37:54] error: internal compiler error: unexpected panic
[00:37:54] 
[00:37:54] 
[00:37:54] error: Unrecognized option: 'crate-version'
[00:37:54] 
[00:37:54] error: Could not document `std`.
[00:37:54] Caused by:
[00:37:54] Caused by:
[00:37:54]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c86963c2a26cc128.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-e180558fe34ae6bc.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-6963e43f438b3358.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-05d2657f6da12b97.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-e7e70c4a2cf80ed6.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-55bcaf893ddeb68b.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-b62bd40ef4d1d24e.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-7380903ff91029aa.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-2ed10bf5bac77323.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0455e6bd7a2b0465.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-bdb0138699fb453f.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-858ca2795931096f.rmeta --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-a54ec09e15b57e90.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-50159098f8cc2895.rmeta` (exit code: 101)
[00:37:54] 
[00:37:54] 
[00:37:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:37:54] 
[00:37:54] 
[00:37:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:37:54] Build completed unsuccessfully in 0:04:44
[00:37:54] Build completed unsuccessfully in 0:04:44
[00:37:54] Makefile:28: recipe for target 'all' failed
[00:37:54] make: *** [all] Error 1
98360 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
97520 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
94752 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
94748 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
