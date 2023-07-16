plain
2019-07-22T17:20:59.9833070Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-22T17:21:00.0061536Z ##[command]git config gc.auto 0
2019-07-22T17:21:00.8819579Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-22T17:21:00.8823863Z ##[command]git config --get-all http.proxy
2019-07-22T17:21:00.8827738Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62871/merge:refs/remotes/pull/62871/merge
---
2019-07-22T17:21:35.0597876Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T17:21:35.0598356Z 
2019-07-22T17:21:35.0599066Z   git checkout -b <new-branch-name>
2019-07-22T17:21:35.0599603Z 
2019-07-22T17:21:35.0600013Z HEAD is now at 6af10d5d0 Merge bbae44167a303c28f0c235743a1a1c09bdc4f905 into 4bc1ce7bdb7f5dc9ea07c0b630c087d8e11140e4
2019-07-22T17:21:35.0823353Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-22T17:21:35.0825903Z ==============================================================================
2019-07-22T17:21:35.0825962Z Task         : Bash
2019-07-22T17:21:35.0826011Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-22T18:20:00.5162218Z .................................................................................................... 200/5841
2019-07-22T18:20:04.6439245Z F................................................................................................... 300/5841
2019-07-22T18:20:08.3688820Z .................................................................................................... 400/5841
2019-07-22T18:20:12.1565647Z .................................................................................................... 500/5841
2019-07-22T18:20:15.8760977Z ........................................................................i........................... 600/5841
2019-07-22T18:20:24.7279323Z .................................................................................................... 800/5841
2019-07-22T18:20:30.2555283Z .................................................................................................... 900/5841
2019-07-22T18:20:35.1590758Z ...................................................................................................i 1000/5841
2019-07-22T18:20:35.1590758Z ...................................................................................................i 1000/5841
2019-07-22T18:20:40.6293548Z ...........i........................................................................................ 1100/5841
2019-07-22T18:20:44.5118237Z .............................iiiii.................................................................. 1200/5841
2019-07-22T18:20:50.5054402Z .................................................................................................... 1400/5841
2019-07-22T18:20:53.1780261Z .................................................................................................... 1500/5841
2019-07-22T18:20:56.9577052Z .................................................................................................... 1600/5841
2019-07-22T18:20:59.5713686Z .................................................................................................... 1700/5841
2019-07-22T18:20:59.5713686Z .................................................................................................... 1700/5841
2019-07-22T18:21:03.0006031Z ...................................................................i......F......................... 1800/5841
2019-07-22T18:21:11.3107556Z .................................................................................................... 2000/5841
2019-07-22T18:21:15.5721324Z .................................................................................................... 2100/5841
2019-07-22T18:21:19.2360046Z .................................................................................................... 2200/5841
2019-07-22T18:21:19.2360046Z .................................................................................................... 2200/5841
2019-07-22T18:21:23.0471394Z ...................................................i................................................ 2300/5841
2019-07-22T18:21:32.8263624Z .................................................................................................... 2500/5841
2019-07-22T18:21:36.8169808Z .................................................................................................... 2600/5841
2019-07-22T18:21:41.9403653Z .................................................................................................... 2700/5841
2019-07-22T18:21:45.8351019Z .................................................................................................... 2800/5841
2019-07-22T18:21:45.8351019Z .................................................................................................... 2800/5841
2019-07-22T18:21:50.1782256Z .................................................................................................... 2900/5841
2019-07-22T18:21:55.3029373Z .................................................................................................... 3000/5841
2019-07-22T18:21:59.6585598Z .................................................................................................... 3100/5841
2019-07-22T18:22:04.8089575Z .................................................................................................... 3200/5841
2019-07-22T18:22:08.2260325Z .................................................................................................... 3300/5841
2019-07-22T18:22:11.8649142Z .................................................................................................... 3400/5841
2019-07-22T18:22:16.8680240Z .................................................................................................... 3500/5841
2019-07-22T18:22:20.6170635Z ...................i................................................................................ 3600/5841
2019-07-22T18:22:24.7481481Z ............................................................................................ii...i.. 3700/5841
2019-07-22T18:22:28.6638986Z ii.................................................................................................. 3800/5841
2019-07-22T18:22:37.2682962Z .................................................................................................... 4000/5841
2019-07-22T18:22:37.2682962Z .................................................................................................... 4000/5841
2019-07-22T18:22:40.9352592Z ......ii............................................................................................ 4100/5841
2019-07-22T18:22:42.9361177Z ...........................i........................................................................ 4200/5841
2019-07-22T18:22:44.9420145Z .............................................................................................i...... 4300/5841
2019-07-22T18:22:51.9409711Z .................................................................................................... 4500/5841
2019-07-22T18:23:09.1826116Z .................................................................................................... 4600/5841
2019-07-22T18:23:12.6241313Z .................................................................................................... 4700/5841
2019-07-22T18:23:16.3884848Z .................................................................................................... 4800/5841
---
2019-07-22T18:23:49.3375117Z .................................................................................................... 5400/5841
2019-07-22T18:23:53.3487354Z .................................................................................................... 5500/5841
2019-07-22T18:23:57.4259163Z .................................................................................................... 5600/5841
2019-07-22T18:24:00.5330490Z .................................................................................................... 5700/5841
2019-07-22T18:24:03.4786291Z .................................................................................i.................. 5800/5841
2019-07-22T18:24:04.9425263Z failures:
2019-07-22T18:24:04.9471140Z 
2019-07-22T18:24:04.9471636Z ---- [ui] ui/async-await/recursive-async-impl-trait-type.rs stdout ----
2019-07-22T18:24:04.9471696Z diff of stderr:
2019-07-22T18:24:04.9471696Z diff of stderr:
2019-07-22T18:24:04.9471730Z 
2019-07-22T18:24:04.9471994Z - error[E0720]: opaque type expands to a recursive type
2019-07-22T18:24:04.9472047Z + error[E0733]: recursion in an `async fn` requires boxing
2019-07-22T18:24:04.9472339Z 3    |
2019-07-22T18:24:04.9472339Z 3    |
2019-07-22T18:24:04.9472556Z 4 LL | async fn recursive_async_function() -> () {
2019-07-22T18:24:04.9472826Z -    |                                        ^^ expands to a recursive type
2019-07-22T18:24:04.9472826Z -    |                                        ^^ expands to a recursive type
2019-07-22T18:24:04.9472901Z +    |                                        ^^ an `async fn` cannot invoke itself directly
2019-07-22T18:24:04.9472950Z 6    |
2019-07-22T18:24:04.9473287Z -    = note: expanded type is `std::future::GenFuture<[static generator@$DIR/recursive-async-impl-trait-type.rs:7:43: 9:2 {impl std::future::Future, ()}]>`
2019-07-22T18:24:04.9473376Z +    = note: to create a recursive async fn, it must be rewritten to return a
2019-07-22T18:24:04.9473426Z +            boxed future.
2019-07-22T18:24:04.9473678Z +            For more information, see https://rust-lang.github.io/async-book/
2019-07-22T18:24:04.9473782Z 9 error: aborting due to previous error
2019-07-22T18:24:04.9473823Z 10 
2019-07-22T18:24:04.9473864Z 
2019-07-22T18:24:04.9474098Z - For more information about this error, try `rustc --explain E0720`.
2019-07-22T18:24:04.9474098Z - For more information about this error, try `rustc --explain E0720`.
2019-07-22T18:24:04.9474336Z + For more information about this error, try `rustc --explain E0733`.
2019-07-22T18:24:04.9474381Z 12 
2019-07-22T18:24:04.9474422Z 
2019-07-22T18:24:04.9474447Z 
2019-07-22T18:24:04.9474491Z The actual stderr differed from the expected stderr.
2019-07-22T18:24:04.9474965Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2019-07-22T18:24:04.9475287Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T18:24:04.9475567Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2019-07-22T18:24:04.9475663Z error: 1 errors occurred comparing output.
2019-07-22T18:24:04.9475707Z status: exit code: 1
2019-07-22T18:24:04.9475707Z status: exit code: 1
2019-07-22T18:24:04.9476503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary" "-A" "unused"
2019-07-22T18:24:04.9476960Z ------------------------------------------
2019-07-22T18:24:04.9477015Z 
2019-07-22T18:24:04.9477227Z ------------------------------------------
2019-07-22T18:24:04.9477271Z stderr:
2019-07-22T18:24:04.9477271Z stderr:
2019-07-22T18:24:04.9477488Z ------------------------------------------
2019-07-22T18:24:04.9477537Z error[E0733]: recursion in an `async fn` requires boxing
2019-07-22T18:24:04.9477858Z    |
2019-07-22T18:24:04.9477858Z    |
2019-07-22T18:24:04.9478382Z LL | async fn recursive_async_function() -> () { //~ ERROR
2019-07-22T18:24:04.9478450Z    |                                        ^^ an `async fn` cannot invoke itself directly
2019-07-22T18:24:04.9478515Z    |
2019-07-22T18:24:04.9478574Z    = note: to create a recursive async fn, it must be rewritten to return a
2019-07-22T18:24:04.9478627Z            boxed future.
2019-07-22T18:24:04.9478920Z            For more information, see https://rust-lang.github.io/async-book/
2019-07-22T18:24:04.9478998Z error: aborting due to previous error
2019-07-22T18:24:04.9479027Z 
2019-07-22T18:24:04.9479274Z For more information about this error, try `rustc --explain E0733`.
2019-07-22T18:24:04.9479309Z 
2019-07-22T18:24:04.9479309Z 
2019-07-22T18:24:04.9479513Z ------------------------------------------
2019-07-22T18:24:04.9479543Z 
2019-07-22T18:24:04.9479567Z 
2019-07-22T18:24:04.9479805Z ---- [ui] ui/generator/async-recursion-forbid.rs stdout ----
2019-07-22T18:24:04.9480069Z diff of stderr:
2019-07-22T18:24:04.9480100Z 
2019-07-22T18:24:04.9480368Z - error[E0733]: recursion in an async fn requires boxing
2019-07-22T18:24:04.9480440Z + error[E0733]: recursion in an `async fn` requires boxing
2019-07-22T18:24:04.9480654Z 2   --> $DIR/async-recursion-forbid.rs:6:24
2019-07-22T18:24:04.9480699Z 3    |
2019-07-22T18:24:04.9480774Z 4 LL | async fn foo(n: usize) {
2019-07-22T18:24:04.9480804Z 
2019-07-22T18:24:04.9481045Z -    |                        ^ async fn cannot invoke themselves directly
2019-07-22T18:24:04.9481114Z +    |                        ^ an `async fn` cannot invoke itself directly
2019-07-22T18:24:04.9481160Z 6    |
2019-07-22T18:24:04.9481207Z 7    = note: to create a recursive async fn, it must be rewritten to return a
2019-07-22T18:24:04.9481253Z 8            boxed future.
2019-07-22T18:24:04.9481321Z 
2019-07-22T18:24:04.9481364Z The actual stderr differed from the expected stderr.
2019-07-22T18:24:04.9481364Z The actual stderr differed from the expected stderr.
2019-07-22T18:24:04.9481671Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-recursion-forbid/async-recursion-forbid.stderr
2019-07-22T18:24:04.9481929Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T18:24:04.9482199Z To only update this specific test, also pass `--test-args generator/async-recursion-forbid.rs`
2019-07-22T18:24:04.9482427Z error: 1 errors occurred comparing output.
2019-07-22T18:24:04.9482470Z status: exit code: 1
2019-07-22T18:24:04.9482470Z status: exit code: 1
2019-07-22T18:24:04.9483269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/async-recursion-forbid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-recursion-forbid" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-recursion-forbid/auxiliary" "-A" "unused"
2019-07-22T18:24:04.9483591Z ------------------------------------------
2019-07-22T18:24:04.9483624Z 
2019-07-22T18:24:04.9483832Z ------------------------------------------
2019-07-22T18:24:04.9483876Z stderr:
2019-07-22T18:24:04.9483876Z stderr:
2019-07-22T18:24:04.9486486Z ------------------------------------------
2019-07-22T18:24:04.9486560Z error[E0733]: recursion in an `async fn` requires boxing
2019-07-22T18:24:04.9486922Z   --> /checkout/src/test/ui/generator/async-recursion-forbid.rs:6:24
2019-07-22T18:24:04.9487000Z    |
2019-07-22T18:24:04.9487049Z LL | async fn foo(n: usize) { //~ ERROR recursion in an async fn requires boxing [E0733]
2019-07-22T18:24:04.9487104Z    |                        ^ an `async fn` cannot invoke itself directly
2019-07-22T18:24:04.9487163Z    |
2019-07-22T18:24:04.9487209Z    = note: to create a recursive async fn, it must be rewritten to return a
2019-07-22T18:24:04.9487256Z            boxed future.
2019-07-22T18:24:04.9487522Z            For more information, see https://rust-lang.github.io/async-book/
2019-07-22T18:24:04.9487600Z error: aborting due to previous error
2019-07-22T18:24:04.9487628Z 
2019-07-22T18:24:04.9487889Z For more information about this error, try `rustc --explain E0733`.
2019-07-22T18:24:04.9487931Z 
---
2019-07-22T18:24:04.9491927Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-22T18:24:04.9492000Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-22T18:24:04.9499956Z 
2019-07-22T18:24:04.9500156Z 
2019-07-22T18:24:04.9511470Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-22T18:24:04.9511806Z 
2019-07-22T18:24:04.9511837Z 
2019-07-22T18:24:05.8171140Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-22T18:24:05.8178394Z Build completed unsuccessfully in 0:56:12
2019-07-22T18:24:05.8178394Z Build completed unsuccessfully in 0:56:12
2019-07-22T18:24:05.9290655Z ##[error]Bash exited with code '1'.
2019-07-22T18:24:05.9331722Z ##[section]Starting: Checkout
2019-07-22T18:24:05.9333877Z ==============================================================================
2019-07-22T18:24:05.9333966Z Task         : Get sources
2019-07-22T18:24:05.9334022Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
