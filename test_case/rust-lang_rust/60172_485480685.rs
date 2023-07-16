plain
travis_time:end:2fe38fd8:start=1555948828481503574,finish=1555948830846110146,duration=2364606572
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:03] .................................................................................................... 300/5461
[01:10:07] .................................................................................................... 400/5461
[01:10:10] ...........................................................................................i........ 500/5461
[01:10:15] .................................................................................................... 600/5461
[01:10:19] ......................................................................................F............. 700/5461
[01:10:30] ......................................................i...............i............................. 900/5461
[01:10:34] .......................................................................................iiiii........ 1000/5461
[01:10:38] .................................................................................................... 1100/5461
[01:10:40] .................................................................................................... 1200/5461
---
[01:12:36] .................................................................................................... 4300/5461
[01:12:40] .................................................................................................... 4400/5461
[01:12:43] .................................................................................................... 4500/5461
[01:12:47] .................................................................................................... 4600/5461
[01:12:52] ...........................................................................FFFF..F.................. 4700/5461
[01:13:00] .................................................................................................... 4900/5461
[01:13:04] .................................................................................................... 5000/5461
[01:13:07] .................................................................................................... 5100/5461
[01:13:11] .................................................................................................... 5200/5461
---
[01:13:19] 1 error[E0308]: mismatched types
[01:13:19] -   --> $DIR/const-eval-overflow-3b.rs:24:22
[01:13:19] +   --> $DIR/const-eval-overflow-3b.rs:18:22
[01:13:19] 3    |
[01:13:19] 4 LL |     = [0; (i8::MAX + 1u8) as usize];
[01:13:19] 5    |                      ^^^ expected i8, found u8
[01:13:19] 6 
[01:13:19] 7 error[E0277]: cannot add `u8` to `i8`
[01:13:19] -   --> $DIR/const-eval-overflow-3b.rs:24:20
[01:13:19] +   --> $DIR/const-eval-overflow-3b.rs:18:20
[01:13:19] +   --> $DIR/const-eval-overflow-3b.rs:18:20
[01:13:19] 9    |
[01:13:19] 10 LL |     = [0; (i8::MAX + 1u8) as usize];
[01:13:19] 11    |                    ^ no implementation for `i8 + u8`
[01:13:19] 
[01:13:19] The actual stderr differed from the expected stderr.
[01:13:19] The actual stderr differed from the expected stderr.
[01:13:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b/const-eval-overflow-3b.stderr
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-3b.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 1
[01:13:19] status: exit code: 1
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3b/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
[01:13:19] stderr:
[01:13:19] ------------------------------------------
[01:13:19] error[E0308]: mismatched types
[01:13:19]   --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-3b.rs:18:22
[01:13:19]    |
[01:13:19] LL |     = [0; (i8::MAX + 1u8) as usize];
[01:13:19]    |                      ^^^ expected i8, found u8
[01:13:19] error[E0277]: cannot add `u8` to `i8`
[01:13:19]   --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-3b.rs:18:20
[01:13:19]    |
[01:13:19]    |
[01:13:19] LL |     = [0; (i8::MAX + 1u8) as usize];
[01:13:19]    |                    ^ no implementation for `i8 + u8`
[01:13:19]    |
[01:13:19]    = help: the trait `std::ops::Add<u8>` is not implemented for `i8`
[01:13:19] error: aborting due to 2 previous errors
[01:13:19] 
[01:13:19] Some errors have detailed explanations: E0277, E0308.
[01:13:19] For more information about an error, try `rustc --explain E0277`.
[01:13:19] For more information about an error, try `rustc --explain E0277`.
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] 
[01:13:19] ---- [ui] ui/rust-2018/extern-crate-idiomatic-in-2018.rs stdout ----
[01:13:19] diff of fixed:
[01:13:19] 
[01:13:19] 23     // But this should be a use of the (renamed) crate:
[01:13:19] 24     crate::bar::foo();
[01:13:19] - 
[01:13:19] 27 
[01:13:19] 
[01:13:19] 
[01:13:19] 
[01:13:19] The actual fixed differed from the expected fixed.
[01:13:19] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.fixed
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic-in-2018.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 1
[01:13:19] status: exit code: 1
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
---
[01:13:19]    |
[01:13:19] note: lint level defined here
[01:13:19]   --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:9:9
[01:13:19]    |
[01:13:19] LL | #![deny(rust_2018_idioms)]
[01:13:19]    |         ^^^^^^^^^^^^^^^^
[01:13:19]    = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]
[01:13:19] error: `extern crate` is not idiomatic in the new edition
[01:13:19]   --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:15:1
[01:13:19]    |
[01:13:19]    |
[01:13:19] LL | extern crate edition_lint_paths as bar;
[01:13:19] 
[01:13:19] error: aborting due to 2 previous errors
[01:13:19] 
[01:13:19] 
---
[01:13:19] - 
[01:13:19] 20 
[01:13:19] 
[01:13:19] 
[01:13:19] The actual fixed differed from the expected fixed.
[01:13:19] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/extern-crate-idiomatic.fixed
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 0
[01:13:19] status: exit code: 0
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
---
[01:13:19] - 
[01:13:19] 19 
[01:13:19] 
[01:13:19] 
[01:13:19] The actual fixed differed from the expected fixed.
[01:13:19] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-referenced-by-self-path/extern-crate-referenced-by-self-path.fixed
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args rust-2018/extern-crate-referenced-by-self-path.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 0
[01:13:19] status: exit code: 0
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-referenced-by-self-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-referenced-by-self-path/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-referenced-by-self-path/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
---
[01:13:19] - 
[01:13:19] 20 
[01:13:19] 
[01:13:19] 
[01:13:19] The actual fixed differed from the expected fixed.
[01:13:19] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-rename/extern-crate-rename.fixed
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args rust-2018/extern-crate-rename.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 1
[01:13:19] status: exit code: 1
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-rename/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-rename/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
[01:13:19] stderr:
[01:13:19] ------------------------------------------
[01:13:19] error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[01:13:19]    |
[01:13:19] LL | use my_crate::foo;
[01:13:19] LL | use my_crate::foo;
[01:13:19]    |     ^^^^^^^^^^^^^ help: use `crate`: `crate::my_crate::foo`
[01:13:19] note: lint level defined here
[01:13:19]   --> /checkout/src/test/ui/rust-2018/extern-crate-rename.rs:8:9
[01:13:19]    |
[01:13:19]    |
[01:13:19] LL | #![deny(absolute_paths_not_starting_with_crate)]
[01:13:19]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[01:13:19]    = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>
[01:13:19] 
[01:13:19] error: aborting due to previous error
---
[01:13:19] - 
[01:13:19] 27 
[01:13:19] 
[01:13:19] 
[01:13:19] The actual fixed differed from the expected fixed.
[01:13:19] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-submod/extern-crate-submod.fixed
[01:13:19] To update references, rerun the tests and pass the `--bless` flag
[01:13:19] To only update this specific test, also pass `--test-args rust-2018/extern-crate-submod.rs`
[01:13:19] error: 1 errors occurred comparing output.
[01:13:19] status: exit code: 1
[01:13:19] status: exit code: 1
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-submod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-submod/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-submod/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
[01:13:19] stderr:
[01:13:19] ------------------------------------------
[01:13:19] error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[01:13:19]    |
[01:13:19] LL | use m::edition_lint_paths::foo;
[01:13:19] LL | use m::edition_lint_paths::foo;
[01:13:19]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::m::edition_lint_paths::foo`
[01:13:19] note: lint level defined here
[01:13:19]   --> /checkout/src/test/ui/rust-2018/extern-crate-submod.rs:9:9
[01:13:19]    |
[01:13:19]    |
[01:13:19] LL | #![deny(absolute_paths_not_starting_with_crate)]
[01:13:19]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[01:13:19]    = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>
[01:13:19] 
[01:13:19] error: aborting due to previous error
---
[01:13:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:13:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:19] 
[01:13:19] 
[01:13:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:19] 
[01:13:19] 
[01:13:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:19] Build completed unsuccessfully in 0:04:36
[01:13:19] Build completed unsuccessfully in 0:04:36
[01:13:19] make: *** [check] Error 1
[01:13:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:206ab5c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 22 17:14:00 UTC 2019
