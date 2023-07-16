plain
[00:24:08] [RUSTC-TIMING] alloc_jemalloc test:false 0.128
[00:24:12] [RUSTC-TIMING] alloc test:false 5.378
[00:24:12]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:24:12] [RUSTC-TIMING] panic_unwind test:false 0.244
[00:24:14] error: unused imports: `AtomicUsize`, `Ordering`
[00:24:14]   --> libstd/sys/unix/fs.rs:21:20
[00:24:14]    |
[00:24:14] 21 | use sync::atomic::{AtomicUsize, Ordering};
[00:24:14]    |                    ^^^^^^^^^^^  ^^^^^^^^
[00:24:14]    = note: `-D unused-imports` implied by `-D warnings`
[00:24:14] 
[00:24:17] error: aborting due to previous error
[00:24:17] 
[00:24:17] 
[00:24:17] [RUSTC-TIMING] std test:false 4.625
[00:24:17] error: Could not compile `std`.
[00:24:17] 
[00:24:17] Caused by:
[00:24:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=7e1dffde0459f357 -C extra-filename=-7e1dffde0459f357 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps --target i686-unknown-freebsd -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libcore-15b3763320d9f15b.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc-534fdec6a7cc6c6e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libcompiler_builtins-7fec55910f7c0bc5.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libpanic_abort-c8867dd5f1d2e891.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libunwind-e36226b3b8dfdb5a.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc_jemalloc-b778d71ef482ac41.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liblibc-25bd4962eb25de7a.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libpanic_unwind-e1af0cc97919a446.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/libstd_unicode-511a543a282af164.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/deps/liballoc_system-d209545a9e6002ad.rlib -L native=/checkout/obj/build/i686-unknown-freebsd/native/libbacktrace/.libs -l static=backtrace -l execinfo -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/i686-unknown-freebsd/release/build/compiler_builtins-82c7222d1f5c03fd/out -L native=/checkout/obj/build/i686-unknown-freebsd/native/jemalloc/lib` (exit code: 101)
[00:24:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-freebsd" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:24:17] expected success, got: exit code: 101
[00:24:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:24:17] travis_fold:end:stage1-std

[00:24:17] travis_time:end:stage1-std:start=1526202895036725931,finish=1526202951675144271,duration=56638418340

