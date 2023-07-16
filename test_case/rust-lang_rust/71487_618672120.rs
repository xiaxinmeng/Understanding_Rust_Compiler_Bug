plain
2020-04-23T20:06:16.5636968Z ========================== Starting Command Output ===========================
2020-04-23T20:06:16.5640647Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ee2f95fb-796f-4248-bf44-a6bcf6772f68.sh
2020-04-23T20:06:16.5641055Z 
2020-04-23T20:06:16.5645386Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T20:06:16.5663310Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71487/merge to s
2020-04-23T20:06:16.5666422Z Task         : Get sources
2020-04-23T20:06:16.5666705Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T20:06:16.5666979Z Version      : 1.0.0
2020-04-23T20:06:16.5667184Z Author       : Microsoft
---
2020-04-23T20:06:17.5548926Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T20:06:17.5554831Z ##[command]git config gc.auto 0
2020-04-23T20:06:17.5559517Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T20:06:17.5564687Z ##[command]git config --get-all http.proxy
2020-04-23T20:06:17.5572414Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71487/merge:refs/remotes/pull/71487/merge
---
2020-04-23T20:09:30.1897730Z  ---> 318032b5f0e2
2020-04-23T20:09:30.1898849Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T20:09:30.1900761Z  ---> Using cache
2020-04-23T20:09:30.1902931Z  ---> d44a858fd1ce
2020-04-23T20:09:30.1904252Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T20:09:30.1905836Z  ---> 58b910f50f5a
2020-04-23T20:09:30.1906189Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T20:09:30.1906671Z  ---> Using cache
2020-04-23T20:09:30.1907155Z  ---> ee7702aadba1
---
2020-04-23T20:09:30.2469784Z Looks like docker image is the same as before, not uploading
2020-04-23T20:09:38.7538827Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T20:09:38.7848091Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T20:09:38.7880005Z == clock drift check ==
2020-04-23T20:09:38.7888015Z   local time: Thu Apr 23 20:09:38 UTC 2020
2020-04-23T20:09:38.8879190Z   network time: Thu, 23 Apr 2020 20:09:38 GMT
2020-04-23T20:09:38.8903945Z Starting sccache server...
2020-04-23T20:09:38.9719658Z configure: processing command line
2020-04-23T20:09:38.9720036Z configure: 
2020-04-23T20:09:38.9721269Z configure: rust.dist-src        := False
---
2020-04-23T20:14:36.7745237Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T20:14:38.2420028Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T20:14:39.7683306Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T20:14:40.5394758Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T20:14:49.1818504Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T20:14:51.3267123Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T20:14:55.4855400Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T20:14:59.5099406Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T20:15:08.5186087Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T20:36:03.6405697Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T20:36:05.1509737Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T20:36:06.7431438Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T20:36:06.9175813Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T20:36:16.7688848Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T20:36:18.9306243Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T20:36:23.3924874Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T20:36:27.6411988Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T20:36:37.3162413Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T20:58:30.3640929Z .................................................................................................... 1700/9918
2020-04-23T20:58:34.4743843Z .................................................................................................... 1800/9918
2020-04-23T20:58:42.5811234Z .................................................................................................... 1900/9918
2020-04-23T20:58:50.2971600Z .....i.............................................................................................. 2000/9918
2020-04-23T20:58:56.2677023Z ...............................................................................................iiiii 2100/9918
2020-04-23T20:59:15.1410456Z .................................................................................................... 2300/9918
2020-04-23T20:59:17.3027127Z .................................................................................................... 2400/9918
2020-04-23T20:59:19.5071373Z .................................................................................................... 2500/9918
2020-04-23T20:59:25.0203694Z .................................................................................................... 2600/9918
---
2020-04-23T21:02:13.1700565Z .................................................................................................... 5100/9918
2020-04-23T21:02:19.9919346Z .................................................................................................... 5200/9918
2020-04-23T21:02:24.3610195Z ..................i................................................................................. 5300/9918
2020-04-23T21:02:33.6806869Z ........i........................................................................................... 5400/9918
2020-04-23T21:02:38.9365883Z ........ii.ii........i...i.......................................................................... 5500/9918
2020-04-23T21:02:46.2276187Z .......................................................i............................................ 5700/9918
2020-04-23T21:02:54.5445753Z ........................................................................................ii.......... 5800/9918
2020-04-23T21:03:01.0252218Z ...........................i........................................................................ 5900/9918
2020-04-23T21:03:06.2009326Z .................................................................................................... 6000/9918
2020-04-23T21:03:06.2009326Z .................................................................................................... 6000/9918
2020-04-23T21:03:16.1325045Z .................................................................................................... 6100/9918
2020-04-23T21:03:25.8191108Z .....................ii...i..ii...........i......................................................... 6200/9918
2020-04-23T21:03:39.8659435Z .................................................................................................... 6400/9918
2020-04-23T21:03:43.2045439Z .................................................................................................... 6500/9918
2020-04-23T21:03:43.2045439Z .................................................................................................... 6500/9918
2020-04-23T21:03:51.3639919Z ...................................................i..ii............................................ 6600/9918
2020-04-23T21:04:13.4766266Z .................................................................................................... 6800/9918
2020-04-23T21:04:15.8010696Z ....................................................i............................................... 6900/9918
2020-04-23T21:04:17.7558789Z .................................................................................................... 7000/9918
2020-04-23T21:04:19.7796871Z ....F........................................................................................i...... 7100/9918
---
2020-04-23T21:05:53.1100202Z .................................................................................................... 7900/9918
2020-04-23T21:05:58.3625828Z .................................................................................................... 8000/9918
2020-04-23T21:06:04.7420159Z ............................................................i....................................... 8100/9918
2020-04-23T21:06:13.9992519Z .................................................................................................... 8200/9918
2020-04-23T21:06:19.3133440Z .........iiiiii.iiiii.i............................................................................. 8300/9918
2020-04-23T21:06:32.6088943Z .................................................................................................... 8500/9918
2020-04-23T21:06:39.5592668Z .................................................................................................... 8600/9918
2020-04-23T21:06:52.2312218Z .................................................................................................... 8700/9918
2020-04-23T21:06:58.2882091Z .................................................................................................... 8800/9918
---
2020-04-23T21:08:40.0939716Z failures:
2020-04-23T21:08:40.0976142Z 
2020-04-23T21:08:40.0976915Z ---- [ui] ui/parser/issue-71471-ignore-tidy.rs stdout ----
2020-04-23T21:08:40.0977184Z normalized stderr:
2020-04-23T21:08:40.0977386Z error: expected `[`, found `B`
2020-04-23T21:08:40.0977828Z   --> $DIR/issue-71471-ignore-tidy.rs:2:3
2020-04-23T21:08:40.0978171Z LL | #!B
2020-04-23T21:08:40.0978325Z    |   ^ expected `[`
2020-04-23T21:08:40.0978465Z 
2020-04-23T21:08:40.0978636Z error: aborting due to previous error
2020-04-23T21:08:40.0978636Z error: aborting due to previous error
2020-04-23T21:08:40.0978789Z 
2020-04-23T21:08:40.0978880Z 
2020-04-23T21:08:40.0978969Z 
2020-04-23T21:08:40.0979257Z 
2020-04-23T21:08:40.0979449Z The actual stderr differed from the expected stderr.
2020-04-23T21:08:40.0980219Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-71471-ignore-tidy/issue-71471-ignore-tidy.stderr
2020-04-23T21:08:40.0980851Z To update references, rerun the tests and pass the `--bless` flag
2020-04-23T21:08:40.0981411Z To only update this specific test, also pass `--test-args parser/issue-71471-ignore-tidy.rs`
2020-04-23T21:08:40.0981820Z error: 1 errors occurred comparing output.
2020-04-23T21:08:40.0982058Z status: exit code: 1
2020-04-23T21:08:40.0982058Z status: exit code: 1
2020-04-23T21:08:40.0983883Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-71471-ignore-tidy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-71471-ignore-tidy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-71471-ignore-tidy/auxiliary"
2020-04-23T21:08:40.0985620Z ------------------------------------------
2020-04-23T21:08:40.0985779Z 
2020-04-23T21:08:40.0986139Z ------------------------------------------
2020-04-23T21:08:40.0986328Z stderr:
2020-04-23T21:08:40.0986328Z stderr:
2020-04-23T21:08:40.0986678Z ------------------------------------------
2020-04-23T21:08:40.0986920Z error: expected `[`, found `B`
2020-04-23T21:08:40.0987384Z   --> /checkout/src/test/ui/parser/issue-71471-ignore-tidy.rs:2:3
2020-04-23T21:08:40.0987618Z    |
2020-04-23T21:08:40.0987809Z LL | #!B //~ expected `[`, found `B`
2020-04-23T21:08:40.0988007Z    |   ^ expected `[`
2020-04-23T21:08:40.0988316Z error: aborting due to previous error
2020-04-23T21:08:40.0988731Z 
2020-04-23T21:08:40.0988828Z 
2020-04-23T21:08:40.0989182Z ------------------------------------------
---
2020-04-23T21:08:40.1006261Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T21:08:40.1006659Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-23T21:08:40.1016689Z 
2020-04-23T21:08:40.1017831Z 
2020-04-23T21:08:40.1021689Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T21:08:40.1024346Z 
2020-04-23T21:08:40.1024533Z 
2020-04-23T21:08:40.1025116Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-23T21:08:40.1025456Z Build completed unsuccessfully in 0:57:26
2020-04-23T21:08:40.1025456Z Build completed unsuccessfully in 0:57:26
2020-04-23T21:08:40.1084677Z == clock drift check ==
2020-04-23T21:08:40.1102158Z   local time: Thu Apr 23 21:08:40 UTC 2020
2020-04-23T21:08:40.2133655Z   network time: Thu, 23 Apr 2020 21:08:40 GMT
2020-04-23T21:08:40.7340917Z 
2020-04-23T21:08:40.7340917Z 
2020-04-23T21:08:40.7412481Z ##[error]Bash exited with code '1'.
2020-04-23T21:08:40.7426924Z ##[section]Finishing: Run build
2020-04-23T21:08:40.7484611Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71487/merge to s
2020-04-23T21:08:40.7489608Z Task         : Get sources
2020-04-23T21:08:40.7489918Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T21:08:40.7490202Z Version      : 1.0.0
2020-04-23T21:08:40.7490421Z Author       : Microsoft
2020-04-23T21:08:40.7490421Z Author       : Microsoft
2020-04-23T21:08:40.7490744Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T21:08:40.7491162Z ==============================================================================
2020-04-23T21:08:41.0835383Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T21:08:41.0881664Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71487/merge to s
2020-04-23T21:08:41.0982305Z Cleaning up task key
2020-04-23T21:08:41.0983623Z Start cleaning up orphan processes.
2020-04-23T21:08:41.1164692Z Terminate orphan process: pid (3758) (python)
2020-04-23T21:08:41.1396052Z ##[section]Finishing: Finalize Job
