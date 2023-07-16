plain
2019-12-22T23:24:49.2282949Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T23:24:49.2471819Z ##[command]git config gc.auto 0
2019-12-22T23:24:49.2554788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T23:24:49.2626061Z ##[command]git config --get-all http.proxy
2019-12-22T23:24:49.2782325Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67543/merge:refs/remotes/pull/67543/merge
---
2019-12-23T00:25:32.0047413Z .................................................................................................... 1600/9432
2019-12-23T00:25:36.6849805Z .................................................................................................... 1700/9432
2019-12-23T00:25:47.1506429Z .......................................................................................i............ 1800/9432
2019-12-23T00:25:54.6132077Z .................................................................................................... 1900/9432
2019-12-23T00:26:01.1869084Z ........................................................................iiiii....................... 2000/9432
2019-12-23T00:26:22.4957836Z .................................................................................................... 2200/9432
2019-12-23T00:26:24.8814728Z .................................................................................................... 2300/9432
2019-12-23T00:26:27.5219408Z .................................................................................................... 2400/9432
2019-12-23T00:26:40.1805404Z .................................................................................................... 2500/9432
---
2019-12-23T00:29:31.4673662Z ....i...............i............................................................................... 4900/9432
2019-12-23T00:29:41.5399215Z .................................................................................................... 5000/9432
2019-12-23T00:29:46.4283765Z ................................................i................................................... 5100/9432
2019-12-23T00:29:56.0752965Z .................................................................................................... 5200/9432
2019-12-23T00:30:02.0604731Z ...............ii.ii...........i.................................................................... 5300/9432
2019-12-23T00:30:11.1970665Z .................................................................................................... 5500/9432
2019-12-23T00:30:22.8187311Z .................................................................................................i.. 5600/9432
2019-12-23T00:30:30.2455937Z .................................................................................................... 5700/9432
2019-12-23T00:30:35.3665583Z .................................................................................................... 5800/9432
2019-12-23T00:30:35.3665583Z .................................................................................................... 5800/9432
2019-12-23T00:30:44.9317369Z .....................................................................................ii...i..ii..... 5900/9432
2019-12-23T00:31:07.1966779Z .................................................................................................... 6100/9432
2019-12-23T00:31:15.1929921Z .................................................................................................... 6200/9432
2019-12-23T00:31:23.1206318Z .................................................................................................... 6300/9432
2019-12-23T00:31:23.1206318Z .................................................................................................... 6300/9432
2019-12-23T00:31:40.4370184Z ............i..ii................................................................................... 6400/9432
2019-12-23T00:32:01.1315868Z ........................................................................................i........... 6600/9432
2019-12-23T00:32:03.2458991Z .................................................................................................... 6700/9432
2019-12-23T00:32:05.5445555Z ........................................................................................i........... 6800/9432
2019-12-23T00:32:08.3453617Z .................................................................................................... 6900/9432
---
2019-12-23T00:33:46.5522778Z .................................................................................................... 7500/9432
2019-12-23T00:33:51.7007585Z .................................................................................................... 7600/9432
2019-12-23T00:33:58.6801066Z .................................................................................................... 7700/9432
2019-12-23T00:34:09.3414473Z .................................................................................................... 7800/9432
2019-12-23T00:34:15.7543914Z ...iiii............................................................................................. 7900/9432
2019-12-23T00:34:30.4232147Z .................................................................................................... 8100/9432
2019-12-23T00:34:41.9664038Z .................................................................................................... 8200/9432
2019-12-23T00:34:53.8898603Z .................................................................................................... 8300/9432
2019-12-23T00:34:59.6022987Z .................................................................................................... 8400/9432
---
2019-12-23T00:36:52.7715523Z ---- [ui] ui/typeck/issue-66868-closure-typeck.rs stdout ----
2019-12-23T00:36:52.7715896Z diff of stderr:
2019-12-23T00:36:52.7716133Z 
2019-12-23T00:36:52.7716389Z 12    |
2019-12-23T00:36:52.7717080Z 13    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>`
2019-12-23T00:36:52.7718535Z 14    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}`
2019-12-23T00:36:52.7720082Z -    = note: required because it appears within the type `[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T00:36:52.7723355Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(14:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T00:36:52.7725299Z +    = note: required because it appears within the type `[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T00:36:52.7727102Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T00:36:52.7728029Z 18    = note: required because it appears within the type `impl std::future::Future`
2019-12-23T00:36:52.7728312Z 19 
2019-12-23T00:36:52.7728524Z 
2019-12-23T00:36:52.7728730Z 
2019-12-23T00:36:52.7728730Z 
2019-12-23T00:36:52.7728979Z The actual stderr differed from the expected stderr.
2019-12-23T00:36:52.7729675Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck/issue-66868-closure-typeck.stderr
2019-12-23T00:36:52.7730310Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T00:36:52.7731045Z To only update this specific test, also pass `--test-args typeck/issue-66868-closure-typeck.rs`
2019-12-23T00:36:52.7731646Z error: 1 errors occurred comparing output.
2019-12-23T00:36:52.7731894Z status: exit code: 1
2019-12-23T00:36:52.7731894Z status: exit code: 1
2019-12-23T00:36:52.7733203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-66868-closure-typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-66868-closure-typeck/auxiliary" "-A" "unused"
2019-12-23T00:36:52.7735514Z ------------------------------------------
2019-12-23T00:36:52.7735723Z 
2019-12-23T00:36:52.7737595Z ------------------------------------------
2019-12-23T00:36:52.7738212Z stderr:
2019-12-23T00:36:52.7738212Z stderr:
2019-12-23T00:36:52.7738708Z ------------------------------------------
2019-12-23T00:36:52.7739268Z error[E0277]: `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>` cannot be sent between threads safely
2019-12-23T00:36:52.7740823Z    |
2019-12-23T00:36:52.7740823Z    |
2019-12-23T00:36:52.7741008Z LL | pub fn g<T>(task: T)
2019-12-23T00:36:52.7741611Z LL | where
2019-12-23T00:36:52.7741782Z LL |     T: Send,
2019-12-23T00:36:52.7742185Z    |        ---- required by this bound in `g`
2019-12-23T00:36:52.7742377Z ...
2019-12-23T00:36:52.7742377Z ...
2019-12-23T00:36:52.7742577Z LL |     g(issue_66868_closure_typeck::f()); //~ ERROR: cannot be sent between threads safely
2019-12-23T00:36:52.7743073Z    |     ^ `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>` cannot be sent between threads safely
2019-12-23T00:36:52.7743304Z    |
2019-12-23T00:36:52.7743845Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::RwLockWriteGuard<'_, issue_66868_closure_typeck::S>`
2019-12-23T00:36:52.7745539Z    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}`
2019-12-23T00:36:52.7747116Z    = note: required because it appears within the type `[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]`
2019-12-23T00:36:52.7749430Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@DefId(15:16 ~ issue_66868_closure_typeck[8787]::f[0]::{{closure}}[0]) for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::sync::RwLock<issue_66868_closure_typeck::S>, &'r std::sync::RwLock<issue_66868_closure_typeck::S>, std::sync::RwLock<issue_66868_closure_typeck::S>, std::result::Result<std::sync::RwLockWriteGuard<'s, issue_66868_closure_typeck::S>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'t0, issue_66868_closure_typeck::S>>>, &'t1 mut std::sync::RwLockWriteGuard<'t2, issue_66868_closure_typeck::S>, std::sync::RwLockWriteGuard<'t3, issue_66868_closure_typeck::S>, issue_66868_closure_typeck::S, &'t4 mut issue_66868_closure_typeck::S, &'t5 mut issue_66868_closure_typeck::S, ()}]>`
2019-12-23T00:36:52.7749974Z    = note: required because it appears within the type `impl std::future::Future`
2019-12-23T00:36:52.7750117Z 
2019-12-23T00:36:52.7750289Z error: aborting due to previous error
2019-12-23T00:36:52.7750432Z 
---
2019-12-23T00:36:52.7758654Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T00:36:52.7759051Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T00:36:52.7759115Z 
2019-12-23T00:36:52.7759145Z 
2019-12-23T00:36:52.7760981Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T00:36:52.7761478Z 
2019-12-23T00:36:52.7761507Z 
2019-12-23T00:36:52.7767647Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T00:36:52.7767835Z Build completed unsuccessfully in 1:05:29
2019-12-23T00:36:52.7767835Z Build completed unsuccessfully in 1:05:29
2019-12-23T00:36:52.7833164Z == clock drift check ==
2019-12-23T00:36:52.7852285Z   local time: Mon Dec 23 00:36:52 UTC 2019
2019-12-23T00:36:52.9424065Z   network time: Mon, 23 Dec 2019 00:36:52 GMT
2019-12-23T00:36:52.9427862Z == end clock drift check ==
2019-12-23T00:36:53.7424887Z 
2019-12-23T00:36:53.7542950Z ##[error]Bash exited with code '1'.
2019-12-23T00:36:53.7589001Z ##[section]Starting: Checkout
2019-12-23T00:36:53.7590642Z ==============================================================================
2019-12-23T00:36:53.7590724Z Task         : Get sources
2019-12-23T00:36:53.7590766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
