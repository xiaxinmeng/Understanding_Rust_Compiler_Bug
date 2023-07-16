plain
2019-09-23T12:22:38.3127064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-23T12:22:38.3317447Z ##[command]git config gc.auto 0
2019-09-23T12:22:38.3404717Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-23T12:22:39.3312958Z ##[command]git config --get-all http.proxy
2019-09-23T12:22:39.3319935Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64706/merge:refs/remotes/pull/64706/merge
---
2019-09-23T13:34:36.5693210Z .................................................................................................... 1500/9039
2019-09-23T13:34:43.6692455Z .................................................................................................... 1600/9039
2019-09-23T13:34:57.6022903Z .........................................................................i...............i.......... 1700/9039
2019-09-23T13:35:05.6530062Z .................................................................................................... 1800/9039
2019-09-23T13:35:15.7553619Z ................................................................iiiii............................... 1900/9039
2019-09-23T13:35:38.4828485Z .................................................................................................... 2100/9039
2019-09-23T13:35:41.4634542Z .................................................................................................... 2200/9039
2019-09-23T13:35:45.2497914Z .................................................................................................... 2300/9039
2019-09-23T13:35:54.9007519Z .................................................................................................... 2400/9039
---
2019-09-23T13:39:18.1207745Z .................................................................................................... 4600/9039
2019-09-23T13:39:25.4452710Z .....................................................i...............i.............................. 4700/9039
2019-09-23T13:39:37.0260946Z .................................................................................................... 4800/9039
2019-09-23T13:39:46.7879684Z .................................................................................................... 4900/9039
2019-09-23T13:39:55.3112584Z .F.................................................................................................. 5000/9039
2019-09-23T13:40:06.3878172Z .........................................ii.ii...................................................... 5100/9039
2019-09-23T13:40:17.8546437Z .................................................................................................... 5300/9039
2019-09-23T13:40:29.8258961Z .................................................................................................... 5400/9039
2019-09-23T13:40:38.6373373Z ......i............................................................................................. 5500/9039
2019-09-23T13:40:44.9513100Z .................................................................................................... 5600/9039
2019-09-23T13:40:44.9513100Z .................................................................................................... 5600/9039
2019-09-23T13:40:58.5192847Z .................................................................................................... 5700/9039
2019-09-23T13:41:13.8959309Z .ii...i..ii...........i............................................................................. 5800/9039
2019-09-23T13:41:39.3016964Z .................................................................................................... 6000/9039
2019-09-23T13:41:50.1588512Z .................................................................................................... 6100/9039
2019-09-23T13:41:50.1588512Z .................................................................................................... 6100/9039
2019-09-23T13:42:10.0224722Z ...i..ii............................................................................................ 6200/9039
2019-09-23T13:42:33.0505953Z ...............................................................i.................................... 6400/9039
2019-09-23T13:42:35.8159245Z .................................................................................................... 6500/9039
2019-09-23T13:42:38.9807614Z ...................................i................................................................ 6600/9039
2019-09-23T13:42:43.9082431Z .................................................................................................... 6700/9039
---
2019-09-23T13:47:34.1245446Z 
2019-09-23T13:47:34.1245984Z ---- [ui] ui/issues/issue-60218.rs stdout ----
2019-09-23T13:47:34.1246043Z diff of stderr:
2019-09-23T13:47:34.1246075Z 
2019-09-23T13:47:34.1246449Z 1 error[E0277]: the trait bound `for<'t> <std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item: Foo` is not satisfied
2019-09-23T13:47:34.1246674Z 2   --> $DIR/issue-60218.rs:18:5
2019-09-23T13:47:34.1246744Z 3    |
2019-09-23T13:47:34.1246963Z - LL | / pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-09-23T13:47:34.1247141Z - LL | | where
2019-09-23T13:47:34.1247341Z - LL | |     for<'t> &'t I: IntoIterator,
2019-09-23T13:47:34.1247605Z - LL | | for<'t> Map<<&'t I as IntoIterator>::IntoIter, F>: Iterator,
2019-09-23T13:47:34.1247851Z - LL | | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-09-23T13:47:34.1248025Z - LL | | {
2019-09-23T13:47:34.1248777Z - LL | | }
2019-09-23T13:47:34.1248997Z -    | |_- required by `trigger_error`
2019-09-23T13:47:34.1249049Z + LL | pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-09-23T13:47:34.1249320Z 12 ...
2019-09-23T13:47:34.1249320Z 12 ...
2019-09-23T13:47:34.1249542Z - LL |       trigger_error(vec![], |x: &u32| x)
2019-09-23T13:47:34.1250131Z -    |       ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-09-23T13:47:34.1250474Z + LL | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-09-23T13:47:34.1250781Z +    |                                                                        --- required by this bound in `trigger_error`
2019-09-23T13:47:34.1250884Z + ...
2019-09-23T13:47:34.1250931Z + LL |     trigger_error(vec![], |x: &u32| x)
2019-09-23T13:47:34.1251280Z +    |     ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-09-23T13:47:34.1251405Z 16 error: aborting due to previous error
2019-09-23T13:47:34.1251468Z 17 
2019-09-23T13:47:34.1251499Z 
2019-09-23T13:47:34.1251524Z 
2019-09-23T13:47:34.1251524Z 
2019-09-23T13:47:34.1251571Z The actual stderr differed from the expected stderr.
2019-09-23T13:47:34.1251894Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218/issue-60218.stderr
2019-09-23T13:47:34.1252147Z To update references, rerun the tests and pass the `--bless` flag
2019-09-23T13:47:34.1252409Z To only update this specific test, also pass `--test-args issues/issue-60218.rs`
2019-09-23T13:47:34.1252517Z error: 1 errors occurred comparing output.
2019-09-23T13:47:34.1252563Z status: exit code: 1
2019-09-23T13:47:34.1252563Z status: exit code: 1
2019-09-23T13:47:34.1253314Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60218.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60218/auxiliary" "-A" "unused"
2019-09-23T13:47:34.1253645Z ------------------------------------------
2019-09-23T13:47:34.1253679Z 
2019-09-23T13:47:34.1253894Z ------------------------------------------
2019-09-23T13:47:34.1253940Z stderr:
2019-09-23T13:47:34.1253940Z stderr:
2019-09-23T13:47:34.1254167Z ------------------------------------------
2019-09-23T13:47:34.1254638Z error[E0277]: the trait bound `for<'t> <std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item: Foo` is not satisfied
2019-09-23T13:47:34.1254884Z   --> /checkout/src/test/ui/issues/issue-60218.rs:18:5
2019-09-23T13:47:34.1254957Z    |
2019-09-23T13:47:34.1255003Z LL | pub fn trigger_error<I, F>(iterable: I, functor: F)
2019-09-23T13:47:34.1255268Z ...
2019-09-23T13:47:34.1255268Z ...
2019-09-23T13:47:34.1255672Z LL | for<'t> <Map<<&'t I as IntoIterator>::IntoIter, F> as Iterator>::Item: Foo,
2019-09-23T13:47:34.1255964Z    |                                                                        --- required by this bound in `trigger_error`
2019-09-23T13:47:34.1256032Z ...
2019-09-23T13:47:34.1256077Z LL |     trigger_error(vec![], |x: &u32| x) //~ ERROR E0277
2019-09-23T13:47:34.1256410Z    |     ^^^^^^^^^^^^^ the trait `for<'t> Foo` is not implemented for `<std::iter::Map<<&'t _ as std::iter::IntoIterator>::IntoIter, _> as std::iter::Iterator>::Item`
2019-09-23T13:47:34.1256523Z error: aborting due to previous error
2019-09-23T13:47:34.1256553Z 
2019-09-23T13:47:34.1256795Z For more information about this error, try `rustc --explain E0277`.
2019-09-23T13:47:34.1256851Z 
---
2019-09-23T13:47:34.1285572Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-23T13:47:34.1285898Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-23T13:47:34.1304903Z 
2019-09-23T13:47:34.1305147Z 
2019-09-23T13:47:34.1306953Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-23T13:47:34.1307209Z 
2019-09-23T13:47:34.1307265Z 
2019-09-23T13:47:34.1316254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-23T13:47:34.1316332Z Build completed unsuccessfully in 1:17:28
2019-09-23T13:47:34.1316332Z Build completed unsuccessfully in 1:17:28
2019-09-23T13:47:34.1377123Z == clock drift check ==
2019-09-23T13:47:34.1392902Z   local time: Mon Sep 23 13:47:34 UTC 2019
2019-09-23T13:47:34.2386054Z   network time: Mon, 23 Sep 2019 13:47:34 GMT
2019-09-23T13:47:34.2389520Z == end clock drift check ==
2019-09-23T13:47:35.0013619Z ##[error]Bash exited with code '1'.
2019-09-23T13:47:35.0056944Z ##[section]Starting: Checkout
2019-09-23T13:47:35.0059354Z ==============================================================================
2019-09-23T13:47:35.0059415Z Task         : Get sources
2019-09-23T13:47:35.0059466Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
