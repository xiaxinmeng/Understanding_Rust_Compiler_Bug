plain
2020-03-24T13:36:59.5992714Z ========================== Starting Command Output ===========================
2020-03-24T13:36:59.5995203Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ef88b8ea-8ebd-4054-bd77-106bd1edfc59.sh
2020-03-24T13:36:59.5995486Z 
2020-03-24T13:36:59.5999337Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T13:36:59.6022536Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T13:36:59.6028091Z Task         : Get sources
2020-03-24T13:36:59.6028399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T13:36:59.6028697Z Version      : 1.0.0
2020-03-24T13:36:59.6028898Z Author       : Microsoft
---
2020-03-24T13:37:00.5946225Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T13:37:00.5951484Z ##[command]git config gc.auto 0
2020-03-24T13:37:00.5955764Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T13:37:00.5960487Z ##[command]git config --get-all http.proxy
2020-03-24T13:37:00.5968981Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70352/merge:refs/remotes/pull/70352/merge
---
2020-03-24T13:44:39.8864960Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T13:44:39.9229316Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T13:44:50.0129180Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T13:44:57.7547361Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T13:45:01.6913433Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T13:45:05.2502459Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T13:45:39.0433554Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T13:45:47.4597795Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T13:46:37.0358520Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T14:06:35.5633070Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T14:06:37.1936377Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T14:06:47.9042698Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T14:06:59.4069187Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T14:07:04.7027659Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T14:07:06.7295428Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T14:07:49.6320456Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T14:07:59.7452911Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T14:09:04.2160818Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T14:32:21.9086046Z .................................................................................................... 1700/9832
2020-03-24T14:32:26.0291568Z .................................................................................................... 1800/9832
2020-03-24T14:32:35.9643617Z .......................................................................................i............ 1900/9832
2020-03-24T14:32:42.8698569Z .................................................................................................... 2000/9832
2020-03-24T14:32:48.9265703Z .............................................................................iiiii.................. 2100/9832
2020-03-24T14:33:09.8313984Z .................................................................................................... 2300/9832
2020-03-24T14:33:12.0158063Z .................................................................................................... 2400/9832
2020-03-24T14:33:14.4556482Z .................................................................................................... 2500/9832
2020-03-24T14:33:23.6442860Z .................................................................................................... 2600/9832
---
2020-03-24T14:36:16.2213458Z ....................................................i...............i............................... 5000/9832
2020-03-24T14:36:24.0752190Z .................................................................................................... 5100/9832
2020-03-24T14:36:31.5396181Z .................................................................................................i.. 5200/9832
2020-03-24T14:36:36.6469693Z .................................................................................................... 5300/9832
2020-03-24T14:36:47.2319720Z ................................................................................ii.ii........i...i.. 5400/9832
2020-03-24T14:36:54.6908122Z ....................i............................................................................... 5600/9832
2020-03-24T14:37:02.0996204Z .........................i.......................................................................... 5700/9832
2020-03-24T14:37:10.0553601Z ..........................................ii....................................i................... 5800/9832
2020-03-24T14:37:17.3381964Z .................................................................................................... 5900/9832
2020-03-24T14:37:17.3381964Z .................................................................................................... 5900/9832
2020-03-24T14:37:22.9154671Z .................................................................................................... 6000/9832
2020-03-24T14:37:32.2153247Z ..........................................................................ii....i.ii...........i.... 6100/9832
2020-03-24T14:37:52.4389597Z .................................................................................................... 6300/9832
2020-03-24T14:37:59.5847296Z .................................................................................................... 6400/9832
2020-03-24T14:38:03.5692750Z .................................................................................................... 6500/9832
2020-03-24T14:38:03.5692750Z .................................................................................................... 6500/9832
2020-03-24T14:38:15.5372059Z ....i..ii........................................................................................... 6600/9832
2020-03-24T14:38:35.3705737Z .................................................................................................... 6800/9832
2020-03-24T14:38:37.4681964Z ...i................................................................................................ 6900/9832
2020-03-24T14:38:39.5107348Z .................................................................................................... 7000/9832
2020-03-24T14:38:41.8603435Z ......................................i............................................................. 7100/9832
---
2020-03-24T14:40:19.8957220Z .................................................................................................... 7800/9832
2020-03-24T14:40:24.6445022Z .................................................................................................... 7900/9832
2020-03-24T14:40:31.3332718Z ..............................................................................................i..... 8000/9832
2020-03-24T14:40:39.2479057Z .................................................................................................... 8100/9832
2020-03-24T14:40:46.4647320Z ...........................................iiiiiiiiii.i............................................. 8200/9832
2020-03-24T14:41:00.1607904Z .................................................................................................... 8400/9832
2020-03-24T14:41:05.2869228Z .................................................................................................... 8500/9832
2020-03-24T14:41:18.3930423Z .................................................................................................... 8600/9832
2020-03-24T14:41:27.3539741Z .................................................................................................... 8700/9832
---
2020-03-24T14:43:19.6725813Z 
2020-03-24T14:43:19.6725923Z 
2020-03-24T14:43:19.6726170Z The actual stderr differed from the expected stderr.
2020-03-24T14:43:19.6726883Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/tool_lints.stderr
2020-03-24T14:43:19.6727589Z To update references, rerun the tests and pass the `--bless` flag
2020-03-24T14:43:19.6728211Z To only update this specific test, also pass `--test-args tool_lints.rs`
2020-03-24T14:43:19.6728705Z error: 1 errors occurred comparing output.
2020-03-24T14:43:19.6728978Z status: exit code: 1
2020-03-24T14:43:19.6728978Z status: exit code: 1
2020-03-24T14:43:19.6731002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary"
2020-03-24T14:43:19.6732762Z ------------------------------------------
2020-03-24T14:43:19.6732979Z 
2020-03-24T14:43:19.6733383Z ------------------------------------------
2020-03-24T14:43:19.6733615Z stderr:
2020-03-24T14:43:19.6733615Z stderr:
2020-03-24T14:43:19.6734043Z ------------------------------------------
2020-03-24T14:43:19.6734433Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T14:43:19.6735028Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T14:43:19.6735286Z    |
2020-03-24T14:43:19.6735482Z LL | #[warn(foo::bar)]
2020-03-24T14:43:19.6735838Z 
2020-03-24T14:43:19.6736148Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T14:43:19.6736749Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T14:43:19.6736994Z    |
2020-03-24T14:43:19.6736994Z    |
2020-03-24T14:43:19.6737202Z LL | #[warn(foo::bar)]
2020-03-24T14:43:19.6737564Z 
2020-03-24T14:43:19.6737865Z error[E0710]: an unknown tool name found in scoped lint: `foo::bar`
2020-03-24T14:43:19.6739228Z   --> /checkout/src/test/ui/tool_lints.rs:1:8
2020-03-24T14:43:19.6739469Z    |
2020-03-24T14:43:19.6739469Z    |
2020-03-24T14:43:19.6739660Z LL | #[warn(foo::bar)]
2020-03-24T14:43:19.6740034Z 
2020-03-24T14:43:19.6740249Z error: aborting due to 3 previous errors
2020-03-24T14:43:19.6740443Z 
2020-03-24T14:43:19.6740944Z For more information about this error, try `rustc --explain E0710`.
---
2020-03-24T14:43:19.6763464Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T14:43:19.6764177Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-24T14:43:19.6773518Z 
2020-03-24T14:43:19.6773827Z 
2020-03-24T14:43:19.6778216Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T14:43:19.6781523Z 
2020-03-24T14:43:19.6781731Z 
2020-03-24T14:43:19.6784689Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T14:43:19.6785250Z Build completed unsuccessfully in 1:02:04
2020-03-24T14:43:19.6785250Z Build completed unsuccessfully in 1:02:04
2020-03-24T14:43:19.6841673Z == clock drift check ==
2020-03-24T14:43:19.6861830Z   local time: Tue Mar 24 14:43:19 UTC 2020
2020-03-24T14:43:19.9819308Z   network time: Tue, 24 Mar 2020 14:43:19 GMT
2020-03-24T14:43:19.9823903Z == end clock drift check ==
2020-03-24T14:43:20.4307448Z 
2020-03-24T14:43:20.4383978Z ##[error]Bash exited with code '1'.
2020-03-24T14:43:20.4398610Z ##[section]Finishing: Run build
2020-03-24T14:43:20.4459357Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T14:43:20.4464505Z Task         : Get sources
2020-03-24T14:43:20.4464878Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T14:43:20.4465202Z Version      : 1.0.0
2020-03-24T14:43:20.4465428Z Author       : Microsoft
2020-03-24T14:43:20.4465428Z Author       : Microsoft
2020-03-24T14:43:20.4465809Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T14:43:20.4466224Z ==============================================================================
2020-03-24T14:43:20.7791223Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T14:43:20.7839881Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T14:43:20.7927841Z Cleaning up task key
2020-03-24T14:43:20.7929163Z Start cleaning up orphan processes.
2020-03-24T14:43:20.8102764Z Terminate orphan process: pid (3437) (python)
2020-03-24T14:43:20.8272028Z ##[section]Finishing: Finalize Job
