plain
travis_time:end:11d1a0a4:start=1552557739794898745,finish=1552557741944438818,duration=2149540073
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:10:55] .................................................................................................... 500/5467
[01:10:59] ..........................................i......................................................... 600/5467
[01:11:03] .................................................................................................... 700/5467
[01:11:08] .................................................................................................... 800/5467
[01:11:12] ...................................................................................................i 900/5467
[01:11:17] ...............i..........F......................................................................... 1000/5467
[01:11:21] ............................iiiii................................................................... 1100/5467
[01:11:27] .................................................................................................... 1300/5467
[01:11:30] .................................................................................................... 1400/5467
[01:11:33] .................................................................................................... 1500/5467
[01:11:36] .................................................................................................... 1600/5467
---
[01:14:11] 
[01:14:11] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:14:11] diff of stderr:
[01:14:11] 
[01:14:11] - error: no path from `x::<impl Foo for char>` to `TypeckTables`
[01:14:11] + error: OK
[01:14:11] 3    |
[01:14:11] 3    |
[01:14:11] 4 LL |     #[rustc_then_this_would_need(TypeckTables)]
[01:14:11] 
[01:14:11] The actual stderr differed from the expected stderr.
[01:14:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/dep-graph-trait-impl-two-traits.stderr
[01:14:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/dep-graph-trait-impl-two-traits.stderr
[01:14:11] To update references, rerun the tests and pass the `--bless` flag
[01:14:11] To only update this specific test, also pass `--test-args dep-graph/dep-graph-trait-impl-two-traits.rs`
[01:14:11] error: 1 errors occurred comparing output.
[01:14:11] status: exit code: 1
[01:14:11] status: exit code: 1
[01:14:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] stderr:
[01:14:11] stderr:
[01:14:11] ------------------------------------------
[01:14:11] {"message":"OK","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs","byte_start":483,"byte_end":526,"line_start":31,"line_end":31,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: OK\n  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs:31:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:14:11] {"message":"no path from `x::<impl Foo for char>` to `TypeckTables`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs","byte_start":626,"byte_end":669,"line_start":40,"line_end":40,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no path from `x::<impl Foo for char>` to `TypeckTables`\n  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs:40:5\n   |\nLL |     #[rustc_then_this_would_need(TypeckTables)] //~ ERROR no path\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:14:11] 
[01:14:11] ------------------------------------------
[01:14:11] 
[01:14:11] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:14:11] 
[01:14:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:11] 
[01:14:11] 
[01:14:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:11] 
[01:14:11] 
[01:14:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:11] Build completed unsuccessfully in 0:04:32
[01:14:11] Build completed unsuccessfully in 0:04:32
[01:14:11] make: *** [check] Error 1
[01:14:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0030632a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 14 11:16:57 UTC 2019
