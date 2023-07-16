plain
2020-02-16T06:57:53.0406565Z ========================== Starting Command Output ===========================
2020-02-16T06:57:53.0410518Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a30af605-ab9a-4be0-95e8-36e627f5e5e6.sh
2020-02-16T06:57:53.0410707Z 
2020-02-16T06:57:53.0415093Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T06:57:53.0423927Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-16T06:57:53.0425786Z Task         : Get sources
2020-02-16T06:57:53.0425831Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T06:57:53.0425932Z Version      : 1.0.0
2020-02-16T06:57:53.0425979Z Author       : Microsoft
---
2020-02-16T06:57:58.2089077Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T06:57:58.2266953Z ##[command]git config gc.auto 0
2020-02-16T06:57:58.2339540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T06:57:58.2384956Z ##[command]git config --get-all http.proxy
2020-02-16T06:57:58.2501230Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67272/merge:refs/remotes/pull/67272/merge
---
2020-02-16T07:47:39.2809183Z .................................................................................................... 1700/9654
2020-02-16T07:47:43.2562247Z .................................................................................................... 1800/9654
2020-02-16T07:47:52.8791982Z ..................................i................................................................. 1900/9654
2020-02-16T07:47:59.0713832Z .................................................................................................... 2000/9654
2020-02-16T07:48:10.4871827Z ........................iiiii....................................................................... 2100/9654
2020-02-16T07:48:18.6102131Z .................................................................................................... 2300/9654
2020-02-16T07:48:20.7239814Z .................................................................................................... 2400/9654
2020-02-16T07:48:24.6496656Z .................................................................................................... 2500/9654
2020-02-16T07:48:41.3938429Z .................................................................................................... 2600/9654
---
2020-02-16T07:51:25.3082868Z .................................................................................................... 5600/9654
2020-02-16T07:51:33.7336821Z .......................................................................................i............ 5700/9654
2020-02-16T07:51:40.1704941Z .................................................................................................... 5800/9654
2020-02-16T07:51:44.4555335Z .....................................................................................i.............. 5900/9654
2020-02-16T07:51:52.4323195Z ...............................................................................ii...i..ii........... 6000/9654
2020-02-16T07:52:02.8986992Z i................................................................................................... 6100/9654
2020-02-16T07:52:15.2203814Z .................................................................................................... 6300/9654
2020-02-16T07:52:18.5360803Z .................................................................................................... 6400/9654
2020-02-16T07:52:18.5360803Z .................................................................................................... 6400/9654
2020-02-16T07:52:29.2337382Z .......i..ii........................................................................................ 6500/9654
2020-02-16T07:52:45.5370857Z ...............................................................................................i.... 6700/9654
2020-02-16T07:52:47.3869413Z .................................................................................................... 6800/9654
2020-02-16T07:52:49.1145018Z .................................................................................................... 6900/9654
2020-02-16T07:52:51.1291165Z .....i.............................................................................................. 7000/9654
---
2020-02-16T07:54:08.0516031Z .................................................................................................... 7600/9654
2020-02-16T07:54:12.3111810Z .................................................................................................... 7700/9654
2020-02-16T07:54:17.3451602Z .................................................................................................... 7800/9654
2020-02-16T07:54:22.5617101Z .................................................................................................... 7900/9654
2020-02-16T07:54:30.6439560Z ...........................................................................................iiiiiii.i 8000/9654
2020-02-16T07:54:43.6549232Z ...............................i......i............................................................. 8200/9654
2020-02-16T07:54:47.7315000Z .................................................................................................... 8300/9654
2020-02-16T07:54:56.9348488Z .................................................................................................... 8400/9654
2020-02-16T07:55:06.6637677Z .................................................................................................... 8500/9654
---
2020-02-16T07:56:42.2543632Z 
2020-02-16T07:56:42.2543736Z 4 LL | test!(test);
2020-02-16T07:56:42.2543854Z 5    | ^^^^^^^^^^^^
2020-02-16T07:56:42.2543956Z 6    |
2020-02-16T07:56:42.2544237Z -    = help: consider adding a `#![recursion_limit="0"]` attribute to your crate
2020-02-16T07:56:42.2544382Z +    = help: consider adding a `#![recursion_limit="0"]` attribute to your crate (`zero`)
2020-02-16T07:56:42.2544605Z 9 error: aborting due to previous error
2020-02-16T07:56:42.2544703Z 10 
2020-02-16T07:56:42.2544790Z 
2020-02-16T07:56:42.2544882Z 
2020-02-16T07:56:42.2544882Z 
2020-02-16T07:56:42.2545020Z The actual stderr differed from the expected stderr.
2020-02-16T07:56:42.2545315Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero/zero.stderr
2020-02-16T07:56:42.2545595Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T07:56:42.2545866Z To only update this specific test, also pass `--test-args recursion_limit/zero.rs`
2020-02-16T07:56:42.2546094Z error: 1 errors occurred comparing output.
2020-02-16T07:56:42.2546195Z status: exit code: 1
2020-02-16T07:56:42.2546195Z status: exit code: 1
2020-02-16T07:56:42.2546965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion_limit/zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero/auxiliary" "-A" "unused"
2020-02-16T07:56:42.2547400Z ------------------------------------------
2020-02-16T07:56:42.2547502Z 
2020-02-16T07:56:42.2547830Z ------------------------------------------
2020-02-16T07:56:42.2547958Z stderr:
2020-02-16T07:56:42.2547958Z stderr:
2020-02-16T07:56:42.2548187Z ------------------------------------------
2020-02-16T07:56:42.2548304Z error: recursion limit reached while expanding `test!`
2020-02-16T07:56:42.2548563Z   --> /checkout/src/test/ui/recursion_limit/zero.rs:10:1
2020-02-16T07:56:42.2548680Z    |
2020-02-16T07:56:42.2548788Z LL | test!(test); //~ ERROR 10:1: 10:13: recursion limit reached while expanding `test!`
2020-02-16T07:56:42.2549010Z    |
2020-02-16T07:56:42.2549010Z    |
2020-02-16T07:56:42.2549137Z    = help: consider adding a `#![recursion_limit="0"]` attribute to your crate (`zero`)
2020-02-16T07:56:42.2549331Z error: aborting due to previous error
2020-02-16T07:56:42.2549417Z 
2020-02-16T07:56:42.2549518Z 
2020-02-16T07:56:42.2549749Z ------------------------------------------
---
2020-02-16T07:56:42.2563476Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T07:56:42.2563760Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T07:56:42.2575214Z 
2020-02-16T07:56:42.2575373Z 
2020-02-16T07:56:42.2579407Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T07:56:42.2579869Z 
2020-02-16T07:56:42.2579897Z 
2020-02-16T07:56:43.0245864Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T07:56:43.0251346Z Build completed unsuccessfully in 0:52:25
2020-02-16T07:56:43.0251346Z Build completed unsuccessfully in 0:52:25
2020-02-16T07:56:43.0265632Z == clock drift check ==
2020-02-16T07:56:43.0265764Z   local time: Sun Feb 16 07:56:42 UTC 2020
2020-02-16T07:56:43.0265838Z   network time: Sun, 16 Feb 2020 07:56:42 GMT
2020-02-16T07:56:43.0266002Z == end clock drift check ==
2020-02-16T07:56:43.0890350Z 
2020-02-16T07:56:43.0958590Z ##[error]Bash exited with code '1'.
2020-02-16T07:56:43.0967899Z ##[section]Finishing: Run build
2020-02-16T07:56:43.0987091Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-16T07:56:43.0988603Z Task         : Get sources
2020-02-16T07:56:43.0988645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T07:56:43.0988685Z Version      : 1.0.0
2020-02-16T07:56:43.0988739Z Author       : Microsoft
2020-02-16T07:56:43.0988739Z Author       : Microsoft
2020-02-16T07:56:43.0988875Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T07:56:43.0988919Z ==============================================================================
2020-02-16T07:56:43.4317757Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T07:56:43.4369379Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-16T07:56:43.4481225Z Cleaning up task key
2020-02-16T07:56:43.4481933Z Start cleaning up orphan processes.
2020-02-16T07:56:43.4552920Z Terminate orphan process: pid (4186) (python)
2020-02-16T07:56:43.5161493Z ##[section]Finishing: Finalize Job
