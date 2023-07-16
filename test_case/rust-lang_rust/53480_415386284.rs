plain
[00:46:38]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:46:39] [RUSTC-TIMING] panic_unwind test:false 0.262
[00:46:45] warning: redundant linker flag specified for library `zircon`
[00:46:45] 
[00:46:53] error: linking with `x86_64-fuchsia-clang` failed: exit code: 1
[00:46:53]   |
[00:46:53]   = note: "x86_64-fuchsia-clang" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.std.1u5ozpa0-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libstd-ac7d46d5a402ccb9.so" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.149plxj13bw4oocp.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-ac7d46d5a402ccb9.5enw71tjnstngsbb.rcgu.o" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-fuchsia/native/libbacktrace/" "-L" "/checkout/obj/build/x86_64-fuchsia/native/libbacktrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-681b240a62e2a59d/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "-lzircon" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lbacktrace" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lfdio" "-Wl,-Bstatic" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/libpanic_unwind-ba52fd8624227ec2.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/libunwind-82403010bb9e09f1.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/liballoc_system-87c64992ba02c64b.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/liblibc-71a64769a3cc7f56.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/liballoc-61b1411bf1fae4e4.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcYKVCI1/libcore-1be421870ae3d9f0.rlib" "-Wl,--no-whole-archive" "/tmp/rustcYKVCI1/libcompiler_builtins-786b4c4e4c013408.rlib" "-Wl,-Bdynamic" "-lunwind" "-lc" "-lfdio" "-shared" "-Wl,-rpath,$ORIGIN/../lib"
[00:46:53]   = note: ld.lld: error: unable to find library -lzircon
[00:46:53]           ld.lld: error: unable to find library -lfdio
[00:46:53]           ld.lld: error: unable to find library -lc
[00:46:53]           ld.lld: error: unable to find library -lfdio
[00:46:53]           clang-8: error: ld.lld command failed with exit code 1 (use -v to see invocation)
[00:46:53] 
[00:46:53] error: aborting due to previous error
[00:46:53] 
[00:46:53] [RUSTC-TIMING] std test:false 14.624
[00:46:53] [RUSTC-TIMING] std test:false 14.624
[00:46:53] error: Could not compile `std`.
[00:46:53] 
[00:46:53] Caused by:
[00:46:53]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=ac7d46d5a402ccb9 -C extra-filename=-ac7d46d5a402ccb9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps --target x86_64-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc-61b1411bf1fae4e4.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_jemalloc-fc8c17fe7d6b5e22.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_system-87c64992ba02c64b.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcompiler_builtins-786b4c4e4c013408.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcore-1be421870ae3d9f0.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liblibc-71a64769a3cc7f56.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_abort-5392cab9e79d34b6.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_unwind-ba52fd8624227ec2.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libunwind-82403010bb9e09f1.rlib -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace -l static=backtrace -l static=backtrace -l zircon -l fdio -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-681b240a62e2a59d/out` (exit code: 1)
[00:46:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:46:53] expected success, got: exit code: 101
[00:46:53] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:46:53] travis_fold:end:stage2-std

[00:46:53] travis_time:end:stage2-std:start=1535024664707850173,finish=1535024720174932231,duration=55467082058

---
travis_time:end:22d3eb96:start=1535024721555997451,finish=1535024721563510827,duration=7513376
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04531908
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07602920
travis_time:start:07602920
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04d91c00
$ dmesg | grep -i kill
