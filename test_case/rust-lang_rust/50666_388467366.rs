plain
[01:06:00]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:06:00] [RUSTC-TIMING] panic_unwind test:false 0.204
[01:06:01] warning: dropping unsupported crate type `dylib` for target `asmjs-unknown-emscripten`
[01:06:01] 
[01:06:05] error[E0599]: no method named `try_into` found for type `i64` in the current scope
[01:06:05]   --> libstd/sys/unix/fd.rs:80:35
[01:06:05]    |
[01:06:05] 80 |             if let Ok(o) = offset.try_into() {
[01:06:05]    |
[01:06:05]    = help: items from traits can only be used if the trait is in scope
[01:06:05]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]            candidate #1: `use core::convert::TryInto;`
[01:06:05]            candidate #1: `use core::convert::TryInto;`
[01:06:05] 
[01:06:05] error[E0599]: no method named `try_into` found for type `i64` in the current scope
[01:06:05]    --> libstd/sys/unix/fd.rs:127:35
[01:06:05]     |
[01:06:05] 127 |             if let Ok(o) = offset.try_into() {
[01:06:05]     |
[01:06:05]     = help: items from traits can only be used if the trait is in scope
[01:06:05]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]             candidate #1: `use core::convert::TryInto;`
---
[01:06:05] [RUSTC-TIMING] std test:false 4.579
[01:06:05] error: Could not compile `std`.
[01:06:05] 
[01:06:05] Caused by:
[01:06:05]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=dd7e1cb952f01bfe -C extra-filename=-dd7e1cb952f01bfe --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps --target asmjs-unknown-emscripten -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/liblibc-51a399ba7e674717.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libstd_unicode-f3aad318045bbfb9.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libpanic_unwind-99544cd741ab91fb.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libcore-6ada1f8f2c20c8b5.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libunwind-654a884bfdda8b1d.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/liballoc-74412e5d25bc61ac.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libpanic_abort-9d726bd413a86f8b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/liballoc_jemalloc-2c1a44b9ee26baac.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/liballoc_system-4a31f9b6a00b0663.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/asmjs-unknown-emscripten/release/deps/libcompiler_builtins-93fbfd71748200b0.rlib` (exit code: 101)
[01:06:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "asmjs-unknown-emscripten" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:06:05] expected success, got: exit code: 101
[01:06:05] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:06:05] travis_fold:end:stage2-std

[01:06:05] travis_time:end:stage2-std:start=1526067900171653875,finish=1526067959117672997,duration=58946019122

