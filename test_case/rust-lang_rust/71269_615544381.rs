plain
2020-04-18T01:47:57.9622238Z ========================== Starting Command Output ===========================
2020-04-18T01:47:57.9639044Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/87809e39-ebf0-44b4-9066-486e1ba8a668.sh
2020-04-18T01:47:57.9858846Z 
2020-04-18T01:47:57.9916560Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T01:47:57.9934268Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71269/merge to s
2020-04-18T01:47:57.9937598Z Task         : Get sources
2020-04-18T01:47:57.9937901Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T01:47:57.9938186Z Version      : 1.0.0
2020-04-18T01:47:57.9938381Z Author       : Microsoft
---
2020-04-18T01:47:58.7347605Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T01:47:58.7353617Z ##[command]git config gc.auto 0
2020-04-18T01:47:58.7357197Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T01:47:58.7361655Z ##[command]git config --get-all http.proxy
2020-04-18T01:47:58.7461558Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71269/merge:refs/remotes/pull/71269/merge
---
2020-04-18T01:50:52.7195089Z  ---> f58a2bb1e753
2020-04-18T01:50:52.7195809Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T01:50:52.7196367Z  ---> Using cache
2020-04-18T01:50:52.7196702Z  ---> d079cc6b6db8
2020-04-18T01:50:52.7197595Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T01:50:52.7199646Z  ---> 4183ca46ee56
2020-04-18T01:50:52.7199853Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T01:50:52.7200194Z  ---> Using cache
2020-04-18T01:50:52.7200530Z  ---> 69e7f8a2a2fb
---
2020-04-18T01:50:52.7704268Z Looks like docker image is the same as before, not uploading
2020-04-18T01:50:59.1177994Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-18T01:50:59.1456595Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-18T01:50:59.1494144Z == clock drift check ==
2020-04-18T01:50:59.1494969Z   local time: Sat Apr 18 01:50:59 UTC 2020
2020-04-18T01:50:59.4291923Z   network time: Sat, 18 Apr 2020 01:50:59 GMT
2020-04-18T01:50:59.4317623Z Starting sccache server...
2020-04-18T01:50:59.5172634Z configure: processing command line
2020-04-18T01:50:59.5173740Z configure: 
2020-04-18T01:50:59.5175702Z configure: rust.dist-src        := False
---
2020-04-18T01:56:09.6660111Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T01:56:11.0091773Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T01:56:12.5676962Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T01:56:12.8952160Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T01:56:22.2482819Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T01:56:23.8099556Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T01:56:27.9988697Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T01:56:31.9883077Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T01:56:41.8837173Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T02:18:20.3393143Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T02:18:22.0181047Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T02:18:23.9364716Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T02:18:24.8703950Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T02:18:35.8959377Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T02:18:38.0048804Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T02:18:43.1453240Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T02:18:48.3550533Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T02:18:59.7229851Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T02:44:10.5595513Z .................................................................................................... 1700/9905
2020-04-18T02:44:14.8094918Z .................................................................................................... 1800/9905
2020-04-18T02:44:24.2779782Z .................................................................................................i.. 1900/9905
2020-04-18T02:44:32.3891603Z .................................................................................................... 2000/9905
2020-04-18T02:44:38.9621186Z .......................................................................................iiiii........ 2100/9905
2020-04-18T02:44:59.8639959Z .................................................................................................... 2300/9905
2020-04-18T02:45:02.1147330Z .................................................................................................... 2400/9905
2020-04-18T02:45:04.4383823Z .................................................................................................... 2500/9905
2020-04-18T02:45:10.7445703Z .................................................................................................... 2600/9905
---
2020-04-18T02:47:56.3463737Z .................................................................................................... 4900/9905
2020-04-18T02:48:01.1330416Z ...............................................................i...............i.................... 5000/9905
2020-04-18T02:48:08.2842875Z .................................................................................................... 5100/9905
2020-04-18T02:48:15.7179628Z .................................................................................................... 5200/9905
2020-04-18T02:48:20.8104511Z .........i.........................................................................................i 5300/9905
2020-04-18T02:48:30.7823943Z ...................................................................................................i 5400/9905
2020-04-18T02:48:35.5698849Z i.ii........i...i................................................................................... 5500/9905
2020-04-18T02:48:43.1455555Z .............................................i...................................................... 5700/9905
2020-04-18T02:48:52.3610355Z .............................................................................ii..................... 5800/9905
2020-04-18T02:48:59.0557905Z ................i................................................................................... 5900/9905
2020-04-18T02:49:04.4437217Z .................................................................................................... 6000/9905
2020-04-18T02:49:04.4437217Z .................................................................................................... 6000/9905
2020-04-18T02:49:14.8659427Z .................................................................................................... 6100/9905
2020-04-18T02:49:25.3258126Z ..........ii...i..ii...........i.................................................................... 6200/9905
2020-04-18T02:49:37.4906751Z .................................................................................................... 6400/9905
2020-04-18T02:49:40.8131593Z .................................................................................................... 6500/9905
2020-04-18T02:49:40.8131593Z .................................................................................................... 6500/9905
2020-04-18T02:49:52.3481652Z ........................................i..ii....................................................... 6600/9905
2020-04-18T02:50:13.8860860Z .................................................................................................... 6800/9905
2020-04-18T02:50:16.0250713Z .........................................i.......................................................... 6900/9905
2020-04-18T02:50:18.1155910Z .................................................................................................... 7000/9905
2020-04-18T02:50:20.1565169Z .................................................................................i.................. 7100/9905
---
2020-04-18T02:51:56.3229760Z .................................................................................................... 7800/9905
2020-04-18T02:52:00.6476914Z .................................................................................................... 7900/9905
2020-04-18T02:52:07.0305673Z .................................................................................................... 8000/9905
2020-04-18T02:52:12.8236858Z ...............................................i.................................................... 8100/9905
2020-04-18T02:52:22.4858751Z ................................................................................................iiii 8200/9905
2020-04-18T02:52:28.0691097Z ii.iiiii.i.......................................................................................... 8300/9905
2020-04-18T02:52:40.9823375Z .................................................................................................... 8500/9905
2020-04-18T02:52:48.9762471Z .................................................................................................... 8600/9905
2020-04-18T02:53:02.7495667Z .................................................................................................... 8700/9905
2020-04-18T02:53:09.3304191Z .................................................................................................... 8800/9905
---
2020-04-18T02:55:23.8804661Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T02:55:23.8992041Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T02:55:24.0943391Z 
2020-04-18T02:55:24.0943848Z running 186 tests
2020-04-18T02:55:26.9180154Z iiii......i.............ii.i..iiii....i....i...........i............i..i..................i....i.... 100/186
2020-04-18T02:55:29.5444088Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii..........F....ii......
2020-04-18T02:55:29.5445987Z 
2020-04-18T02:55:29.5446421Z ---- [codegen] codegen/unchecked-float-casts.rs stdout ----
2020-04-18T02:55:29.5446622Z 
2020-04-18T02:55:29.5446622Z 
2020-04-18T02:55:29.5446984Z error: verification with 'FileCheck' failed
2020-04-18T02:55:29.5447209Z status: exit code: 1
2020-04-18T02:55:29.5448029Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts/unchecked-float-casts.ll" "/checkout/src/test/codegen/unchecked-float-casts.rs"
2020-04-18T02:55:29.5448853Z ------------------------------------------
2020-04-18T02:55:29.5449010Z 
2020-04-18T02:55:29.5449355Z ------------------------------------------
2020-04-18T02:55:29.5449543Z stderr:
2020-04-18T02:55:29.5449543Z stderr:
2020-04-18T02:55:29.5449915Z ------------------------------------------
2020-04-18T02:55:29.5450514Z /checkout/src/test/codegen/unchecked-float-casts.rs:11:12: error: CHECK: expected string not found in input
2020-04-18T02:55:29.5450845Z  // CHECK: fptoui
2020-04-18T02:55:29.5451030Z            ^
2020-04-18T02:55:29.5451651Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts/unchecked-float-casts.ll:97:23: note: scanning from here
2020-04-18T02:55:29.5452055Z define i32 @f32_to_u32(float %x) unnamed_addr #1 {
2020-04-18T02:55:29.5452287Z                       ^
2020-04-18T02:55:29.5452939Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts/unchecked-float-casts.ll:99:29: note: possible intended match here
2020-04-18T02:55:29.5453368Z ; call core::f32::<impl f32>::to_int_unchecked
2020-04-18T02:55:29.5454191Z /checkout/src/test/codegen/unchecked-float-casts.rs:21:12: error: CHECK: expected string not found in input
2020-04-18T02:55:29.5454519Z  // CHECK: fptosi
2020-04-18T02:55:29.5454698Z            ^
2020-04-18T02:55:29.5454698Z            ^
2020-04-18T02:55:29.5455470Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts/unchecked-float-casts.ll:108:23: note: scanning from here
2020-04-18T02:55:29.5455955Z define i32 @f32_to_i32(float %x) unnamed_addr #1 {
2020-04-18T02:55:29.5456187Z                       ^
2020-04-18T02:55:29.5456852Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unchecked-float-casts/unchecked-float-casts.ll:110:29: note: possible intended match here
2020-04-18T02:55:29.5457281Z ; call core::f32::<impl f32>::to_int_unchecked
2020-04-18T02:55:29.5457668Z 
2020-04-18T02:55:29.5458012Z ------------------------------------------
2020-04-18T02:55:29.5458169Z 
2020-04-18T02:55:29.5458277Z 
---
2020-04-18T02:55:29.5461839Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T02:55:29.5462262Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T02:55:29.5468965Z 
2020-04-18T02:55:29.5469143Z 
2020-04-18T02:55:29.5472988Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T02:55:29.5475323Z 
2020-04-18T02:55:29.5475420Z 
2020-04-18T02:55:29.5488286Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T02:55:29.5488645Z Build completed unsuccessfully in 1:02:51
2020-04-18T02:55:29.5488645Z Build completed unsuccessfully in 1:02:51
2020-04-18T02:55:29.5544644Z == clock drift check ==
2020-04-18T02:55:29.5565532Z   local time: Sat Apr 18 02:55:29 UTC 2020
2020-04-18T02:55:29.7053404Z   network time: Sat, 18 Apr 2020 02:55:29 GMT
2020-04-18T02:55:31.8928810Z 
2020-04-18T02:55:31.8928810Z 
2020-04-18T02:55:31.9004244Z ##[error]Bash exited with code '1'.
2020-04-18T02:55:31.9017604Z ##[section]Finishing: Run build
2020-04-18T02:55:31.9077414Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71269/merge to s
2020-04-18T02:55:31.9082284Z Task         : Get sources
2020-04-18T02:55:31.9082593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T02:55:31.9082891Z Version      : 1.0.0
2020-04-18T02:55:31.9083099Z Author       : Microsoft
2020-04-18T02:55:31.9083099Z Author       : Microsoft
2020-04-18T02:55:31.9083421Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T02:55:31.9083794Z ==============================================================================
2020-04-18T02:55:32.2334531Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T02:55:32.2376255Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71269/merge to s
2020-04-18T02:55:32.2468503Z Cleaning up task key
2020-04-18T02:55:32.2469767Z Start cleaning up orphan processes.
2020-04-18T02:55:32.2648082Z Terminate orphan process: pid (3586) (python)
2020-04-18T02:55:32.2812188Z ##[section]Finishing: Finalize Job
