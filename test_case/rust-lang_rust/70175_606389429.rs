plain
2020-03-31T03:03:56.0151456Z ========================== Starting Command Output ===========================
2020-03-31T03:03:56.0154395Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4230d7db-0d97-4fdc-ad52-eee0141938d3.sh
2020-03-31T03:03:56.0154709Z 
2020-03-31T03:03:56.0159485Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T03:03:56.0183989Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-31T03:03:56.0187747Z Task         : Get sources
2020-03-31T03:03:56.0188070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T03:03:56.0188382Z Version      : 1.0.0
2020-03-31T03:03:56.0188594Z Author       : Microsoft
---
2020-03-31T03:03:57.0176867Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T03:03:57.0183755Z ##[command]git config gc.auto 0
2020-03-31T03:03:57.0188526Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T03:03:57.0192267Z ##[command]git config --get-all http.proxy
2020-03-31T03:03:57.0199173Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70175/merge:refs/remotes/pull/70175/merge
---
2020-03-31T03:11:47.8816369Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T03:11:49.4790197Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T03:11:51.2131612Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T03:11:52.8325616Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T03:12:02.0479688Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T03:12:05.5001762Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T03:12:10.6369884Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T03:12:15.1237661Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T03:12:25.0951293Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T03:36:52.7339409Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T03:36:54.6698257Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T03:36:56.8308724Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T03:36:58.8568430Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T03:37:10.3111829Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T03:37:14.3177645Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T03:37:20.2967539Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T03:37:26.3571131Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T03:37:37.6743667Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T04:05:16.4698064Z .................................................................................................... 1700/9856
2020-03-31T04:05:20.7237506Z .................................................................................................... 1800/9856
2020-03-31T04:05:31.3175896Z ..........................................................................................i......... 1900/9856
2020-03-31T04:05:38.7666985Z .................................................................................................... 2000/9856
2020-03-31T04:05:45.7024714Z ................................................................................iiiii............... 2100/9856
2020-03-31T04:06:08.4996442Z .................................................................................................... 2300/9856
2020-03-31T04:06:10.8589646Z .................................................................................................... 2400/9856
2020-03-31T04:06:13.4088295Z .................................................................................................... 2500/9856
2020-03-31T04:06:20.2813122Z .................................................................................................... 2600/9856
---
2020-03-31T04:09:24.7623103Z ......................................................i...............i............................. 5000/9856
2020-03-31T04:09:33.3944523Z .................................................................................................... 5100/9856
2020-03-31T04:09:40.9536161Z ...................................................................................................i 5200/9856
2020-03-31T04:09:46.1719185Z .................................................................................................... 5300/9856
2020-03-31T04:09:57.6404797Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T04:10:01.2923872Z ..i................................................................................................. 5500/9856
2020-03-31T04:10:10.6922919Z ..............................i..................................................................... 5700/9856
2020-03-31T04:10:21.0557868Z ................................................ii....................................i............. 5800/9856
2020-03-31T04:10:28.7867712Z .................................................................................................... 5900/9856
2020-03-31T04:10:34.0634196Z .................................................................................................... 6000/9856
2020-03-31T04:10:34.0634196Z .................................................................................................... 6000/9856
2020-03-31T04:10:43.5074734Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T04:10:56.2281813Z .i.................................................................................................. 6200/9856
2020-03-31T04:11:12.6929199Z .................................................................................................... 6400/9856
2020-03-31T04:11:18.3735110Z .................................................................................................... 6500/9856
2020-03-31T04:11:18.3735110Z .................................................................................................... 6500/9856
2020-03-31T04:11:31.8572793Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T04:11:53.8515145Z .................................................................................................... 6800/9856
2020-03-31T04:11:56.2162911Z ..........i......................................................................................... 6900/9856
2020-03-31T04:11:58.5420642Z .................................................................................................... 7000/9856
2020-03-31T04:12:01.0128442Z ...............................................i.................................................... 7100/9856
---
2020-03-31T04:13:49.3862802Z .................................................................................................... 7800/9856
2020-03-31T04:13:54.9020381Z .................................................................................................... 7900/9856
2020-03-31T04:14:01.0917632Z .................................................................................................... 8000/9856
2020-03-31T04:14:10.1102468Z .......i............................................................................................ 8100/9856
2020-03-31T04:14:18.7762154Z ........................................................iiiiiiiiii.i................................ 8200/9856
2020-03-31T04:14:34.0132297Z i......i............................................................................................ 8400/9856
2020-03-31T04:14:39.4595759Z .................................................................................................... 8500/9856
2020-03-31T04:14:52.8511906Z .................................................................................................... 8600/9856
2020-03-31T04:15:02.9154330Z .................................................................................................... 8700/9856
---
2020-03-31T04:17:34.8590444Z ---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
2020-03-31T04:17:34.8590674Z 
2020-03-31T04:17:34.8590847Z error: compilation failed!
2020-03-31T04:17:34.8591058Z status: exit code: 1
2020-03-31T04:17:34.8593217Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/generator-tiny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/auxiliary"
2020-03-31T04:17:34.8594950Z ------------------------------------------
2020-03-31T04:17:34.8595125Z 
2020-03-31T04:17:34.8595485Z ------------------------------------------
2020-03-31T04:17:34.8595709Z stderr:
2020-03-31T04:17:34.8595709Z stderr:
2020-03-31T04:17:34.8596076Z ------------------------------------------
2020-03-31T04:17:34.8596676Z error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`
2020-03-31T04:17:34.8597168Z error: aborting due to previous error
2020-03-31T04:17:34.8597335Z 
2020-03-31T04:17:34.8597430Z 
2020-03-31T04:17:34.8597808Z ------------------------------------------
---
2020-03-31T04:17:34.8604733Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T04:17:34.8605258Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T04:17:34.8610504Z 
2020-03-31T04:17:34.8610673Z 
2020-03-31T04:17:34.8618651Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T04:17:34.8621355Z 
2020-03-31T04:17:34.8621452Z 
2020-03-31T04:17:34.8627026Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T04:17:34.8627469Z Build completed unsuccessfully in 1:09:40
2020-03-31T04:17:34.8627469Z Build completed unsuccessfully in 1:09:40
2020-03-31T04:17:34.8685839Z == clock drift check ==
2020-03-31T04:17:34.8706224Z   local time: Tue Mar 31 04:17:34 UTC 2020
2020-03-31T04:17:35.0818337Z   network time: Tue, 31 Mar 2020 04:17:35 GMT
2020-03-31T04:17:35.0822898Z == end clock drift check ==
2020-03-31T04:17:37.2015635Z 
2020-03-31T04:17:37.2109377Z ##[error]Bash exited with code '1'.
2020-03-31T04:17:37.2125010Z ##[section]Finishing: Run build
2020-03-31T04:17:37.2184867Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-31T04:17:37.2190457Z Task         : Get sources
2020-03-31T04:17:37.2190857Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T04:17:37.2191210Z Version      : 1.0.0
2020-03-31T04:17:37.2191459Z Author       : Microsoft
2020-03-31T04:17:37.2191459Z Author       : Microsoft
2020-03-31T04:17:37.2191867Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T04:17:37.2192326Z ==============================================================================
2020-03-31T04:17:37.6028085Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T04:17:37.6081056Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-31T04:17:37.6183012Z Cleaning up task key
2020-03-31T04:17:37.6184458Z Start cleaning up orphan processes.
2020-03-31T04:17:37.6411552Z Terminate orphan process: pid (3426) (python)
2020-03-31T04:17:37.6611529Z ##[section]Finishing: Finalize Job
