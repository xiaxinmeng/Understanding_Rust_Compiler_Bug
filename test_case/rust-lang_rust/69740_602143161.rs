plain
2020-03-22T02:33:12.8400260Z ========================== Starting Command Output ===========================
2020-03-22T02:33:12.8417687Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6d68d504-c5bf-4374-8f4a-fc0995647c6c.sh
2020-03-22T02:33:12.8578747Z 
2020-03-22T02:33:12.8635558Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T02:33:12.8655456Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T02:33:12.8659933Z Task         : Get sources
2020-03-22T02:33:12.8660385Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T02:33:12.8660671Z Version      : 1.0.0
2020-03-22T02:33:12.8660837Z Author       : Microsoft
---
2020-03-22T02:33:13.6519663Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T02:33:13.6525319Z ##[command]git config gc.auto 0
2020-03-22T02:33:13.6529932Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T02:33:13.6533734Z ##[command]git config --get-all http.proxy
2020-03-22T02:33:13.6540651Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69740/merge:refs/remotes/pull/69740/merge
---
2020-03-22T03:29:52.9210012Z .................................................................................................... 1700/9818
2020-03-22T03:29:56.8172422Z .................................................................................................... 1800/9818
2020-03-22T03:30:07.1074442Z ..............................................................................i..................... 1900/9818
2020-03-22T03:30:13.1486456Z .................................................................................................... 2000/9818
2020-03-22T03:30:20.2350016Z ....................................................................iiiii........................... 2100/9818
2020-03-22T03:30:38.8470037Z .................................................................................................... 2300/9818
2020-03-22T03:30:40.9511766Z .................................................................................................... 2400/9818
2020-03-22T03:30:43.4912939Z .................................................................................................... 2500/9818
2020-03-22T03:30:59.1296572Z .................................................................................................... 2600/9818
---
2020-03-22T03:33:34.1977998Z ..........................................i...............i......................................... 5000/9818
2020-03-22T03:33:42.5352149Z .................................................................................................... 5100/9818
2020-03-22T03:33:48.6811701Z ......................................................................................i............. 5200/9818
2020-03-22T03:33:54.4632106Z .................................................................................................... 5300/9818
2020-03-22T03:34:04.5566847Z .....................................................................ii.ii........i...i............. 5400/9818
2020-03-22T03:34:12.7204957Z .........i.......................................................................................... 5600/9818
2020-03-22T03:34:21.7771903Z ..............i.F................................................................................... 5700/9818
2020-03-22T03:34:27.9927941Z ...............................ii...................................i............................... 5800/9818
2020-03-22T03:34:34.8223260Z .................................................................................................... 5900/9818
2020-03-22T03:34:34.8223260Z .................................................................................................... 5900/9818
2020-03-22T03:34:41.3694344Z .................................................................................................... 6000/9818
2020-03-22T03:34:50.6589465Z ..............................................................ii...i..ii...........i................ 6100/9818
2020-03-22T03:35:10.1224385Z .................................................................................................... 6300/9818
2020-03-22T03:35:17.0201943Z .................................................................................................... 6400/9818
2020-03-22T03:35:17.0201943Z .................................................................................................... 6400/9818
2020-03-22T03:35:23.8306355Z ............................................................................................i..ii... 6500/9818
2020-03-22T03:35:45.0404246Z .................................................................................................... 6700/9818
2020-03-22T03:35:55.0177439Z ...........................................................................................i........ 6800/9818
2020-03-22T03:35:56.9538011Z .................................................................................................... 6900/9818
2020-03-22T03:35:58.9769152Z .................................................................................................... 7000/9818
---
2020-03-22T03:37:38.6943749Z .................................................................................................... 7800/9818
2020-03-22T03:37:43.2913959Z .................................................................................................... 7900/9818
2020-03-22T03:37:49.1353181Z ................................................................................i................... 8000/9818
2020-03-22T03:37:58.2726335Z .................................................................................................... 8100/9818
2020-03-22T03:38:03.9306378Z .............................iiiiiiiiii.i........................................................... 8200/9818
2020-03-22T03:38:17.2933400Z .................................................................................................... 8400/9818
2020-03-22T03:38:22.2697854Z .................................................................................................... 8500/9818
2020-03-22T03:38:36.9890947Z .................................................................................................... 8600/9818
2020-03-22T03:38:44.8114669Z .................................................................................................... 8700/9818
---
2020-03-22T03:40:36.9406330Z - error: missing documentation for crate
2020-03-22T03:40:36.9406727Z + error: missing documentation for a crate
2020-03-22T03:40:36.9407287Z 2   --> $DIR/issue-10656.rs:1:1
2020-03-22T03:40:36.9407604Z 3    |
2020-03-22T03:40:36.9407883Z 4 LL | / #![deny(missing_docs)]
2020-03-22T03:40:36.9408344Z 
2020-03-22T03:40:36.9408639Z The actual stderr differed from the expected stderr.
2020-03-22T03:40:36.9409393Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10656/issue-10656.stderr
2020-03-22T03:40:36.9409393Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10656/issue-10656.stderr
2020-03-22T03:40:36.9410127Z To update references, rerun the tests and pass the `--bless` flag
2020-03-22T03:40:36.9410816Z To only update this specific test, also pass `--test-args issues/issue-10656.rs`
2020-03-22T03:40:36.9411688Z error: 1 errors occurred comparing output.
2020-03-22T03:40:36.9412017Z status: exit code: 1
2020-03-22T03:40:36.9412017Z status: exit code: 1
2020-03-22T03:40:36.9414526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10656.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10656" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10656/auxiliary"
2020-03-22T03:40:36.9416261Z ------------------------------------------
2020-03-22T03:40:36.9416544Z 
2020-03-22T03:40:36.9417010Z ------------------------------------------
2020-03-22T03:40:36.9417335Z stderr:
2020-03-22T03:40:36.9417335Z stderr:
2020-03-22T03:40:36.9417782Z ------------------------------------------
2020-03-22T03:40:36.9418163Z error: missing documentation for a crate
2020-03-22T03:40:36.9418889Z   --> /checkout/src/test/ui/issues/issue-10656.rs:1:1
2020-03-22T03:40:36.9419223Z    |
2020-03-22T03:40:36.9419519Z LL | / #![deny(missing_docs)]
2020-03-22T03:40:36.9419816Z LL | | #![crate_type="lib"]
2020-03-22T03:40:36.9420374Z    |
2020-03-22T03:40:36.9420640Z note: the lint level is defined here
2020-03-22T03:40:36.9421209Z   --> /checkout/src/test/ui/issues/issue-10656.rs:1:9
2020-03-22T03:40:36.9421701Z    |
---
2020-03-22T03:40:36.9424495Z 
2020-03-22T03:40:36.9425099Z ---- [ui] ui/lint/lints-in-foreign-macros.rs stdout ----
2020-03-22T03:40:36.9425323Z diff of stderr:
2020-03-22T03:40:36.9425437Z 
2020-03-22T03:40:36.9425657Z 26 LL | mod d { baz2!(use std::string::ToString;); }
2020-03-22T03:40:36.9426114Z 28 
2020-03-22T03:40:36.9426464Z - warning: missing documentation for crate
2020-03-22T03:40:36.9426711Z + warning: missing documentation for a crate
2020-03-22T03:40:36.9427265Z 30   --> $DIR/lints-in-foreign-macros.rs:4:1
2020-03-22T03:40:36.9427265Z 30   --> $DIR/lints-in-foreign-macros.rs:4:1
2020-03-22T03:40:36.9427472Z 31    |
2020-03-22T03:40:36.9427636Z 32 LL | / #![warn(unused_imports)]
2020-03-22T03:40:36.9427770Z 
2020-03-22T03:40:36.9427853Z 
2020-03-22T03:40:36.9428045Z The actual stderr differed from the expected stderr.
2020-03-22T03:40:36.9428830Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/lints-in-foreign-macros.stderr
2020-03-22T03:40:36.9429405Z To update references, rerun the tests and pass the `--bless` flag
2020-03-22T03:40:36.9429948Z To only update this specific test, also pass `--test-args lint/lints-in-foreign-macros.rs`
2020-03-22T03:40:36.9430349Z error: 1 errors occurred comparing output.
2020-03-22T03:40:36.9430574Z status: exit code: 0
2020-03-22T03:40:36.9430574Z status: exit code: 0
2020-03-22T03:40:36.9432437Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lints-in-foreign-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/auxiliary"
2020-03-22T03:40:36.9434389Z ------------------------------------------
2020-03-22T03:40:36.9434550Z 
2020-03-22T03:40:36.9434884Z ------------------------------------------
2020-03-22T03:40:36.9435069Z stderr:
2020-03-22T03:40:36.9435069Z stderr:
2020-03-22T03:40:36.9435466Z ------------------------------------------
2020-03-22T03:40:36.9435739Z warning: unused import: `std::string::ToString`
2020-03-22T03:40:36.9436266Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:11:16
2020-03-22T03:40:36.9436500Z    |
2020-03-22T03:40:36.9436773Z LL |     () => {use std::string::ToString;} //~ WARN: unused import
2020-03-22T03:40:36.9437400Z ...
2020-03-22T03:40:36.9437541Z LL | mod a { foo!(); }
2020-03-22T03:40:36.9437918Z    |         ------- in this macro invocation
2020-03-22T03:40:36.9438097Z    |
2020-03-22T03:40:36.9438097Z    |
2020-03-22T03:40:36.9438262Z note: the lint level is defined here
2020-03-22T03:40:36.9438904Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:4:9
2020-03-22T03:40:36.9439136Z    |
2020-03-22T03:40:36.9439554Z LL | #![warn(unused_imports)] //~ missing documentation for crate [missing_docs]
2020-03-22T03:40:36.9440355Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-22T03:40:36.9440791Z 
2020-03-22T03:40:36.9441008Z warning: unused import: `std::string::ToString`
2020-03-22T03:40:36.9441839Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:16:18
2020-03-22T03:40:36.9441839Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:16:18
2020-03-22T03:40:36.9442069Z    |
2020-03-22T03:40:36.9442340Z LL | mod c { baz!(use std::string::ToString;); } //~ WARN: unused import
2020-03-22T03:40:36.9442808Z 
2020-03-22T03:40:36.9443003Z warning: unused import: `std::string::ToString`
2020-03-22T03:40:36.9443510Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:17:19
2020-03-22T03:40:36.9443744Z    |
2020-03-22T03:40:36.9443744Z    |
2020-03-22T03:40:36.9444000Z LL | mod d { baz2!(use std::string::ToString;); } //~ WARN: unused import
2020-03-22T03:40:36.9444633Z 
2020-03-22T03:40:36.9444801Z warning: missing documentation for a crate
2020-03-22T03:40:36.9445267Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:4:1
2020-03-22T03:40:36.9445487Z    |
2020-03-22T03:40:36.9445487Z    |
2020-03-22T03:40:36.9445724Z LL | / #![warn(unused_imports)] //~ missing documentation for crate [missing_docs]
2020-03-22T03:40:36.9446016Z LL | | #![warn(missing_docs)]
2020-03-22T03:40:36.9446472Z LL | | #[macro_use]
2020-03-22T03:40:36.9446608Z ...  |
2020-03-22T03:40:36.9446740Z LL | |
2020-03-22T03:40:36.9446875Z LL | | fn main() {}
---
2020-03-22T03:40:36.9448620Z 
2020-03-22T03:40:36.9448810Z warning: missing documentation for a function
2020-03-22T03:40:36.9449270Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:18:6
2020-03-22T03:40:36.9449492Z    |
2020-03-22T03:40:36.9449765Z LL | baz!(pub fn undocumented() {}); //~ WARN: missing documentation for a function
2020-03-22T03:40:36.9450196Z 
2020-03-22T03:40:36.9450369Z warning: missing documentation for a function
2020-03-22T03:40:36.9450843Z   --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:19:7
2020-03-22T03:40:36.9451063Z    |
2020-03-22T03:40:36.9451063Z    |
2020-03-22T03:40:36.9451323Z LL | baz2!(pub fn undocumented2() {}); //~ WARN: missing documentation for a function
2020-03-22T03:40:36.9451930Z 
2020-03-22T03:40:36.9452092Z 
2020-03-22T03:40:36.9452419Z ------------------------------------------
2020-03-22T03:40:36.9453373Z 
---
2020-03-22T03:40:36.9466041Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-22T03:40:36.9466441Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-22T03:40:36.9466647Z 
2020-03-22T03:40:36.9466731Z 
2020-03-22T03:40:36.9470645Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-22T03:40:36.9473501Z 
2020-03-22T03:40:36.9473585Z 
2020-03-22T03:40:36.9474041Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-22T03:40:36.9474360Z Build completed unsuccessfully in 1:02:01
2020-03-22T03:40:36.9474360Z Build completed unsuccessfully in 1:02:01
2020-03-22T03:40:36.9492118Z == clock drift check ==
2020-03-22T03:40:36.9512341Z   local time: Sun Mar 22 03:40:36 UTC 2020
2020-03-22T03:40:37.0996141Z   network time: Sun, 22 Mar 2020 03:40:37 GMT
2020-03-22T03:40:37.0998029Z == end clock drift check ==
2020-03-22T03:40:37.6328311Z 
2020-03-22T03:40:37.6403680Z ##[error]Bash exited with code '1'.
2020-03-22T03:40:37.6419170Z ##[section]Finishing: Run build
2020-03-22T03:40:37.6468152Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T03:40:37.6473036Z Task         : Get sources
2020-03-22T03:40:37.6473342Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T03:40:37.6473621Z Version      : 1.0.0
2020-03-22T03:40:37.6473850Z Author       : Microsoft
2020-03-22T03:40:37.6473850Z Author       : Microsoft
2020-03-22T03:40:37.6474160Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T03:40:37.6474668Z ==============================================================================
2020-03-22T03:40:37.9827077Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T03:40:37.9868778Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T03:40:37.9960767Z Cleaning up task key
2020-03-22T03:40:37.9961839Z Start cleaning up orphan processes.
2020-03-22T03:40:38.0157702Z Terminate orphan process: pid (3511) (python)
2020-03-22T03:40:38.0313766Z ##[section]Finishing: Finalize Job
