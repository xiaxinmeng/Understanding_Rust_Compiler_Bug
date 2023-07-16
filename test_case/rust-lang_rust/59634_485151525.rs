plain
travis_time:end:1dc754f9:start=1555782917211603137,finish=1555782917966670199,duration=755067062
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:58] ...........................................i........................................................ 4000/5547
[01:06:00] .................................................................................................... 4100/5547
[01:06:02] ...i................................................................................................ 4200/5547
[01:06:06] .................................................................................................... 4300/5547
[01:06:17] ..........................................................................................F......... 4400/5547
[01:06:24] .................................................................................................... 4600/5547
[01:06:27] .................................................................................................... 4700/5547
[01:06:34] .................................................................................................... 4800/5547
[01:06:37] .................................................................................................... 4900/5547
---
[01:06:59] 53 
[01:06:59] 
[01:06:59] 
[01:06:59] The actual stderr differed from the expected stderr.
[01:06:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pub/pub-restricted/pub-restricted.stderr
[01:06:59] To update references, rerun the tests and pass the `--bless` flag
[01:06:59] To only update this specific test, also pass `--test-args pub/pub-restricted.rs`
[01:06:59] error: 1 errors occurred comparing output.
[01:06:59] status: exit code: 1
[01:06:59] status: exit code: 1
[01:06:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pub/pub-restricted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pub/pub-restricted/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pub/pub-restricted/auxiliary" "-A" "unused"
[01:06:59] ------------------------------------------
[01:06:59] 
[01:06:59] ------------------------------------------
[01:06:59] stderr:
[01:06:59] stderr:
[01:06:59] ------------------------------------------
[01:06:59] error[E0704]: incorrect visibility restriction
[01:06:59]   --> /checkout/src/test/ui/pub/pub-restricted.rs:5:6
[01:06:59]    |
[01:06:59] LL | pub (a) fn afn() {} //~ incorrect visibility restriction
[01:06:59]    |      ^ help: make this visible only to module `a` with `in`: `in a`
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]            `pub(crate)`: visible only on the current crate
[01:06:59]            `pub(super)`: visible only in the current module's parent
[01:06:59]            `pub(in path::to::module)`: visible only on the specified path
[01:06:59] error[E0704]: incorrect visibility restriction
[01:06:59]   --> /checkout/src/test/ui/pub/pub-restricted.rs:6:6
[01:06:59]    |
[01:06:59]    |
[01:06:59] LL | pub (b) fn bfn() {} //~ incorrect visibility restriction
[01:06:59]    |      ^ help: make this visible only to module `b` with `in`: `in b`
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]            `pub(crate)`: visible only on the current crate
[01:06:59]            `pub(super)`: visible only in the current module's parent
[01:06:59]            `pub(in path::to::module)`: visible only on the specified path
[01:06:59] error[E0704]: incorrect visibility restriction
[01:06:59]   --> /checkout/src/test/ui/pub/pub-restricted.rs:22:14
[01:06:59]    |
[01:06:59]    |
[01:06:59] LL |         pub (a) invalid: usize, //~ incorrect visibility restriction
[01:06:59]    |              ^ help: make this visible only to module `a` with `in`: `in a`
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]            `pub(crate)`: visible only on the current crate
[01:06:59]            `pub(super)`: visible only in the current module's parent
[01:06:59]            `pub(in path::to::module)`: visible only on the specified path
[01:06:59] error[E0704]: incorrect visibility restriction
[01:06:59]   --> /checkout/src/test/ui/pub/pub-restricted.rs:31:6
[01:06:59]    |
[01:06:59]    |
[01:06:59] LL | pub (xyz) fn xyz() {} //~ incorrect visibility restriction
[01:06:59]    |      ^^^ help: make this visible only to module `xyz` with `in`: `in xyz`
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]    = help: some possible visibility restrictions are:
[01:06:59]            `pub(crate)`: visible only on the current crate
[01:06:59]            `pub(super)`: visible only in the current module's parent
[01:06:59]            `pub(in path::to::module)`: visible only on the specified path
[01:06:59] 
[01:06:59] error: visibilities can only be restricted to ancestor modules
[01:06:59]    |
[01:06:59]    |
[01:06:59] LL |         pub (in x) non_parent_invalid: usize, //~ ERROR visibilities can only be restricted
[01:06:59] 
[01:06:59] error: aborting due to 5 previous errors
[01:06:59] 
[01:06:59] For more information about this error, try `rustc --explain E0704`.
---
[01:06:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:59] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:59] 
[01:06:59] 
[01:06:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:59] 
[01:06:59] 
[01:06:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:59] Build completed unsuccessfully in 0:04:22
[01:06:59] Build completed unsuccessfully in 0:04:22
[01:06:59] make: *** [check] Error 1
[01:06:59] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a21882
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 19:02:28 UTC 2019
---
travis_time:end:07b905eb:start=1555786949870420640,finish=1555786949877880668,duration=7460028
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aa04f24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:139f37c0
$ dmesg | grep -i kill
