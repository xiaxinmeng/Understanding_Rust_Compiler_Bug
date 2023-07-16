plain
travis_time:end:0af23df9:start=1552472370872613646,finish=1552472371789196158,duration=916582512
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:13:10] .................................................................................................... 3400/5468
[01:13:14] .................................................................ii...i..ii......................... 3500/5468
[01:13:18] .................................................................................................... 3600/5468
[01:13:22] .................................................................................................... 3700/5468
[01:13:25] .......................F....................................................ii...................... 3800/5468
[01:13:30] .................................................................................................... 4000/5468
[01:13:32] ....................................................i............................................... 4100/5468
[01:13:35] .................................................................................................... 4200/5468
[01:13:44] .................................................................................................... 4300/5468
---
[01:14:29] 
[01:14:29] ---- [ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs stdout ----
[01:14:29] diff of stderr:
[01:14:29] 
[01:14:29] 4 LL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:14:29] 5    |                        -- lifetime `'a` defined here
[01:14:29] 6 ...
[01:14:29] - LL |     y //~ ERROR unsatisfied lifetime constraints
[01:14:29] + LL |     y
[01:14:29] 8    |     ^ returning this value requires that `'a` must outlive `'static`
[01:14:29] 10 error: lifetime may not live long enough
[01:14:29] 
[01:14:29] 
[01:14:29] 13 LL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:14:29] 14    |                      -- lifetime `'a` defined here
[01:14:29] 15 ...
[01:14:29] - LL |     y //~ ERROR unsatisfied lifetime constraints
[01:14:29] + LL |     y
[01:14:29] 17    |     ^ returning this value requires that `'a` must outlive `'static`
[01:14:29] 19 error: lifetime may not live long enough
[01:14:29] 
[01:14:29] 
[01:14:29] 22 LL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:14:29] 23    |                      -- lifetime `'a` defined here
[01:14:29] 24 ...
[01:14:29] - LL |     y //~ ERROR unsatisfied lifetime constraints
[01:14:29] + LL |     y
[01:14:29] 26    |     ^ returning this value requires that `'a` must outlive `'static`
[01:14:29] 28 error: aborting due to 3 previous errors
[01:14:29] 
[01:14:29] 
[01:14:29] The actual stderr differed from the expected stderr.
[01:14:29] The actual stderr differed from the expected stderr.
[01:14:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/issue-55748-pat-types-constrain-bindings.stderr
[01:14:29] To update references, rerun the tests and pass the `--bless` flag
[01:14:29] To only update this specific test, also pass `--test-args nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs`
[01:14:29] error: 1 errors occurred comparing output.
[01:14:29] status: exit code: 1
[01:14:29] status: exit code: 1
[01:14:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/auxiliary" "-A" "unused"
[01:14:29] ------------------------------------------
[01:14:29] 
[01:14:29] ------------------------------------------
[01:14:29] stderr:
[01:14:29] stderr:
[01:14:29] ------------------------------------------
[01:14:29] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":903,"byte_end":905,"line_start":28,"line_end":28,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":24,"highlight_end":26}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1175,"byte_end":1176,"line_start":35,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:35:5\n   |\nLL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                        -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:14:29] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1343,"byte_end":1345,"line_start":42,"line_end":42,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1612,"byte_end":1613,"line_start":49,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:49:5\n   |\nLL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:14:29] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1780,"byte_end":1782,"line_start":56,"line_end":56,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":2041,"byte_end":2042,"line_start":62,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:62:5\n   |\nLL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:14:29] 
[01:14:29] ------------------------------------------
[01:14:29] 
[01:14:29] thread '[ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:14:29] 
[01:14:29] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:29] 
[01:14:29] 
[01:14:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:29] 
[01:14:29] 
[01:14:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:29] Build completed unsuccessfully in 0:04:21
[01:14:29] Build completed unsuccessfully in 0:04:21
[01:14:29] make: *** [check] Error 1
[01:14:29] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:105e4114
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 11:34:12 UTC 2019
---
travis_time:end:006e1b40:start=1552476853851533607,finish=1552476853856763896,duration=5230289
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01ad86f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:266819f4
travis_time:start:266819f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05832280
$ dmesg | grep -i kill
