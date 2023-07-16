plain
travis_time:end:0acc755f:start=1559148490593146262,finish=1559148493102367268,duration=2509221006
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:53] 
[01:09:53] running 5599 tests
[01:09:56] .................................................................................................... 100/5599
[01:10:02] ...........F..........FF.................F.......................................................... 200/5599
[01:10:08] .................................................................................................... 400/5599
[01:10:11] ...................................................................................................i 500/5599
[01:10:15] .................................................................................................... 600/5599
[01:10:19] .................................................................................................... 700/5599
---
[01:13:26] failures:
[01:13:26] 
[01:13:26] ---- [ui] ui/async-await/argument-patterns.rs stdout ----
[01:13:26] 
[01:13:26] error: test compilation failed although it shouldn't!
[01:13:26] status: exit code: 1
[01:13:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/argument-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/argument-patterns/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/argument-patterns/auxiliary" "-A" "unused"
[01:13:26] ------------------------------------------
[01:13:26] 
[01:13:26] ------------------------------------------
[01:13:26] stderr:
[01:13:26] stderr:
[01:13:26] ------------------------------------------
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/argument-patterns.rs:14:20
[01:13:26]    |
[01:13:26] LL | async fn b(n: u32, ref mut vec: A) {
[01:13:26]    |                    ^^^^^^^^^^^     - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26] LL |     vec.push(n);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/argument-patterns.rs:18:12
[01:13:26]    |
[01:13:26] LL | async fn c(ref vec: A) {
[01:13:26]    |            ^^^^^^^     - opaque type requires that borrow lasts for `'static`
[01:13:26]    |            creates a temporary which is freed while still in use
[01:13:26] LL |     vec.contains(&0);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
---
[01:13:26] ---- [ui] ui/async-await/dont-print-desugared-async.rs stdout ----
[01:13:26] 
[01:13:26] error: ui test compiled successfully!
[01:13:26] status: exit code: 0
[01:13:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/dont-print-desugared-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-print-desugared-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-print-desugared-async/auxiliary" "-A" "unused"
[01:13:26] ------------------------------------------
[01:13:26] 
[01:13:26] ------------------------------------------
[01:13:26] stderr:
[01:13:26] stderr:
[01:13:26] ------------------------------------------
[01:13:26] warning[E0596]: cannot borrow data in a `&` reference as mutable
[01:13:26]    |
[01:13:26]    |
[01:13:26] LL | async fn async_fn(&ref mut s: &[i32]) {}
[01:13:26]    |                    ^^^^^^^^^ cannot borrow as mutable
[01:13:26]    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:13:26]    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:13:26]    = note: for more information, try `rustc --explain E0729`
[01:13:26] 
[01:13:26] 
[01:13:26] 
[01:13:26] ------------------------------------------
[01:13:26] 
[01:13:26] 
[01:13:26] ---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs stdout ----
[01:13:26] 
[01:13:26] error: test compilation failed although it shouldn't!
[01:13:26] status: exit code: 1
[01:13:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/auxiliary" "-A" "unused"
[01:13:26] ------------------------------------------
[01:13:26] 
[01:13:26] ------------------------------------------
[01:13:26] stderr:
[01:13:26] stderr:
[01:13:26] ------------------------------------------
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:45:20
[01:13:26]    |
[01:13:26] LL | async fn foo_async(ref mut x: D, ref mut _y: D) {
[01:13:26]    |                    ^^^^^^^^^                    - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26] LL |     x.1.borrow_mut().push(DropOrder::Function);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:54:20
[01:13:26]    |
[01:13:26]    |
[01:13:26] LL | async fn bar_async(ref mut x: D, _: D) {
[01:13:26]    |                    ^^^^^^^^^           - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26] LL |     x.1.borrow_mut().push(DropOrder::Function);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:64:20
[01:13:26]    |
[01:13:26]    |
[01:13:26] LL | async fn baz_async((ref mut x, _): (D, D)) {
[01:13:26]    |                    ^^^^^^^^^^^^^^          - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26]    |                    creates a temporary which is freed while still in use
[01:13:26] LL |     x.1.borrow_mut().push(DropOrder::Function);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:74:23
[01:13:26]    |
[01:13:26]    |
[01:13:26] LL | async fn foobar_async(ref mut x: D, (ref mut a, _, ref mut _c): (D, D, D), _: D, ref mut _y: D) {
[01:13:26]    |                       ^^^^^^^^^ creates a temporary which is freed while still in use           - opaque type requires that borrow lasts for `'static`
[01:13:26] LL |     x.1.borrow_mut().push(DropOrder::Function);
[01:13:26] LL | }
[01:13:26]    | - temporary value is freed at the end of this statement
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:86:24
[01:13:26]    |
[01:13:26]    |
[01:13:26] LL |     async fn foo_async(ref mut x: D, ref mut _y: D) {
[01:13:26]    |                        ^^^^^^^^^                    - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:95:24
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:95:24
[01:13:26]    |
[01:13:26] LL |     async fn bar_async(ref mut x: D, _: D) {
[01:13:26]    |                        ^^^^^^^^^           - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:105:24
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:105:24
[01:13:26]    |
[01:13:26] LL |     async fn baz_async((ref mut x, _): (D, D)) {
[01:13:26]    |                        ^^^^^^^^^^^^^^          - opaque type requires that borrow lasts for `'static`
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26]    |                        creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:116:9
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:116:9
[01:13:26]    |
[01:13:26] LL |         ref mut x: D, (ref mut a, _, ref mut _c): (D, D, D), _: D, ref mut _y: D,
[01:13:26]    |         ^^^^^^^^^ creates a temporary which is freed while still in use
[01:13:26] LL |     ) {
[01:13:26]    |       - opaque type requires that borrow lasts for `'static`
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:132:34
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:132:34
[01:13:26]    |
[01:13:26] LL | impl<'a> Bar<'a> {
[01:13:26] LL |     /// Check that unused bindings are dropped after the method with self is polled.
[01:13:26] LL |     /// Check that unused bindings are dropped after the method with self is polled.
[01:13:26] LL |     async fn foo_async(&'a self, ref mut x: D, ref mut _y: D) {
[01:13:26]    |                                  ^^^^^^^^^                    - opaque type requires that borrow lasts for `'a`
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:141:34
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:141:34
[01:13:26]    |
[01:13:26] LL | impl<'a> Bar<'a> {
[01:13:26] ...
[01:13:26] ...
[01:13:26] LL |     async fn bar_async(&'a self, ref mut x: D, _: D) {
[01:13:26]    |                                  ^^^^^^^^^           - opaque type requires that borrow lasts for `'a`
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:151:34
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:151:34
[01:13:26]    |
[01:13:26] LL | impl<'a> Bar<'a> {
[01:13:26] ...
[01:13:26] ...
[01:13:26] LL |     async fn baz_async(&'a self, (ref mut x, _): (D, D)) {
[01:13:26]    |                                  ^^^^^^^^^^^^^^          - opaque type requires that borrow lasts for `'a`
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26]    |                                  creates a temporary which is freed while still in use
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error[E0716]: temporary value dropped while borrowed
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:162:19
[01:13:26]   --> /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs:162:19
[01:13:26]    |
[01:13:26] LL | impl<'a> Bar<'a> {
[01:13:26] ...
[01:13:26] ...
[01:13:26] LL |         &'a self, ref mut x: D, (ref mut a, _, ref mut _c): (D, D, D), _: D, ref mut _y: D,
[01:13:26]    |                   ^^^^^^^^^ creates a temporary which is freed while still in use
[01:13:26] LL |     ) {
[01:13:26]    |       - opaque type requires that borrow lasts for `'a`
[01:13:26] LL |         x.1.borrow_mut().push(DropOrder::Function);
[01:13:26]    |     - temporary value is freed at the end of this statement
[01:13:26] 
[01:13:26] error: aborting due to 12 previous errors
[01:13:26] 
---
[01:13:26] ------------------------------------------
[01:13:26] stderr:
[01:13:26] ------------------------------------------
[01:13:26] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:13:26]   left: `[Val("_y"), Function, Val("x")]`,
[01:13:26]  right: `[Function, Val("_y"), Val("x")]`', /checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs:181:5
[01:13:26] 
[01:13:26] ------------------------------------------
[01:13:26] 
[01:13:26] 
---
[01:13:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:13:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:26] 
[01:13:26] 
[01:13:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:26] 
[01:13:26] 
[01:13:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:26] Build completed unsuccessfully in 0:04:45
[01:13:26] Build completed unsuccessfully in 0:04:45
[01:13:26] Makefile:48: recipe for target 'check' failed
[01:13:26] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e3a2bf2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 29 18:01:51 UTC 2019
---
travis_time:end:0ef0ca7e:start=1559152912409229411,finish=1559152912414525220,duration=5295809
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:099c10f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03f56e1a
travis_time:start:03f56e1a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:288b99e8
$ dmesg | grep -i kill
