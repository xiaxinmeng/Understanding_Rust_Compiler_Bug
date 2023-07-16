\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/issues/issue-46101.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `issue_46101`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/issues/issue-46101.rs`\n\n"}
[01:03:56] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:03:56] {"message":"Some errors occurred: E0433, E0601.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0433, E0601.\n"}
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] 
[01:03:56] thread '[ui] ui/issues/issue-46101.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3301:9
[01:03:56] thread '[ui] ui/issues/issue-46101.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3301:9
[01:03:56] 
[01:03:56] ---- [ui] ui/issues/issue-55731.rs stdout ----
[01:03:56] diff of stderr:
[01:03:56] 
[01:03:56] 1 error: implementation of `DistributedIteratorMulti` is not general enough
[01:03:56] 3    |
[01:03:56] 3    |
[01:03:56] - LL |     multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough
[01:03:56] + LL |     multi(Map {
[01:03:56] 6    |
[01:03:56] 6    |
[01:03:56] 7    = note: `DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`
[01:03:56] 
[01:03:56] The actual stderr differed from the expected stderr.
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/issue-55731.stderr
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/issue-55731.stderr
[01:03:56] To update references, rerun the tests and pass the `--bless` flag
[01:03:56] To only update this specific test, also pass `--test-args issues/issue-55731.rs`
[01:03:56] error: 1 errors occurred comparing output.
[01:03:56] status: exit code: 1
[01:03:56] status: exit code: 1
[01:03:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55731.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/auxiliary" "-A" "unused"
[01:03:56] ------------------------------------------
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] stderr:
[01:03:56] stderr:
[01:03:56] ------------------------------------------
[01:03:56] {"message":"implementation of `DistributedIteratorMulti` is not general enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-55731.rs","byte_start":926,"byte_end":931,"line_start":48,"line_end":48,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"but `DistributedIteratorMulti<&'1 ()>` is actually implemented for the type `Cloned<&'1 ()>`, for some specific lifetime `'1`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: implementation of `DistributedIteratorMulti` is not general enough\n  --> /checkout/src/test/ui/issues/issue-55731.rs:48:5\n   |\nLL |     multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough\n   |     ^^^^^\n   |\n   = note: `DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`\n   = note: but `DistributedIteratorMulti<&'1 ()>` is actually implemented for the type `Cloned<&'1 ()>`, for some specific lifetime `'1`\n\n"}
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] 
[01:03:56] thread '[ui] ui/issues/issue-55731.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3301:9
---
[01:03:56] 
[01:03:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:03:56] 
[01:03:56] 
[01:03:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:56] 
[01:03:56] 
[01:03:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[01:03:56] Build completed unsuccessfully in 0:59:25
---
travis_time:end:01770609:start=1552356213573429043,finish=1552356213588959137,duration=15530094
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12b5cbf6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c1e654c
travis_time:start:0c1e654c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:200472cc
$ dmesg | grep -i kill
