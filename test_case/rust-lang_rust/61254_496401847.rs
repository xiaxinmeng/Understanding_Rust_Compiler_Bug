plain
travis_time:end:1e2f9d9c:start=1559024616460807741,finish=1559024617302691999,duration=841884258
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:15:31] .................................................................................................... 4700/5590
[01:15:38] .................................................................................................... 4800/5590
[01:15:45] .................................................................................................... 4900/5590
[01:15:48] .................................................................................................... 5000/5590
[01:15:53] ...........................................................................F........................ 5100/5590
[01:16:02] .................................................................................................... 5300/5590
[01:16:05] .................................................................................................... 5400/5590
[01:16:08] .................................................................................................... 5500/5590
[01:16:11] ............................i.............................................................
---
[01:16:11] 1 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:28:20
[01:16:11] +   --> $DIR/suggest-move-types.rs:26:20
[01:16:11] 3    |
[01:16:11] 4 LL | struct A<T, M: One<A=(), T>> {
[01:16:11] 5    |                    ^^^^^^^ help: reorder the arguments: `T, A = ()`
[01:16:11] 6 
[01:16:11] 7 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:34:37
[01:16:11] +   --> $DIR/suggest-move-types.rs:32:37
[01:16:11] +   --> $DIR/suggest-move-types.rs:32:37
[01:16:11] 9    |
[01:16:11] 10 LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
[01:16:11] 11    |                                     ^^^^^^^^^^^ help: reorder the arguments: `'a, T, A = ()`
[01:16:11] 12 
[01:16:11] 13 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:40:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:38:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:38:28
[01:16:11] 15    |
[01:16:11] 16 LL | struct B<T, U, V, M: Three<A=(), B=(), C=(), T, U, V>> {
[01:16:11] 17    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] 18 
[01:16:11] 19 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:47:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:45:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:45:53
[01:16:11] 21    |
[01:16:11] 22 LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
[01:16:11] 23    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] 24 
[01:16:11] 25 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:55:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:53:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:53:28
[01:16:11] 27    |
[01:16:11] 28 LL | struct C<T, U, V, M: Three<T, A=(), B=(), C=(), U, V>> {
[01:16:11] 29    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] 30 
[01:16:11] 31 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:62:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:60:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:60:53
[01:16:11] 33    |
[01:16:11] 34 LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
[01:16:11] 35    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] 36 
[01:16:11] 37 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:70:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:68:28
[01:16:11] +   --> $DIR/suggest-move-types.rs:68:28
[01:16:11] 39    |
[01:16:11] 40 LL | struct D<T, U, V, M: Three<T, A=(), B=(), U, C=(), V>> {
[01:16:11] 41    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] 42 
[01:16:11] 43 error: incorrect parameter order
[01:16:11] -   --> $DIR/suggest-move-types.rs:77:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:75:53
[01:16:11] +   --> $DIR/suggest-move-types.rs:75:53
[01:16:11] 45    |
[01:16:11] 46 LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
[01:16:11] 47    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] 
[01:16:11] The actual stderr differed from the expected stderr.
[01:16:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/suggest-move-types.stderr
[01:16:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/suggest-move-types.stderr
[01:16:11] To update references, rerun the tests and pass the `--bless` flag
[01:16:11] To only update this specific test, also pass `--test-args suggestions/suggest-move-types.rs`
[01:16:11] error: 1 errors occurred comparing output.
[01:16:11] status: exit code: 1
[01:16:11] status: exit code: 1
[01:16:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-move-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/auxiliary" "-A" "unused"
[01:16:11] ------------------------------------------
[01:16:11] 
[01:16:11] ------------------------------------------
[01:16:11] stderr:
[01:16:11] stderr:
[01:16:11] ------------------------------------------
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:26:20
[01:16:11]    |
[01:16:11] LL | struct A<T, M: One<A=(), T>> { //~ ERROR incorrect parameter order
[01:16:11]    |                    ^^^^^^^ help: reorder the arguments: `T, A = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:32:37
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
[01:16:11]    |                                     ^^^^^^^^^^^ help: reorder the arguments: `'a, T, A = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:38:28
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct B<T, U, V, M: Three<A=(), B=(), C=(), T, U, V>> { //~ ERROR incorrect parameter order
[01:16:11]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:45:53
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
[01:16:11]    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:53:28
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct C<T, U, V, M: Three<T, A=(), B=(), C=(), U, V>> { //~ ERROR incorrect parameter order
[01:16:11]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:60:53
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
[01:16:11]    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:68:28
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct D<T, U, V, M: Three<T, A=(), B=(), U, C=(), V>> { //~ ERROR incorrect parameter order
[01:16:11]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `T, U, V, A = (), B = (), C = ()`
[01:16:11] error: incorrect parameter order
[01:16:11]   --> /checkout/src/test/ui/suggestions/suggest-move-types.rs:75:53
[01:16:11]    |
[01:16:11]    |
[01:16:11] LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
[01:16:11]    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: reorder the arguments: `'a, 'b, 'c, T, U, V, A = (), B = (), C = ()`
[01:16:11] error: aborting due to 8 previous errors
[01:16:11] 
[01:16:11] 
[01:16:11] ------------------------------------------
---
[01:16:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:16:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:11] 
[01:16:11] 
[01:16:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:11] 
[01:16:11] 
[01:16:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:11] Build completed unsuccessfully in 0:05:15
[01:16:11] Build completed unsuccessfully in 0:05:15
[01:16:11] make: *** [check] Error 1
[01:16:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a6d5b96
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 07:40:00 UTC 2019
