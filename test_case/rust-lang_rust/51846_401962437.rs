plain
[00:03:28]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:30]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:30]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:34]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:36] error[E0658]: use of unstable library feature 'alloc_hashmap': HashMap in liballoc is unstable
[00:03:36]     |
[00:03:36]     |
[00:03:36] 437 | pub use alloc_crate::collections::{hash_map, hash_set};
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(alloc_hashmap)] to the crate attributes to enable
[00:03:36] 
[00:03:36] error[E0658]: use of unstable library feature 'alloc_hashmap': HashMap in liballoc is unstable
[00:03:36]     |
[00:03:36]     |
[00:03:36] 437 | pub use alloc_crate::collections::{hash_map, hash_set};
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(alloc_hashmap)] to the crate attributes to enable
[00:03:38] error: aborting due to 2 previous errors
[00:03:38] 
[00:03:38] For more information about this error, try `rustc --explain E0658`.
[00:03:38] error: Could not compile `std`.
[00:03:38] error: Could not compile `std`.
[00:03:38] 
[00:03:38] Caused by:
[00:03:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=dcc25629039acb53 -C extra-filename=-dcc25629039acb53 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-dab08365760adda8.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-2bfe3082744c992b.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-19594d83bbdd6633.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-8c383f2d2bbbcb1d.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-c8f7a210a5d40dd6.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-e67af0f0fda6d67c.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-25d6bfec677c2cc1.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-df29f51552b7081c.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-19acac464f503382.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-1691bf078a2ccad2.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-ed6ca6bad7af674f.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-1f2e12e7aa536e40.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-354a9da054095dd7.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-45eff864955bd111.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-c2bb2884be27c6a0/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:38] expected success, got: exit code: 101
[00:03:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:38] travis_fold:end:stage0-std

[00:03:38] travis_time:end:stage0-std:start=1530572294955867323,finish=1530572332153307090,duration=37197439767


[00:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:38] Build completed unsuccessfully in 0:00:38
[00:03:38] make: *** [tidy] Error 1
[00:03:38] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16937d7d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24d24bab:start=1530572332665250223,finish=1530572332671440897,duration=6190674
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20364d30
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0040ad3c
$ dmesg | grep -i kill
