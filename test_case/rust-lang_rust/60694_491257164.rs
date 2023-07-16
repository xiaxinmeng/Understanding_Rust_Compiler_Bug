plain
travis_time:end:1bf08eba:start=1557483375632649680,finish=1557483463902656580,duration=88270006900
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:23] failures:
[01:14:23] 
[01:14:23] ---- [ui] ui/issues/issue-60662.rs stdout ----
[01:14:23] normalized stdout:
[01:14:23] #[prelude_import]
[01:14:23] use ::std::prelude::v1::*;
[01:14:23] #[macro_use]
[01:14:23] extern crate std;
[01:14:23] // compile-flags: -Z unpretty=hir
[01:14:23] 
[01:14:23] trait Animal { }
[01:14:23] 
[01:14:23] fn main() {
[01:14:23] fn main() {
[01:14:23]               pub existential type ServeFut : Animal;
[01:14:23] 
[01:14:23] 
[01:14:23] 
[01:14:23] The actual stdout differed from the expected stdout.
[01:14:23] The actual stdout differed from the expected stdout.
[01:14:23] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/issue-60662.stdout
[01:14:23] normalized stderr:
[01:14:23] error[E0658]: existential types are unstable
[01:14:23]   --> $DIR/issue-60662.rs:8:5
[01:14:23]    |
[01:14:23] LL |     pub existential type ServeFut: Animal;
[01:14:23]    |
[01:14:23]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[01:14:23]    = help: add #![feature(existential_type)] to the crate attributes to enable
[01:14:23] 
---
[01:14:23] 
[01:14:23] 
[01:14:23] The actual stderr differed from the expected stderr.
[01:14:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/issue-60662.stderr
[01:14:23] To update references, rerun the tests and pass the `--bless` flag
[01:14:23] To only update this specific test, also pass `--test-args issues/issue-60662.rs`
[01:14:23] error: 2 errors occurred comparing output.
[01:14:23] status: exit code: 1
[01:14:23] status: exit code: 1
[01:14:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/auxiliary" "-A" "unused"
[01:14:23] ------------------------------------------
[01:14:23] ------------------------------------------
[01:14:23] #[prelude_import]
[01:14:23] use ::std::prelude::v1::*;
[01:14:23] #[macro_use]
[01:14:23] extern crate std;
[01:14:23] // compile-flags: -Z unpretty=hir
[01:14:23] 
[01:14:23] trait Animal { }
[01:14:23] 
[01:14:23] fn main() {
[01:14:23] fn main() {
[01:14:23]               pub existential type ServeFut : Animal;
[01:14:23] 
[01:14:23] ------------------------------------------
[01:14:23] stderr:
[01:14:23] ------------------------------------------
[01:14:23] ------------------------------------------
[01:14:23] error[E0658]: existential types are unstable
[01:14:23]   --> /checkout/src/test/ui/issues/issue-60662.rs:8:5
[01:14:23]    |
[01:14:23] LL |     pub existential type ServeFut: Animal;
[01:14:23]    |
[01:14:23]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[01:14:23]    = help: add #![feature(existential_type)] to the crate attributes to enable
[01:14:23] 
---
[01:14:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:14:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:23] 
[01:14:23] 
[01:14:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:23] 
[01:14:23] 
[01:14:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:23] Build completed unsuccessfully in 0:05:00
[01:14:23] Build completed unsuccessfully in 0:05:00
[01:14:23] make: *** [check] Error 1
[01:14:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:042a36dd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 10 11:32:17 UTC 2019
---
travis_time:end:12bc382c:start=1557487938678571608,finish=1557487938686837185,duration=8265577
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16e3d86e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ce86d6a
$ dmesg | grep -i kill
