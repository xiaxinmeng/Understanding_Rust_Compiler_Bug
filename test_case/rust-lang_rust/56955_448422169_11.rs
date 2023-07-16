\n\nwhere `P1, ..., Pm` are the type parameters of the `impl` and `T0, ..., Tn`\nare types. One of the types `T0, ..., Tn` must be a local type (this is another\norphan rule, see the explanation for E0117). Let `i` be the smallest integer\nsuch that `Ti` is a local type. Then no type parameter can appear in any of the\n`Tj` for `j < i`.\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-41974.rs","byte_start":518,"byte_end":547,"line_start":17,"line_end":17,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"impl<T> Drop for T where T: A { //~ ERROR E0119","highlight_start":1,"highlight_end":30}],"label":"type parameter `T` must be used as the type parameter for some local type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"only traits defined in the current crate can be implemented for a type parameter","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)\n  --> /checkout/src/test/ui/issues/issue-41974.rs:17:1\n   |\nLL | impl<T> Drop for T where T: A { //~ ERROR E0119\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type\n   |\n   = note: only traits defined in the current crate can be implemented for a type parameter\n\n"}
[00:48:39] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:39] {"message":"Some errors occurred: E0119, E0120, E0210.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0119, E0120, E0210.\n"}
[00:48:39] {"message":"For more information about an error, try `rustc --explain E0119`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0119`.\n"}
[00:48:39] ------------------------------------------
[00:48:39] 
[00:48:39] thread '[ui] ui/issues/issue-41974.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:48:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:39] 
[00:48:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:48:39] 
[00:48:39] 
[00:48:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:39] 
[00:48:39] 
[00:48:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:39] Build completed unsuccessfully in 0:03:55
[00:48:39] Build completed unsuccessfully in 0:03:55
[00:48:39] make: *** [check] Error 1
[00:48:39] Makefile:58: recipe for target 'check' failed
124324 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
124320 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
118652 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115344 ./src/llvm/test/CodeGen
---
travis_time:end:3284e09b:start=1545178872154075995,finish=1545178872160617996,duration=6542001
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12aab3a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2735e02c
travis_time:start:2735e02c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16a8e9e0
$ dmesg | grep -i kill
