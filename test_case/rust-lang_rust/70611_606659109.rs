plain
2020-03-31T13:17:31.3804286Z ========================== Starting Command Output ===========================
2020-03-31T13:17:31.3806701Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/86a7b800-8623-4170-95b2-6d9e92fbe25d.sh
2020-03-31T13:17:31.3806969Z 
2020-03-31T13:17:31.3810679Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T13:17:31.3829938Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-03-31T13:17:31.3833164Z Task         : Get sources
2020-03-31T13:17:31.3833470Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T13:17:31.3833786Z Version      : 1.0.0
2020-03-31T13:17:31.3833988Z Author       : Microsoft
---
2020-03-31T13:17:32.3734470Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T13:17:32.3740713Z ##[command]git config gc.auto 0
2020-03-31T13:17:32.3745534Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T13:17:32.3750280Z ##[command]git config --get-all http.proxy
2020-03-31T13:17:32.3757917Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70611/merge:refs/remotes/pull/70611/merge
---
2020-03-31T13:24:18.9588445Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:24:20.3396364Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:24:21.8645887Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:24:22.0482952Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:24:31.3050127Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:24:32.8504098Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:24:37.0177767Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:24:40.9284860Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:24:50.3541468Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T13:45:55.1181131Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:45:56.7968909Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:45:58.7645692Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:45:59.5086469Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:46:10.5191617Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:46:12.4998292Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:46:17.6352012Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:46:22.9039926Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:46:34.3989751Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T14:11:01.9849144Z .................................................................................................... 1700/9856
2020-03-31T14:11:06.0709968Z .................................................................................................... 1800/9856
2020-03-31T14:11:16.0362834Z ..........................................................................................i......... 1900/9856
2020-03-31T14:11:22.7283679Z .................................................................................................... 2000/9856
2020-03-31T14:11:29.0440377Z ................................................................................iiiii............... 2100/9856
2020-03-31T14:11:49.5385230Z .................................................................................................... 2300/9856
2020-03-31T14:11:51.6951696Z .................................................................................................... 2400/9856
2020-03-31T14:11:54.0665389Z .................................................................................................... 2500/9856
2020-03-31T14:12:00.5365411Z .................................................................................................... 2600/9856
---
2020-03-31T14:14:45.3756974Z ......................................................i...............i............................. 5000/9856
2020-03-31T14:14:52.8896445Z .................................................................................................... 5100/9856
2020-03-31T14:14:59.9029665Z ...................................................................................................i 5200/9856
2020-03-31T14:15:04.7297947Z .................................................................................................... 5300/9856
2020-03-31T14:15:15.0289164Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T14:15:18.5067796Z ..i................................................................................................. 5500/9856
2020-03-31T14:15:27.2555774Z ..............................i..................................................................... 5700/9856
2020-03-31T14:15:36.6867089Z ................................................ii....................................i............. 5800/9856
2020-03-31T14:15:43.8178427Z .................................................................................................... 5900/9856
2020-03-31T14:15:48.6617050Z .................................................................................................... 6000/9856
2020-03-31T14:15:48.6617050Z .................................................................................................... 6000/9856
2020-03-31T14:15:57.5459984Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T14:16:09.4055444Z .i.................................................................................................. 6200/9856
2020-03-31T14:16:21.4260608Z .................................................................................................... 6400/9856
2020-03-31T14:16:24.8147428Z .................................................................................................... 6500/9856
2020-03-31T14:16:24.8147428Z .................................................................................................... 6500/9856
2020-03-31T14:16:36.5736770Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T14:16:55.9728435Z .................................................................................................... 6800/9856
2020-03-31T14:16:58.0139553Z ..........i......................................................................................... 6900/9856
2020-03-31T14:17:00.0599281Z .................................................................................................... 7000/9856
2020-03-31T14:17:02.1865100Z ...............................................i.................................................... 7100/9856
---
2020-03-31T14:18:39.0890230Z .................................................................................................... 7800/9856
2020-03-31T14:18:44.2222397Z .................................................................................................... 7900/9856
2020-03-31T14:18:49.8881923Z .................................................................................................... 8000/9856
2020-03-31T14:18:58.2909225Z .......i............................................................................................ 8100/9856
2020-03-31T14:19:06.4899080Z ........................................................iiiiiiiiii.i................................ 8200/9856
2020-03-31T14:19:20.0463647Z i......i............................................................................................ 8400/9856
2020-03-31T14:19:25.1065794Z .................................................................................................... 8500/9856
2020-03-31T14:19:37.3015948Z .................................................................................................... 8600/9856
2020-03-31T14:19:46.6640405Z .................................................................................................... 8700/9856
---
2020-03-31T14:21:38.6463900Z diff of stderr:
2020-03-31T14:21:38.6464036Z 
2020-03-31T14:21:38.6464135Z 
2020-03-31T14:21:38.6464345Z The actual stderr differed from the expected stderr.
2020-03-31T14:21:38.6465168Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure/no-params-non-move-async-closure.stderr
2020-03-31T14:21:38.6465887Z To update references, rerun the tests and pass the `--bless` flag
2020-03-31T14:21:38.6466545Z To only update this specific test, also pass `--test-args async-await/no-params-non-move-async-closure.rs`
2020-03-31T14:21:38.6467052Z error: 1 errors occurred comparing output.
2020-03-31T14:21:38.6467297Z status: exit code: 1
2020-03-31T14:21:38.6467297Z status: exit code: 1
2020-03-31T14:21:38.6469472Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-params-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure/auxiliary"
2020-03-31T14:21:38.6471241Z ------------------------------------------
2020-03-31T14:21:38.6471417Z 
2020-03-31T14:21:38.6472008Z ------------------------------------------
2020-03-31T14:21:38.6472238Z stderr:
2020-03-31T14:21:38.6472238Z stderr:
2020-03-31T14:21:38.6472626Z ------------------------------------------
2020-03-31T14:21:38.6473188Z error[E0708]: `async` non-`move` closures with parameters are not currently supported
2020-03-31T14:21:38.6474146Z    |
2020-03-31T14:21:38.6474146Z    |
2020-03-31T14:21:38.6474346Z LL |     let _ = async |x: u8| {};
2020-03-31T14:21:38.6474878Z    |
2020-03-31T14:21:38.6475230Z    = help: consider using `let` statements to manually capture variables by reference before entering an `async move` closure
2020-03-31T14:21:38.6475568Z 
2020-03-31T14:21:38.6475757Z error: aborting due to previous error
---
2020-03-31T14:21:38.6544856Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T14:21:38.6545356Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T14:21:38.6555863Z 
2020-03-31T14:21:38.6557034Z 
2020-03-31T14:21:38.6561102Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T14:21:38.6564422Z 
2020-03-31T14:21:38.6564542Z 
2020-03-31T14:21:38.6579631Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T14:21:38.6580030Z Build completed unsuccessfully in 1:00:40
2020-03-31T14:21:38.6580030Z Build completed unsuccessfully in 1:00:40
2020-03-31T14:21:38.6633273Z == clock drift check ==
2020-03-31T14:21:38.6649775Z   local time: Tue Mar 31 14:21:38 UTC 2020
2020-03-31T14:21:38.8313043Z   network time: Tue, 31 Mar 2020 14:21:38 GMT
2020-03-31T14:21:38.8321522Z == end clock drift check ==
2020-03-31T14:21:39.2156706Z 
2020-03-31T14:21:39.2222764Z ##[error]Bash exited with code '1'.
2020-03-31T14:21:39.2237094Z ##[section]Finishing: Run build
2020-03-31T14:21:39.2287249Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-03-31T14:21:39.2293103Z Task         : Get sources
2020-03-31T14:21:39.2293484Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T14:21:39.2293815Z Version      : 1.0.0
2020-03-31T14:21:39.2294045Z Author       : Microsoft
2020-03-31T14:21:39.2294045Z Author       : Microsoft
2020-03-31T14:21:39.2294429Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T14:21:39.2294852Z ==============================================================================
2020-03-31T14:21:39.5577018Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T14:21:39.5629325Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-03-31T14:21:39.5713511Z Cleaning up task key
2020-03-31T14:21:39.5714800Z Start cleaning up orphan processes.
2020-03-31T14:21:39.5887974Z Terminate orphan process: pid (3170) (python)
2020-03-31T14:21:39.6085281Z ##[section]Finishing: Finalize Job
