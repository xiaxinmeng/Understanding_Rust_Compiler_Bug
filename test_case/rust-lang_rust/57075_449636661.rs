plain
[00:53:18] 
[00:53:18] ---- [ui] ui/panic-runtime/runtime-depend-on-needs-runtime.rs stdout ----
[00:53:18] diff of stderr:
[00:53:18] 
[00:53:18] 1 error: the crate `depends` cannot depend on a crate that needs a panic runtime, but it depends on `needs_panic_runtime`
[00:53:18] - error: aborting due to previous error
[00:53:18] - error: aborting due to previous error
[00:53:18] + error: language item required, but not found: `eh_personality`
[00:53:18] + error: aborting due to 2 previous errors
[00:53:18] 4 
[00:53:18] 5 
[00:53:18] 
[00:53:18] 
[00:53:18] 
[00:53:18] The actual stderr differed from the expected stderr.
[00:53:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/runtime-depend-on-needs-runtime/runtime-depend-on-needs-runtime.stderr
[00:53:18] To update references, rerun the tests and pass the `--bless` flag
[00:53:18] To only update this specific test, also pass `--test-args panic-runtime/runtime-depend-on-needs-runtime.rs`
[00:53:18] error: 1 errors occurred comparing output.
[00:53:18] status: exit code: 1
[00:53:18] status: exit code: 1
[00:53:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/runtime-depend-on-needs-runtime.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/runtime-depend-on-needs-runtime/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/runtime-depend-on-needs-runtime/auxiliary" "-A" "unused"
[00:53:18] ------------------------------------------
[00:53:18] 
[00:53:18] ------------------------------------------
[00:53:18] stderr:
[00:53:18] stderr:
[00:53:18] ------------------------------------------
[00:53:18] {"message":"the crate `depends` cannot depend on a crate that needs a panic runtime, but it depends on `needs_panic_runtime`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the crate `depends` cannot depend on a crate that needs a panic runtime, but it depends on `needs_panic_runtime`\n\n"}
[00:53:18] {"message":"language item required, but not found: `eh_personality`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: language item required, but not found: `eh_personality`\n\n"}
[00:53:18] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:53:18] ------------------------------------------
[00:53:18] 
[00:53:18] thread '[ui] ui/panic-runtime/runtime-depend-on-needs-runtime.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:53:18] 
---
[00:53:18] 
[00:53:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[00:53:18] 
[00:53:18] 
[00:53:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:18] 
[00:53:18] 
[00:53:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:53:18] Build completed unsuccessfully in 0:49:58
---
travis_time:end:11069a71:start=1545571893828299660,finish=1545571893839466558,duration=11166898
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2f1c178d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2e3756c1
travis_time:start:2e3756c1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b155d9a
$ dmesg | grep -i kill
