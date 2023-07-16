plain
travis_time:end:220fce08:start=1560186087293135422,finish=1560186088102498570,duration=809363148
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:55:20] .................................................................................................... 400/5666
[00:55:24] .................................................................................................... 500/5666
[00:55:27] ...................................i................................................................ 600/5666
[00:55:31] .................................................................................................... 700/5666
[00:55:36] ..............................................................................F..................... 800/5666
[00:55:40] .....................................................F.............................................. 900/5666
[00:55:48] ................................................................iiiii............................... 1100/5666
[00:55:52] .................................................................................................... 1200/5666
[00:55:54] .................................................................................................... 1300/5666
[00:55:56] .................................................................................................... 1400/5666
---
[00:56:15] .................................................................................................... 2000/5666
[00:56:19] .................................................................................................... 2100/5666
[00:56:22] .....................................i.............................................................. 2200/5666
[00:56:26] .................................................................................................... 2300/5666
[00:56:30] ...........................................................................F.F...................... 2400/5666
[00:56:38] .................................................................................................... 2600/5666
[00:56:38] .................................................................................................... 2600/5666
[00:56:42] .............................................................F...................................... 2700/5666
[00:56:45] ...........................................................F........................................ 2800/5666
[00:56:54] .................................................................................................... 3000/5666
[00:56:58] .................................................................................................... 3100/5666
[00:57:00] .................................................................................................... 3200/5666
[00:57:05] .................................................................................................... 3300/5666
---
[00:58:39] failures:
[00:58:39] 
[00:58:39] ---- [ui] ui/consts/const-eval/enum_discr.rs stdout ----
[00:58:39] 
[00:58:39] error: test compilation failed although it shouldn't!
[00:58:39] status: exit code: 1
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/enum_discr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error[E0391]: cycle detected when const-evaluating + checking `Foo::Y::{{constant}}#0`
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     Y = Foo::X as isize - 3,
[00:58:39]    |
[00:58:39]    |
[00:58:39] note: ...which requires const-evaluating `Foo::Y::{{constant}}#0`...
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     Y = Foo::X as isize - 3,
[00:58:39]    |         ^^^^^^
[00:58:39]    = note: ...which requires computing layout of `Foo`...
[00:58:39]    = note: ...which again requires const-evaluating + checking `Foo::Y::{{constant}}#0`, completing the cycle
[00:58:39] note: cycle used when collecting item types in top-level module
[00:58:39]    |
[00:58:39] LL | enum Foo {
[00:58:39]    | ^^^^^^^^
[00:58:39] 
---
[00:58:39] ---- [ui] ui/consts/const-prop-ice2.rs stdout ----
[00:58:39] 
[00:58:39] error: ui test compiled successfully!
[00:58:39] status: exit code: 0
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-prop-ice2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-prop-ice2/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
---
[00:58:39] 
[00:58:39] ---- [ui] ui/issues/issue-23302-1.rs stdout ----
[00:58:39] diff of stderr:
[00:58:39] 
[00:58:39] - error[E0391]: cycle detected when processing `X::A::{{constant}}#0`
[00:58:39] + error[E0391]: cycle detected when const-evaluating + checking `X::A::{{constant}}#0`
[00:58:39] 3    |
[00:58:39] 3    |
[00:58:39] 4 LL |     A = X::A as isize,
[00:58:39] 5    |         ^^^^^^^^^^^^^
[00:58:39] 6    |
[00:58:39] 6    |
[00:58:39] -    = note: ...which again requires processing `X::A::{{constant}}#0`, completing the cycle
[00:58:39] - note: cycle used when processing `X::A::{{constant}}#0`
[00:58:39] + note: ...which requires const-evaluating `X::A::{{constant}}#0`...
[00:58:39] 10    |
[00:58:39] 10    |
[00:58:39] 11 LL |     A = X::A as isize,
[00:58:39] 12    |         ^^^^^^^^^^^^^
[00:58:39] 12    |         ^^^^^^^^^^^^^
[00:58:39] +    = note: ...which again requires const-evaluating + checking `X::A::{{constant}}#0`, completing the cycle
[00:58:39] + note: cycle used when collecting item types in top-level module
[00:58:39] +    |
[00:58:39] +    |
[00:58:39] + LL | enum X {
[00:58:39] 13 
[00:58:39] 14 error: aborting due to previous error
[00:58:39] 15 
[00:58:39] 
[00:58:39] 
[00:58:39] 
[00:58:39] The actual stderr differed from the expected stderr.
[00:58:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/issue-23302-1.stderr
[00:58:39] To update references, rerun the tests and pass the `--bless` flag
[00:58:39] To only update this specific test, also pass `--test-args issues/issue-23302-1.rs`
[00:58:39] error: 1 errors occurred comparing output.
[00:58:39] status: exit code: 1
[00:58:39] status: exit code: 1
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error[E0391]: cycle detected when const-evaluating + checking `X::A::{{constant}}#0`
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     A = X::A as isize, //~ ERROR E0391
[00:58:39]    |
[00:58:39]    |
[00:58:39] note: ...which requires const-evaluating `X::A::{{constant}}#0`...
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     A = X::A as isize, //~ ERROR E0391
[00:58:39]    |         ^^^^^^^^^^^^^
[00:58:39]    = note: ...which again requires const-evaluating + checking `X::A::{{constant}}#0`, completing the cycle
[00:58:39] note: cycle used when collecting item types in top-level module
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL | enum X {
[00:58:39] 
[00:58:39] error: aborting due to previous error
[00:58:39] 
[00:58:39] For more information about this error, try `rustc --explain E0391`.
[00:58:39] For more information about this error, try `rustc --explain E0391`.
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] 
[00:58:39] ---- [ui] ui/issues/issue-23302-2.rs stdout ----
[00:58:39] diff of stderr:
[00:58:39] 
[00:58:39] - error[E0391]: cycle detected when processing `Y::A::{{constant}}#0`
[00:58:39] + error[E0391]: cycle detected when const-evaluating + checking `Y::A::{{constant}}#0`
[00:58:39] 3    |
[00:58:39] 3    |
[00:58:39] 4 LL |     A = Y::B as isize,
[00:58:39] 5    |         ^^^^^^^^^^^^^
[00:58:39] 6    |
[00:58:39] 6    |
[00:58:39] -    = note: ...which again requires processing `Y::A::{{constant}}#0`, completing the cycle
[00:58:39] - note: cycle used when processing `Y::A::{{constant}}#0`
[00:58:39] + note: ...which requires const-evaluating `Y::A::{{constant}}#0`...
[00:58:39] 10    |
[00:58:39] 10    |
[00:58:39] 11 LL |     A = Y::B as isize,
[00:58:39] -    |         ^^^^^^^^^^^^^
[00:58:39] +    |         ^^^^
[00:58:39] +    |         ^^^^
[00:58:39] +    = note: ...which requires computing layout of `Y`...
[00:58:39] +    = note: ...which again requires const-evaluating + checking `Y::A::{{constant}}#0`, completing the cycle
[00:58:39] + note: cycle used when collecting item types in top-level module
[00:58:39] +    |
[00:58:39] +    |
[00:58:39] + LL | enum Y {
[00:58:39] 13 
[00:58:39] 14 error: aborting due to previous error
[00:58:39] 15 
[00:58:39] 
[00:58:39] 
[00:58:39] 
[00:58:39] The actual stderr differed from the expected stderr.
[00:58:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2/issue-23302-2.stderr
[00:58:39] To update references, rerun the tests and pass the `--bless` flag
[00:58:39] To only update this specific test, also pass `--test-args issues/issue-23302-2.rs`
[00:58:39] error: 1 errors occurred comparing output.
[00:58:39] status: exit code: 1
[00:58:39] status: exit code: 1
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error[E0391]: cycle detected when const-evaluating + checking `Y::A::{{constant}}#0`
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     A = Y::B as isize, //~ ERROR E0391
[00:58:39]    |
[00:58:39]    |
[00:58:39] note: ...which requires const-evaluating `Y::A::{{constant}}#0`...
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     A = Y::B as isize, //~ ERROR E0391
[00:58:39]    |         ^^^^
[00:58:39]    = note: ...which requires computing layout of `Y`...
[00:58:39]    = note: ...which again requires const-evaluating + checking `Y::A::{{constant}}#0`, completing the cycle
[00:58:39] note: cycle used when collecting item types in top-level module
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL | enum Y {
[00:58:39] 
[00:58:39] error: aborting due to previous error
[00:58:39] 
[00:58:39] For more information about this error, try `rustc --explain E0391`.
[00:58:39] For more information about this error, try `rustc --explain E0391`.
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] 
[00:58:39] ---- [ui] ui/issues/issue-36163.rs stdout ----
[00:58:39] diff of stderr:
[00:58:39] 
[00:58:39] - error[E0391]: cycle detected when processing `Foo::B::{{constant}}#0`
[00:58:39] + error[E0391]: cycle detected when const-evaluating + checking `Foo::B::{{constant}}#0`
[00:58:39] 3    |
[00:58:39] 4 LL |     B = A,
[00:58:39] 
[00:58:39] 5    |         ^
[00:58:39] 5    |         ^
[00:58:39] 6    |
[00:58:39] - note: ...which requires processing `A`...
[00:58:39] -    |
[00:58:39] -    |
[00:58:39] - LL | const A: isize = Foo::B as isize;
[00:58:39] -    |                  ^^^^^^^^^^^^^^^
[00:58:39] -    = note: ...which again requires processing `Foo::B::{{constant}}#0`, completing the cycle
[00:58:39] - note: cycle used when processing `Foo::B::{{constant}}#0`
[00:58:39] + note: ...which requires const-evaluating `Foo::B::{{constant}}#0`...
[00:58:39] 15    |
[00:58:39] 16 LL |     B = A,
[00:58:39] 
[00:58:39] 17    |         ^
[00:58:39] 17    |         ^
[00:58:39] + note: ...which requires const-evaluating `A`...
[00:58:39] +    |
[00:58:39] +    |
[00:58:39] + LL | const A: isize = Foo::B as isize;
[00:58:39] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:58:39] +    = note: ...which again requires const-evaluating + checking `Foo::B::{{constant}}#0`, completing the cycle
[00:58:39] + note: cycle used when collecting item types in top-level module
[00:58:39] +    |
[00:58:39] +    |
[00:58:39] + LL | / const A: isize = Foo::B as isize;
[00:58:39] + LL | |
[00:58:39] + LL | | enum Foo {
[00:58:39] + LL | |     B = A,
[00:58:39] + LL | | }
[00:58:39] + LL | |
[00:58:39] + LL | | fn main() {}
[00:58:39] 18 
[00:58:39] 19 error: aborting due to previous error
[00:58:39] 20 
[00:58:39] 
[00:58:39] 
[00:58:39] 
[00:58:39] The actual stderr differed from the expected stderr.
[00:58:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/issue-36163.stderr
[00:58:39] To update references, rerun the tests and pass the `--bless` flag
[00:58:39] To only update this specific test, also pass `--test-args issues/issue-36163.rs`
[00:58:39] error: 1 errors occurred comparing output.
[00:58:39] status: exit code: 1
[00:58:39] status: exit code: 1
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36163.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error[E0391]: cycle detected when const-evaluating + checking `Foo::B::{{constant}}#0`
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     B = A, //~ ERROR E0391
[00:58:39]    |
[00:58:39]    |
[00:58:39] note: ...which requires const-evaluating `Foo::B::{{constant}}#0`...
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     B = A, //~ ERROR E0391
[00:58:39]    |         ^
[00:58:39] note: ...which requires const-evaluating `A`...
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL | const A: isize = Foo::B as isize;
[00:58:39]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:58:39]    = note: ...which again requires const-evaluating + checking `Foo::B::{{constant}}#0`, completing the cycle
[00:58:39] note: cycle used when collecting item types in top-level module
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL | / const A: isize = Foo::B as isize;
[00:58:39] LL | | enum Foo {
[00:58:39] LL | | enum Foo {
[00:58:39] LL | |     B = A, //~ ERROR E0391
[00:58:39] LL | |
[00:58:39] LL | | fn main() {}
[00:58:39]    | |____________^
[00:58:39] 
---
[00:58:39] 10 
[00:58:39] - error[E0080]: evaluation of constant value failed
[00:58:39] -   --> $DIR/issue-41394.rs:7:9
[00:58:39] -    |
[00:58:39] - LL |     A = Foo::A as isize
[00:58:39] -    |         ^^^^^^^^^^^^^^^ referenced constant has errors
[00:58:39] + error: aborting due to previous error
[00:58:39] - error: aborting due to 2 previous errors
[00:58:39] - 
[00:58:39] - Some errors have detailed explanations: E0080, E0369.
[00:58:39] - For more information about an error, try `rustc --explain E0080`.
[00:58:39] - For more information about an error, try `rustc --explain E0080`.
[00:58:39] + For more information about this error, try `rustc --explain E0369`.
[00:58:39] 21 
[00:58:39] 
[00:58:39] 
[00:58:39] The actual stderr differed from the expected stderr.
[00:58:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/issue-41394.stderr
[00:58:39] To update references, rerun the tests and pass the `--bless` flag
[00:58:39] To only update this specific test, also pass `--test-args issues/issue-41394.rs`
[00:58:39] error: 1 errors occurred comparing output.
[00:58:39] status: exit code: 1
[00:58:39] status: exit code: 1
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary" "-A" "unused"
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error[E0369]: binary operation `+` cannot be applied to type `&str`
[00:58:39]    |
[00:58:39]    |
[00:58:39] LL |     A = "" + 1
[00:58:39]    |         -- ^ - {integer}
[00:58:39]    |         &str
[00:58:39]    |
[00:58:39]    = note: an implementation of `std::ops::Add` might be missing for `&str`
[00:58:39] 
---
[00:58:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:58:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:39] 
[00:58:39] 
[00:58:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:39] 
[00:58:39] 
[00:58:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:39] Build completed unsuccessfully in 0:53:46
---
travis_time:end:0921742c:start=1560189619341171649,finish=1560189619346163769,duration=4992120
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b789137
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2dbfc042
travis_time:start:2dbfc042
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:183d8b60
$ dmesg | grep -i kill
