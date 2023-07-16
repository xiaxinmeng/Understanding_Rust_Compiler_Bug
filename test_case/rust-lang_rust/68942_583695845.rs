plain
2020-02-08T02:05:35.8309237Z ========================== Starting Command Output ===========================
2020-02-08T02:05:35.8310391Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/350e9701-d378-434c-bd83-72c4d0e74dd5.sh
2020-02-08T02:05:35.8310418Z 
2020-02-08T02:05:35.8312264Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T02:05:35.8364557Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68942/merge to s
2020-02-08T02:05:35.8366914Z Task         : Get sources
2020-02-08T02:05:35.8366942Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T02:05:35.8366967Z Version      : 1.0.0
2020-02-08T02:05:35.8366991Z Author       : Microsoft
---
2020-02-08T02:05:36.8460630Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T02:05:36.8541540Z ##[command]git config gc.auto 0
2020-02-08T02:05:36.8545410Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T02:05:36.8549214Z ##[command]git config --get-all http.proxy
2020-02-08T02:05:36.8557091Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68942/merge:refs/remotes/pull/68942/merge
---
2020-02-08T02:52:16.1917677Z .................................................................................................... 1700/9610
2020-02-08T02:52:20.0798646Z .................................................................................................... 1800/9610
2020-02-08T02:52:29.9542311Z .............................i...................................................................... 1900/9610
2020-02-08T02:52:35.5874580Z .................................................................................................... 2000/9610
2020-02-08T02:52:46.4366311Z ...................iiiii............................................................................ 2100/9610
2020-02-08T02:52:53.8158262Z .................................................................................................... 2300/9610
2020-02-08T02:52:55.5756971Z .................................................................................................... 2400/9610
2020-02-08T02:52:59.2481821Z .................................................................................................... 2500/9610
2020-02-08T02:53:15.3727116Z .................................................................................................... 2600/9610
---
2020-02-08T02:55:16.0749352Z .......................................................................i...............i............ 4900/9610
2020-02-08T02:55:22.0179893Z .................................................................................................... 5000/9610
2020-02-08T02:55:28.2440089Z .................................................................................................... 5100/9610
2020-02-08T02:55:31.8730776Z ..............i..................................................................................... 5200/9610
2020-02-08T02:55:40.6678038Z ........................................................................................ii.ii....... 5300/9610
2020-02-08T02:55:43.6446119Z .i...i.............................................................................................. 5400/9610
2020-02-08T02:55:52.7198236Z .................................................................................................... 5600/9610
2020-02-08T02:55:59.1970898Z ............................................................................i....................... 5700/9610
2020-02-08T02:56:04.9727731Z .................................................................................................... 5800/9610
2020-02-08T02:56:09.8242432Z .................................................................................................... 5900/9610
2020-02-08T02:56:09.8242432Z .................................................................................................... 5900/9610
2020-02-08T02:56:17.4537952Z ....................................................................ii...i..ii............i......... 6000/9610
2020-02-08T02:56:33.8727717Z .................................................................................................... 6200/9610
2020-02-08T02:56:39.5624199Z .................................................................................................... 6300/9610
2020-02-08T02:56:45.5616255Z ................................................................................................i..i 6400/9610
2020-02-08T02:56:58.6903251Z i................................................................................................... 6500/9610
---
2020-02-08T02:58:33.5322950Z .................................................................................................... 7600/9610
2020-02-08T02:58:37.2547814Z .................................................................................................... 7700/9610
2020-02-08T02:58:41.1394916Z .................................................................................................... 7800/9610
2020-02-08T02:58:48.2705607Z .................................................................................................... 7900/9610
2020-02-08T02:58:54.9164459Z ......................................................iiiiiii.i..................................... 8000/9610
2020-02-08T02:59:06.3238158Z .i.................................................................................................. 8200/9610
2020-02-08T02:59:10.6018578Z .................................................................................................... 8300/9610
2020-02-08T02:59:22.3667847Z .................................................................................................... 8400/9610
2020-02-08T02:59:28.8316892Z .................................................................................................... 8500/9610
---
2020-02-08T03:00:59.6945401Z 
2020-02-08T03:00:59.6946281Z ---- [ui] ui/feature-gates/feature-gate-no_sanitize.rs stdout ----
2020-02-08T03:00:59.6946516Z diff of stderr:
2020-02-08T03:00:59.6946621Z 
2020-02-08T03:00:59.6946776Z 4 LL | #[no_sanitize(address)]
2020-02-08T03:00:59.6947000Z 6    |
2020-02-08T03:00:59.6947000Z 6    |
2020-02-08T03:00:59.6947492Z -    = note: for more information, see ***/issues/39699
2020-02-08T03:00:59.6947903Z +    = note: see issue #39699 <***/issues/39699> for more information
2020-02-08T03:00:59.6948248Z 8    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-08T03:00:59.6948493Z 10 error: aborting due to previous error
2020-02-08T03:00:59.6948592Z 
2020-02-08T03:00:59.6948685Z 
2020-02-08T03:00:59.6948797Z The actual stderr differed from the expected stderr.
2020-02-08T03:00:59.6948797Z The actual stderr differed from the expected stderr.
2020-02-08T03:00:59.6949212Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/feature-gate-no_sanitize.stderr
2020-02-08T03:00:59.6949561Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T03:00:59.6949947Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-no_sanitize.rs`
2020-02-08T03:00:59.6950196Z error: 1 errors occurred comparing output.
2020-02-08T03:00:59.6950320Z status: exit code: 1
2020-02-08T03:00:59.6950320Z status: exit code: 1
2020-02-08T03:00:59.6951083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/auxiliary" "-A" "unused"
2020-02-08T03:00:59.6951654Z ------------------------------------------
2020-02-08T03:00:59.6951783Z 
2020-02-08T03:00:59.6952063Z ------------------------------------------
2020-02-08T03:00:59.6952212Z stderr:
2020-02-08T03:00:59.6952212Z stderr:
2020-02-08T03:00:59.6952494Z ------------------------------------------
2020-02-08T03:00:59.6952667Z error[E0658]: the `#[no_sanitize]` attribute is an experimental feature
2020-02-08T03:00:59.6952996Z   --> /checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs:1:1
2020-02-08T03:00:59.6953164Z    |
2020-02-08T03:00:59.6953283Z LL | #[no_sanitize(address)]
2020-02-08T03:00:59.6953506Z    |
2020-02-08T03:00:59.6953506Z    |
2020-02-08T03:00:59.6953892Z    = note: see issue #39699 <***/issues/39699> for more information
2020-02-08T03:00:59.6954061Z    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-08T03:00:59.6954281Z error: aborting due to previous error
2020-02-08T03:00:59.6954401Z 
2020-02-08T03:00:59.6954709Z For more information about this error, try `rustc --explain E0658`.
2020-02-08T03:00:59.6954843Z 
---
2020-02-08T03:00:59.6964258Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-08T03:00:59.6964558Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-08T03:00:59.6973296Z 
2020-02-08T03:00:59.6973456Z 
2020-02-08T03:00:59.6975089Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-08T03:00:59.6975925Z 
2020-02-08T03:00:59.6976052Z 
2020-02-08T03:00:59.6989026Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-08T03:00:59.6989336Z Build completed unsuccessfully in 0:49:41
2020-02-08T03:00:59.6989336Z Build completed unsuccessfully in 0:49:41
2020-02-08T03:00:59.7040834Z == clock drift check ==
2020-02-08T03:00:59.7061701Z   local time: Sat Feb  8 03:00:59 UTC 2020
2020-02-08T03:01:00.2665942Z   network time: Sat, 08 Feb 2020 03:01:00 GMT
2020-02-08T03:01:00.2669524Z == end clock drift check ==
2020-02-08T03:01:00.7297641Z 
2020-02-08T03:01:00.7382258Z ##[error]Bash exited with code '1'.
2020-02-08T03:01:00.7392166Z ##[section]Finishing: Run build
2020-02-08T03:01:00.7411287Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68942/merge to s
2020-02-08T03:01:00.7412767Z Task         : Get sources
2020-02-08T03:01:00.7412819Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T03:01:00.7412855Z Version      : 1.0.0
2020-02-08T03:01:00.7412887Z Author       : Microsoft
2020-02-08T03:01:00.7412887Z Author       : Microsoft
2020-02-08T03:01:00.7412939Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T03:01:00.7412990Z ==============================================================================
2020-02-08T03:01:01.0930755Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T03:01:01.0967051Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68942/merge to s
2020-02-08T03:01:01.1051972Z Cleaning up task key
2020-02-08T03:01:01.1052583Z Start cleaning up orphan processes.
2020-02-08T03:01:01.1136608Z Terminate orphan process: pid (6663) (python)
2020-02-08T03:01:01.1295436Z ##[section]Finishing: Finalize Job
