plain
2020-04-01T04:48:37.9909334Z ========================== Starting Command Output ===========================
2020-04-01T04:48:37.9914212Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/87e490c3-58e3-486b-bd0a-501aa5bf94d2.sh
2020-04-01T04:48:37.9914563Z 
2020-04-01T04:48:37.9918866Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T04:48:37.9936235Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T04:48:37.9940097Z Task         : Get sources
2020-04-01T04:48:37.9940323Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T04:48:37.9940578Z Version      : 1.0.0
2020-04-01T04:48:37.9940725Z Author       : Microsoft
---
2020-04-01T04:48:39.2943424Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T04:48:39.2951884Z ##[command]git config gc.auto 0
2020-04-01T04:48:39.2962139Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T04:48:39.2975835Z ##[command]git config --get-all http.proxy
2020-04-01T04:48:39.2984772Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70611/merge:refs/remotes/pull/70611/merge
---
2020-04-01T04:50:53.8288819Z Looks like docker image is the same as before, not uploading
2020-04-01T04:51:00.6323998Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T04:51:00.6594604Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T04:51:00.6614417Z == clock drift check ==
2020-04-01T04:51:00.6618598Z   local time: Wed Apr  1 04:51:00 UTC 2020
2020-04-01T04:51:01.1858779Z   network time: Wed, 01 Apr 2020 04:51:01 GMT
2020-04-01T04:51:01.1883043Z Starting sccache server...
2020-04-01T04:51:01.2660775Z configure: processing command line
2020-04-01T04:51:01.2661812Z configure: 
2020-04-01T04:51:01.2662912Z configure: rust.dist-src        := False
---
2020-04-01T04:55:52.9269468Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T04:55:54.3501879Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T04:55:55.7708005Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T04:55:57.3558562Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T04:56:05.2374082Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T04:56:08.2487569Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T04:56:12.3387247Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T04:56:16.2580653Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T04:56:24.3186875Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T05:17:05.6313949Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T05:17:07.4056538Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T05:17:09.2488113Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T05:17:11.2779789Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T05:17:20.7781966Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T05:17:24.3935281Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T05:17:29.3120500Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T05:17:34.4292776Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T05:17:43.7203705Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T05:41:02.3793169Z .................................................................................................... 1700/9865
2020-04-01T05:41:06.0022584Z .................................................................................................... 1800/9865
2020-04-01T05:41:14.0490725Z ...............................................................................................i.... 1900/9865
2020-04-01T05:41:21.1206410Z .................................................................................................... 2000/9865
2020-04-01T05:41:26.8247546Z .....................................................................................iiiii.......... 2100/9865
2020-04-01T05:41:45.5698674Z .................................................................................................... 2300/9865
2020-04-01T05:41:47.5574083Z .................................................................................................... 2400/9865
2020-04-01T05:41:49.5642827Z .................................................................................................... 2500/9865
2020-04-01T05:41:54.9818808Z .................................................................................................... 2600/9865
---
2020-04-01T05:44:30.0771347Z ...........................................................i...............i........................ 5000/9865
2020-04-01T05:44:36.9974758Z .................................................................................................... 5100/9865
2020-04-01T05:44:43.8989460Z .................................................................................................... 5200/9865
2020-04-01T05:44:48.2941530Z ....i............................................................................................... 5300/9865
2020-04-01T05:44:57.6126415Z ..........................................................................................ii.ii..... 5400/9865
2020-04-01T05:45:01.3042726Z ...i...i............................................................................................ 5500/9865
2020-04-01T05:45:09.1364860Z ...................................i................................................................ 5700/9865
2020-04-01T05:45:18.0271247Z .......................................................ii....................................i...... 5800/9865
2020-04-01T05:45:24.8502436Z .................................................................................................... 5900/9865
2020-04-01T05:45:29.1875144Z .................................................................................................... 6000/9865
2020-04-01T05:45:29.1875144Z .................................................................................................... 6000/9865
2020-04-01T05:45:37.2192746Z .......................................................................................ii...i...ii.. 6100/9865
2020-04-01T05:45:55.8258401Z .................................................................................................... 6300/9865
2020-04-01T05:46:02.2911639Z .................................................................................................... 6400/9865
2020-04-01T05:46:08.5586738Z .................................................................................................... 6500/9865
2020-04-01T05:46:08.5586738Z .................................................................................................... 6500/9865
2020-04-01T05:46:20.2299121Z .................i..ii.............................................................................. 6600/9865
2020-04-01T05:46:39.3675631Z .................................................................................................... 6800/9865
2020-04-01T05:46:41.3124133Z .................i.................................................................................. 6900/9865
2020-04-01T05:46:43.2595433Z .................................................................................................... 7000/9865
2020-04-01T05:46:45.2587874Z ........................................................i........................................... 7100/9865
---
2020-04-01T05:48:15.6869979Z .................................................................................................... 7800/9865
2020-04-01T05:48:20.5638841Z .................................................................................................... 7900/9865
2020-04-01T05:48:25.2736398Z .................................................................................................... 8000/9865
2020-04-01T05:48:32.6416729Z ................i................................................................................... 8100/9865
2020-04-01T05:48:40.2375859Z .................................................................iiiiiiiiii.i....................... 8200/9865
2020-04-01T05:48:53.8483094Z .........i......i................................................................................... 8400/9865
2020-04-01T05:48:58.1228057Z .................................................................................................... 8500/9865
2020-04-01T05:49:08.3715831Z .................................................................................................... 8600/9865
2020-04-01T05:49:18.5847750Z .................................................................................................... 8700/9865
---
2020-04-01T05:51:03.2086581Z ---- [ui] ui/async-await/no-params-non-move-async-closure.rs stdout ----
2020-04-01T05:51:03.2087033Z 
2020-04-01T05:51:03.2087374Z error: ui test compiled successfully!
2020-04-01T05:51:03.2087750Z status: exit code: 0
2020-04-01T05:51:03.2089780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-params-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-params-non-move-async-closure/auxiliary"
2020-04-01T05:51:03.2091887Z ------------------------------------------
2020-04-01T05:51:03.2092272Z 
2020-04-01T05:51:03.2092809Z ------------------------------------------
2020-04-01T05:51:03.2093193Z stderr:
---
2020-04-01T05:51:03.2106351Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-01T05:51:03.2107003Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-01T05:51:03.2116538Z 
2020-04-01T05:51:03.2118718Z 
2020-04-01T05:51:03.2126747Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-01T05:51:03.2128796Z 
2020-04-01T05:51:03.2128871Z 
2020-04-01T05:51:03.2141016Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T05:51:03.2141353Z Build completed unsuccessfully in 0:58:26
2020-04-01T05:51:03.2141353Z Build completed unsuccessfully in 0:58:26
2020-04-01T05:51:03.2190372Z == clock drift check ==
2020-04-01T05:51:03.2202667Z   local time: Wed Apr  1 05:51:03 UTC 2020
2020-04-01T05:51:03.4913883Z   network time: Wed, 01 Apr 2020 05:51:03 GMT
2020-04-01T05:51:03.8526270Z 
2020-04-01T05:51:03.8526270Z 
2020-04-01T05:51:03.8603958Z ##[error]Bash exited with code '1'.
2020-04-01T05:51:03.8615710Z ##[section]Finishing: Run build
2020-04-01T05:51:03.8656640Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T05:51:03.8660799Z Task         : Get sources
2020-04-01T05:51:03.8661076Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T05:51:03.8661308Z Version      : 1.0.0
2020-04-01T05:51:03.8661472Z Author       : Microsoft
2020-04-01T05:51:03.8661472Z Author       : Microsoft
2020-04-01T05:51:03.8661750Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T05:51:03.8662050Z ==============================================================================
2020-04-01T05:51:04.1615819Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T05:51:04.1662958Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T05:51:04.1753103Z Cleaning up task key
2020-04-01T05:51:04.1754178Z Start cleaning up orphan processes.
2020-04-01T05:51:04.1965910Z Terminate orphan process: pid (6084) (python)
2020-04-01T05:51:04.2144784Z ##[section]Finishing: Finalize Job
