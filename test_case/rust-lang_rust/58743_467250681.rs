plain
travis_time:end:022db5ee:start=1551139271445597141,finish=1551139346131469005,duration=74685871864
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:51] .................................................................................................... 2600/5431
[01:05:55] .................................................................................................... 2700/5431
[01:05:59] .................................................................................................... 2800/5431
[01:06:03] .................................................................................................... 2900/5431
[01:06:07] .........................F.......................................................................... 3000/5431
[01:06:14] .................................................................................................... 3200/5431
[01:06:18] ...................................................................i................................ 3300/5431
[01:06:21] .................................................................................................... 3400/5431
[01:06:25] ......................................ii...i..ii.................................................... 3500/5431
---
[01:07:37] 
[01:07:37] ---- [ui] ui/issues/issue-55731.rs stdout ----
[01:07:37] diff of stderr:
[01:07:37] 
[01:07:37] 4 LL |     multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough
[01:07:37] 6    |
[01:07:37] 6    |
[01:07:37] -    = note: Due to a where-clause on `multi`,
[01:07:37] -    = note: `Map<Cloned<&()>, X>` must implement `DistributedIteratorMulti<&'0 ()>`, for any lifetime `'0`
[01:07:37] -    = note: but `Map<Cloned<&()>, X>` actually implements `DistributedIteratorMulti<&'1 ()>`, for some specific lifetime `'1`
[01:07:37] +    = note: `DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`
[01:07:37] +    = note: but `DistributedIteratorMulti<&'1 ()>` is actually implemented for the type `Cloned<&'1 ()>`, for some specific lifetime `'1`
[01:07:37] 11 error: aborting due to previous error
[01:07:37] 12 
[01:07:37] 
[01:07:37] 
[01:07:37] 
[01:07:37] The actual stderr differed from the expected stderr.
[01:07:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/issue-55731.stderr
[01:07:37] To update references, rerun the tests and pass the `--bless` flag
[01:07:37] To only update this specific test, also pass `--test-args issues/issue-55731.rs`
[01:07:37] error: 1 errors occurred comparing output.
[01:07:37] status: exit code: 1
[01:07:37] status: exit code: 1
[01:07:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55731.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55731/auxiliary" "-A" "unused"
[01:07:37] ------------------------------------------
[01:07:37] 
[01:07:37] ------------------------------------------
[01:07:37] stderr:
[01:07:37] stderr:
[01:07:37] ------------------------------------------
[01:07:37] {"message":"implementation of `DistributedIteratorMulti` is not general enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-55731.rs","byte_start":926,"byte_end":931,"line_start":48,"line_end":48,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"but `DistributedIteratorMulti<&'1 ()>` is actually implemented for the type `Cloned<&'1 ()>`, for some specific lifetime `'1`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: implementation of `DistributedIteratorMulti` is not general enough\n  --> /checkout/src/test/ui/issues/issue-55731.rs:48:5\n   |\nLL |     multi(Map { //~ ERROR implementation of `DistributedIteratorMulti` is not general enough\n   |     ^^^^^\n   |\n   = note: `DistributedIteratorMulti<&'0 ()>` would have to be implemented for the type `Cloned<&()>`, for any lifetime `'0`\n   = note: but `DistributedIteratorMulti<&'1 ()>` is actually implemented for the type `Cloned<&'1 ()>`, for some specific lifetime `'1`\n\n"}
[01:07:37] 
[01:07:37] ------------------------------------------
[01:07:37] 
[01:07:37] thread '[ui] ui/issues/issue-55731.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:07:37] 
[01:07:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:07:37] 
[01:07:37] 
[01:07:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:37] 
[01:07:37] 
[01:07:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:37] Build completed unsuccessfully in 0:04:13
[01:07:37] Build completed unsuccessfully in 0:04:13
[01:07:37] make: *** [check] Error 1
[01:07:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:063d4519
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 26 01:10:13 UTC 2019
---
travis_time:end:297aea8c:start=1551143415139057416,finish=1551143415144755202,duration=5697786
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02495916
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0adc5f38
travis_time:start:0adc5f38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16b2fb5f
$ dmesg | grep -i kill
