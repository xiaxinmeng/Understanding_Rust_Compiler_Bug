plain
travis_time:end:00195538:start=1559763313378914626,finish=1559763316026408448,duration=2647493822
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:32] .................................................................................................... 3000/5626
[00:58:36] .................................................................................................... 3100/5626
[00:58:39] .................................................................................................... 3200/5626
[00:58:44] .................................................................................................... 3300/5626
[00:58:47] ..................F........................i.........................................F.............. 3400/5626
[00:58:55] .................ii...i..ii......................................................................... 3600/5626
[00:58:59] .................................................................................................... 3700/5626
[00:59:02] .................................................................................................... 3800/5626
[00:59:06] ..........................ii........................................................................ 3900/5626
---
[01:00:20] 
[01:00:20] ---- [ui] ui/macros/assert-trailing-junk.rs stdout ----
[01:00:20] diff of stderr:
[01:00:20] 
[01:00:20] 10 LL |     assert!(true some extra junk);
[01:00:20] 11    |                  ^^^^ expected one of `,`, `.`, `?`, or an operator here
[01:00:20] 12 
[01:00:20] - error: no rules expected the token `blah`
[01:00:20] -   --> $DIR/assert-trailing-junk.rs:12:30
[01:00:20] + error: no rules expected the token `"whatever"`
[01:00:20] 15    |
[01:00:20] 15    |
[01:00:20] 16 LL |     assert!(true, "whatever" blah);
[01:00:20] -    |                             -^^^^ no rules expected this token in macro call
[01:00:20] -    |                             help: missing comma here
[01:00:20] +    |                   ^^^^^^^^^^- help: missing comma here
[01:00:20] +    |                   |
[01:00:20] +    |                   no rules expected this token in macro call
[01:00:20] +    |                   no rules expected this token in macro call
[01:00:20] 20 
[01:00:20] 21 warning: unexpected string literal
[01:00:20] 22   --> $DIR/assert-trailing-junk.rs:15:18
[01:00:20] 
[01:00:20] 28    |
[01:00:20] 29    = note: this is going to be an error in the future
[01:00:20] 30 
[01:00:20] - error: no rules expected the token `blah`
[01:00:20] -   --> $DIR/assert-trailing-junk.rs:15:29
[01:00:20] + error: no rules expected the token `"whatever"`
[01:00:20] 33    |
[01:00:20] 33    |
[01:00:20] 34 LL |     assert!(true "whatever" blah);
[01:00:20] -    |                            -^^^^ no rules expected this token in macro call
[01:00:20] -    |                            help: missing comma here
[01:00:20] +    |                  ^^^^^^^^^^- help: missing comma here
[01:00:20] +    |                  |
[01:00:20] +    |                  no rules expected this token in macro call
[01:00:20] +    |                  no rules expected this token in macro call
[01:00:20] 38 
[01:00:20] 39 warning: macro requires an expression as an argument
[01:00:20] 40   --> $DIR/assert-trailing-junk.rs:19:5
[01:00:20] 
[01:00:20] 
[01:00:20] The actual stderr differed from the expected stderr.
[01:00:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-trailing-junk/assert-trailing-junk.stderr
[01:00:20] To update references, rerun the tests and pass the `--bless` flag
[01:00:20] To only update this specific test, also pass `--test-args macros/assert-trailing-junk.rs`
[01:00:20] error: 1 errors occurred comparing output.
[01:00:20] status: exit code: 1
[01:00:20] status: exit code: 1
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/assert-trailing-junk.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-trailing-junk" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-trailing-junk/auxiliary" "-A" "unused"
[01:00:20] ------------------------------------------
[01:00:20] 
[01:00:20] ------------------------------------------
[01:00:20] stderr:
[01:00:20] stderr:
[01:00:20] ------------------------------------------
[01:00:20] error: expected one of `,`, `.`, `?`, or an operator, found `some`
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL |     assert!(true some extra junk, "whatever");
[01:00:20]    |                  ^^^^ expected one of `,`, `.`, `?`, or an operator here
[01:00:20] 
[01:00:20] error: expected one of `,`, `.`, `?`, or an operator, found `some`
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL |     assert!(true some extra junk);
[01:00:20]    |                  ^^^^ expected one of `,`, `.`, `?`, or an operator here
[01:00:20] 
[01:00:20] error: no rules expected the token `"whatever"`
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL |     assert!(true, "whatever" blah);
[01:00:20]    |                   ^^^^^^^^^^- help: missing comma here
[01:00:20]    |                   no rules expected this token in macro call
[01:00:20] 
[01:00:20] warning: unexpected string literal
[01:00:20]   --> /checkout/src/test/ui/macros/assert-trailing-junk.rs:15:18
[01:00:20]   --> /checkout/src/test/ui/macros/assert-trailing-junk.rs:15:18
[01:00:20]    |
[01:00:20] LL |     assert!(true "whatever" blah);
[01:00:20]    |                 -^^^^^^^^^^
[01:00:20]    |                 help: try adding a comma
[01:00:20]    |
[01:00:20]    = note: this is going to be an error in the future
[01:00:20] 
[01:00:20] 
[01:00:20] error: no rules expected the token `"whatever"`
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL |     assert!(true "whatever" blah);
[01:00:20]    |                  ^^^^^^^^^^- help: missing comma here
[01:00:20]    |                  no rules expected this token in macro call
[01:00:20] 
[01:00:20] warning: macro requires an expression as an argument
[01:00:20]   --> /checkout/src/test/ui/macros/assert-trailing-junk.rs:19:5
---
[01:00:20] 
[01:00:20] warning: unexpected string literal
[01:00:20]   --> /checkout/src/test/ui/macros/assert-trailing-junk.rs:22:27
[01:00:20]    |
[01:00:20] LL |     assert!(false || true "error message");
[01:00:20]    |                          -^^^^^^^^^^^^^^^
[01:00:20]    |                          help: try adding a comma
[01:00:20]    |
[01:00:20]    = note: this is going to be an error in the future
[01:00:20] 
---
[01:00:20] 16    |           help: missing comma here
[01:00:20] 17 
[01:00:20] - error: no rules expected the token `e`
[01:00:20] -   --> $DIR/missing-comma.rs:23:21
[01:00:20] + error: no rules expected the token `,`
[01:00:20] 20    |
[01:00:20] 20    |
[01:00:20] 21 LL | macro_rules! foo {
[01:00:20] 22    | ---------------- when calling this macro
[01:00:20] 23 ...
[01:00:20] 23 ...
[01:00:20] 24 LL |     foo!(a, b, c, d e);
[01:00:20] -    |                    -^ no rules expected this token in macro call
[01:00:20] -    |                    help: missing comma here
[01:00:20] +    |           ^        - help: missing comma here
[01:00:20] +    |           |
[01:00:20] +    |           no rules expected this token in macro call
[01:00:20] +    |           no rules expected this token in macro call
[01:00:20] 28 
[01:00:20] - error: no rules expected the token `d`
[01:00:20] -   --> $DIR/missing-comma.rs:25:18
[01:00:20] + error: no rules expected the token `,`
[01:00:20] 31    |
[01:00:20] 31    |
[01:00:20] 32 LL | macro_rules! foo {
[01:00:20] 33    | ---------------- when calling this macro
[01:00:20] 34 ...
[01:00:20] 34 ...
[01:00:20] 35 LL |     foo!(a, b, c d, e);
[01:00:20] -    |                 -^ no rules expected this token in macro call
[01:00:20] -    |                 help: missing comma here
[01:00:20] +    |           ^     - help: missing comma here
[01:00:20] +    |           |
[01:00:20] +    |           no rules expected this token in macro call
[01:00:20] +    |           no rules expected this token in macro call
[01:00:20] 39 
[01:00:20] - error: no rules expected the token `d`
[01:00:20] -   --> $DIR/missing-comma.rs:27:18
[01:00:20] + error: no rules expected the token `,`
[01:00:20] 42    |
[01:00:20] 42    |
[01:00:20] 43 LL | macro_rules! foo {
[01:00:20] 44    | ---------------- when calling this macro
[01:00:20] 45 ...
[01:00:20] 45 ...
[01:00:20] 46 LL |     foo!(a, b, c d e);
[01:00:20] -    |                  ^ no rules expected this token in macro call
[01:00:20] +    |           ^ no rules expected this token in macro call
[01:00:20] 49 error: unexpected end of macro invocation
[01:00:20] 50   --> $DIR/missing-comma.rs:29:23
[01:00:20] 
[01:00:20] 
[01:00:20] 
[01:00:20] The actual stderr differed from the expected stderr.
[01:00:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-comma/missing-comma.stderr
[01:00:20] To update references, rerun the tests and pass the `--bless` flag
[01:00:20] To only update this specific test, also pass `--test-args macros/missing-comma.rs`
[01:00:20] error: 1 errors occurred comparing output.
[01:00:20] status: exit code: 1
[01:00:20] status: exit code: 1
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/missing-comma.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-comma" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/missing-comma/auxiliary" "-A" "unused"
[01:00:20] ------------------------------------------
[01:00:20] 
[01:00:20] ------------------------------------------
[01:00:20] stderr:
---
[01:00:20] 
[01:00:20] error: no rules expected the token `b`
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:21:12
[01:00:20]    |
[01:00:20] LL | macro_rules! foo {
[01:00:20]    | ---------------- when calling this macro
[01:00:20] ...
[01:00:20] LL |     foo!(a b);
[01:00:20]    |           -^ no rules expected this token in macro call
[01:00:20]    |           help: missing comma here
[01:00:20] 
[01:00:20] error: no rules expected the token `,`
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:23:11
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:23:11
[01:00:20]    |
[01:00:20] LL | macro_rules! foo {
[01:00:20]    | ---------------- when calling this macro
[01:00:20] ...
[01:00:20] LL |     foo!(a, b, c, d e);
[01:00:20]    |           ^        - help: missing comma here
[01:00:20]    |           no rules expected this token in macro call
[01:00:20] 
[01:00:20] error: no rules expected the token `,`
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:25:11
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:25:11
[01:00:20]    |
[01:00:20] LL | macro_rules! foo {
[01:00:20]    | ---------------- when calling this macro
[01:00:20] ...
[01:00:20] LL |     foo!(a, b, c d, e);
[01:00:20]    |           ^     - help: missing comma here
[01:00:20]    |           no rules expected this token in macro call
[01:00:20] 
[01:00:20] error: no rules expected the token `,`
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:27:11
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:27:11
[01:00:20]    |
[01:00:20] LL | macro_rules! foo {
[01:00:20]    | ---------------- when calling this macro
[01:00:20] ...
[01:00:20] LL |     foo!(a, b, c d e);
[01:00:20]    |           ^ no rules expected this token in macro call
[01:00:20] error: unexpected end of macro invocation
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:29:23
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL | macro_rules! bar {
[01:00:20]    | ---------------- when calling this macro
[01:00:20] ...
[01:00:20] LL |     bar!(Level::Error, );
[01:00:20]    |                       ^ missing tokens in macro arguments
[01:00:20] error: no rules expected the token `,`
[01:00:20]   --> /checkout/src/test/ui/macros/missing-comma.rs:32:38
[01:00:20]    |
[01:00:20]    |
[01:00:20] LL | macro_rules! check {
[01:00:20]    | ------------------ when calling this macro
[01:00:20] ...
[01:00:20] LL |     check!(<str as Debug>::fmt, "fmt",);
[01:00:20]    |                                      ^ no rules expected this token in macro call
[01:00:20] error: aborting due to 7 previous errors
[01:00:20] 
[01:00:20] 
[01:00:20] ------------------------------------------
---
[01:00:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:00:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:00:20] 
[01:00:20] 
[01:00:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:20] 
[01:00:20] 
[01:00:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:20] Build completed unsuccessfully in 0:55:30
---
travis_time:end:1b3e5464:start=1559766947710963652,finish=1559766947718939024,duration=7975372
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1076d9b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02702004
$ dmesg | grep -i kill
