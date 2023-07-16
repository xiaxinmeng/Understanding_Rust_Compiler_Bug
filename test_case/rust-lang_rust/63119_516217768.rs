plain
2019-07-29T23:46:07.7718438Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T23:46:07.7941101Z ##[command]git config gc.auto 0
2019-07-29T23:46:07.8065673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T23:46:07.8161508Z ##[command]git config --get-all http.proxy
2019-07-29T23:46:07.8298232Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63119/merge:refs/remotes/pull/63119/merge
---
2019-07-29T23:46:43.0619859Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T23:46:43.0619896Z 
2019-07-29T23:46:43.0620143Z   git checkout -b <new-branch-name>
2019-07-29T23:46:43.0620397Z 
2019-07-29T23:46:43.0620474Z HEAD is now at d76835390 Merge f9cf2d67a03b3cbd2b14bf2f27fe4f03ac9d8763 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T23:46:43.0832281Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T23:46:43.0835919Z ==============================================================================
2019-07-29T23:46:43.0835994Z Task         : Bash
2019-07-29T23:46:43.0836041Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T00:50:43.3901616Z .................................................................................................... 1400/8812
2019-07-30T00:50:49.6045327Z .................................................................................................... 1500/8812
2019-07-30T00:51:02.9312063Z ..................................................................i...............i................. 1600/8812
2019-07-30T00:51:10.8212885Z .................................................................................................... 1700/8812
2019-07-30T00:51:26.4473800Z ....................................................iiiii........................................... 1800/8812
2019-07-30T00:51:38.4130941Z .................................................................................................... 2000/8812
2019-07-30T00:51:41.0677618Z .................................................................................................... 2100/8812
2019-07-30T00:51:44.8615481Z .................................................................................................... 2200/8812
2019-07-30T00:51:51.7180455Z .................................................................................................... 2300/8812
---
2019-07-30T00:56:00.0043876Z .................................................................................................... 5300/8812
2019-07-30T00:56:07.7270549Z ..........i......................................................................................... 5400/8812
2019-07-30T00:56:13.6419945Z .................................................................................................... 5500/8812
2019-07-30T00:56:26.5531987Z .................................................................................................... 5600/8812
2019-07-30T00:56:41.2768020Z ....ii...i..ii...........i.......................................................................... 5700/8812
2019-07-30T00:56:56.6727147Z .................................................................................................... 5900/8812
2019-07-30T00:57:01.6647229Z .................................................................................................... 6000/8812
2019-07-30T00:57:01.6647229Z .................................................................................................... 6000/8812
2019-07-30T00:57:16.8146875Z .....i..ii.......................................................................................... 6100/8812
2019-07-30T00:57:36.7338956Z ................................................i................................................... 6300/8812
2019-07-30T00:57:38.9972030Z .................................................................................................... 6400/8812
2019-07-30T00:57:41.6534176Z ..................i................................................................................. 6500/8812
2019-07-30T00:57:46.4573627Z .................................................................................................... 6600/8812
---
2019-07-30T01:01:58.1348815Z ---- [ui] ui/hygiene/generic_params.rs stdout ----
2019-07-30T01:01:58.1350580Z diff of stderr:
2019-07-30T01:01:58.1350632Z 
2019-07-30T01:01:58.1350675Z 3    |
2019-07-30T01:01:58.1350743Z 4 LL | #![feature(decl_macro, rustc_attrs, const_generics)]
2019-07-30T01:01:58.1351015Z +    |
2019-07-30T01:01:58.1351075Z +    = note: `#[warn(incomplete_features)]` on by default
2019-07-30T01:01:58.1351118Z 6 
2019-07-30T01:01:58.1351156Z 7 
2019-07-30T01:01:58.1351156Z 7 
2019-07-30T01:01:58.1351291Z 
2019-07-30T01:01:58.1351332Z 
2019-07-30T01:01:58.1351376Z The actual stderr differed from the expected stderr.
2019-07-30T01:01:58.1351988Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/generic_params/generic_params.stderr
2019-07-30T01:01:58.1352394Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T01:01:58.1352656Z To only update this specific test, also pass `--test-args hygiene/generic_params.rs`
2019-07-30T01:01:58.1352735Z error: 1 errors occurred comparing output.
2019-07-30T01:01:58.1352793Z status: exit code: 0
2019-07-30T01:01:58.1352793Z status: exit code: 0
2019-07-30T01:01:58.1353827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/generic_params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/generic_params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/generic_params/auxiliary" "-A" "unused"
2019-07-30T01:01:58.1354356Z ------------------------------------------
2019-07-30T01:01:58.1354393Z 
2019-07-30T01:01:58.1354642Z ------------------------------------------
2019-07-30T01:01:58.1354687Z stderr:
2019-07-30T01:01:58.1354687Z stderr:
2019-07-30T01:01:58.1354909Z ------------------------------------------
2019-07-30T01:01:58.1354978Z warning: the feature `const_generics` is incomplete and may cause the compiler to crash
2019-07-30T01:01:58.1355230Z   --> /checkout/src/test/ui/hygiene/generic_params.rs:6:37
2019-07-30T01:01:58.1355292Z    |
2019-07-30T01:01:58.1355360Z LL | #![feature(decl_macro, rustc_attrs, const_generics)]
2019-07-30T01:01:58.1355450Z    |
2019-07-30T01:01:58.1355513Z    = note: `#[warn(incomplete_features)]` on by default
2019-07-30T01:01:58.1355543Z 
2019-07-30T01:01:58.1355568Z 
---
2019-07-30T01:01:58.1356575Z 
2019-07-30T01:01:58.1356607Z 
2019-07-30T01:01:58.1356650Z The actual stderr differed from the expected stderr.
2019-07-30T01:01:58.1357356Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/issue-61574-const-parameters/issue-61574-const-parameters.stderr
2019-07-30T01:01:58.1357658Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T01:01:58.1357949Z To only update this specific test, also pass `--test-args hygiene/issue-61574-const-parameters.rs`
2019-07-30T01:01:58.1358049Z error: 1 errors occurred comparing output.
2019-07-30T01:01:58.1358092Z status: exit code: 0
2019-07-30T01:01:58.1358092Z status: exit code: 0
2019-07-30T01:01:58.1358902Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/issue-61574-const-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/issue-61574-const-parameters" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/issue-61574-const-parameters/auxiliary" "-A" "unused"
2019-07-30T01:01:58.1359250Z ------------------------------------------
2019-07-30T01:01:58.1359285Z 
2019-07-30T01:01:58.1359511Z ------------------------------------------
2019-07-30T01:01:58.1359556Z stderr:
---
2019-07-30T01:01:58.1385233Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-30T01:01:58.1385311Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-30T01:01:58.1410037Z 
2019-07-30T01:01:58.1410112Z 
2019-07-30T01:01:58.1411796Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-30T01:01:58.1412087Z 
2019-07-30T01:01:58.1412115Z 
2019-07-30T01:01:58.1412161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T01:01:58.1412231Z Build completed unsuccessfully in 1:08:47
2019-07-30T01:01:58.1412231Z Build completed unsuccessfully in 1:08:47
2019-07-30T01:01:59.1584292Z ##[error]Bash exited with code '1'.
2019-07-30T01:01:59.1647460Z ##[section]Starting: Checkout
2019-07-30T01:01:59.1649356Z ==============================================================================
2019-07-30T01:01:59.1649434Z Task         : Get sources
2019-07-30T01:01:59.1649486Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
