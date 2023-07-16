plain
travis_time:end:0551a5ea:start=1558563413946988738,finish=1558563501872335169,duration=87925346431
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:31] .................................................................................................... 300/5572
[01:11:34] .................................................................................................... 400/5572
[01:11:38] ................................................................................................i... 500/5572
[01:11:41] .................................................................................................... 600/5572
[01:11:45] ...................................................................F................................ 700/5572
[01:11:55] .............................................................................i...............i...... 900/5572
[01:11:59] .................................................................................................... 1000/5572
[01:12:03] ..........iiiii..................................................................................... 1100/5572
[01:12:06] .................................................................................................... 1200/5572
---
[01:13:30] ..................................................................................ii...i...ii....... 3500/5572
[01:13:34] .................................................................................................... 3600/5572
[01:13:37] .................................................................................................... 3700/5572
[01:13:41] .......................................................................................ii........... 3800/5572
[01:13:44] ................................F................................................................... 3900/5572
[01:13:48] .......................................................................i............................ 4100/5572
[01:13:51] .................................................................................................... 4200/5572
[01:13:51] .................................................................................................... 4200/5572
[01:13:58] ........F........................................................................................... 4300/5572
[01:14:09] .................................................................................................... 4500/5572
[01:14:12] .................................................................................................... 4600/5572
[01:14:17] .................................................................................................... 4700/5572
[01:14:25] .................................................................................................... 4800/5572
---
[01:14:54] 
[01:14:54] + error: expected type, found `1`
[01:14:54] +   --> $DIR/const-expression-parameter.rs:9:21
[01:14:54] +    |
[01:14:54] + LL |     i32_identity::<-1>(); // ok
[01:14:54] + 
[01:14:54] + 
[01:14:54] 1 error: expected one of `,` or `>`, found `+`
[01:14:54] 3    |
[01:14:54] 
[01:14:54] 10 LL | #![feature(const_generics)]
[01:14:54] 11    |            ^^^^^^^^^^^^^^
---
[01:14:54] 
[01:14:54] 
[01:14:54] The actual stderr differed from the expected stderr.
[01:14:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-expression-parameter/const-expression-parameter.stderr
[01:14:54] To update references, rerun the tests and pass the `--bless` flag
[01:14:54] To only update this specific test, also pass `--test-args const-generics/const-expression-parameter.rs`
[01:14:54] error: 1 errors occurred comparing output.
[01:14:54] status: exit code: 1
[01:14:54] status: exit code: 1
[01:14:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-expression-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-expression-parameter" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-expression-parameter/auxiliary" "-A" "unused"
[01:14:54] ------------------------------------------
[01:14:54] 
[01:14:54] ------------------------------------------
[01:14:54] stderr:
[01:14:54] stderr:
[01:14:54] ------------------------------------------
[01:14:54] error: expected type, found `1`
[01:14:54]   --> /checkout/src/test/ui/const-generics/const-expression-parameter.rs:9:21
[01:14:54]    |
[01:14:54] LL |     i32_identity::<-1>(); // ok
[01:14:54] 
[01:14:54] 
[01:14:54] error: expected one of `,` or `>`, found `+`
[01:14:54]    |
[01:14:54]    |
[01:14:54] LL |     i32_identity::<1 + 2>(); //~ ERROR expected one of `,` or `>`, found `+`
[01:14:54]    |                      ^ expected one of `,` or `>` here
[01:14:54] warning: the feature `const_generics` is incomplete and may cause the compiler to crash
[01:14:54]   --> /checkout/src/test/ui/const-generics/const-expression-parameter.rs:1:12
[01:14:54]    |
[01:14:54] LL | #![feature(const_generics)]
---
[01:14:54] diff of stderr:
[01:14:54] 
[01:14:54] - error: expected expression, found keyword `in`
[01:14:54] -   --> $DIR/bad.rs:6:5
[01:14:54] + error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `<-`
[01:14:54] 3    |
[01:14:54] 3    |
[01:14:54] - LL |     in(foo) { bar };
[01:14:54] -    |     ^^ expected expression
[01:14:54] + LL |     x <- y; // ok: parses as a comparison
[01:14:54] +    |       ^^ expected one of 8 possible tokens here
[01:14:54] - error: aborting due to previous error
[01:14:54] + error[E0308]: mismatched types
[01:14:54] +   --> $DIR/bad.rs:5:5
[01:14:54] +    |
[01:14:54] +    |
[01:14:54] + LL | fn main() {
[01:14:54] +    |           - expected `()` because of default return type
[01:14:54] + LL |     let (x, y, foo, bar) = (0, 0, 0, 0);
[01:14:54] + LL |     x <- y; // ok: parses as a comparison
[01:14:54] +    |     ^ expected (), found integer
[01:14:54] +    = note: expected type `()`
[01:14:54] +               found type `{integer}`
[01:14:54] 8 
[01:14:54] + error: aborting due to 2 previous errors
[01:14:54] + error: aborting due to 2 previous errors
[01:14:54] + 
[01:14:54] + For more information about this error, try `rustc --explain E0308`.
[01:14:54] 9 
[01:14:54] 
[01:14:54] 
[01:14:54] The actual stderr differed from the expected stderr.
[01:14:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/obsolete-in-place/bad/bad.stderr
[01:14:54] To update references, rerun the tests and pass the `--bless` flag
[01:14:54] To only update this specific test, also pass `--test-args obsolete-in-place/bad.rs`
[01:14:54] error: 1 errors occurred comparing output.
[01:14:54] status: exit code: 1
[01:14:54] status: exit code: 1
[01:14:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/obsolete-in-place/bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/obsolete-in-place/bad" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/obsolete-in-place/bad/auxiliary" "-A" "unused"
[01:14:54] ------------------------------------------
[01:14:54] 
[01:14:54] ------------------------------------------
[01:14:54] stderr:
[01:14:54] stderr:
[01:14:54] ------------------------------------------
[01:14:54] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `<-`
[01:14:54]   --> /checkout/src/test/ui/obsolete-in-place/bad.rs:5:7
[01:14:54]    |
[01:14:54] LL |     x <- y; // ok: parses as a comparison
[01:14:54]    |       ^^ expected one of 8 possible tokens here
[01:14:54] error[E0308]: mismatched types
[01:14:54]   --> /checkout/src/test/ui/obsolete-in-place/bad.rs:5:5
[01:14:54]    |
[01:14:54] LL | fn main() {
[01:14:54] LL | fn main() {
[01:14:54]    |           - expected `()` because of default return type
[01:14:54] LL |     let (x, y, foo, bar) = (0, 0, 0, 0);
[01:14:54] LL |     x <- y; // ok: parses as a comparison
[01:14:54]    |     ^ expected (), found integer
[01:14:54]    = note: expected type `()`
[01:14:54]               found type `{integer}`
[01:14:54] 
[01:14:54] error: aborting due to 2 previous errors
---
[01:14:54] 
[01:14:54] 
[01:14:54] ---- [ui] ui/placement-syntax.rs stdout ----
[01:14:54] 
[01:14:54] error: test compilation failed although it shouldn't!
[01:14:54] status: exit code: 1
[01:14:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/placement-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/placement-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/placement-syntax/auxiliary" "-A" "unused"
[01:14:54] ------------------------------------------
[01:14:54] 
[01:14:54] ------------------------------------------
[01:14:54] stderr:
[01:14:54] stderr:
[01:14:54] ------------------------------------------
[01:14:54] error: expected `{`, found `<-`
[01:14:54]    |
[01:14:54]    |
[01:14:54] LL |     if x<-1 { // ok: parses as a comparison
[01:14:54]    |     --  ^^ expected `{`
[01:14:54]    |     |
[01:14:54]    |     this `if` statement has a condition, but no block
[01:14:54] error: aborting due to previous error
[01:14:54] 
[01:14:54] 
[01:14:54] ------------------------------------------
---
[01:14:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:14:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:54] 
[01:14:54] 
[01:14:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:54] 
[01:14:54] 
[01:14:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:54] Build completed unsuccessfully in 0:04:51
[01:14:54] Build completed unsuccessfully in 0:04:51
[01:14:54] Makefile:48: recipe for target 'check' failed
[01:14:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20a5514c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 22 23:33:25 UTC 2019
