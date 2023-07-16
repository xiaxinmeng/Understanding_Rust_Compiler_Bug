plain
[01:00:49] 
[01:00:49] error: Could not document `std`.
[01:00:49] 
[01:00:49] Caused by:
[01:00:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std src/libstd/lib.rs --color always --target i686-unknown-linux-gnu -o /checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/liballoc-233b1476e9a2ff1b.rmeta --extern compiler_builtins=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libcompiler_builtins-1901ee4587b6c362.rmeta --extern core=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libcore-81f471f1e579e116.rmeta --extern libc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/liblibc-b1cb4d19776ffff3.rmeta --extern panic_abort=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libpanic_abort-aaecd4470656c7aa.rmeta --extern panic_unwind=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libpanic_unwind-1729cc30c01c3b5f.rmeta --extern profiler_builtins=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libprofiler_builtins-63e1db89657628d0.rmeta --extern unwind=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libunwind-6cc708a2d7a2aa3f.rmeta` (exit code: 1)
[01:00:49] 
[01:00:49] 
[01:00:49] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[01:00:49] 
[01:00:49] 
[01:00:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build i686-unknown-linux-gnu --host i686-unknown-linux-gnu --target i686-unknown-linux-gnu
[01:00:49] Build completed unsuccessfully in 0:55:44
---
travis_time:end:0e13fbc4:start=1544225121175510039,finish=1544225121182884561,duration=7374522
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:225132b5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11ee9048
travis_time:start:11ee9048
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09caec20
$ dmesg | grep -i kill
