plain
2019-12-20T20:14:57.1426623Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-20T20:14:57.1627709Z ##[command]git config gc.auto 0
2019-12-20T20:14:57.1698673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-20T20:14:57.1761286Z ##[command]git config --get-all http.proxy
2019-12-20T20:14:57.1914756Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67160/merge:refs/remotes/pull/67160/merge
---
2019-12-20T21:16:16.3779229Z .................................................................................................... 1600/9410
2019-12-20T21:16:21.2300778Z .................................................................................................... 1700/9410
2019-12-20T21:16:33.1222159Z ................................................................................i................... 1800/9410
2019-12-20T21:16:40.1431238Z .................................................................................................... 1900/9410
2019-12-20T21:16:49.0310599Z .................................................................iiiii.............................. 2000/9410
2019-12-20T21:17:08.3687174Z .................................................................................................... 2200/9410
2019-12-20T21:17:10.8998010Z .................................................................................................... 2300/9410
2019-12-20T21:17:13.7769692Z .................................................................................................... 2400/9410
2019-12-20T21:17:35.3234889Z .................................................................................................... 2500/9410
---
2019-12-20T21:20:27.6111299Z ..........i......................................................................................... 4900/9410
2019-12-20T21:20:37.7884424Z .................................................................................................... 5000/9410
2019-12-20T21:20:42.9690192Z ......................................i............................................................. 5100/9410
2019-12-20T21:20:53.5415382Z .................................................................................................... 5200/9410
2019-12-20T21:20:59.2964501Z .....ii.ii...........i.............................................................................. 5300/9410
2019-12-20T21:21:09.4629821Z .................................................................................................... 5500/9410
2019-12-20T21:21:20.1931436Z .......................................................................................i............ 5600/9410
2019-12-20T21:21:28.4020613Z .................................................................................................... 5700/9410
2019-12-20T21:21:34.2193138Z .................................................................................................... 5800/9410
2019-12-20T21:21:34.2193138Z .................................................................................................... 5800/9410
2019-12-20T21:21:44.5950974Z ...........................................................................ii...i..ii...........i... 5900/9410
2019-12-20T21:22:07.4060082Z .................................................................................................... 6100/9410
2019-12-20T21:22:15.6450328Z .................................................................................................... 6200/9410
2019-12-20T21:22:24.3728552Z .................................................................................................... 6300/9410
2019-12-20T21:22:24.3728552Z .................................................................................................... 6300/9410
2019-12-20T21:22:40.3641250Z i..ii............................................................................................... 6400/9410
2019-12-20T21:22:59.6650777Z .........................................................................i.......................... 6600/9410
2019-12-20T21:23:01.9537463Z ..............F..................................................................................... 6700/9410
2019-12-20T21:23:04.2494965Z .......................................................................i............................ 6800/9410
2019-12-20T21:23:07.0243537Z .................................................................................................... 6900/9410
---
2019-12-20T21:24:43.6689838Z .................................................................................................... 7400/9410
2019-12-20T21:24:48.3942585Z .................................................................................................... 7500/9410
2019-12-20T21:24:54.2803556Z .................................................................................................... 7600/9410
2019-12-20T21:25:01.9228042Z .................................................................................................... 7700/9410
2019-12-20T21:25:12.1540786Z .................................................................................iiii............... 7800/9410
2019-12-20T21:25:28.9473174Z ............i......i................................................................................ 8000/9410
2019-12-20T21:25:34.2823349Z .................................................................................................... 8100/9410
2019-12-20T21:25:48.4524288Z .................................................................................................... 8200/9410
2019-12-20T21:25:58.4901026Z .................................................................................................... 8300/9410
---
2019-12-20T21:28:01.6087797Z 
2019-12-20T21:28:01.6089854Z ---- [ui] ui/parser/impl-item-type-no-body-semantic-fail.rs stdout ----
2019-12-20T21:28:01.6090198Z diff of stderr:
2019-12-20T21:28:01.6090334Z 
2019-12-20T21:28:01.6090486Z 68 LL |     type W: Ord where Self: Eq;
2019-12-20T21:28:01.6090802Z 70 
2019-12-20T21:28:01.6091520Z - error: aborting due to 9 previous errors
2019-12-20T21:28:01.6091744Z + error[E0202]: associated types are not yet supported in inherent impls (see #8995)
2019-12-20T21:28:01.6092112Z +   --> $DIR/impl-item-type-no-body-semantic-fail.rs:20:5
2019-12-20T21:28:01.6092112Z +   --> $DIR/impl-item-type-no-body-semantic-fail.rs:20:5
2019-12-20T21:28:01.6092290Z +    |
2019-12-20T21:28:01.6092461Z + LL |     type W where Self: Eq;
2019-12-20T21:28:01.6092948Z + 
2019-12-20T21:28:01.6093099Z + error: aborting due to 10 previous errors
2019-12-20T21:28:01.6093242Z 72 
2019-12-20T21:28:01.6093660Z 73 For more information about this error, try `rustc --explain E0202`.
2019-12-20T21:28:01.6093660Z 73 For more information about this error, try `rustc --explain E0202`.
2019-12-20T21:28:01.6093841Z 74 
2019-12-20T21:28:01.6093964Z 
2019-12-20T21:28:01.6094089Z 
2019-12-20T21:28:01.6094254Z The actual stderr differed from the expected stderr.
2019-12-20T21:28:01.6094707Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-item-type-no-body-semantic-fail/impl-item-type-no-body-semantic-fail.stderr
2019-12-20T21:28:01.6095137Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T21:28:01.6095567Z To only update this specific test, also pass `--test-args parser/impl-item-type-no-body-semantic-fail.rs`
2019-12-20T21:28:01.6096368Z error: 1 errors occurred comparing output.
2019-12-20T21:28:01.6096550Z status: exit code: 1
2019-12-20T21:28:01.6096550Z status: exit code: 1
2019-12-20T21:28:01.6097673Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-item-type-no-body-semantic-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-item-type-no-body-semantic-fail/auxiliary" "-A" "unused"
2019-12-20T21:28:01.6098340Z ------------------------------------------
2019-12-20T21:28:01.6098516Z 
2019-12-20T21:28:01.6099616Z ------------------------------------------
2019-12-20T21:28:01.6100247Z stderr:
2019-12-20T21:28:01.6100247Z stderr:
2019-12-20T21:28:01.6100677Z ------------------------------------------
2019-12-20T21:28:01.6100894Z error: associated type in `impl` without body
2019-12-20T21:28:01.6101275Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:9:5
2019-12-20T21:28:01.6101454Z    |
2019-12-20T21:28:01.6101625Z LL |     type Y;
2019-12-20T21:28:01.6101928Z    |     ^^^^^^-
2019-12-20T21:28:01.6102097Z    |           |
2019-12-20T21:28:01.6102266Z    |           help: provide a definition for the type: `= <type>;`
2019-12-20T21:28:01.6102542Z error: associated type in `impl` without body
2019-12-20T21:28:01.6102933Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:12:5
2019-12-20T21:28:01.6103126Z    |
2019-12-20T21:28:01.6103126Z    |
2019-12-20T21:28:01.6103266Z LL |     type Z: Ord;
2019-12-20T21:28:01.6103771Z    |                |
2019-12-20T21:28:01.6103771Z    |                |
2019-12-20T21:28:01.6103917Z    |                help: provide a definition for the type: `= <type>;`
2019-12-20T21:28:01.6104216Z error: bounds on associated `type`s in `impl`s have no effect
2019-12-20T21:28:01.6104591Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:12:13
2019-12-20T21:28:01.6104788Z    |
2019-12-20T21:28:01.6104788Z    |
2019-12-20T21:28:01.6104928Z LL |     type Z: Ord;
2019-12-20T21:28:01.6105208Z 
2019-12-20T21:28:01.6105347Z error: associated type in `impl` without body
2019-12-20T21:28:01.6105713Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:16:5
2019-12-20T21:28:01.6105909Z    |
2019-12-20T21:28:01.6105909Z    |
2019-12-20T21:28:01.6106562Z LL |     type W: Ord where Self: Eq;
2019-12-20T21:28:01.6107417Z    |                               |
2019-12-20T21:28:01.6107417Z    |                               |
2019-12-20T21:28:01.6107584Z    |                               help: provide a definition for the type: `= <type>;`
2019-12-20T21:28:01.6107909Z error: bounds on associated `type`s in `impl`s have no effect
2019-12-20T21:28:01.6108431Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:16:13
2019-12-20T21:28:01.6108705Z    |
2019-12-20T21:28:01.6108705Z    |
2019-12-20T21:28:01.6108978Z LL |     type W: Ord where Self: Eq;
2019-12-20T21:28:01.6109429Z 
2019-12-20T21:28:01.6111221Z error: associated type in `impl` without body
2019-12-20T21:28:01.6111601Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:20:5
2019-12-20T21:28:01.6111668Z    |
2019-12-20T21:28:01.6111668Z    |
2019-12-20T21:28:01.6111706Z LL |     type W where Self: Eq;
2019-12-20T21:28:01.6111941Z    |                          |
2019-12-20T21:28:01.6111941Z    |                          |
2019-12-20T21:28:01.6112018Z    |                          help: provide a definition for the type: `= <type>;`
2019-12-20T21:28:01.6112095Z warning: the feature `generic_associated_types` is incomplete and may cause the compiler to crash
2019-12-20T21:28:01.6112362Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:1:12
2019-12-20T21:28:01.6112407Z    |
2019-12-20T21:28:01.6112447Z LL | #![feature(generic_associated_types)]
---
2019-12-20T21:28:01.6113081Z 
2019-12-20T21:28:01.6113125Z error[E0202]: associated types are not yet supported in inherent impls (see #8995)
2019-12-20T21:28:01.6113380Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:12:5
2019-12-20T21:28:01.6113424Z    |
2019-12-20T21:28:01.6113461Z LL |     type Z: Ord;
2019-12-20T21:28:01.6113550Z 
2019-12-20T21:28:01.6113594Z error[E0202]: associated types are not yet supported in inherent impls (see #8995)
2019-12-20T21:28:01.6113829Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:16:5
2019-12-20T21:28:01.6113890Z    |
2019-12-20T21:28:01.6113890Z    |
2019-12-20T21:28:01.6113929Z LL |     type W: Ord where Self: Eq;
2019-12-20T21:28:01.6114015Z 
2019-12-20T21:28:01.6114057Z error[E0202]: associated types are not yet supported in inherent impls (see #8995)
2019-12-20T21:28:01.6114302Z   --> /checkout/src/test/ui/parser/impl-item-type-no-body-semantic-fail.rs:20:5
2019-12-20T21:28:01.6114362Z    |
2019-12-20T21:28:01.6114362Z    |
2019-12-20T21:28:01.6114400Z LL |     type W where Self: Eq;
2019-12-20T21:28:01.6114464Z 
2019-12-20T21:28:01.6114521Z error: aborting due to 10 previous errors
2019-12-20T21:28:01.6114548Z 
2019-12-20T21:28:01.6114777Z For more information about this error, try `rustc --explain E0202`.
---
2019-12-20T21:28:01.6123346Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-20T21:28:01.6123571Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-20T21:28:01.6144413Z 
2019-12-20T21:28:01.6144526Z 
2019-12-20T21:28:01.6147057Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-20T21:28:01.6147373Z 
2019-12-20T21:28:01.6147403Z 
2019-12-20T21:28:01.6155463Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-20T21:28:01.6155567Z Build completed unsuccessfully in 1:06:43
2019-12-20T21:28:01.6155567Z Build completed unsuccessfully in 1:06:43
2019-12-20T21:28:01.6214533Z == clock drift check ==
2019-12-20T21:28:01.6241157Z   local time: Fri Dec 20 21:28:01 UTC 2019
2019-12-20T21:28:02.1710293Z   network time: Fri, 20 Dec 2019 21:28:02 GMT
2019-12-20T21:28:02.1714685Z == end clock drift check ==
2019-12-20T21:28:03.5040943Z 
2019-12-20T21:28:03.5177713Z ##[error]Bash exited with code '1'.
2019-12-20T21:28:03.5221227Z ##[section]Starting: Checkout
2019-12-20T21:28:03.5223540Z ==============================================================================
2019-12-20T21:28:03.5223597Z Task         : Get sources
2019-12-20T21:28:03.5223643Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
