plain
travis_time:end:1f484b7f:start=1557059359687140790,finish=1557059361785522737,duration=2098381947
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:45] .................................................................................................... 4600/5494
[01:13:50] .................................................................................................... 4700/5494
[01:13:55] .................................................................................................... 4800/5494
[01:13:59] .................................................................................................... 4900/5494
[01:14:03] .....................................................................................F.............. 5000/5494
[01:14:11] .................................................................................................... 5200/5494
[01:14:14] .................................................................................................... 5300/5494
[01:14:17] .................................................................................................... 5400/5494
[01:14:19] ................................i.............................................................
---
[01:14:19] 27    |
[01:14:19] 28 LL |         &v => {},
[01:14:19] -    |         ^^
[01:14:19] -    |         |
[01:14:19] -    |         expected i32, found reference
[01:14:19] -    |         help: you can probably remove the explicit borrow: `v`
[01:14:19] +    |         ^^ expected i32, found reference
[01:14:19] 34    = note: expected type `i32`
[01:14:19] 35               found type `&_`
[01:14:19] 
[01:14:19] 
[01:14:19] 
[01:14:19] The actual stderr differed from the expected stderr.
[01:14:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-ergonomics/match-ergonomics.stderr
[01:14:19] To update references, rerun the tests and pass the `--bless` flag
[01:14:19] To only update this specific test, also pass `--test-args suggestions/match-ergonomics.rs`
[01:14:19] error: 1 errors occurred comparing output.
[01:14:19] status: exit code: 1
[01:14:19] status: exit code: 1
[01:14:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-ergonomics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-ergonomics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-ergonomics/auxiliary" "-A" "unused"
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] stderr:
[01:14:19] stderr:
[01:14:19] ------------------------------------------
[01:14:19] error[E0308]: mismatched types
[01:14:19]   --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:4:10
[01:14:19]    |
[01:14:19] LL |         [&v] => {}, //~ ERROR mismatched types
[01:14:19]    |          |
[01:14:19]    |          expected i32, found reference
[01:14:19]    |          expected i32, found reference
[01:14:19]    |          help: you can probably remove the explicit borrow: `v`
[01:14:19]    = note: expected type `i32`
[01:14:19]               found type `&_`
[01:14:19] 
[01:14:19] error[E0529]: expected an array or slice, found `std::vec::Vec<i32>`
[01:14:19] error[E0529]: expected an array or slice, found `std::vec::Vec<i32>`
[01:14:19]   --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:8:9
[01:14:19]    |
[01:14:19] LL |         [&v] => {}, //~ ERROR expected an array or slice
[01:14:19]    |         ^^^^ pattern cannot match with input type `std::vec::Vec<i32>`
[01:14:19] error[E0529]: expected an array or slice, found `std::vec::Vec<i32>`
[01:14:19]   --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:20:9
[01:14:19]    |
[01:14:19]    |
[01:14:19] LL |         [v] => {}, //~ ERROR expected an array or slice
[01:14:19]    |         ^^^ pattern cannot match with input type `std::vec::Vec<i32>`
[01:14:19] error[E0308]: mismatched types
[01:14:19]   --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:29:9
[01:14:19]    |
[01:14:19]    |
[01:14:19] LL |         &v => {}, //~ ERROR mismatched types
[01:14:19]    |         ^^ expected i32, found reference
[01:14:19]    = note: expected type `i32`
[01:14:19]               found type `&_`
[01:14:19] 
[01:14:19] error[E0308]: mismatched types
[01:14:19] error[E0308]: mismatched types
[01:14:19]   --> /checkout/src/test/ui/suggestions/match-ergonomics.rs:40:13
[01:14:19]    |
[01:14:19] LL |     if let [&v] = &x[..] {} //~ ERROR mismatched types
[01:14:19]    |             |
[01:14:19]    |             expected i32, found reference
[01:14:19]    |             expected i32, found reference
[01:14:19]    |             help: you can probably remove the explicit borrow: `v`
[01:14:19]    = note: expected type `i32`
[01:14:19]               found type `&_`
[01:14:19] 
[01:14:19] error: aborting due to 5 previous errors
---
[01:14:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:14:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:19] 
[01:14:19] 
[01:14:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:19] 
[01:14:19] 
[01:14:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:19] Build completed unsuccessfully in 0:04:32
[01:14:19] Build completed unsuccessfully in 0:04:32
[01:14:19] make: *** [check] Error 1
[01:14:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d84dc2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May  5 13:43:52 UTC 2019
