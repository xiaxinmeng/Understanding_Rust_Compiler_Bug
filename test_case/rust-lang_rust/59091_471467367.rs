plain
travis_time:end:04b93c88:start=1552292506574220836,finish=1552292580570039258,duration=73995818422
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:27] ....................................i............................................................... 600/5445
[01:09:31] .................................................................................................... 700/5445
[01:09:36] .................................................................................................... 800/5445
[01:09:40] .............................................................................................i...... 900/5445
[01:09:45] .........i.............F............................................................................ 1000/5445
[01:09:48] ......................iiiii......................................................................... 1100/5445
[01:09:54] .................................................................................................... 1300/5445
[01:09:57] .................................................................................................... 1400/5445
[01:10:00] .................................................................................................... 1500/5445
[01:10:02] .................................................................................................... 1600/5445
---
[01:12:27] 
[01:12:27] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:12:27] diff of stderr:
[01:12:27] 
[01:12:27] - error: OK
[01:12:27] + error: no path from `TypeAlias` to `ItemVariances`
[01:12:27] 3    |
[01:12:27] 3    |
[01:12:27] 4 LL | #[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK
[01:12:27] 
[01:12:27] The actual stderr differed from the expected stderr.
[01:12:27] The actual stderr differed from the expected stderr.
[01:12:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.stderr
[01:12:27] To update references, rerun the tests and pass the `--bless` flag
[01:12:27] To only update this specific test, also pass `--test-args dep-graph/dep-graph-variance-alias.rs`
[01:12:27] error: 1 errors occurred comparing output.
[01:12:27] status: exit code: 1
[01:12:27] status: exit code: 1
[01:12:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:12:27] ------------------------------------------
[01:12:27] 
[01:12:27] ------------------------------------------
[01:12:27] stderr:
[01:12:27] stderr:
[01:12:27] ------------------------------------------
[01:12:27] {"message":"no path from `TypeAlias` to `ItemVariances`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs","byte_start":315,"byte_end":359,"line_start":19,"line_end":19,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK","highlight_start":1,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `ItemVariances`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs:19:1\n   |\nLL | #[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:12:27] 
[01:12:27] ------------------------------------------
[01:12:27] 
[01:12:27] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:12:27] 
[01:12:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:27] 
[01:12:27] 
[01:12:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:27] 
[01:12:27] 
[01:12:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:27] Build completed unsuccessfully in 0:04:18
[01:12:27] Build completed unsuccessfully in 0:04:18
[01:12:27] Makefile:48: recipe for target 'check' failed
[01:12:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a43118
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 11 09:35:37 UTC 2019
---
travis_time:end:25fe278a:start=1552296938874870317,finish=1552296938879674241,duration=4803924
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13197904
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1382efd4
travis_time:start:1382efd4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1bbaea80
$ dmesg | grep -i kill
