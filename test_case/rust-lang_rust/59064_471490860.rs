plain
travis_time:end:1afde2be:start=1552296699888209224,finish=1552296775795550620,duration=75907341396
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:08] .......................................i............................................................ 600/5448
[01:11:12] .................................................................................................... 700/5448
[01:11:16] .................................................................................................... 800/5448
[01:11:21] ................................................................................................i... 900/5448
[01:11:25] ............i.............F......................................................................... 1000/5448
[01:11:29] .........................iiiii...................................................................... 1100/5448
[01:11:34] .................................................................................................... 1300/5448
[01:11:37] .................................................................................................... 1400/5448
[01:11:40] .................................................................................................... 1500/5448
[01:11:43] .................................................................................................... 1600/5448
---
[01:14:08] 
[01:14:08] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:14:08] diff of stderr:
[01:14:08] 
[01:14:08] - error: OK
[01:14:08] + error: no path from `TypeAlias` to `ItemVariances`
[01:14:08] 3    |
[01:14:08] 3    |
[01:14:08] 4 LL | #[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK
[01:14:08] 
[01:14:08] The actual stderr differed from the expected stderr.
[01:14:08] The actual stderr differed from the expected stderr.
[01:14:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.stderr
[01:14:08] To update references, rerun the tests and pass the `--bless` flag
[01:14:08] To only update this specific test, also pass `--test-args dep-graph/dep-graph-variance-alias.rs`
[01:14:08] error: 1 errors occurred comparing output.
[01:14:08] status: exit code: 1
[01:14:08] status: exit code: 1
[01:14:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:14:08] ------------------------------------------
[01:14:08] 
[01:14:08] ------------------------------------------
[01:14:08] stderr:
[01:14:08] stderr:
[01:14:08] ------------------------------------------
[01:14:08] {"message":"no path from `TypeAlias` to `ItemVariances`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs","byte_start":315,"byte_end":359,"line_start":19,"line_end":19,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"#[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK","highlight_start":1,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `TypeAlias` to `ItemVariances`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs:19:1\n   |\nLL | #[rustc_then_this_would_need(ItemVariances)] //~ ERROR OK\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:14:08] 
[01:14:08] ------------------------------------------
[01:14:08] 
[01:14:08] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:14:08] 
[01:14:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:08] 
[01:14:08] 
[01:14:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:08] 
[01:14:08] 
[01:14:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:08] Build completed unsuccessfully in 0:04:20
[01:14:08] Build completed unsuccessfully in 0:04:20
[01:14:08] make: *** [check] Error 1
[01:14:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:027fb9c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 11 10:47:14 UTC 2019
