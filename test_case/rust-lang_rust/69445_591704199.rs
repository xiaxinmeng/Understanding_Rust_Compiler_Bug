plain
2020-02-26T22:39:14.5638230Z ========================== Starting Command Output ===========================
2020-02-26T22:39:14.5640491Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ff717144-c36b-4936-8e8c-75741e30e3d1.sh
2020-02-26T22:39:14.5640684Z 
2020-02-26T22:39:14.5643242Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T22:39:14.5668469Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-26T22:39:14.5674039Z Task         : Get sources
2020-02-26T22:39:14.5675552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T22:39:14.5675907Z Version      : 1.0.0
2020-02-26T22:39:14.5676102Z Author       : Microsoft
---
2020-02-26T22:39:15.5606238Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T22:39:15.5617293Z ##[command]git config gc.auto 0
2020-02-26T22:39:15.5784615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T22:39:15.5787477Z ##[command]git config --get-all http.proxy
2020-02-26T22:39:15.5792792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69445/merge:refs/remotes/pull/69445/merge
---
2020-02-26T23:35:52.5879320Z .................................................................................................... 1700/9735
2020-02-26T23:35:56.4812292Z .................................................................................................... 1800/9735
2020-02-26T23:36:06.5736846Z ....................................................................i............................... 1900/9735
2020-02-26T23:36:12.6565322Z .................................................................................................... 2000/9735
2020-02-26T23:36:25.8368766Z ..........................................................iiii.i.................................... 2100/9735
2020-02-26T23:36:34.9191952Z .................................................................................................... 2300/9735
2020-02-26T23:36:36.8564594Z .................................................................................................... 2400/9735
2020-02-26T23:36:39.5175007Z .................................................................................................... 2500/9735
2020-02-26T23:36:58.2101728Z .................................................................................................... 2600/9735
---
2020-02-26T23:39:15.5446834Z ..................i...............i................................................................. 5000/9735
2020-02-26T23:39:23.9585076Z .................................................................................................... 5100/9735
2020-02-26T23:39:29.0252839Z .............................................................i...................................... 5200/9735
2020-02-26T23:39:35.0649901Z .................................................................................................... 5300/9735
2020-02-26T23:39:42.9213134Z ......................................ii.ii........i...i............................................ 5400/9735
2020-02-26T23:39:51.4685869Z .................................................................................................... 5600/9735
2020-02-26T23:39:59.6866181Z .................................................................................................... 5700/9735
2020-02-26T23:40:05.7645013Z .............................i...................................................................... 5800/9735
2020-02-26T23:40:10.9599705Z .................................................................................................... 5900/9735
2020-02-26T23:40:10.9599705Z .................................................................................................... 5900/9735
2020-02-26T23:40:21.2096353Z .................................................................................................... 6000/9735
2020-02-26T23:40:29.9327912Z ....................ii...i..ii...........i.......................................................... 6100/9735
2020-02-26T23:40:45.2125798Z .................................................................................................... 6300/9735
2020-02-26T23:40:51.3622703Z .................................................................................................... 6400/9735
2020-02-26T23:40:51.3622703Z .................................................................................................... 6400/9735
2020-02-26T23:41:04.0918672Z ...................................................i..ii............................................ 6500/9735
2020-02-26T23:41:26.0317663Z .................................................................................................... 6700/9735
2020-02-26T23:41:28.0786241Z ...........................................i........................................................ 6800/9735
2020-02-26T23:41:29.9400013Z .................................................................................................... 6900/9735
2020-02-26T23:41:31.9375026Z .........................................................................i.......................... 7000/9735
---
2020-02-26T23:43:00.1411871Z .................................................................................................... 7700/9735
2020-02-26T23:43:04.5050609Z .................................................................................................... 7800/9735
2020-02-26T23:43:09.5178252Z .................................................................................................... 7900/9735
2020-02-26T23:43:16.8394020Z ....................i............................................................................... 8000/9735
2020-02-26T23:43:24.5848524Z .....................................................................iiiiiii.i...................... 8100/9735
2020-02-26T23:43:38.0167064Z ..........i......i.................................................................................. 8300/9735
2020-02-26T23:43:42.5775504Z .................................................................................................... 8400/9735
2020-02-26T23:43:53.8225708Z .................................................................................................... 8500/9735
2020-02-26T23:44:01.8641460Z .................................................................................................... 8600/9735
---
2020-02-26T23:46:04.8898686Z  finished in 6.280
2020-02-26T23:46:04.9049415Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:05.0759711Z 
2020-02-26T23:46:05.0760053Z running 178 tests
2020-02-26T23:46:07.4630186Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-26T23:46:09.4961575Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-26T23:46:09.4963582Z 
2020-02-26T23:46:09.4967759Z  finished in 4.592
2020-02-26T23:46:09.5116929Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:09.6536922Z 
---
2020-02-26T23:46:11.3218998Z  finished in 1.810
2020-02-26T23:46:11.3391795Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:11.4689313Z 
2020-02-26T23:46:11.4690085Z running 9 tests
2020-02-26T23:46:11.4691933Z iiiiiiiii
2020-02-26T23:46:11.4694113Z 
2020-02-26T23:46:11.4695458Z  finished in 0.130
2020-02-26T23:46:11.4880817Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:11.6941174Z 
---
2020-02-26T23:46:28.4032407Z  finished in 16.915
2020-02-26T23:46:28.4200290Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:28.5842080Z 
2020-02-26T23:46:28.5842376Z running 116 tests
2020-02-26T23:46:39.9297423Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-26T23:46:41.4883387Z ....iiii.....ii.
2020-02-26T23:46:41.4886985Z 
2020-02-26T23:46:41.4887362Z  finished in 13.068
2020-02-26T23:46:41.4889850Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-26T23:46:41.4890496Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-26T23:53:04.6519494Z ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
2020-02-26T23:53:04.6519649Z 
2020-02-26T23:53:04.6519792Z error: rustdoc failed!
2020-02-26T23:53:04.6519946Z status: exit code: 1
2020-02-26T23:53:04.6521145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs" "--playground-url=https://example.com/" "-Z" "unstable-options"
2020-02-26T23:53:04.6522181Z ------------------------------------------
2020-02-26T23:53:04.6522315Z 
2020-02-26T23:53:04.6522611Z ------------------------------------------
2020-02-26T23:53:04.6522766Z stderr:
2020-02-26T23:53:04.6522766Z stderr:
2020-02-26T23:53:04.6523042Z ------------------------------------------
2020-02-26T23:53:04.6523798Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-26T23:53:04.6524545Z 
2020-02-26T23:53:04.6524889Z ------------------------------------------
2020-02-26T23:53:04.6525020Z 
2020-02-26T23:53:04.6525573Z 
2020-02-26T23:53:04.6525573Z 
2020-02-26T23:53:04.6529849Z ---- [rustdoc] rustdoc/process-termination.rs stdout ----
2020-02-26T23:53:04.6530060Z 
2020-02-26T23:53:04.6530191Z error: rustdoc failed!
2020-02-26T23:53:04.6530362Z status: exit code: 101
2020-02-26T23:53:04.6532031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
2020-02-26T23:53:04.6533170Z ------------------------------------------
2020-02-26T23:53:04.6533326Z 
2020-02-26T23:53:04.6533436Z running 3 tests
2020-02-26T23:53:04.6533864Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2020-02-26T23:53:04.6533864Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2020-02-26T23:53:04.6534549Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
2020-02-26T23:53:04.6535135Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) ... ok
2020-02-26T23:53:04.6535349Z 
2020-02-26T23:53:04.6535467Z failures:
2020-02-26T23:53:04.6535552Z 
2020-02-26T23:53:04.6535957Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) stdout ----
2020-02-26T23:53:04.6536718Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-26T23:53:04.6537561Z 
2020-02-26T23:53:04.6538742Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2020-02-26T23:53:04.6538742Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2020-02-26T23:53:04.6539781Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-26T23:53:04.6540480Z 
2020-02-26T23:53:04.6540635Z failures:
2020-02-26T23:53:04.6541198Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)
2020-02-26T23:53:04.6542369Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)
---
2020-02-26T23:53:04.6546772Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-26T23:53:04.6547101Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-26T23:53:04.6547313Z 
2020-02-26T23:53:04.6547386Z 
2020-02-26T23:53:04.6550408Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-26T23:53:04.6552643Z 
2020-02-26T23:53:04.6552717Z 
2020-02-26T23:53:04.6564946Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-26T23:53:04.6565222Z Build completed unsuccessfully in 1:07:47
2020-02-26T23:53:04.6565222Z Build completed unsuccessfully in 1:07:47
2020-02-26T23:53:04.6595600Z == clock drift check ==
2020-02-26T23:53:04.6613144Z   local time: Wed Feb 26 23:53:04 UTC 2020
2020-02-26T23:53:05.2119883Z   network time: Wed, 26 Feb 2020 23:53:05 GMT
2020-02-26T23:53:05.2121634Z == end clock drift check ==
2020-02-26T23:53:06.5917241Z 
2020-02-26T23:53:06.5982222Z ##[error]Bash exited with code '1'.
2020-02-26T23:53:06.5996052Z ##[section]Finishing: Run build
2020-02-26T23:53:06.6039091Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-26T23:53:06.6043649Z Task         : Get sources
2020-02-26T23:53:06.6044336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T23:53:06.6044764Z Version      : 1.0.0
2020-02-26T23:53:06.6044956Z Author       : Microsoft
2020-02-26T23:53:06.6044956Z Author       : Microsoft
2020-02-26T23:53:06.6045265Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T23:53:06.6045600Z ==============================================================================
2020-02-26T23:53:06.9185821Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T23:53:06.9226103Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-26T23:53:06.9301473Z Cleaning up task key
2020-02-26T23:53:06.9302453Z Start cleaning up orphan processes.
2020-02-26T23:53:06.9592385Z Terminate orphan process: pid (4437) (python)
2020-02-26T23:53:06.9651200Z ##[section]Finishing: Finalize Job
