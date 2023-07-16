plain
travis_time:end:02dd8b0c:start=1561151670851845201,finish=1561151673513485618,duration=2661640417
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:49] .................................................................................................... 400/2920
[00:58:58] .................................................................................................... 500/2920
[00:59:10] .................................................................................................... 600/2920
[00:59:24] .................................................................................................... 700/2920
[00:59:35] ...................F..F............................................................................. 800/2920
[00:59:44] .................................................................................................... 900/2920
[00:59:58] ............................F....................................................................... 1000/2920
[01:00:19] .................................................................................................... 1200/2920
[01:00:29] .................................................................................................... 1300/2920
[01:00:41] ....................ii.............................................................................. 1400/2920
[01:00:52] .................................................................................................... 1500/2920
[01:00:52] .................................................................................................... 1500/2920
[01:01:01] .........................................................................i......i................... 1600/2920
[01:01:15] .................................................................................................... 1700/2920
[01:01:28] .................................................................................................... 1800/2920
[01:01:39] ..............F..................................................................................... 1900/2920
[01:02:21] .................................................................................................... 2100/2920
[01:02:41] .......................................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:02:43] ............................. 2200/2920
[01:02:43] ............................. 2200/2920
[01:02:54] ......................................................................FF............................ 2300/2920
[01:03:22] .................................................................................................... 2500/2920
[01:03:54] .................................................................................................... 2600/2920
[01:04:03] .................................................................................................... 2700/2920
[01:04:13] .................................................................................................... 2800/2920
[01:04:13] .................................................................................................... 2800/2920
[01:04:25] .................................................................................................... 2900/2920
[01:04:28] ....................
[01:04:28] failures:
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/extern/extern-prelude-core.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | #![feature(extern_prelude, lang_items, start)]
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(stable_features)] on by default
[01:04:28] -    = note: #[warn(stable_features)] on by default
[01:04:28] +    = note: `#[warn(stable_features)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-core/extern-prelude-core.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args extern/extern-prelude-core.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/extern/extern-prelude-core.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-core/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: the feature `extern_prelude` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:04:28]   --> /checkout/src/test/run-pass/extern/extern-prelude-core.rs:2:12
[01:04:28]    |
[01:04:28] LL | #![feature(extern_prelude, lang_items, start)]
[01:04:28]    |
[01:04:28]    |
[01:04:28]    = note: `#[warn(stable_features)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/extern/extern-prelude-std.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | #![feature(extern_prelude)]
[01:04:28] 5    |            ^^^^^^^^^^^^^^
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(stable_features)] on by default
[01:04:28] +    = note: `#[warn(stable_features)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-std/extern-prelude-std.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args extern/extern-prelude-std.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/extern/extern-prelude-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern/extern-prelude-std/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: the feature `extern_prelude` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:04:28]   --> /checkout/src/test/run-pass/extern/extern-prelude-std.rs:2:12
[01:04:28]    |
[01:04:28] LL | #![feature(extern_prelude)]
[01:04:28]    |            ^^^^^^^^^^^^^^
[01:04:28]    |
[01:04:28]    = note: `#[warn(stable_features)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/if-ret.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | fn foo() { if (return) { } }
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(unreachable_code)] on by default
[01:04:28] -    = note: #[warn(unreachable_code)] on by default
[01:04:28] +    = note: `#[warn(unreachable_code)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/if-ret/if-ret.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args if-ret.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/if-ret.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/if-ret/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/if-ret/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: unreachable block in `if` expression
[01:04:28]   --> /checkout/src/test/run-pass/if-ret.rs:4:24
[01:04:28]    |
[01:04:28] LL | fn foo() { if (return) { } }
[01:04:28]    |
[01:04:28]    |
[01:04:28]    = note: `#[warn(unreachable_code)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/macros/macro-use-all-and-none.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | #[macro_use()]
[01:04:28] 5    | ^^^^^^^^^^^^^^
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(unused_attributes)] on by default
[01:04:28] +    = note: `#[warn(unused_attributes)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-use-all-and-none/macro-use-all-and-none.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args macros/macro-use-all-and-none.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/macro-use-all-and-none.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-use-all-and-none/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-use-all-and-none/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: unused attribute
[01:04:28]   --> /checkout/src/test/run-pass/macros/macro-use-all-and-none.rs:5:1
[01:04:28]    |
[01:04:28] LL | #[macro_use()]
[01:04:28]    | ^^^^^^^^^^^^^^
[01:04:28]    |
[01:04:28]    = note: `#[warn(unused_attributes)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | #![feature(crate_in_paths)]
[01:04:28] 5    |            ^^^^^^^^^^^^^^
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(stable_features)] on by default
[01:04:28] +    = note: `#[warn(stable_features)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity/crate-path-visibility-ambiguity.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: the feature `crate_in_paths` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:04:28]   --> /checkout/src/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs:2:12
[01:04:28]    |
[01:04:28] LL | #![feature(crate_in_paths)]
[01:04:28]    |            ^^^^^^^^^^^^^^
[01:04:28]    |
[01:04:28]    = note: `#[warn(stable_features)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] ---- [run-pass] run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs stdout ----
[01:04:28] diff of stderr:
[01:04:28] 
[01:04:28] 4 LL | #![feature(crate_in_paths)]
[01:04:28] 5    |            ^^^^^^^^^^^^^^
[01:04:28] 6    |
[01:04:28] -    = note: #[warn(stable_features)] on by default
[01:04:28] +    = note: `#[warn(stable_features)]` on by default
[01:04:28] 9 
[01:04:28] 
[01:04:28] 
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] The actual stderr differed from the expected stderr.
[01:04:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute/crate-path-absolute.stderr
[01:04:28] To update references, rerun the tests and pass the `--bless` flag
[01:04:28] To only update this specific test, also pass `--test-args rfcs/rfc-2126-crate-paths/crate-path-absolute.rs`
[01:04:28] error: 1 errors occurred comparing output.
[01:04:28] status: exit code: 0
[01:04:28] status: exit code: 0
[01:04:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute/auxiliary"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] warning: the feature `crate_in_paths` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:04:28]   --> /checkout/src/test/run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs:2:12
[01:04:28]    |
[01:04:28] LL | #![feature(crate_in_paths)]
[01:04:28]    |            ^^^^^^^^^^^^^^
[01:04:28]    |
[01:04:28]    = note: `#[warn(stable_features)]` on by default
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] 
---
[01:04:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:04:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:28] 
[01:04:28] 
[01:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:28] 
[01:04:28] 
[01:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:28] Build completed unsuccessfully in 0:59:43
---
travis_time:end:28e5262d:start=1561155554682216443,finish=1561155554687239904,duration=5023461
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c697bf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b5e3dd0
travis_time:start:1b5e3dd0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f617ff8
$ dmesg | grep -i kill
