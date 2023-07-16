plain
2020-03-20T20:45:38.3310430Z ========================== Starting Command Output ===========================
2020-03-20T20:45:38.3313975Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8b3cd783-f78f-4a28-b6f4-d52e7cbe7b3f.sh
2020-03-20T20:45:38.3314264Z 
2020-03-20T20:45:38.3318537Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T20:45:38.3338162Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70200/merge to s
2020-03-20T20:45:38.3341590Z Task         : Get sources
2020-03-20T20:45:38.3341916Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T20:45:38.3342231Z Version      : 1.0.0
2020-03-20T20:45:38.3342443Z Author       : Microsoft
---
2020-03-20T20:45:39.5657333Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T20:45:39.5669858Z ##[command]git config gc.auto 0
2020-03-20T20:45:39.5680220Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T20:45:39.5689266Z ##[command]git config --get-all http.proxy
2020-03-20T20:45:39.5709150Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70200/merge:refs/remotes/pull/70200/merge
---
2020-03-20T20:49:26.0550184Z Successfully built c61914066be7
2020-03-20T20:49:26.0597367Z Successfully tagged rust-ci:latest
2020-03-20T20:49:26.0858384Z Built container sha256:c61914066be78cfdfe996802f44ad70e26787b7c92b53f6ce1e59a4f33fbfba3
2020-03-20T20:49:26.0875303Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/fc0b9a6e3051d635da7d94215f84d5e9794477aa0cb68449a3fa307fcd796c24419e8a5e04fb3ebd554f2f8d17623f528e7a0f3c6d8c2a3324e25c8632491b33
2020-03-20T20:50:14.4549807Z upload failed: - to s3://rust-lang-ci-sccache2/docker/fc0b9a6e3051d635da7d94215f84d5e9794477aa0cb68449a3fa307fcd796c24419e8a5e04fb3ebd554f2f8d17623f528e7a0f3c6d8c2a3324e25c8632491b33 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-03-20T20:50:14.9272509Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-03-20T20:50:14.9323688Z == clock drift check ==
2020-03-20T20:50:14.9334562Z   local time: Fri Mar 20 20:50:14 UTC 2020
2020-03-20T20:50:15.0972087Z   network time: Fri, 20 Mar 2020 20:50:15 GMT
---
2020-03-20T21:43:35.7680188Z .................................................................................................... 1700/9804
2020-03-20T21:43:40.3249928Z .................................................................................................... 1800/9804
2020-03-20T21:43:52.0532005Z ..........................................................................i......................... 1900/9804
2020-03-20T21:43:58.6602909Z .................................................................................................... 2000/9804
2020-03-20T21:44:06.9249154Z ................................................................iiiii............................... 2100/9804
2020-03-20T21:44:24.5041623Z .................................................................................................... 2300/9804
2020-03-20T21:44:26.8321835Z .................................................................................................... 2400/9804
2020-03-20T21:44:29.8761222Z .................................................................................................... 2500/9804
2020-03-20T21:44:49.8197493Z .................................................................................................... 2600/9804
---
2020-03-20T21:47:29.5862583Z .....................................i...............i.............................................. 5000/9804
2020-03-20T21:47:38.7479664Z .................................................................................................... 5100/9804
2020-03-20T21:47:45.0946831Z ................................................................................i................... 5200/9804
2020-03-20T21:47:50.6222679Z .................................................................................................... 5300/9804
2020-03-20T21:48:00.6710080Z .............................................................ii.ii........i...i..................... 5400/9804
2020-03-20T21:48:08.8007644Z i................................................................................................... 5600/9804
2020-03-20T21:48:18.0933704Z .....i.............................................................................................. 5700/9804
2020-03-20T21:48:24.3744943Z ........................................................i........................................... 5800/9804
2020-03-20T21:48:30.8696270Z .................................................................................................... 5900/9804
2020-03-20T21:48:30.8696270Z .................................................................................................... 5900/9804
2020-03-20T21:48:38.9981684Z .................................................................................................... 6000/9804
2020-03-20T21:48:46.8272635Z ..................................................ii...i..ii...........i............................ 6100/9804
2020-03-20T21:49:06.7459154Z .................................................................................................... 6300/9804
2020-03-20T21:49:11.0760740Z .................................................................................................... 6400/9804
2020-03-20T21:49:11.0760740Z .................................................................................................... 6400/9804
2020-03-20T21:49:14.9443313Z ................................................................................i..ii............... 6500/9804
2020-03-20T21:49:36.9320868Z .................................................................................................... 6700/9804
2020-03-20T21:49:46.2516434Z ...............................................................................i.................... 6800/9804
2020-03-20T21:49:48.3025906Z .................................................................................................... 6900/9804
2020-03-20T21:49:50.3163400Z .................................................................................................... 7000/9804
---
2020-03-20T21:51:30.3386602Z .................................................................................................... 7800/9804
2020-03-20T21:51:35.1657896Z .................................................................................................... 7900/9804
2020-03-20T21:51:40.9315756Z ..................................................................i................................. 8000/9804
2020-03-20T21:51:50.7889059Z .................................................................................................... 8100/9804
2020-03-20T21:51:56.0585550Z ................iiiiiiiiiii......................................................................... 8200/9804
2020-03-20T21:52:09.5567661Z .................................................................................................... 8400/9804
2020-03-20T21:52:15.5823365Z .................................................................................................... 8500/9804
2020-03-20T21:52:30.0853134Z .................................................................................................... 8600/9804
2020-03-20T21:52:36.4638811Z .................................................................................................... 8700/9804
---
2020-03-20T21:54:52.8567934Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-20T21:54:52.8755997Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-20T21:54:53.0951936Z 
2020-03-20T21:54:53.0952934Z running 183 tests
2020-03-20T21:54:55.9823261Z iiii......i............ii.i..iiii....i....i...........i.............i.i..................i....i..... 100/183
2020-03-20T21:54:58.5533852Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-20T21:54:58.5536717Z 
2020-03-20T21:54:58.5536906Z  finished in 5.677
2020-03-20T21:54:58.5540931Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-20T21:54:58.5727622Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-20T21:55:00.7756867Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-20T21:55:00.7950342Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-20T21:55:00.9523886Z 
2020-03-20T21:55:00.9524226Z running 9 tests
2020-03-20T21:55:00.9525367Z iiiiiiiii
2020-03-20T21:55:00.9526769Z 
2020-03-20T21:55:00.9531569Z  finished in 0.158
2020-03-20T21:55:00.9539933Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-20T21:55:00.9731523Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-20T21:55:21.2813521Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-20T21:55:21.3071191Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-20T21:55:21.5249830Z 
2020-03-20T21:55:21.5250108Z running 115 tests
2020-03-20T21:55:34.8961676Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-20T21:55:36.4433196Z ...iiii.....ii.
2020-03-20T21:55:36.4435424Z 
2020-03-20T21:55:36.4440831Z  finished in 15.137
2020-03-20T21:55:36.4447584Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-20T21:55:36.4450842Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-20T21:56:05.1565660Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2020-03-20T21:56:05.1565882Z 
2020-03-20T21:56:05.1566326Z error: test compilation failed although it shouldn't!
2020-03-20T21:56:05.1566608Z status: exit code: 1
2020-03-20T21:56:05.1568610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2020-03-20T21:56:05.1570273Z ------------------------------------------
2020-03-20T21:56:05.1570477Z 
2020-03-20T21:56:05.1570851Z ------------------------------------------
2020-03-20T21:56:05.1571065Z stderr:
---
2020-03-20T21:56:05.1578384Z test result: FAILED. 63 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-20T21:56:05.1578666Z 
2020-03-20T21:56:05.1578959Z 
2020-03-20T21:56:05.1579142Z 
2020-03-20T21:56:05.1583164Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-20T21:56:05.1587503Z 
2020-03-20T21:56:05.1587611Z 
2020-03-20T21:56:05.1588206Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-20T21:56:05.1588665Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-20T21:56:05.1588665Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-20T21:56:05.1593969Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-20T21:56:05.1594615Z Build completed unsuccessfully in 1:04:00
2020-03-20T21:56:05.1622782Z == clock drift check ==
2020-03-20T21:56:05.1644103Z   local time: Fri Mar 20 21:56:05 UTC 2020
2020-03-20T21:56:05.4605649Z   network time: Fri, 20 Mar 2020 21:56:05 GMT
2020-03-20T21:56:05.4606225Z == end clock drift check ==
2020-03-20T21:56:06.3810911Z 
2020-03-20T21:56:06.3920058Z ##[error]Bash exited with code '1'.
2020-03-20T21:56:06.3944146Z ##[section]Finishing: Run build
2020-03-20T21:56:06.4006718Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70200/merge to s
2020-03-20T21:56:06.4012763Z Task         : Get sources
2020-03-20T21:56:06.4013152Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T21:56:06.4013493Z Version      : 1.0.0
2020-03-20T21:56:06.4013737Z Author       : Microsoft
2020-03-20T21:56:06.4013737Z Author       : Microsoft
2020-03-20T21:56:06.4014148Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T21:56:06.4014583Z ==============================================================================
2020-03-20T21:56:06.7425481Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T21:56:06.7488702Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70200/merge to s
2020-03-20T21:56:06.7585244Z Cleaning up task key
2020-03-20T21:56:06.7586608Z Start cleaning up orphan processes.
2020-03-20T21:56:06.7766746Z Terminate orphan process: pid (3590) (python)
2020-03-20T21:56:06.7995636Z ##[section]Finishing: Finalize Job
