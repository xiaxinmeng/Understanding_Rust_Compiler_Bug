plain
2020-01-06T21:10:04.1693644Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T21:10:05.1389223Z ##[command]git config gc.auto 0
2020-01-06T21:10:05.1394569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T21:10:05.1398756Z ##[command]git config --get-all http.proxy
2020-01-06T21:10:05.1403505Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67806/merge:refs/remotes/pull/67806/merge
---
2020-01-06T22:02:18.2850094Z .................................................................................................... 1500/9477
2020-01-06T22:02:23.9034932Z .................................................................................................... 1600/9477
2020-01-06T22:02:28.7055762Z .................................................................................................... 1700/9477
2020-01-06T22:02:37.7909748Z .................................................................................................... 1800/9477
2020-01-06T22:02:45.6618498Z .i.................................................................................................. 1900/9477
2020-01-06T22:02:52.1090368Z .........................................................................................iiiii...... 2000/9477
2020-01-06T22:03:12.9950050Z .................................................................................................... 2200/9477
2020-01-06T22:03:15.3062204Z .................................................................................................... 2300/9477
2020-01-06T22:03:17.6314386Z .................................................................................................... 2400/9477
2020-01-06T22:03:23.4123140Z .................................................................................................... 2500/9477
---
2020-01-06T22:06:14.8496931Z .....................i...............i.............................................................. 4900/9477
2020-01-06T22:06:24.5956030Z .................................................................................................... 5000/9477
2020-01-06T22:06:31.0109386Z ..................................................................i................................. 5100/9477
2020-01-06T22:06:38.0781347Z .................................................................................................... 5200/9477
2020-01-06T22:06:45.3880539Z .................................ii.ii...........i.................................................. 5300/9477
2020-01-06T22:06:54.3921395Z .................................................................................................... 5500/9477
2020-01-06T22:07:03.7073952Z .................................................................................................... 5600/9477
2020-01-06T22:07:10.7021290Z .................i.................................................................................. 5700/9477
2020-01-06T22:07:16.5824473Z .................................................................................................... 5800/9477
2020-01-06T22:07:16.5824473Z .................................................................................................... 5800/9477
2020-01-06T22:07:27.3376672Z .................................................................................................... 5900/9477
2020-01-06T22:07:38.4563025Z .......ii...i..ii...........i....................................................................... 6000/9477
2020-01-06T22:07:55.1643102Z .................................................................................................... 6200/9477
2020-01-06T22:08:02.5723440Z .................................................................................................... 6300/9477
2020-01-06T22:08:02.5723440Z .................................................................................................... 6300/9477
2020-01-06T22:08:17.5383530Z ..................................i..ii............................................................. 6400/9477
2020-01-06T22:08:37.0728820Z .................................................................................................... 6600/9477
2020-01-06T22:08:39.0831549Z .........i.......................................................................................... 6700/9477
2020-01-06T22:08:41.2987218Z .................................................................................................... 6800/9477
2020-01-06T22:08:43.7811219Z .........i.......................................................................................... 6900/9477
---
2020-01-06T22:10:17.4105826Z .................................................................................................... 7500/9477
2020-01-06T22:10:21.3683210Z .................................................................................................... 7600/9477
2020-01-06T22:10:26.4033624Z .................................................................................................... 7700/9477
2020-01-06T22:10:36.6516838Z .................................................................................................... 7800/9477
2020-01-06T22:10:44.4921487Z .............................................iiii................................................... 7900/9477
2020-01-06T22:10:58.7263819Z .................................................................................................... 8100/9477
2020-01-06T22:11:05.1090401Z .................................................................................................... 8200/9477
2020-01-06T22:11:19.6581710Z .................................................................................................... 8300/9477
2020-01-06T22:11:27.0022151Z .................................................................................................... 8400/9477
---
2020-01-06T22:13:43.3825422Z  finished in 6.564
2020-01-06T22:13:43.4014188Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:13:43.5596653Z 
2020-01-06T22:13:43.5597196Z running 166 tests
2020-01-06T22:13:46.4898133Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-06T22:13:48.6767672Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-06T22:13:48.6768292Z 
2020-01-06T22:13:48.6768400Z  finished in 5.275
2020-01-06T22:13:48.6948267Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:13:48.8497900Z 
---
2020-01-06T22:13:50.7247143Z  finished in 2.030
2020-01-06T22:13:50.7445654Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:13:50.8981311Z 
2020-01-06T22:13:50.8981972Z running 9 tests
2020-01-06T22:13:50.8982812Z iiiiiiiii
2020-01-06T22:13:50.8983349Z 
2020-01-06T22:13:50.8983396Z  finished in 0.153
2020-01-06T22:13:50.9164871Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:13:51.0703330Z 
---
2020-01-06T22:14:10.4261799Z  finished in 19.509
2020-01-06T22:14:10.4440023Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:14:10.5961733Z 
2020-01-06T22:14:10.5961939Z running 124 tests
2020-01-06T22:14:33.4562617Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-06T22:14:37.2919109Z .i.iii.....iiiiii.....ii
2020-01-06T22:14:37.2920534Z 
2020-01-06T22:14:37.2921909Z  finished in 26.847
2020-01-06T22:14:37.2928396Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T22:14:37.2928720Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-06T22:15:16.7433687Z + 
2020-01-06T22:15:16.7433813Z + error: cannot determine resolution for the macro `impl_lint_pass`
2020-01-06T22:15:16.7434498Z +   --> $DIR/lint_pass_impl_without_macro.rs:43:1
2020-01-06T22:15:16.7434680Z +    |
2020-01-06T22:15:16.7434806Z + LL | impl_lint_pass!(Bar => [TEST_LINT]);
2020-01-06T22:15:16.7435066Z +    |
2020-01-06T22:15:16.7435188Z +    = note: import resolution is stuck, try simplifying macro imports
2020-01-06T22:15:16.7435325Z + 
2020-01-06T22:15:16.7435464Z + error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-06T22:15:16.7435464Z + error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-06T22:15:16.7435850Z +   --> $DIR/lint_pass_impl_without_macro.rs:45:1
2020-01-06T22:15:16.7436016Z +    |
2020-01-06T22:15:16.7436141Z + LL | declare_lint_pass!(Baz => [TEST_LINT]);
2020-01-06T22:15:16.7436414Z +    |
2020-01-06T22:15:16.7436538Z +    = note: import resolution is stuck, try simplifying macro imports
2020-01-06T22:15:16.7436670Z + 
2020-01-06T22:15:16.7437234Z 1 error: implementing `LintPass` by hand
2020-01-06T22:15:16.7437234Z 1 error: implementing `LintPass` by hand
2020-01-06T22:15:16.7437727Z 2   --> $DIR/lint_pass_impl_without_macro.rs:21:6
2020-01-06T22:15:16.7437923Z 3    |
2020-01-06T22:15:16.7438040Z 
2020-01-06T22:15:16.7438167Z 22    |
2020-01-06T22:15:16.7438297Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T22:15:16.7438831Z - error: aborting due to 2 previous errors
2020-01-06T22:15:16.7439029Z + error: aborting due to 5 previous errors
2020-01-06T22:15:16.7439407Z 26 
2020-01-06T22:15:16.7439880Z + For more information about this error, try `rustc --explain E0432`.
2020-01-06T22:15:16.7439880Z + For more information about this error, try `rustc --explain E0432`.
2020-01-06T22:15:16.7440213Z 27 
2020-01-06T22:15:16.7440319Z 
2020-01-06T22:15:16.7440419Z 
2020-01-06T22:15:16.7440558Z The actual stderr differed from the expected stderr.
2020-01-06T22:15:16.7441032Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2020-01-06T22:15:16.7441450Z To update references, rerun the tests and pass the `--bless` flag
2020-01-06T22:15:16.7441994Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2020-01-06T22:15:16.7442300Z error: 1 errors occurred comparing output.
2020-01-06T22:15:16.7442416Z status: exit code: 1
2020-01-06T22:15:16.7442416Z status: exit code: 1
2020-01-06T22:15:16.7443381Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2020-01-06T22:15:16.7443937Z ------------------------------------------
2020-01-06T22:15:16.7444071Z 
2020-01-06T22:15:16.7444397Z ------------------------------------------
2020-01-06T22:15:16.7444544Z stderr:
---
2020-01-06T22:15:16.7446394Z 
2020-01-06T22:15:16.7446627Z error: cannot determine resolution for the macro `impl_lint_pass`
2020-01-06T22:15:16.7447548Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:43:1
2020-01-06T22:15:16.7447801Z    |
2020-01-06T22:15:16.7447959Z LL | impl_lint_pass!(Bar => [TEST_LINT]);
2020-01-06T22:15:16.7448211Z    |
2020-01-06T22:15:16.7448359Z    = note: import resolution is stuck, try simplifying macro imports
2020-01-06T22:15:16.7448472Z 
2020-01-06T22:15:16.7448614Z error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-06T22:15:16.7448614Z error: cannot determine resolution for the macro `declare_lint_pass`
2020-01-06T22:15:16.7449062Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:45:1
2020-01-06T22:15:16.7449248Z    |
2020-01-06T22:15:16.7449400Z LL | declare_lint_pass!(Baz => [TEST_LINT]);
2020-01-06T22:15:16.7449654Z    |
2020-01-06T22:15:16.7449799Z    = note: import resolution is stuck, try simplifying macro imports
2020-01-06T22:15:16.7449928Z 
2020-01-06T22:15:16.7450077Z error: implementing `LintPass` by hand
2020-01-06T22:15:16.7450077Z error: implementing `LintPass` by hand
2020-01-06T22:15:16.7451002Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:21:6
2020-01-06T22:15:16.7451172Z    |
2020-01-06T22:15:16.7451318Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2020-01-06T22:15:16.7451558Z    |
2020-01-06T22:15:16.7451689Z note: lint level defined here
2020-01-06T22:15:16.7452082Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2020-01-06T22:15:16.7452652Z    |
2020-01-06T22:15:16.7452652Z    |
2020-01-06T22:15:16.7452882Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2020-01-06T22:15:16.7453043Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-06T22:15:16.7453174Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T22:15:16.7453421Z error: implementing `LintPass` by hand
2020-01-06T22:15:16.7453860Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:31:14
2020-01-06T22:15:16.7454058Z    |
2020-01-06T22:15:16.7454058Z    |
2020-01-06T22:15:16.7454214Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2020-01-06T22:15:16.7454462Z ...
2020-01-06T22:15:16.7454462Z ...
2020-01-06T22:15:16.7454616Z LL | custom_lint_pass_macro!();
2020-01-06T22:15:16.7455156Z    |
2020-01-06T22:15:16.7455156Z    |
2020-01-06T22:15:16.7455422Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2020-01-06T22:15:16.7455673Z error: aborting due to 5 previous errors
2020-01-06T22:15:16.7455793Z 
2020-01-06T22:15:16.7456153Z For more information about this error, try `rustc --explain E0432`.
2020-01-06T22:15:16.7456302Z 
---
2020-01-06T22:15:16.7464745Z test result: FAILED. 63 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-06T22:15:16.7464909Z 
2020-01-06T22:15:16.7465019Z 
2020-01-06T22:15:16.7465118Z 
2020-01-06T22:15:16.7467349Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-06T22:15:16.7527525Z 
2020-01-06T22:15:16.7527655Z 
2020-01-06T22:15:16.7528059Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-06T22:15:16.7528425Z Build completed unsuccessfully in 0:58:50
2020-01-06T22:15:16.7528425Z Build completed unsuccessfully in 0:58:50
2020-01-06T22:15:16.7532819Z == clock drift check ==
2020-01-06T22:15:16.7552336Z   local time: Mon Jan  6 22:15:16 UTC 2020
2020-01-06T22:15:17.0475311Z   network time: Mon, 06 Jan 2020 22:15:17 GMT
2020-01-06T22:15:17.0475920Z == end clock drift check ==
2020-01-06T22:15:18.4592608Z 
2020-01-06T22:15:18.4685863Z ##[error]Bash exited with code '1'.
2020-01-06T22:15:18.4755165Z ##[section]Starting: Checkout
2020-01-06T22:15:18.4756967Z ==============================================================================
2020-01-06T22:15:18.4757647Z Task         : Get sources
2020-01-06T22:15:18.4757698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
