plain
travis_time:end:11ec65fe:start=1557598173129579784,finish=1557598258577645449,duration=85448065665
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:29] 
[01:09:29] running 5527 tests
[01:09:32] ....................................F............................................................... 100/5527
[01:09:41] .................................................................................................... 300/5527
[01:09:44] .................................................................................................... 400/5527
[01:09:44] .................................................................................................... 400/5527
[01:09:47] ...........................................................................F......................i. 500/5527
[01:09:55] .................................................................................................... 700/5527
[01:10:00] .................................................................................................... 800/5527
[01:10:05] ...............................................................................i...............i.... 900/5527
[01:10:09] .................................................................................................... 1000/5527
---
[01:12:36] .................................................................................................... 4900/5527
[01:12:39] .................................................................................................... 5000/5527
[01:12:44] .................................................................................................... 5100/5527
[01:12:48] .................................................................................................... 5200/5527
[01:12:51] .............F...................................................................................... 5300/5527
[01:12:57] .................................................................i.................................. 5500/5527
[01:12:58] ...........................
[01:12:58] failures:
[01:12:58] 
[01:12:58] 
[01:12:58] ---- [ui] ui/array-break-length.rs stdout ----
[01:12:58] diff of stderr:
[01:12:58] 
[01:12:58] 10 LL |         |_: [_; continue]| {}
[01:12:58] 11    |                 ^^^^^^^^ cannot break outside of a loop
[01:12:58] - error: aborting due to 2 previous errors
[01:12:58] + error[E0308]: mismatched types
[01:12:58] +   --> $DIR/array-break-length.rs:3:9
[01:12:58] +    |
[01:12:58] +    |
[01:12:58] + LL |         |_: [_; break]| {}
[01:12:58] +    |         ^^^^^^^^^^^^^^^^^^ expected (), found closure
[01:12:58] +    = note: expected type `()`
[01:12:58] +    = note: expected type `()`
[01:12:58] +               found type `[closure@$DIR/array-break-length.rs:3:9: 3:27]`
[01:12:58] - For more information about this error, try `rustc --explain E0268`.
[01:12:58] + error[E0308]: mismatched types
[01:12:58] +   --> $DIR/array-break-length.rs:7:9
[01:12:58] +    |
[01:12:58] +    |
[01:12:58] + LL |         |_: [_; continue]| {}
[01:12:58] +    |         ^^^^^^^^^^^^^^^^^^^^^ expected (), found closure
[01:12:58] +    = note: expected type `()`
[01:12:58] +    = note: expected type `()`
[01:12:58] +               found type `[closure@$DIR/array-break-length.rs:7:9: 7:30]`
[01:12:58] + error: aborting due to 4 previous errors
[01:12:58] + 
[01:12:58] + Some errors have detailed explanations: E0268, E0308.
[01:12:58] + For more information about an error, try `rustc --explain E0268`.
[01:12:58] + For more information about an error, try `rustc --explain E0268`.
[01:12:58] 16 
[01:12:58] 
[01:12:58] 
[01:12:58] The actual stderr differed from the expected stderr.
[01:12:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-break-length/array-break-length.stderr
[01:12:58] To update references, rerun the tests and pass the `--bless` flag
[01:12:58] To only update this specific test, also pass `--test-args array-break-length.rs`
[01:12:58] error: 1 errors occurred comparing output.
[01:12:58] status: exit code: 1
[01:12:58] status: exit code: 1
[01:12:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-break-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-break-length" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-break-length/auxiliary" "-A" "unused"
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] stderr:
[01:12:58] stderr:
[01:12:58] ------------------------------------------
[01:12:58] error[E0268]: `break` outside of loop
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |         |_: [_; break]| {} //~ ERROR: `break` outside of loop
[01:12:58]    |                 ^^^^^ cannot break outside of a loop
[01:12:58] 
[01:12:58] error[E0268]: `continue` outside of loop
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |         |_: [_; continue]| {} //~ ERROR: `continue` outside of loop
[01:12:58]    |                 ^^^^^^^^ cannot break outside of a loop
[01:12:58] error[E0308]: mismatched types
[01:12:58]   --> /checkout/src/test/ui/array-break-length.rs:3:9
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |         |_: [_; break]| {} //~ ERROR: `break` outside of loop
[01:12:58]    |         ^^^^^^^^^^^^^^^^^^ expected (), found closure
[01:12:58]    = note: expected type `()`
[01:12:58]    = note: expected type `()`
[01:12:58]               found type `[closure@/checkout/src/test/ui/array-break-length.rs:3:9: 3:27]`
[01:12:58] error[E0308]: mismatched types
[01:12:58]   --> /checkout/src/test/ui/array-break-length.rs:7:9
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |         |_: [_; continue]| {} //~ ERROR: `continue` outside of loop
[01:12:58]    |         ^^^^^^^^^^^^^^^^^^^^^ expected (), found closure
[01:12:58]    = note: expected type `()`
[01:12:58]    = note: expected type `()`
[01:12:58]               found type `[closure@/checkout/src/test/ui/array-break-length.rs:7:9: 7:30]`
[01:12:58] error: aborting due to 4 previous errors
[01:12:58] 
[01:12:58] Some errors have detailed explanations: E0268, E0308.
[01:12:58] For more information about an error, try `rustc --explain E0268`.
[01:12:58] For more information about an error, try `rustc --explain E0268`.
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] 
[01:12:58] ---- [ui] ui/closures/closure-array-break-length.rs stdout ----
[01:12:58] diff of stderr:
[01:12:58] 
[01:12:58] 16 LL |     while |_: [_; break]| {} {}
[01:12:58] 17    |                   ^^^^^ cannot break outside of a loop
[01:12:58] - error: aborting due to 3 previous errors
[01:12:58] + error[E0308]: mismatched types
[01:12:58] +   --> $DIR/closure-array-break-length.rs:4:11
[01:12:58] +    |
[01:12:58] +    |
[01:12:58] + LL |     while |_: [_; continue]| {} {}
[01:12:58] +    |
[01:12:58] +    = note: expected type `bool`
[01:12:58] +    = note: expected type `bool`
[01:12:58] +               found type `[closure@$DIR/closure-array-break-length.rs:4:11: 4:32]`
[01:12:58] - For more information about this error, try `rustc --explain E0268`.
[01:12:58] + error[E0308]: mismatched types
[01:12:58] +   --> $DIR/closure-array-break-length.rs:6:11
[01:12:58] +    |
[01:12:58] +    |
[01:12:58] + LL |     while |_: [_; break]| {} {}
[01:12:58] +    |
[01:12:58] +    = note: expected type `bool`
[01:12:58] +    = note: expected type `bool`
[01:12:58] +               found type `[closure@$DIR/closure-array-break-length.rs:6:11: 6:29]`
[01:12:58] + error: aborting due to 5 previous errors
[01:12:58] + 
[01:12:58] + Some errors have detailed explanations: E0268, E0308.
[01:12:58] + For more information about an error, try `rustc --explain E0268`.
[01:12:58] + For more information about an error, try `rustc --explain E0268`.
[01:12:58] 22 
[01:12:58] 
[01:12:58] 
[01:12:58] The actual stderr differed from the expected stderr.
[01:12:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length/closure-array-break-length.stderr
[01:12:58] To update references, rerun the tests and pass the `--bless` flag
[01:12:58] To only update this specific test, also pass `--test-args closures/closure-array-break-length.rs`
[01:12:58] error: 1 errors occurred comparing output.
[01:12:58] status: exit code: 1
[01:12:58] status: exit code: 1
[01:12:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-array-break-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length/auxiliary" "-A" "unused"
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] stderr:
[01:12:58] stderr:
[01:12:58] ------------------------------------------
[01:12:58] error[E0268]: `continue` outside of loop
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     |_: [_; continue]| {}; //~ ERROR: `continue` outside of loop
[01:12:58]    |             ^^^^^^^^ cannot break outside of a loop
[01:12:58] 
[01:12:58] error[E0268]: `continue` outside of loop
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of loop
[01:12:58]    |                   ^^^^^^^^ cannot break outside of a loop
[01:12:58] 
[01:12:58] error[E0268]: `break` outside of loop
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     while |_: [_; break]| {} {} //~ ERROR: `break` outside of loop
[01:12:58]    |                   ^^^^^ cannot break outside of a loop
[01:12:58] error[E0308]: mismatched types
[01:12:58]   --> /checkout/src/test/ui/closures/closure-array-break-length.rs:4:11
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of loop
[01:12:58]    |
[01:12:58]    = note: expected type `bool`
[01:12:58]    = note: expected type `bool`
[01:12:58]               found type `[closure@/checkout/src/test/ui/closures/closure-array-break-length.rs:4:11: 4:32]`
[01:12:58] error[E0308]: mismatched types
[01:12:58]   --> /checkout/src/test/ui/closures/closure-array-break-length.rs:6:11
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     while |_: [_; break]| {} {} //~ ERROR: `break` outside of loop
[01:12:58]    |
[01:12:58]    = note: expected type `bool`
[01:12:58]    = note: expected type `bool`
[01:12:58]               found type `[closure@/checkout/src/test/ui/closures/closure-array-break-length.rs:6:11: 6:29]`
[01:12:58] error: aborting due to 5 previous errors
[01:12:58] 
[01:12:58] Some errors have detailed explanations: E0268, E0308.
[01:12:58] For more information about an error, try `rustc --explain E0268`.
[01:12:58] For more information about an error, try `rustc --explain E0268`.
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] 
[01:12:58] ---- [ui] ui/type/type-dependent-def-issue-49241.rs stdout ----
[01:12:58] diff of stderr:
[01:12:58] 
[01:12:58] 10 LL |     let s: [u32; l] = v.into_iter().collect();
[01:12:58] 11    |                  ^ referenced constant has errors
[01:12:58] - error: aborting due to 2 previous errors
[01:12:58] - error: aborting due to 2 previous errors
[01:12:58] + error[E0277]: a collection of type `[u32; _]` cannot be built from an iterator over elements of type `{integer}`
[01:12:58] +    |
[01:12:58] +    |
[01:12:58] + LL |     let s: [u32; l] = v.into_iter().collect();
[01:12:58] +    |                                     ^^^^^^^ a collection of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
[01:12:58] +    |
[01:12:58] +    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
[01:12:58] - Some errors have detailed explanations: E0080, E0435.
[01:12:58] + error: aborting due to 3 previous errors
[01:12:58] + 
[01:12:58] + Some errors have detailed explanations: E0080, E0277, E0435.
[01:12:58] + Some errors have detailed explanations: E0080, E0277, E0435.
[01:12:58] 16 For more information about an error, try `rustc --explain E0080`.
[01:12:58] 17 
[01:12:58] 
[01:12:58] 
[01:12:58] The actual stderr differed from the expected stderr.
[01:12:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
[01:12:58] To update references, rerun the tests and pass the `--bless` flag
[01:12:58] To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`
[01:12:58] error: 1 errors occurred comparing output.
[01:12:58] status: exit code: 1
[01:12:58] status: exit code: 1
[01:12:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary" "-A" "unused"
[01:12:58] ------------------------------------------
[01:12:58] 
[01:12:58] ------------------------------------------
[01:12:58] stderr:
[01:12:58] stderr:
[01:12:58] ------------------------------------------
[01:12:58] error[E0435]: attempt to use a non-constant value in a constant
[01:12:58]   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
[01:12:58]    |
[01:12:58] LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
[01:12:58]    |                      ^ non-constant value
[01:12:58] error[E0080]: evaluation of constant value failed
[01:12:58]   --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:18
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     let s: [u32; l] = v.into_iter().collect();
[01:12:58]    |                  ^ referenced constant has errors
[01:12:58] 
[01:12:58] error[E0277]: a collection of type `[u32; _]` cannot be built from an iterator over elements of type `{integer}`
[01:12:58]    |
[01:12:58]    |
[01:12:58] LL |     let s: [u32; l] = v.into_iter().collect();
[01:12:58]    |                                     ^^^^^^^ a collection of type `[u32; _]` cannot be built from `std::iter::Iterator<Item={integer}>`
[01:12:58]    |
[01:12:58]    = help: the trait `std::iter::FromIterator<{integer}>` is not implemented for `[u32; _]`
[01:12:58] error: aborting due to 3 previous errors
[01:12:58] 
[01:12:58] Some errors have detailed explanations: E0080, E0277, E0435.
[01:12:58] For more information about an error, try `rustc --explain E0080`.
---
[01:12:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:12:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:58] 
[01:12:58] 
[01:12:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:58] 
[01:12:58] 
[01:12:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:58] Build completed unsuccessfully in 0:04:44
[01:12:58] Build completed unsuccessfully in 0:04:44
[01:12:58] Makefile:48: recipe for target 'check' failed
[01:12:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:068d1c50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 11 19:24:05 UTC 2019
---
travis_time:end:0d2b9dc9:start=1557602646801958089,finish=1557602646809653344,duration=7695255
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15e1e6b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03b423d9
$ dmesg | grep -i kill
