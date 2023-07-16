plain
[00:02:25] Successfully tagged rust-ci:latest
[00:02:25] Built container sha256:7a9655e1a3cd58914567a0d17754c3441424590f4d8c000fb1674cb6e4818412
[00:02:25] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:26] 
[00:02:26] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:33] xargs: docker: terminated by signal 13

[00:02:33] travis_time:end:032f0a32:start=1532646923391723004,finish=1532647063447594803,duration=140055871799
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:33] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:06:36]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:06:41] error: This node does not have a stability attribute
[00:06:41]     --> libstd/thread/mod.rs:1279:1
[00:06:41]      |
[00:06:41] 1279 | unsafe impl<T> Send for JoinHandle<T> {}
[00:06:41] 
[00:06:41] error: This node does not have a stability attribute
[00:06:41]     --> libstd/thread/mod.rs:1280:1
[00:06:41]      |
[00:06:41]      |
[00:06:41] 1280 | unsafe impl<T> Sync for JoinHandle<T> {}
[00:06:41] 
[00:06:42] error: aborting due to 2 previous errors
[00:06:42] 
[00:06:42] error: Could not compile `std`.
[00:06:42] error: Could not compile `std`.
[00:06:42] 
[00:06:42] Caused by:
[00:06:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=38eda8139e25afc6 -C extra-filename=-38eda8139e25afc6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-0fa369be6843d38b.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-d44ab1cdae3c5d5f.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-61125bfca7e472f5.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b64848753b7b1fae.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-13801a3823f081b4.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-6b8b73430c4ddfaf.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-6dd09c3866f99ef9.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-8ca8a77f92241276.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-c00ded6ab9dc527a.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0045e7e201e984fa.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-8f8910030c71780d.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-06bfec7617605ba9.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-ea8c3e35572b37f6.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-ff87cc4f36401518.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-70f4afff694b17e5/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:06:42] expected success, got: exit code: 101
[00:06:42] expected success, got: exit code: 101
[00:06:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:06:42] travis_fold:end:stage0-std

[00:06:42] travis_time:end:stage0-std:start=1532647275870565395,finish=1532647312298520534,duration=36427955139


[00:06:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:42] Build completed unsuccessfully in 0:00:37
[00:06:42] make: *** [all] Error 1
[00:06:42] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01c1f9ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00251ec6:start=1532647312873454871,finish=1532647312881271435,duration=7816564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:037cf3a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d74e866
travis_time:start:0d74e866
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0234d6a2
$ dmesg | grep -i kill
