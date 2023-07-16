plain
travis_time:end:06426496:start=1544456872056486514,finish=1544456873489662873,duration=1433176359
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:28] .................................................................................................... 400/5170
[00:47:31] .................................................................................................... 500/5170
[00:47:34] ..............................i..................................................................... 600/5170
[00:47:38] .................................................................................................... 700/5170
[00:47:43] .....................................................................................F.............. 800/5170
[00:47:50] ...........................iiiii.................................................................... 1000/5170
[00:47:53] .................................................................................................... 1100/5170
[00:47:55] .................................................................................................... 1200/5170
[00:47:57] .................................................................................................... 1300/5170
---
[00:50:04] .................................................................................................... 5100/5170
[00:50:05] .........i............................................................
[00:50:05] failures:
[00:50:05] 
[00:50:05] ---- [ui] ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs stdout ----
[00:50:05] 
[00:50:05] 
[00:50:05] 10 LL |     const X: Vec<u32> = Vec::new(); //~ ERROR not yet stable as a const fn
[00:50:05] 12    |
[00:50:05] 12    |
[00:50:05] -    = help: in Nightly builds, add `#![feature(const_vec_new)]` to the crate attributes to enable
[00:50:05] +    = help: add `#![feature(const_vec_new)]` to the crate attributes to enable
[00:50:05] 15 error: aborting due to 2 previous errors
[00:50:05] 16 
[00:50:05] 
[00:50:05] 
[00:50:05] 
[00:50:05] The actual stderr differed from the expected stderr.
[00:50:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/feature-gate-unleash_the_miri_inside_of_you.stderr
[00:50:05] To update references, rerun the tests and pass the `--bless` flag
[00:50:05] To only update this specific test, also pass `--test-args consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs`
[00:50:05] error: 1 errors occurred comparing output.
[00:50:05] status: exit code: 1
[00:50:05] status: exit code: 1
[00:50:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/auxiliary" "-A" "unused"
[00:50:05] ------------------------------------------
[00:50:05] 
[00:50:05] ------------------------------------------
[00:50:05] stderr:
[00:50:05] stderr:
[00:50:05] ------------------------------------------
[00:50:05] {"message":"destructors cannot be evaluated at compile-time","code":{"code":"E0493","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs","byte_start":246,"byte_end":256,"line_start":11,"line_end":11,"column_start":20,"column_end":30,"is_primary":true,"text":[{"text":"    const F: u32 = (U::X, 42).1; //~ ERROR destructors cannot be evaluated at compile-time","highlight_start":20,"highlight_end":30}],"label":"constants cannot evaluate destructors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0493]: destructors cannot be evaluated at compile-time\n  --> /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:11:20\n   |\nLL |     const F: u32 = (U::X, 42).1; //~ ERROR destructors cannot be evaluated at compile-time\n   |                    ^^^^^^^^^^ constants cannot evaluate destructors\n\n"}
[00:50:05] {"message":"`<std::vec::Vec<T>>::new` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs","byte_start":425,"byte_end":435,"line_start":18,"line_end":18,"column_start":25,"column_end":35,"is_primary":true,"text":[{"text":"    const X: Vec<u32> = Vec::new(); //~ ERROR not yet stable as a const fn","highlight_start":25,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add `#![feature(const_vec_new)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `<std::vec::Vec<T>>::new` is not yet stable as a const fn\n  --> /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:18:25\n   |\nLL |     const X: Vec<u32> = Vec::new(); //~ ERROR not yet stable as a const fn\n   |                         ^^^^^^^^^^\n   |\n   = help: add `#![feature(const_vec_new)]` to the crate attributes to enable\n\n"}
[00:50:05] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:50:05] {"message":"For more information about this error, try `rustc --explain E0493`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0493`.\n"}
[00:50:05] ------------------------------------------
[00:50:05] 
[00:50:05] 
[00:50:05] thread '[ui] ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:50:05] 
[00:50:05] 
[00:50:05] failures:
[00:50:05] failures:
[00:50:05]     [ui] ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs
[00:50:05] test result: FAILED. 5145 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:50:05] 
[00:50:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:50:05] 
[00:50:05] 
[00:50:05] 
[00:50:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:05] 
[00:50:05] 
[00:50:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:05] Build completed unsuccessfully in 0:03:54
[00:50:05] Build completed unsuccessfully in 0:03:54
[00:50:05] make: *** [check] Error 1
[00:50:05] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b6fdc3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 10 16:38:07 UTC 2018
---
travis_time:end:14364c3c:start=1544459889135129306,finish=1544459889139501247,duration=4371941
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014e4580
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|o
