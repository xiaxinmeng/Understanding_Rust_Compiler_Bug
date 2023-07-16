plain
2020-04-22T21:29:03.7527948Z ========================== Starting Command Output ===========================
2020-04-22T21:29:03.7530754Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad008096-cc5e-447e-93d6-bb3f388a6bc1.sh
2020-04-22T21:29:03.7531043Z 
2020-04-22T21:29:03.7535825Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T21:29:03.7556358Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T21:29:03.7559950Z Task         : Get sources
2020-04-22T21:29:03.7560281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T21:29:03.7560605Z Version      : 1.0.0
2020-04-22T21:29:03.7560843Z Author       : Microsoft
---
2020-04-22T21:29:04.7505973Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T21:29:04.7510517Z ##[command]git config gc.auto 0
2020-04-22T21:29:04.7513355Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T21:29:04.7515935Z ##[command]git config --get-all http.proxy
2020-04-22T21:29:04.7520631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71439/merge:refs/remotes/pull/71439/merge
---
2020-04-22T21:31:48.3270868Z  ---> 318032b5f0e2
2020-04-22T21:31:48.3271403Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T21:31:48.3271818Z  ---> Using cache
2020-04-22T21:31:48.3272038Z  ---> d44a858fd1ce
2020-04-22T21:31:48.3272677Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T21:31:48.3273362Z  ---> 58b910f50f5a
2020-04-22T21:31:48.3273517Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T21:31:48.3274139Z  ---> Using cache
2020-04-22T21:31:48.3274362Z  ---> ee7702aadba1
---
2020-04-22T21:31:48.3563058Z Looks like docker image is the same as before, not uploading
2020-04-22T21:31:56.0319794Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T21:31:56.0522307Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T21:31:56.0545550Z == clock drift check ==
2020-04-22T21:31:56.0591196Z   local time: Wed Apr 22 21:31:56 UTC 2020
2020-04-22T21:31:56.3437590Z   network time: Wed, 22 Apr 2020 21:31:56 GMT
2020-04-22T21:31:56.3460077Z Starting sccache server...
2020-04-22T21:31:56.4087419Z configure: processing command line
2020-04-22T21:31:56.4087798Z configure: 
2020-04-22T21:31:56.4088582Z configure: rust.dist-src        := False
---
2020-04-22T21:33:43.0476536Z    Compiling unicode-width v0.1.6
2020-04-22T21:33:43.1163446Z    Compiling getopts v0.2.21
2020-04-22T21:33:50.4854185Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-22T21:33:56.4807147Z     Finished release [optimized] target(s) in 44.74s
2020-04-22T21:33:56.4807878Z {"reason":"build-finished","success":true}
2020-04-22T21:33:56.5001402Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T21:33:56.9336960Z    Compiling cfg-if v0.1.10
2020-04-22T21:33:56.9342052Z    Compiling libc v0.2.69
2020-04-22T21:33:56.9705464Z    Compiling semver-parser v0.7.0
---
2020-04-22T21:35:47.4823168Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T21:35:48.5562071Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T21:35:49.6964093Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T21:35:50.6588313Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T21:35:56.8782031Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T21:35:58.5880867Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T21:36:01.7493551Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T21:36:04.7475254Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T21:36:11.4602022Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T21:46:43.4791614Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-22T21:47:05.5605553Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-22T21:49:12.6827542Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-22T21:49:13.1398403Z     Finished release [optimized] target(s) in 15m 16s
2020-04-22T21:49:13.1399467Z {"reason":"build-finished","success":true}
2020-04-22T21:49:13.1832310Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-22T21:49:13.1844465Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T21:49:13.4111740Z    Compiling cc v1.0.50
2020-04-22T21:49:13.4113551Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-22T21:49:48.4339587Z    Compiling unicode-width v0.1.6
2020-04-22T21:49:48.5052910Z    Compiling getopts v0.2.21
2020-04-22T21:49:57.8845868Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-22T21:50:04.6198272Z     Finished release [optimized] target(s) in 51.43s
2020-04-22T21:50:04.6201130Z {"reason":"build-finished","success":true}
2020-04-22T21:50:04.6315755Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T21:50:05.0488150Z    Compiling cfg-if v0.1.10
2020-04-22T21:50:05.0489289Z    Compiling libc v0.2.69
2020-04-22T21:50:05.0818745Z    Compiling semver-parser v0.7.0
---
2020-04-22T21:52:05.6635315Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T21:52:06.8122811Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T21:52:08.1672268Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T21:52:08.2085633Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T21:52:16.1132919Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T21:52:17.2348410Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T21:52:20.6271312Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T21:52:23.7888771Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T21:52:31.4461503Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T22:02:11.3876679Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-04-22T22:02:55.7770249Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-22T22:03:18.5103872Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-22T22:06:21.4726364Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-22T22:06:21.9422604Z {"reason":"build-finished","success":true}
2020-04-22T22:06:21.9797266Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-22T22:06:21.9825834Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-22T22:06:21.9836527Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T22:06:21.9837673Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-22T22:06:51.3108204Z    Compiling serde_derive v1.0.81
2020-04-22T22:07:10.7191662Z    Compiling serde_json v1.0.40
2020-04-22T22:07:11.7318690Z    Compiling rustfix v0.5.0
2020-04-22T22:07:13.9282154Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-22T22:07:23.7844228Z   {"reason":"build-finished","success":true}
2020-04-22T22:07:23.8080756Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T22:07:26.8407319Z 
2020-04-22T22:07:26.8407570Z running 9916 tests
2020-04-22T22:07:36.4800882Z .................................................................................................... 100/9916
---
2020-04-22T22:08:55.2730390Z .................................................................................................... 1700/9916
2020-04-22T22:08:58.4138700Z .................................................................................................... 1800/9916
2020-04-22T22:09:04.4598140Z .................................................................................................... 1900/9916
2020-04-22T22:09:10.8052999Z .....i.............................................................................................. 2000/9916
2020-04-22T22:09:15.3711811Z ...............................................................................................iiiii 2100/9916
2020-04-22T22:09:29.6131049Z .................................................................................................... 2300/9916
2020-04-22T22:09:31.2456411Z .................................................................................................... 2400/9916
2020-04-22T22:09:32.9339596Z .................................................................................................... 2500/9916
2020-04-22T22:09:37.0899236Z .................................................................................................... 2600/9916
---
2020-04-22T22:11:45.9675332Z .................................................................................................... 5100/9916
2020-04-22T22:11:51.2166291Z .................................................................................................... 5200/9916
2020-04-22T22:11:54.5349948Z ..................i................................................................................. 5300/9916
2020-04-22T22:12:01.5616587Z ........i........................................................................................... 5400/9916
2020-04-22T22:12:05.5841751Z ........ii.ii........i...i.......................................................................... 5500/9916
2020-04-22T22:12:11.2590380Z .......................................................i............................................ 5700/9916
2020-04-22T22:12:17.5888620Z ........................................................................................ii.......... 5800/9916
2020-04-22T22:12:22.3588577Z ............F..F...........i........................................................................ 5900/9916
2020-04-22T22:12:26.3298255Z .................................................................................................... 6000/9916
2020-04-22T22:12:26.3298255Z .................................................................................................... 6000/9916
2020-04-22T22:12:33.9736685Z .................................................................................................... 6100/9916
2020-04-22T22:12:41.4355657Z .....................ii...i..ii...........i......................................................... 6200/9916
2020-04-22T22:12:53.4897391Z .................................................................................................... 6400/9916
2020-04-22T22:12:56.2481858Z .................................................................................................... 6500/9916
2020-04-22T22:12:56.2481858Z .................................................................................................... 6500/9916
2020-04-22T22:13:02.5010701Z ...................................................i..ii............................................ 6600/9916
2020-04-22T22:13:19.3965132Z .................................................................................................... 6800/9916
2020-04-22T22:13:21.1408651Z ....................................................i............................................... 6900/9916
2020-04-22T22:13:22.6322148Z .................................................................................................... 7000/9916
2020-04-22T22:13:24.1503053Z ............................................................................................i....... 7100/9916
---
2020-04-22T22:14:33.9509692Z .................................................................................................... 7900/9916
2020-04-22T22:14:38.3414821Z .................................................................................................... 8000/9916
2020-04-22T22:14:42.5634716Z ..........................................................i......................................... 8100/9916
2020-04-22T22:14:49.8925665Z .................................................................................................... 8200/9916
2020-04-22T22:14:53.7046733Z .......iiiiii.iiiii.i............................................................................... 8300/9916
2020-04-22T22:15:03.5640344Z .................................................................................................... 8500/9916
2020-04-22T22:15:09.1123099Z .................................................................................................... 8600/9916
2020-04-22T22:15:18.4874504Z .................................................................................................... 8700/9916
2020-04-22T22:15:23.2022254Z .................................................................................................... 8800/9916
---
2020-04-22T22:16:43.7731725Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#core stdout ----
2020-04-22T22:16:43.7731898Z 
2020-04-22T22:16:43.7732319Z error in revision `core`: test compilation failed although it shouldn't!
2020-04-22T22:16:43.7732515Z status: exit code: 1
2020-04-22T22:16:43.7733876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
2020-04-22T22:16:43.7734920Z ------------------------------------------
2020-04-22T22:16:43.7735032Z 
2020-04-22T22:16:43.7735275Z ------------------------------------------
2020-04-22T22:16:43.7735422Z stderr:
---
2020-04-22T22:16:43.7738060Z 
2020-04-22T22:16:43.7738124Z 
2020-04-22T22:16:43.7738398Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#std stdout ----
2020-04-22T22:16:43.7738528Z 
2020-04-22T22:16:43.7738835Z error in revision `std`: test compilation failed although it shouldn't!
2020-04-22T22:16:43.7739187Z status: exit code: 1
2020-04-22T22:16:43.7740559Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
2020-04-22T22:16:43.7741620Z ------------------------------------------
2020-04-22T22:16:43.7741730Z 
2020-04-22T22:16:43.7741969Z ------------------------------------------
2020-04-22T22:16:43.7742100Z stderr:
---
2020-04-22T22:16:43.7769458Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T22:16:43.7769921Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T22:16:43.7778595Z 
2020-04-22T22:16:43.7778734Z 
2020-04-22T22:16:43.7784374Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T22:16:43.7786034Z 
2020-04-22T22:16:43.7786115Z 
2020-04-22T22:16:43.7796852Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T22:16:43.7797269Z Build completed unsuccessfully in 0:43:33
2020-04-22T22:16:43.7797269Z Build completed unsuccessfully in 0:43:33
2020-04-22T22:16:43.7842333Z == clock drift check ==
2020-04-22T22:16:43.7857841Z   local time: Wed Apr 22 22:16:43 UTC 2020
2020-04-22T22:16:44.0793475Z   network time: Wed, 22 Apr 2020 22:16:44 GMT
2020-04-22T22:16:45.0048208Z 
2020-04-22T22:16:45.0048208Z 
2020-04-22T22:16:45.0125212Z ##[error]Bash exited with code '1'.
2020-04-22T22:16:45.0139170Z ##[section]Finishing: Run build
2020-04-22T22:16:45.0176810Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T22:16:45.0180986Z Task         : Get sources
2020-04-22T22:16:45.0181237Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T22:16:45.0181486Z Version      : 1.0.0
2020-04-22T22:16:45.0181663Z Author       : Microsoft
2020-04-22T22:16:45.0181663Z Author       : Microsoft
2020-04-22T22:16:45.0181931Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T22:16:45.0182239Z ==============================================================================
2020-04-22T22:16:45.2746063Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T22:16:45.2781378Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T22:16:45.2844204Z Cleaning up task key
2020-04-22T22:16:45.2845144Z Start cleaning up orphan processes.
2020-04-22T22:16:45.2974135Z Terminate orphan process: pid (3890) (python)
2020-04-22T22:16:45.3170660Z ##[section]Finishing: Finalize Job
