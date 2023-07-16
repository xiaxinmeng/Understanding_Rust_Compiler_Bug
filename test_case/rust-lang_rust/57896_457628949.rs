plain
travis_time:end:2808cc92:start=1548429521201394146,finish=1548429619795964557,duration=98594570411
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:14] ..................................................................................................ii 1000/5330
[01:00:18] iii................................................................................................. 1100/5330
[01:00:21] .................................................................................................... 1200/5330
[01:00:23] .................................................................................................... 1300/5330
[01:00:26] ........................................................................F............F.............. 1400/5330
[01:00:31] ........................................................................................i........... 1600/5330
[01:00:34] ..............................................................i..................................... 1700/5330
[01:00:38] .................................................................................................... 1800/5330
[01:00:41] .................................................................................................... 1900/5330
---
[01:02:46] 
[01:02:46] - error[E0091]: type parameter `U` is unused
[01:02:46] -   --> $DIR/generic_duplicate_param_use.rs:5:25
[01:02:46] -    |
[01:02:46] - LL | existential type Two<T, U>: 'static; //~ ERROR type parameter `U` is unused
[01:02:46] -    |                         ^ unused type parameter
[01:02:46] - error: aborting due to previous error
[01:02:46] - 
[01:02:46] - For more information about this error, try `rustc --explain E0091`.
[01:02:46] - 
[01:02:46] - 
[01:02:46] 
[01:02:46] 
[01:02:46] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/generic_duplicate_param_use/generic_duplicate_param_use.stderr`: No such file or directory (os error 2)
[01:02:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:46] 
[01:02:46] ---- [ui] ui/existential_types/unused_generic_param.rs stdout ----
[01:02:46] diff of stderr:
[01:02:46] diff of stderr:
[01:02:46] 
[01:02:46] - error[E0091]: type parameter `T` is unused
[01:02:46] -   --> $DIR/unused_generic_param.rs:6:35
[01:02:46] -    |
[01:02:46] - LL | existential type PartiallyDefined<T>: 'static; //~ `T` is unused
[01:02:46] -    |                                   ^ unused type parameter
[01:02:46] - 
[01:02:46] - error[E0091]: type parameter `T` is unused
[01:02:46] -   --> $DIR/unused_generic_param.rs:12:36
[01:02:46] -    |
[01:02:46] - LL | existential type PartiallyDefined2<T>: 'static; //~ `T` is unused
[01:02:46] -    |                                    ^ unused type parameter
[01:02:46] - error: aborting due to 2 previous errors
[01:02:46] - 
[01:02:46] - For more information about this error, try `rustc --explain E0091`.
[01:02:46] - 
[01:02:46] - 
[01:02:46] 
[01:02:46] 
[01:02:46] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/unused_generic_param/unused_generic_param.stderr`: No such file or directory (os error 2)
[01:02:46] 
[01:02:46] 
[01:02:46] failures:
[01:02:46]     [ui] ui/existential_types/generic_duplicate_param_use.rs
[01:02:46]     [ui] ui/existential_types/generic_duplicate_param_use.rs
[01:02:46]     [ui] ui/existential_types/unused_generic_param.rs
[01:02:46] 
[01:02:46] test result: FAILED. 5305 passed; 2 failed; 23 ignored; 0 measured; 0 filtered out
[01:02:46] 
[01:02:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:02:46] 
[01:02:46] 
[01:02:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:46] 
[01:02:46] 
[01:02:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:46] Build completed unsuccessfully in 0:04:00
[01:02:46] Build completed unsuccessfully in 0:04:00
[01:02:46] Makefile:48: recipe for target 'check' failed
[01:02:46] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3952679e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 25 16:23:15 UTC 2019
