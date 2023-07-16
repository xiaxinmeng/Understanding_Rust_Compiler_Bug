plain
2019-08-06T11:06:00.3721334Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-06T11:06:00.3892010Z ##[command]git config gc.auto 0
2019-08-06T11:06:00.3958235Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-06T11:06:00.4004364Z ##[command]git config --get-all http.proxy
2019-08-06T11:06:00.4157777Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63259/merge:refs/remotes/pull/63259/merge
---
2019-08-06T11:06:36.9782160Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T11:06:36.9782195Z 
2019-08-06T11:06:36.9782479Z   git checkout -b <new-branch-name>
2019-08-06T11:06:36.9782513Z 
2019-08-06T11:06:36.9782568Z HEAD is now at affea59ac Merge de827b202fd196e5ae5b8a79be6de4b17597b250 into 766b10a8d544550712fd6352863457a86f46db3c
2019-08-06T11:06:36.9936628Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T11:06:36.9940128Z ==============================================================================
2019-08-06T11:06:36.9940215Z Task         : Bash
2019-08-06T11:06:36.9940267Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T12:09:30.1682064Z .................................................................................................... 1400/8832
2019-08-06T12:09:36.3603313Z .................................................................................................... 1500/8832
2019-08-06T12:09:49.0890316Z ......................................................................i...............i............. 1600/8832
2019-08-06T12:09:56.7423733Z .................................................................................................... 1700/8832
2019-08-06T12:10:12.5866746Z ........................................................iiiii....................................... 1800/8832
2019-08-06T12:10:24.5517561Z .................................................................................................... 2000/8832
2019-08-06T12:10:27.1888663Z .................................................................................................... 2100/8832
2019-08-06T12:10:30.5928955Z .................................................................................................... 2200/8832
2019-08-06T12:10:38.7248357Z .................................................................................................... 2300/8832
---
2019-08-06T12:14:32.5424949Z .................................................................................................... 5200/8832
2019-08-06T12:14:43.5703401Z ..........................................................................i......................... 5300/8832
2019-08-06T12:14:51.0982629Z .................................................................................................... 5400/8832
2019-08-06T12:14:57.9336049Z .................................................................................................... 5500/8832
2019-08-06T12:15:09.6982443Z ....................................................................ii...i..ii...........i.......... 5600/8832
2019-08-06T12:15:32.9126580Z .................................................................................................... 5800/8832
2019-08-06T12:15:38.3020216Z .................................................................................................... 5900/8832
2019-08-06T12:15:38.3020216Z .................................................................................................... 5900/8832
2019-08-06T12:15:44.2700516Z .....................................................................i..ii.......................... 6000/8832
2019-08-06T12:16:14.6432720Z .................................................................................................... 6200/8832
2019-08-06T12:16:16.8828196Z ............i....................................................................................... 6300/8832
2019-08-06T12:16:19.0974727Z ....................................................................................i............... 6400/8832
2019-08-06T12:16:21.8364945Z .................................................................................................... 6500/8832
---
2019-08-06T12:20:32.6283800Z 1 error[E0669]: invalid value for constraint in inline assembly
2019-08-06T12:20:32.6284082Z -   --> $DIR/issue-37433.rs:5:24
2019-08-06T12:20:32.6284348Z +   --> $DIR/issue-37433.rs:7:24
2019-08-06T12:20:32.6284394Z 3    |
2019-08-06T12:20:32.6284439Z 4 LL |         asm!("" :: "r"(""));
2019-08-06T12:20:32.6284533Z 
2019-08-06T12:20:32.6284577Z 
2019-08-06T12:20:32.6284630Z The actual stderr differed from the expected stderr.
2019-08-06T12:20:32.6284958Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/issue-37433.stderr
2019-08-06T12:20:32.6284958Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/issue-37433.stderr
2019-08-06T12:20:32.6285255Z To update references, rerun the tests and pass the `--bless` flag
2019-08-06T12:20:32.6285545Z To only update this specific test, also pass `--test-args issues/issue-37433.rs`
2019-08-06T12:20:32.6285642Z error: 1 errors occurred comparing output.
2019-08-06T12:20:32.6285687Z status: exit code: 1
2019-08-06T12:20:32.6285687Z status: exit code: 1
2019-08-06T12:20:32.6286714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37433/auxiliary" "-A" "unused"
2019-08-06T12:20:32.6287199Z ------------------------------------------
2019-08-06T12:20:32.6287259Z 
2019-08-06T12:20:32.6287501Z ------------------------------------------
2019-08-06T12:20:32.6287544Z stderr:
2019-08-06T12:20:32.6287544Z stderr:
2019-08-06T12:20:32.6287791Z ------------------------------------------
2019-08-06T12:20:32.6287841Z error[E0669]: invalid value for constraint in inline assembly
2019-08-06T12:20:32.6288099Z   --> /checkout/src/test/ui/issues/issue-37433.rs:7:24
2019-08-06T12:20:32.6288168Z    |
2019-08-06T12:20:32.6288213Z LL |         asm!("" :: "r"(""));
2019-08-06T12:20:32.6288288Z 
2019-08-06T12:20:32.6288347Z error: aborting due to previous error
2019-08-06T12:20:32.6288377Z 
2019-08-06T12:20:32.6288401Z 
---
2019-08-06T12:20:32.6325400Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-06T12:20:32.6325487Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-06T12:20:32.6336535Z 
2019-08-06T12:20:32.6336609Z 
2019-08-06T12:20:32.6341542Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-06T12:20:32.6341869Z 
2019-08-06T12:20:32.6341919Z 
2019-08-06T12:20:32.6355465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-06T12:20:32.6355543Z Build completed unsuccessfully in 1:07:37
2019-08-06T12:20:32.6355543Z Build completed unsuccessfully in 1:07:37
2019-08-06T12:20:33.3598614Z ##[error]Bash exited with code '1'.
2019-08-06T12:20:33.3661100Z ##[section]Starting: Checkout
2019-08-06T12:20:33.3663161Z ==============================================================================
2019-08-06T12:20:33.3663237Z Task         : Get sources
2019-08-06T12:20:33.3663284Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
