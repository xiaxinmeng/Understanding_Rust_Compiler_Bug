plain
2020-03-20T08:39:06.0006829Z ========================== Starting Command Output ===========================
2020-03-20T08:39:06.0012171Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ffefae1b-addc-4a15-89aa-b1206241ede2.sh
2020-03-20T08:39:06.0012479Z 
2020-03-20T08:39:06.0017990Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T08:39:06.0039667Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T08:39:06.0043315Z Task         : Get sources
2020-03-20T08:39:06.0043659Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T08:39:06.0043976Z Version      : 1.0.0
2020-03-20T08:39:06.0044193Z Author       : Microsoft
---
2020-03-20T08:39:06.9869200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T08:39:06.9874360Z ##[command]git config gc.auto 0
2020-03-20T08:39:06.9877750Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T08:39:06.9881318Z ##[command]git config --get-all http.proxy
2020-03-20T08:39:06.9887821Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
---
2020-03-20T09:41:03.4880783Z .................................................................................................... 1700/9803
2020-03-20T09:41:08.1070884Z .................................................................................................... 1800/9803
2020-03-20T09:41:19.6552186Z ..........................................................................i......................... 1900/9803
2020-03-20T09:41:26.1606340Z .................................................................................................... 2000/9803
2020-03-20T09:41:34.5605751Z ................................................................iiiii............................... 2100/9803
2020-03-20T09:41:52.6683813Z .................................................................................................... 2300/9803
2020-03-20T09:41:54.9237889Z .................................................................................................... 2400/9803
2020-03-20T09:41:57.9612208Z .................................................................................................... 2500/9803
2020-03-20T09:42:18.6176691Z .................................................................................................... 2600/9803
---
2020-03-20T09:45:00.6907023Z .....................................i...............i.............................................. 5000/9803
2020-03-20T09:45:10.1287417Z .................................................................................................... 5100/9803
2020-03-20T09:45:16.3439760Z ................................................................................i................... 5200/9803
2020-03-20T09:45:21.7334645Z .................................................................................................... 5300/9803
2020-03-20T09:45:31.7538961Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-20T09:45:39.7176695Z i................................................................................................... 5600/9803
2020-03-20T09:45:48.9763883Z .....i.............................................................................................. 5700/9803
2020-03-20T09:45:55.2957490Z ........................................................i........................................... 5800/9803
2020-03-20T09:46:01.8355612Z .................................................................................................... 5900/9803
2020-03-20T09:46:01.8355612Z .................................................................................................... 5900/9803
2020-03-20T09:46:09.9765663Z .................................................................................................... 6000/9803
2020-03-20T09:46:17.7919894Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-20T09:46:38.1023897Z .................................................................................................... 6300/9803
2020-03-20T09:46:45.1913879Z .................................................................................................... 6400/9803
2020-03-20T09:46:45.1913879Z .................................................................................................... 6400/9803
2020-03-20T09:46:52.5507460Z ................................................................................i..ii............... 6500/9803
2020-03-20T09:47:16.2736381Z .................................................................................................... 6700/9803
2020-03-20T09:47:26.0056825Z ...............................................................................i.................... 6800/9803
2020-03-20T09:47:28.2952433Z .................................................................................................... 6900/9803
2020-03-20T09:47:30.6824609Z .................................................................................................... 7000/9803
---
2020-03-20T09:49:18.9358023Z .................................................................................................... 7800/9803
2020-03-20T09:49:24.8177540Z .................................................................................................... 7900/9803
2020-03-20T09:49:31.4968261Z ..................................................................i................................. 8000/9803
2020-03-20T09:49:42.5251757Z .................................................................................................... 8100/9803
2020-03-20T09:49:48.4183495Z ...............iiiiiiiiii.i......................................................................... 8200/9803
2020-03-20T09:50:02.6076172Z .................................................................................................... 8400/9803
2020-03-20T09:50:08.8329602Z .................................................................................................... 8500/9803
2020-03-20T09:50:24.1180220Z .................................................................................................... 8600/9803
2020-03-20T09:50:30.9688321Z .................................................................................................... 8700/9803
---
2020-03-20T09:52:25.0006987Z 
2020-03-20T09:52:25.0007846Z ---- [ui] ui/feature-gates/feature-gate-asm2.rs stdout ----
2020-03-20T09:52:25.0008293Z diff of stderr:
2020-03-20T09:52:25.0008519Z 
2020-03-20T09:52:25.0009248Z - serror[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T09:52:25.0010224Z + error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T09:52:25.0011283Z 3    |
2020-03-20T09:52:25.0011605Z 4 LL |         println!("{:?}", asm!(""));
2020-03-20T09:52:25.0011867Z 
2020-03-20T09:52:25.0012057Z 
2020-03-20T09:52:25.0012057Z 
2020-03-20T09:52:25.0012361Z The actual stderr differed from the expected stderr.
2020-03-20T09:52:25.0013127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/feature-gate-asm2.stderr
2020-03-20T09:52:25.0013887Z To update references, rerun the tests and pass the `--bless` flag
2020-03-20T09:52:25.0014618Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-asm2.rs`
2020-03-20T09:52:25.0015266Z error: 1 errors occurred comparing output.
2020-03-20T09:52:25.0015598Z status: exit code: 1
2020-03-20T09:52:25.0015598Z status: exit code: 1
2020-03-20T09:52:25.0018263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-asm2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-asm2/auxiliary"
2020-03-20T09:52:25.0023188Z ------------------------------------------
2020-03-20T09:52:25.0023364Z 
2020-03-20T09:52:25.0023819Z ------------------------------------------
2020-03-20T09:52:25.0023993Z stderr:
2020-03-20T09:52:25.0023993Z stderr:
2020-03-20T09:52:25.0024334Z ------------------------------------------
2020-03-20T09:52:25.0024918Z error[E0658]: use of unstable library feature 'asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T09:52:25.0025742Z    |
2020-03-20T09:52:25.0025742Z    |
2020-03-20T09:52:25.0025976Z LL |         println!("{:?}", asm!("")); //~ ERROR inline assembly is not stable
2020-03-20T09:52:25.0026418Z    |
2020-03-20T09:52:25.0026418Z    |
2020-03-20T09:52:25.0026998Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T09:52:25.0027325Z    = help: add `#![feature(asm)]` to the crate attributes to enable
2020-03-20T09:52:25.0027516Z 
2020-03-20T09:52:25.0028059Z error[E0658]: use of unstable library feature 'llvm_asm': inline assembly is not stable enough for use and is subject to change
2020-03-20T09:52:25.0029100Z    |
2020-03-20T09:52:25.0029100Z    |
2020-03-20T09:52:25.0029342Z LL |         println!("{:?}", llvm_asm!("")); //~ ERROR inline assembly is not stable
2020-03-20T09:52:25.0032626Z    |
2020-03-20T09:52:25.0032626Z    |
2020-03-20T09:52:25.0033794Z    = note: see issue #70173 <***/issues/70173> for more information
2020-03-20T09:52:25.0034150Z    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
2020-03-20T09:52:25.0034502Z error: aborting due to 2 previous errors
2020-03-20T09:52:25.0034644Z 
2020-03-20T09:52:25.0035054Z For more information about this error, try `rustc --explain E0658`.
2020-03-20T09:52:25.0035244Z 
---
2020-03-20T09:52:25.0037794Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-20T09:52:25.0038157Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-20T09:52:25.0038396Z 
2020-03-20T09:52:25.0038481Z 
2020-03-20T09:52:25.0043569Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-20T09:52:25.0046152Z 
2020-03-20T09:52:25.0046236Z 
2020-03-20T09:52:25.0046848Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-20T09:52:25.0047168Z Build completed unsuccessfully in 1:07:40
2020-03-20T09:52:25.0047168Z Build completed unsuccessfully in 1:07:40
2020-03-20T09:52:25.0095925Z == clock drift check ==
2020-03-20T09:52:25.0115141Z   local time: Fri Mar 20 09:52:25 UTC 2020
2020-03-20T09:52:25.3071394Z   network time: Fri, 20 Mar 2020 09:52:25 GMT
2020-03-20T09:52:25.3097705Z == end clock drift check ==
2020-03-20T09:52:25.7355730Z 
2020-03-20T09:52:25.7455066Z ##[error]Bash exited with code '1'.
2020-03-20T09:52:25.7469123Z ##[section]Finishing: Run build
2020-03-20T09:52:25.7527339Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T09:52:25.7539816Z Task         : Get sources
2020-03-20T09:52:25.7540279Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T09:52:25.7541077Z Version      : 1.0.0
2020-03-20T09:52:25.7541801Z Author       : Microsoft
2020-03-20T09:52:25.7541801Z Author       : Microsoft
2020-03-20T09:52:25.7542202Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T09:52:25.7542693Z ==============================================================================
2020-03-20T09:52:26.0860260Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T09:52:26.0901074Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-03-20T09:52:26.1003152Z Cleaning up task key
2020-03-20T09:52:26.1004610Z Start cleaning up orphan processes.
2020-03-20T09:52:26.1354657Z Terminate orphan process: pid (4443) (python)
2020-03-20T09:52:26.1405081Z ##[section]Finishing: Finalize Job
