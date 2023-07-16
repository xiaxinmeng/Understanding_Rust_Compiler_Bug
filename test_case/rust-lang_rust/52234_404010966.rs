plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:08fbd86e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:59:05]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:59:05]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:59:05]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:59:05]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:59:06] error: macro expansion ignores token `{` and any following
[00:59:06]     |
[00:59:06]     |
[00:59:06] 771 |         ($file:expr) => ({ /* compiler built-in */ });
[00:59:06]     |
[00:59:06]     |
[00:59:06] note: caused by the macro expansion here; the usage of `include!` is likely invalid in item context
[00:59:06]    --> libstd/lib.rs:548:1
[00:59:06]     |
[00:59:06] 548 | include!("primitive_docs.rs");
[00:59:06] 
[00:59:06] 
[00:59:06] error: macro expansion ignores token `{` and any following
[00:59:06]     |
[00:59:06]     |
[00:59:06] 771 |         ($file:expr) => ({ /* compiler built-in */ });
[00:59:06]     |
[00:59:06]     |
[00:59:06] note: caused by the macro expansion here; the usage of `include!` is likely invalid in item context
[00:59:06]    --> libstd/lib.rs:553:1
[00:59:06]     |
[00:59:06] 553 | include!("keyword_docs.rs");
[00:59:06] 
[00:59:06] 
[00:59:06] error: Could not document `std`.
[00:59:06] Caused by:
[00:59:06] Caused by:
[00:59:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" --cfg feature="profiler" --cfg feature="profiler_builtins" -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-11f645d5d50ec01d.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-b47ce328a4ba70ee.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-f97b2ca8b6ea07bd.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-a79b1686dcdb6599.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-9dfdb611cdf09a22.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-e113f9b9a0aa6586.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-afa6c0a795054c51.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-4f996473c57ff2d7.rmeta --extern profiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-96fc30c656f77bfd.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-6678a520a240f250.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-2b28bf250e8c19b3.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-72f2ee3ef3ce967d.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-f130431124e944fe.rmeta --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-3448644921326dc8.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-62dce3cd9306a1e4.rmeta` (exit code: 101)
[00:59:06] 
[00:59:06] 
[00:59:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:59:06] 
[00:59:06] 
[00:59:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:59:06] Build completed unsuccessfully in 0:53:26
