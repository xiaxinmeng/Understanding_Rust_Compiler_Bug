plain
2020-02-24T23:11:35.3311918Z ========================== Starting Command Output ===========================
2020-02-24T23:11:35.3316873Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4d46edc7-f56c-42a0-87a9-b8b4757924c7.sh
2020-02-24T23:11:35.3317483Z 
2020-02-24T23:11:35.3321439Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-24T23:11:35.3336570Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-24T23:11:35.3339335Z Task         : Get sources
2020-02-24T23:11:35.3339548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T23:11:35.3339826Z Version      : 1.0.0
2020-02-24T23:11:35.3339971Z Author       : Microsoft
---
2020-02-24T23:11:36.6767091Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-24T23:11:36.6772948Z ##[command]git config gc.auto 0
2020-02-24T23:11:36.6776370Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-24T23:11:36.6779625Z ##[command]git config --get-all http.proxy
2020-02-24T23:11:36.6785653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69445/merge:refs/remotes/pull/69445/merge
---
2020-02-25T00:08:51.6031615Z .................................................................................................... 1700/9706
2020-02-25T00:08:55.5691928Z .................................................................................................... 1800/9706
2020-02-25T00:09:05.7986861Z ...........................................i........................................................ 1900/9706
2020-02-25T00:09:13.5188230Z .................................................................................................... 2000/9706
2020-02-25T00:09:25.8354769Z .................................iiiii.............................................................. 2100/9706
2020-02-25T00:09:34.4637288Z .................................................................................................... 2300/9706
2020-02-25T00:09:36.5068320Z .................................................................................................... 2400/9706
2020-02-25T00:09:40.2663572Z .................................................................................................... 2500/9706
2020-02-25T00:09:58.3838005Z .................................................................................................... 2600/9706
---
2020-02-25T00:12:19.0278922Z .........i.......................................................................................... 5000/9706
2020-02-25T00:12:26.9280831Z .................................................................................................... 5100/9706
2020-02-25T00:12:30.9811414Z ....................................i............................................................... 5200/9706
2020-02-25T00:12:39.6470534Z .................................................................................................... 5300/9706
2020-02-25T00:12:44.8513731Z ............ii.ii........i...i...................................................................... 5400/9706
2020-02-25T00:12:52.2175155Z .................................................................................................... 5600/9706
2020-02-25T00:13:01.6475865Z .................................................................................................... 5700/9706
2020-02-25T00:13:07.8623405Z ...i................................................................................................ 5800/9706
2020-02-25T00:13:12.7791678Z .................................................................................................... 5900/9706
2020-02-25T00:13:12.7791678Z .................................................................................................... 5900/9706
2020-02-25T00:13:21.5542119Z ..............................................................................................ii...i 6000/9706
2020-02-25T00:13:32.1861563Z ..ii...........i.................................................................................... 6100/9706
2020-02-25T00:13:46.7463686Z .................................................................................................... 6300/9706
2020-02-25T00:13:52.3034617Z .................................................................................................... 6400/9706
2020-02-25T00:13:52.3034617Z .................................................................................................... 6400/9706
2020-02-25T00:14:12.7682778Z .........................i..ii...................................................................... 6500/9706
2020-02-25T00:14:30.0683158Z .................................................................................................... 6700/9706
2020-02-25T00:14:32.0080828Z .................i.................................................................................. 6800/9706
2020-02-25T00:14:33.8835321Z .................................................................................................... 6900/9706
2020-02-25T00:14:35.8103449Z ...............................................i.................................................... 7000/9706
---
2020-02-25T00:16:04.1253613Z .................................................................................................... 7700/9706
2020-02-25T00:16:08.4505026Z .................................................................................................... 7800/9706
2020-02-25T00:16:14.4510716Z .............................................................................................i...... 7900/9706
2020-02-25T00:16:21.8833566Z .................................................................................................... 8000/9706
2020-02-25T00:16:28.3814920Z ..........................................iiiiiii.i................................................. 8100/9706
2020-02-25T00:16:41.0393461Z .................................................................................................... 8300/9706
2020-02-25T00:16:46.0817411Z .................................................................................................... 8400/9706
2020-02-25T00:16:59.6267898Z .................................................................................................... 8500/9706
2020-02-25T00:17:06.1621378Z .................................................................................................... 8600/9706
---
2020-02-25T00:19:11.0623621Z  finished in 6.342
2020-02-25T00:19:11.0798205Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:11.2594427Z 
2020-02-25T00:19:11.2595861Z running 178 tests
2020-02-25T00:19:13.7586969Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-25T00:19:15.7546491Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-25T00:19:15.7548504Z 
2020-02-25T00:19:15.7609559Z  finished in 4.675
2020-02-25T00:19:15.7723174Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:15.9230623Z 
---
2020-02-25T00:19:17.5839334Z  finished in 1.811
2020-02-25T00:19:17.6012599Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:17.7284990Z 
2020-02-25T00:19:17.7285196Z running 9 tests
2020-02-25T00:19:17.7286166Z iiiiiiiii
2020-02-25T00:19:17.7287045Z 
2020-02-25T00:19:17.7290626Z  finished in 0.127
2020-02-25T00:19:17.7430653Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:17.9104409Z 
---
2020-02-25T00:19:34.9298112Z  finished in 17.186
2020-02-25T00:19:34.9474386Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:35.1116907Z 
2020-02-25T00:19:35.1117331Z running 116 tests
2020-02-25T00:19:46.9908654Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-25T00:19:48.6577568Z ....iiii.....ii.
2020-02-25T00:19:48.6585245Z 
2020-02-25T00:19:48.6589229Z  finished in 13.711
2020-02-25T00:19:48.6596401Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-25T00:19:48.6597056Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-25T00:26:09.9461639Z ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
2020-02-25T00:26:09.9461809Z 
2020-02-25T00:26:09.9461944Z error: rustdoc failed!
2020-02-25T00:26:09.9462108Z status: exit code: 1
2020-02-25T00:26:09.9463526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/src/test/rustdoc/playground-arg.rs" "--playground-url=https://example.com/" "-Z" "unstable-options"
2020-02-25T00:26:09.9464733Z ------------------------------------------
2020-02-25T00:26:09.9464881Z 
2020-02-25T00:26:09.9465192Z ------------------------------------------
2020-02-25T00:26:09.9465364Z stderr:
2020-02-25T00:26:09.9465364Z stderr:
2020-02-25T00:26:09.9465640Z ------------------------------------------
2020-02-25T00:26:09.9466333Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-25T00:26:09.9467110Z 
2020-02-25T00:26:09.9467404Z ------------------------------------------
2020-02-25T00:26:09.9467546Z 
2020-02-25T00:26:09.9467644Z 
2020-02-25T00:26:09.9467644Z 
2020-02-25T00:26:09.9467968Z ---- [rustdoc] rustdoc/process-termination.rs stdout ----
2020-02-25T00:26:09.9468133Z 
2020-02-25T00:26:09.9468262Z error: rustdoc failed!
2020-02-25T00:26:09.9468448Z status: exit code: 101
2020-02-25T00:26:09.9469748Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
2020-02-25T00:26:09.9471059Z ------------------------------------------
2020-02-25T00:26:09.9471192Z 
2020-02-25T00:26:09.9471317Z running 3 tests
2020-02-25T00:26:09.9471735Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2020-02-25T00:26:09.9471735Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... FAILED
2020-02-25T00:26:09.9472309Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
2020-02-25T00:26:09.9472859Z test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) ... ok
2020-02-25T00:26:09.9473070Z 
2020-02-25T00:26:09.9473169Z failures:
2020-02-25T00:26:09.9473254Z 
2020-02-25T00:26:09.9473670Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) stdout ----
2020-02-25T00:26:09.9474413Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-25T00:26:09.9475077Z 
2020-02-25T00:26:09.9475473Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2020-02-25T00:26:09.9475473Z ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
2020-02-25T00:26:09.9476221Z thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-25T00:26:09.9476645Z 
2020-02-25T00:26:09.9476922Z failures:
2020-02-25T00:26:09.9477346Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14)
2020-02-25T00:26:09.9477865Z     /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20)
---
2020-02-25T00:26:09.9482631Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-25T00:26:09.9482948Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-25T00:26:09.9483125Z 
2020-02-25T00:26:09.9483216Z 
2020-02-25T00:26:09.9488720Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-25T00:26:09.9491382Z 
2020-02-25T00:26:09.9491462Z 
2020-02-25T00:26:09.9491651Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-25T00:26:09.9491942Z Build completed unsuccessfully in 1:07:19
2020-02-25T00:26:09.9491942Z Build completed unsuccessfully in 1:07:19
2020-02-25T00:26:09.9522683Z == clock drift check ==
2020-02-25T00:26:09.9538849Z   local time: Tue Feb 25 00:26:09 UTC 2020
2020-02-25T00:26:10.2433978Z   network time: Tue, 25 Feb 2020 00:26:10 GMT
2020-02-25T00:26:10.2434867Z == end clock drift check ==
2020-02-25T00:26:11.6555318Z 
2020-02-25T00:26:11.6621704Z ##[error]Bash exited with code '1'.
2020-02-25T00:26:11.6632476Z ##[section]Finishing: Run build
2020-02-25T00:26:11.6673694Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-25T00:26:11.6677762Z Task         : Get sources
2020-02-25T00:26:11.6678023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T00:26:11.6678263Z Version      : 1.0.0
2020-02-25T00:26:11.6678445Z Author       : Microsoft
2020-02-25T00:26:11.6678445Z Author       : Microsoft
2020-02-25T00:26:11.6678725Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T00:26:11.6679034Z ==============================================================================
2020-02-25T00:26:11.9686881Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T00:26:11.9732197Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69445/merge to s
2020-02-25T00:26:11.9808543Z Cleaning up task key
2020-02-25T00:26:11.9809835Z Start cleaning up orphan processes.
2020-02-25T00:26:12.0014993Z Terminate orphan process: pid (4297) (python)
2020-02-25T00:26:12.0214961Z ##[section]Finishing: Finalize Job
