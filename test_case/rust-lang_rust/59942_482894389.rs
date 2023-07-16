plain
[01:03:58] diff of stderr:
[01:03:58] 
[01:03:58] 1 error: the type `TYPE` is too big for the current architecture
[01:03:58] 2 
[01:03:58] + thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:03:58] + note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:03:58] 4 
[01:03:58] 5 
[01:03:58] 
[01:03:58] 
[01:03:58] 
[01:03:58] The actual stderr differed from the expected stderr.
[01:03:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-enum/huge-enum.stderr
[01:03:58] To update references, rerun the tests and pass the `--bless` flag
[01:03:58] To only update this specific test, also pass `--test-args huge-enum.rs`
[01:03:58] error: 1 errors occurred comparing output.
[01:03:58] status: exit code: 1
[01:03:58] status: exit code: 1
[01:03:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/huge-enum.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-enum/auxiliary" "-A" "unused"
[01:03:58] ------------------------------------------
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] stderr:
[01:03:58] stderr:
[01:03:58] ------------------------------------------
[01:03:58] {"message":"the type `std::option::Option<[u32; 536870911]>` is too big for the current architecture","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the type `std::option::Option<[u32; 536870911]>` is too big for the current architecture\n\n"}
[01:03:58] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:03:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:58] 
[01:03:58] ------------------------------------------
[01:03:58] 
---
[01:03:58] 
[01:03:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:03:58] 
[01:03:58] 
[01:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "ui" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:58] 
[01:03:58] 
[01:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:03:58] Build completed unsuccessfully in 1:00:37
---
travis_time:end:0048f4f8:start=1555193854506374976,finish=1555193854528977784,duration=22602808
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:013ffec0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0291c946
travis_time:start:0291c946
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:201bf247
$ dmesg | grep -i kill
