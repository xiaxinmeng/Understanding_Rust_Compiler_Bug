plain
[00:46:32]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:46:33] [RUSTC-TIMING] panic_unwind test:false 0.252
[00:46:39] warning: redundant linker flag specified for library `zircon`
[00:46:39] 
[00:46:47] error: linking with `x86_64-fuchsia-clang` failed: exit code: 1
[00:46:47]   |
[00:46:47]   = note: "x86_64-fuchsia-clang" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.std.82r4c1p9-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libstd-c4134ff9a92b454e.so" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.4ichl4vie5umi5bw.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/std-c4134ff9a92b454e.50idbkajsw1eibsc.rcgu.o" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-fuchsia/native/libbacktrace/" "-L" "/checkout/obj/build/x86_64-fuchsia/native/libbacktrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-6824f47f17cd9867/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-fuchsia/lib" "-lzircon" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lbacktrace" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lfdio" "-Wl,-Bstatic" "-Wl,--whole-archive" "/tmp/rustcHshpBO/libpanic_unwind-4e90336c2dc63069.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcHshpBO/libunwind-dd1bd39b57b8e229.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcHshpBO/liballoc_system-5794f65df23a065e.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcHshpBO/liblibc-3955de38374473d4.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcHshpBO/liballoc-2b46423f96c165ca.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcHshpBO/libcore-ded92bdb6f611f3a.rlib" "-Wl,--no-whole-archive" "/tmp/rustcHshpBO/libcompiler_builtins-e6fbe755c60fe6f9.rlib" "-Wl,-Bdynamic" "-lunwind" "-lc" "-lfdio" "-shared" "-Wl,-rpath,$ORIGIN/../lib"
[00:46:47]   = note: ld.lld: error: unable to find library -lzircon
[00:46:47]           ld.lld: error: unable to find library -lfdio
[00:46:47]           ld.lld: error: unable to find library -lc
[00:46:47]           ld.lld: error: unable to find library -lfdio
[00:46:47]           clang-8: error: ld.lld command failed with exit code 1 (use -v to see invocation)
[00:46:47] 
[00:46:47] error: aborting due to previous error
[00:46:47] 
[00:46:47] [RUSTC-TIMING] std test:false 14.776
[00:46:47] [RUSTC-TIMING] std test:false 14.776
[00:46:47] error: Could not compile `std`.
[00:46:47] 
[00:46:47] Caused by:
[00:46:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="alloc_jemalloc"' --cfg 'feature="backtrace"' --cfg 'feature="jemalloc"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' -C metadata=c4134ff9a92b454e -C extra-filename=-c4134ff9a92b454e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps --target x86_64-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc-2b46423f96c165ca.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_jemalloc-24599c07847121d9.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liballoc_system-5794f65df23a065e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcompiler_builtins-e6fbe755c60fe6f9.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libcore-ded92bdb6f611f3a.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/liblibc-3955de38374473d4.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_abort-d4a3131cbb1d98ab.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libpanic_unwind-4e90336c2dc63069.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/deps/libunwind-dd1bd39b57b8e229.rlib -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-fuchsia/native/libbacktrace -l static=backtrace -l static=backtrace -l zircon -l fdio -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-fuchsia/release/build/compiler_builtins-6824f47f17cd9867/out` (exit code: 1)
[00:46:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:46:47] expected success, got: exit code: 101
[00:46:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:46:47] travis_fold:end:stage2-std

[00:46:47] travis_time:end:stage2-std:start=1534844033650996955,finish=1534844089051173264,duration=55400176309

---
travis_time:end:17fae05e:start=1534844090331571055,finish=1534844090339958685,duration=8387630
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d47f4bb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ac4d3aa
travis_time:start:1ac4d3aa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:263be1fc
$ dmesg | grep -i kill
