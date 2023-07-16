plain
2020-02-13T22:17:22.0263017Z ========================== Starting Command Output ===========================
2020-02-13T22:17:22.0264570Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/760341f6-2b57-4ed7-8109-f284b4fac1ff.sh
2020-02-13T22:17:22.0264605Z 
2020-02-13T22:17:22.0267216Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-13T22:17:22.0273875Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-13T22:17:22.0277246Z Task         : Get sources
2020-02-13T22:17:22.0277283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T22:17:22.0277318Z Version      : 1.0.0
2020-02-13T22:17:22.0277403Z Author       : Microsoft
---
2020-02-13T22:17:23.1185483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-13T22:17:23.1271039Z ##[command]git config gc.auto 0
2020-02-13T22:17:23.1363670Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-13T22:17:23.1474959Z ##[command]git config --get-all http.proxy
2020-02-13T22:17:23.1621835Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69113/merge:refs/remotes/pull/69113/merge
---
2020-02-13T23:15:03.2316386Z .................................................................................................... 1700/9636
2020-02-13T23:15:08.4961579Z .................................................................................................... 1800/9636
2020-02-13T23:15:20.8666954Z ..............................i..................................................................... 1900/9636
2020-02-13T23:15:28.7908102Z .................................................................................................... 2000/9636
2020-02-13T23:15:43.2864880Z ....................iiiii........................................................................... 2100/9636
2020-02-13T23:15:53.4568828Z .................................................................................................... 2300/9636
2020-02-13T23:15:55.9313845Z .................................................................................................... 2400/9636
2020-02-13T23:16:00.8469867Z .................................................................................................... 2500/9636
2020-02-13T23:16:21.7962604Z .................................................................................................... 2600/9636
---
2020-02-13T23:18:58.9019110Z ........................................................................i...............i........... 4900/9636
2020-02-13T23:19:06.7506016Z .................................................................................................... 5000/9636
2020-02-13T23:19:14.7318817Z .................................................................................................... 5100/9636
2020-02-13T23:19:19.4593292Z ..............i..................................................................................... 5200/9636
2020-02-13T23:19:30.4888274Z ........................................................................................ii.ii....... 5300/9636
2020-02-13T23:19:34.5131910Z .i...i.............................................................................................. 5400/9636
2020-02-13T23:19:44.8698298Z .................................................................................................... 5600/9636
2020-02-13T23:19:54.9170515Z .............................................................................i...................... 5700/9636
2020-02-13T23:20:02.4413830Z .................................................................................................... 5800/9636
2020-02-13T23:20:08.7073970Z ...........................................................................F........................ 5900/9636
2020-02-13T23:20:08.7073970Z ...........................................................................F........................ 5900/9636
2020-02-13T23:20:18.5544676Z .....................................................................ii...i..ii...........i......... 6000/9636
2020-02-13T23:20:40.4464402Z .................................................................................................... 6200/9636
2020-02-13T23:20:44.7941687Z .................................................................................................... 6300/9636
2020-02-13T23:20:49.0503119Z .................................................................................................i.. 6400/9636
2020-02-13T23:21:02.1916907Z ii.................................................................................................. 6500/9636
---
2020-02-13T23:23:07.1704684Z .................................................................................................... 7600/9636
2020-02-13T23:23:11.5649856Z .................................................................................................... 7700/9636
2020-02-13T23:23:17.7544726Z .................................................................................................... 7800/9636
2020-02-13T23:23:25.7627548Z .................................................................................................... 7900/9636
2020-02-13T23:23:34.7358808Z ...........................................................................iiiiiii.i................ 8000/9636
2020-02-13T23:23:51.1500858Z ...............i......i............................................................................. 8200/9636
2020-02-13T23:23:56.7639613Z .................................................................................................... 8300/9636
2020-02-13T23:24:10.1189568Z .................................................................................................... 8400/9636
2020-02-13T23:24:19.9987493Z .................................................................................................... 8500/9636
---
2020-02-13T23:26:18.9445469Z 
2020-02-13T23:26:18.9446256Z ---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
2020-02-13T23:26:18.9446480Z diff of stderr:
2020-02-13T23:26:18.9446617Z 
2020-02-13T23:26:18.9446766Z 1 error: rustc_peek: bit not set
2020-02-13T23:26:18.9447577Z +   --> $DIR/indirect-mutation-offset.rs:41:14
2020-02-13T23:26:18.9447752Z 3    |
2020-02-13T23:26:18.9447752Z 3    |
2020-02-13T23:26:18.9447898Z 4 LL |     unsafe { rustc_peek(x) };
2020-02-13T23:26:18.9448167Z 
2020-02-13T23:26:18.9448302Z 
2020-02-13T23:26:18.9448448Z The actual stderr differed from the expected stderr.
2020-02-13T23:26:18.9448918Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2020-02-13T23:26:18.9448918Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2020-02-13T23:26:18.9449365Z To update references, rerun the tests and pass the `--bless` flag
2020-02-13T23:26:18.9449796Z To only update this specific test, also pass `--test-args mir-dataflow/indirect-mutation-offset.rs`
2020-02-13T23:26:18.9450118Z error: 1 errors occurred comparing output.
2020-02-13T23:26:18.9450263Z status: exit code: 1
2020-02-13T23:26:18.9450263Z status: exit code: 1
2020-02-13T23:26:18.9451500Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary" "-A" "unused"
2020-02-13T23:26:18.9452195Z ------------------------------------------
2020-02-13T23:26:18.9452338Z 
2020-02-13T23:26:18.9452703Z ------------------------------------------
2020-02-13T23:26:18.9452860Z stderr:
2020-02-13T23:26:18.9452860Z stderr:
2020-02-13T23:26:18.9453203Z ------------------------------------------
2020-02-13T23:26:18.9453391Z error: rustc_peek: bit not set
2020-02-13T23:26:18.9453947Z    |
2020-02-13T23:26:18.9453947Z    |
2020-02-13T23:26:18.9454111Z LL |     unsafe { rustc_peek(x) }; //~ ERROR rustc_peek: bit not set
2020-02-13T23:26:18.9454377Z 
2020-02-13T23:26:18.9454377Z 
2020-02-13T23:26:18.9454536Z error: stop_after_dataflow ended compilation
2020-02-13T23:26:18.9454794Z error: aborting due to 2 previous errors
2020-02-13T23:26:18.9454938Z 
2020-02-13T23:26:18.9455057Z 
2020-02-13T23:26:18.9455404Z ------------------------------------------
---
2020-02-13T23:26:18.9475097Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-13T23:26:18.9475341Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-13T23:26:18.9484458Z 
2020-02-13T23:26:18.9484706Z 
2020-02-13T23:26:18.9487515Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-13T23:26:18.9488156Z 
2020-02-13T23:26:18.9488194Z 
2020-02-13T23:26:18.9501061Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-13T23:26:18.9501276Z Build completed unsuccessfully in 1:02:46
2020-02-13T23:26:18.9501276Z Build completed unsuccessfully in 1:02:46
2020-02-13T23:26:18.9560018Z == clock drift check ==
2020-02-13T23:26:18.9580193Z   local time: Thu Feb 13 23:26:18 UTC 2020
2020-02-13T23:26:19.2545850Z   network time: Thu, 13 Feb 2020 23:26:19 GMT
2020-02-13T23:26:19.2550727Z == end clock drift check ==
2020-02-13T23:26:19.6885941Z 
2020-02-13T23:26:19.6971663Z ##[error]Bash exited with code '1'.
2020-02-13T23:26:19.6983653Z ##[section]Finishing: Run build
2020-02-13T23:26:19.7003319Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-13T23:26:19.7005374Z Task         : Get sources
2020-02-13T23:26:19.7005428Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T23:26:19.7005496Z Version      : 1.0.0
2020-02-13T23:26:19.7005544Z Author       : Microsoft
2020-02-13T23:26:19.7005544Z Author       : Microsoft
2020-02-13T23:26:19.7005596Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T23:26:19.7005667Z ==============================================================================
2020-02-13T23:26:20.1217051Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T23:26:20.1260200Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-13T23:26:20.1369560Z Cleaning up task key
2020-02-13T23:26:20.1370421Z Start cleaning up orphan processes.
2020-02-13T23:26:20.1472434Z Terminate orphan process: pid (4360) (python)
2020-02-13T23:26:20.1758827Z ##[section]Finishing: Finalize Job
