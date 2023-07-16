plain
2020-04-23T02:36:56.8733402Z ========================== Starting Command Output ===========================
2020-04-23T02:36:56.8737187Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b39b413d-6ed9-4ad6-982a-4889ca9e5f2e.sh
2020-04-23T02:36:56.8737557Z 
2020-04-23T02:36:56.8740994Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T02:36:56.8757219Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-23T02:36:56.8760212Z Task         : Get sources
2020-04-23T02:36:56.8760450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T02:36:56.8760682Z Version      : 1.0.0
2020-04-23T02:36:56.8760884Z Author       : Microsoft
---
2020-04-23T02:36:58.1865123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T02:36:58.1874805Z ##[command]git config gc.auto 0
2020-04-23T02:36:58.1880838Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T02:36:58.1885858Z ##[command]git config --get-all http.proxy
2020-04-23T02:36:58.1894956Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71418/merge:refs/remotes/pull/71418/merge
---
2020-04-23T02:39:25.0444003Z  ---> 318032b5f0e2
2020-04-23T02:39:25.0444975Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T02:39:25.0445455Z  ---> Using cache
2020-04-23T02:39:25.0445721Z  ---> d44a858fd1ce
2020-04-23T02:39:25.0446813Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T02:39:25.0448177Z  ---> 58b910f50f5a
2020-04-23T02:39:25.0448485Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T02:39:25.0449201Z  ---> Using cache
2020-04-23T02:39:25.0479816Z  ---> ee7702aadba1
---
2020-04-23T02:39:25.0824986Z Looks like docker image is the same as before, not uploading
2020-04-23T02:39:25.9728272Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T02:39:25.9994729Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T02:39:26.0018777Z == clock drift check ==
2020-04-23T02:39:26.0029056Z   local time: Thu Apr 23 02:39:26 UTC 2020
2020-04-23T02:39:26.0188351Z   network time: Thu, 23 Apr 2020 02:39:26 GMT
2020-04-23T02:39:26.0210838Z Starting sccache server...
2020-04-23T02:39:26.0967966Z configure: processing command line
2020-04-23T02:39:26.0968567Z configure: 
2020-04-23T02:39:26.0969772Z configure: rust.dist-src        := False
---
2020-04-23T02:44:38.4759788Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T02:44:40.0599852Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T02:44:41.7290134Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T02:44:43.5669355Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T02:44:52.1459708Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T02:44:55.0995021Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T02:44:59.6892465Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T02:45:04.0043991Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T02:45:13.1807394Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T03:08:17.9858845Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T03:08:19.7207398Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T03:08:21.5654974Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T03:08:22.6577161Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T03:08:32.5032485Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T03:08:35.6405023Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T03:08:40.4886594Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T03:08:44.9952435Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T03:08:54.8189320Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T03:32:02.5671365Z .................................................................................................... 1700/9916
2020-04-23T03:32:06.7266476Z .................................................................................................... 1800/9916
2020-04-23T03:32:14.9287511Z .................................................................................................... 1900/9916
2020-04-23T03:32:22.7132302Z .....i.............................................................................................. 2000/9916
2020-04-23T03:32:28.8105114Z ...............................................................................................iiiii 2100/9916
2020-04-23T03:32:48.4534126Z .................................................................................................... 2300/9916
2020-04-23T03:32:50.6666496Z .................................................................................................... 2400/9916
2020-04-23T03:32:52.9323161Z .................................................................................................... 2500/9916
2020-04-23T03:32:58.5376189Z .................................................................................................... 2600/9916
---
2020-04-23T03:35:52.9657382Z .................................................................................................... 5100/9916
2020-04-23T03:36:00.2142263Z .................................................................................................... 5200/9916
2020-04-23T03:36:04.7408337Z ..................i................................................................................. 5300/9916
2020-04-23T03:36:14.5059141Z ........i........................................................................................... 5400/9916
2020-04-23T03:36:19.9709604Z ........ii.ii........i...i.......................................................................... 5500/9916
2020-04-23T03:36:27.5498235Z .......................................................i............................................ 5700/9916
2020-04-23T03:36:36.2769197Z ........................................................................................ii.......... 5800/9916
2020-04-23T03:36:42.9873483Z ...........................i........................................................................ 5900/9916
2020-04-23T03:36:48.3916602Z .................................................................................................... 6000/9916
2020-04-23T03:36:48.3916602Z .................................................................................................... 6000/9916
2020-04-23T03:36:58.9451293Z .................................................................................................... 6100/9916
2020-04-23T03:37:08.8301397Z .....................ii...i..ii...........i......................................................... 6200/9916
2020-04-23T03:37:24.5220167Z .................................................................................................... 6400/9916
2020-04-23T03:37:31.1481728Z .................................................................................................... 6500/9916
2020-04-23T03:37:31.1481728Z .................................................................................................... 6500/9916
2020-04-23T03:37:50.3316422Z ...................................................i..ii............................................ 6600/9916
2020-04-23T03:38:11.4353322Z .................................................................................................... 6800/9916
2020-04-23T03:38:13.8258153Z ....................................................i............................................... 6900/9916
2020-04-23T03:38:15.8293134Z .................................................................................................... 7000/9916
2020-04-23T03:38:17.9604026Z ............................................................................................i....... 7100/9916
---
2020-04-23T03:39:54.0296002Z .................................................................................................... 7900/9916
2020-04-23T03:40:00.1524016Z .................................................................................................... 8000/9916
2020-04-23T03:40:05.9169941Z ..........................................................i......................................... 8100/9916
2020-04-23T03:40:15.4001160Z .................................................................................................... 8200/9916
2020-04-23T03:40:20.5952048Z .......iiiiii.iiiii..i.............................................................................. 8300/9916
2020-04-23T03:40:33.9411539Z .................................................................................................... 8500/9916
2020-04-23T03:40:41.6169760Z .................................................................................................... 8600/9916
2020-04-23T03:40:54.3845866Z .................................................................................................... 8700/9916
2020-04-23T03:41:00.8200219Z .................................................................................................... 8800/9916
---
2020-04-23T03:43:14.2548740Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-23T03:43:14.2739705Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T03:43:14.4749475Z 
2020-04-23T03:43:14.4749818Z running 186 tests
2020-04-23T03:43:17.6850876Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-23T03:43:19.9816255Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-23T03:43:19.9820030Z 
2020-04-23T03:43:19.9820414Z  finished in 5.707
2020-04-23T03:43:19.9821290Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-23T03:43:19.9995622Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T03:43:22.0677893Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-23T03:43:22.0860612Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T03:43:22.2321516Z 
2020-04-23T03:43:22.2321856Z running 9 tests
2020-04-23T03:43:22.2322786Z iiiiiiiii
2020-04-23T03:43:22.2323877Z 
2020-04-23T03:43:22.2326912Z  finished in 0.146
2020-04-23T03:43:22.2329356Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-23T03:43:22.2502085Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T03:43:41.6096831Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-23T03:43:41.6303324Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T03:43:41.8088135Z 
2020-04-23T03:43:41.8088395Z running 115 tests
2020-04-23T03:43:54.8651262Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-23T03:43:56.6730986Z ...iiii.....ii.
2020-04-23T03:43:56.6736371Z 
2020-04-23T03:43:56.6738286Z  finished in 15.042
2020-04-23T03:43:56.6773364Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-23T03:43:56.6774794Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T03:44:31.6696298Z ---- [ui] ui-fulldeps/undef_mask.rs stdout ----
2020-04-23T03:44:31.6696580Z 
2020-04-23T03:44:31.6697057Z error: test compilation failed although it shouldn't!
2020-04-23T03:44:31.6697412Z status: exit code: 1
2020-04-23T03:44:31.6698984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/undef_mask.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/undef_mask/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/undef_mask/auxiliary"
2020-04-23T03:44:31.6700405Z ------------------------------------------
2020-04-23T03:44:31.6700684Z 
2020-04-23T03:44:31.6701107Z ------------------------------------------
2020-04-23T03:44:31.6701393Z stderr:
2020-04-23T03:44:31.6701393Z stderr:
2020-04-23T03:44:31.6701841Z ------------------------------------------
2020-04-23T03:44:31.6702238Z error[E0432]: unresolved import `rustc_middle::mir::interpret::UndefMask`
2020-04-23T03:44:31.6702837Z   --> /checkout/src/test/ui-fulldeps/undef_mask.rs:10:5
2020-04-23T03:44:31.6703147Z    |
2020-04-23T03:44:31.6703431Z LL | use rustc_middle::mir::interpret::UndefMask;
2020-04-23T03:44:31.6703856Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `UndefMask` in `mir::interpret`
2020-04-23T03:44:31.6704410Z error: aborting due to previous error
2020-04-23T03:44:31.6704653Z 
2020-04-23T03:44:31.6705133Z For more information about this error, try `rustc --explain E0432`.
2020-04-23T03:44:31.6705419Z 
---
2020-04-23T03:44:31.6715296Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T03:44:31.6715647Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-23T03:44:31.6723499Z 
2020-04-23T03:44:31.6723624Z 
2020-04-23T03:44:31.6729914Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T03:44:31.6732400Z 
2020-04-23T03:44:31.6732649Z 
2020-04-23T03:44:31.6744131Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-23T03:44:31.6744672Z Build completed unsuccessfully in 1:03:34
2020-04-23T03:44:31.6744672Z Build completed unsuccessfully in 1:03:34
2020-04-23T03:44:31.6826198Z == clock drift check ==
2020-04-23T03:44:31.6851468Z   local time: Thu Apr 23 03:44:31 UTC 2020
2020-04-23T03:44:31.7155301Z   network time: Thu, 23 Apr 2020 03:44:31 GMT
2020-04-23T03:44:32.5796822Z 
2020-04-23T03:44:32.5796822Z 
2020-04-23T03:44:32.5872432Z ##[error]Bash exited with code '1'.
2020-04-23T03:44:32.5886812Z ##[section]Finishing: Run build
2020-04-23T03:44:32.5930720Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-23T03:44:32.5936319Z Task         : Get sources
2020-04-23T03:44:32.5936643Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T03:44:32.5936919Z Version      : 1.0.0
2020-04-23T03:44:32.5937120Z Author       : Microsoft
2020-04-23T03:44:32.5937120Z Author       : Microsoft
2020-04-23T03:44:32.5937452Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T03:44:32.5937807Z ==============================================================================
2020-04-23T03:44:32.9305475Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T03:44:32.9354109Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-23T03:44:32.9458553Z Cleaning up task key
2020-04-23T03:44:32.9460186Z Start cleaning up orphan processes.
2020-04-23T03:44:32.9643968Z Terminate orphan process: pid (4706) (python)
2020-04-23T03:44:32.9897938Z ##[section]Finishing: Finalize Job
