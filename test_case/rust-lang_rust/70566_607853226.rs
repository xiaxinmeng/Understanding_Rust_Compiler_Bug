plain
2020-04-02T12:37:44.2158096Z ========================== Starting Command Output ===========================
2020-04-02T12:37:44.2160320Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/72798034-e5b2-4852-a680-176b8bfb908f.sh
2020-04-02T12:37:44.2160582Z 
2020-04-02T12:37:44.2164140Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T12:37:44.2182192Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-02T12:37:44.2185466Z Task         : Get sources
2020-04-02T12:37:44.2185775Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T12:37:44.2186092Z Version      : 1.0.0
2020-04-02T12:37:44.2186296Z Author       : Microsoft
---
2020-04-02T12:37:45.2121064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T12:37:45.2125872Z ##[command]git config gc.auto 0
2020-04-02T12:37:45.2129125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T12:37:45.2131762Z ##[command]git config --get-all http.proxy
2020-04-02T12:37:45.2135962Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70566/merge:refs/remotes/pull/70566/merge
---
2020-04-02T12:40:35.0631758Z Looks like docker image is the same as before, not uploading
2020-04-02T12:40:42.1091703Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T12:40:42.7656992Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T12:40:42.7663645Z == clock drift check ==
2020-04-02T12:40:42.7664041Z   local time: Thu Apr  2 12:40:42 UTC 2020
2020-04-02T12:40:42.7664444Z   network time: Thu, 02 Apr 2020 12:40:42 GMT
2020-04-02T12:40:42.7665062Z Starting sccache server...
2020-04-02T12:40:42.7665345Z configure: processing command line
2020-04-02T12:40:42.7665629Z configure: 
2020-04-02T12:40:42.7666481Z configure: rust.dist-src        := False
---
2020-04-02T12:45:14.5551798Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T12:45:15.8869766Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T12:45:17.3046642Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T12:45:18.3472661Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T12:45:26.3674780Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T12:45:27.8595601Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T12:45:31.7219150Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T12:45:35.3399798Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T12:45:44.4059896Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T13:05:38.1830321Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T13:05:39.7899058Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T13:05:41.5857152Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T13:05:42.9724136Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T13:05:53.0540371Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T13:05:55.4758900Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T13:06:00.1344347Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T13:06:05.2830731Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T13:06:15.7274240Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T13:29:32.7735852Z .................................................................................................... 1700/9869
2020-04-02T13:29:37.0611316Z .................................................................................................... 1800/9869
2020-04-02T13:29:45.8078029Z ................................................................................................i... 1900/9869
2020-04-02T13:29:53.4502625Z .................................................................................................... 2000/9869
2020-04-02T13:29:59.5397294Z ......................................................................................iiiii......... 2100/9869
2020-04-02T13:30:18.9121411Z .................................................................................................... 2300/9869
2020-04-02T13:30:21.0124699Z .................................................................................................... 2400/9869
2020-04-02T13:30:23.2048292Z .................................................................................................... 2500/9869
2020-04-02T13:30:28.9870173Z .................................................................................................... 2600/9869
---
2020-04-02T13:33:05.8315184Z ............................................................i...............i....................... 5000/9869
2020-04-02T13:33:12.2969329Z .................................................................................................... 5100/9869
2020-04-02T13:33:18.9421190Z .................................................................................................... 5200/9869
2020-04-02T13:33:23.4287430Z .....i.............................................................................................. 5300/9869
2020-04-02T13:33:32.6556142Z ...........................................................................................ii.ii.... 5400/9869
2020-04-02T13:33:36.8483485Z ....i...i........................................................................................... 5500/9869
2020-04-02T13:33:44.9323379Z ....................................i............................................................... 5700/9869
2020-04-02T13:33:54.4513970Z ........................................................ii....................................i..... 5800/9869
2020-04-02T13:34:01.6327999Z .................................................................................................... 5900/9869
2020-04-02T13:34:06.0552227Z .................................................................................................... 6000/9869
2020-04-02T13:34:06.0552227Z .................................................................................................... 6000/9869
2020-04-02T13:34:14.4190148Z ........................................................................................ii...i..ii.. 6100/9869
2020-04-02T13:34:33.7048122Z .................................................................................................... 6300/9869
2020-04-02T13:34:37.9924697Z .................................................................................................... 6400/9869
2020-04-02T13:34:40.5531482Z .................................................................................................... 6500/9869
2020-04-02T13:34:40.5531482Z .................................................................................................... 6500/9869
2020-04-02T13:34:51.6250002Z ..................i..ii............................................................................. 6600/9869
2020-04-02T13:35:10.3584369Z .................................................................................................... 6800/9869
2020-04-02T13:35:12.2803244Z ..................i................................................................................. 6900/9869
2020-04-02T13:35:14.1641249Z .................................................................................................... 7000/9869
2020-04-02T13:35:16.0806541Z .........................................................i.......................................... 7100/9869
---
2020-04-02T13:36:43.8537172Z .................................................................................................... 7800/9869
2020-04-02T13:36:47.9764973Z .................................................................................................... 7900/9869
2020-04-02T13:36:53.1365938Z .................................................................................................... 8000/9869
2020-04-02T13:37:00.0883550Z ...................i................................................................................ 8100/9869
2020-04-02T13:37:07.4241330Z ....................................................................iiiiiiiiii.i.................... 8200/9869
2020-04-02T13:37:20.4954020Z ............i......i................................................................................ 8400/9869
2020-04-02T13:37:24.5756666Z .................................................................................................... 8500/9869
2020-04-02T13:37:34.3900263Z .................................................................................................... 8600/9869
2020-04-02T13:37:44.7266863Z .................................................................................................... 8700/9869
---
2020-04-02T13:39:27.7369774Z 
2020-04-02T13:39:27.7370117Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7370617Z   --> $DIR/lints-used-unused.rs:21:35
2020-04-02T13:39:27.7370806Z    |
2020-04-02T13:39:27.7371328Z LL |     const USED: i32 = --T::USED + (-i32::MIN);
2020-04-02T13:39:27.7371830Z 
2020-04-02T13:39:27.7371983Z error: aborting due to 5 previous errors
2020-04-02T13:39:27.7372135Z 
2020-04-02T13:39:27.7372214Z 
2020-04-02T13:39:27.7372214Z 
2020-04-02T13:39:27.7372291Z 
2020-04-02T13:39:27.7372369Z 
2020-04-02T13:39:27.7372538Z The actual stderr differed from the expected stderr.
2020-04-02T13:39:27.7373125Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused/lints-used-unused.stderr
2020-04-02T13:39:27.7373663Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T13:39:27.7374184Z To only update this specific test, also pass `--test-args associated-const/lints-used-unused.rs`
2020-04-02T13:39:27.7374561Z error: 1 errors occurred comparing output.
2020-04-02T13:39:27.7374756Z status: exit code: 1
2020-04-02T13:39:27.7374756Z status: exit code: 1
2020-04-02T13:39:27.7376462Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/lints-used-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused/auxiliary"
2020-04-02T13:39:27.7380580Z ------------------------------------------
2020-04-02T13:39:27.7380729Z 
2020-04-02T13:39:27.7381047Z ------------------------------------------
2020-04-02T13:39:27.7381231Z stderr:
2020-04-02T13:39:27.7381231Z stderr:
2020-04-02T13:39:27.7381534Z ------------------------------------------
2020-04-02T13:39:27.7381775Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7382217Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:9:23
2020-04-02T13:39:27.7382430Z    |
2020-04-02T13:39:27.7382679Z LL |     const USED: i32 = 1 << 42; //~ ERROR this arithmetic operation will overflow
2020-04-02T13:39:27.7383202Z    |
2020-04-02T13:39:27.7383369Z note: the lint level is defined here
2020-04-02T13:39:27.7383783Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:6:9
2020-04-02T13:39:27.7383998Z    |
2020-04-02T13:39:27.7383998Z    |
2020-04-02T13:39:27.7384150Z LL | #![deny(arithmetic_overflow)]
2020-04-02T13:39:27.7384352Z    |         ^^^^^^^^^^^^^^^^^^^
2020-04-02T13:39:27.7384477Z 
2020-04-02T13:39:27.7384640Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7385094Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:10:25
2020-04-02T13:39:27.7385307Z    |
2020-04-02T13:39:27.7385542Z LL |     const UNUSED: i32 = 1 << 123; //~ ERROR this arithmetic operation will overflow
2020-04-02T13:39:27.7386072Z 
2020-04-02T13:39:27.7386234Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7386852Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:16:23
2020-04-02T13:39:27.7387107Z    |
2020-04-02T13:39:27.7387107Z    |
2020-04-02T13:39:27.7387374Z LL |     const USED: i32 = 1 << 4242; //~ ERROR this arithmetic operation will overflow
2020-04-02T13:39:27.7395115Z 
2020-04-02T13:39:27.7395352Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7396402Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:17:25
2020-04-02T13:39:27.7396690Z    |
2020-04-02T13:39:27.7396690Z    |
2020-04-02T13:39:27.7397240Z LL |     const UNUSED: i32 = 1 << 123123; //~ ERROR this arithmetic operation will overflow
2020-04-02T13:39:27.7398020Z 
2020-04-02T13:39:27.7398212Z error: this arithmetic operation will overflow
2020-04-02T13:39:27.7398923Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:21:35
2020-04-02T13:39:27.7399189Z    |
2020-04-02T13:39:27.7399189Z    |
2020-04-02T13:39:27.7399871Z LL |     const USED: i32 = --T::USED + (-i32::MIN); //~ ERROR this arithmetic operation will overflow
2020-04-02T13:39:27.7400667Z 
2020-04-02T13:39:27.7400970Z error: aborting due to 5 previous errors
2020-04-02T13:39:27.7401110Z 
2020-04-02T13:39:27.7401189Z 
---
2020-04-02T13:39:27.7403516Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-02T13:39:27.7403855Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-02T13:39:27.7404045Z 
2020-04-02T13:39:27.7404123Z 
2020-04-02T13:39:27.7407194Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-02T13:39:27.7409425Z 
2020-04-02T13:39:27.7409520Z 
2020-04-02T13:39:27.7430030Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-02T13:39:27.7431500Z Build completed unsuccessfully in 0:57:21
2020-04-02T13:39:27.7431500Z Build completed unsuccessfully in 0:57:21
2020-04-02T13:39:27.7456986Z == clock drift check ==
2020-04-02T13:39:27.7482418Z   local time: Thu Apr  2 13:39:27 UTC 2020
2020-04-02T13:39:27.9153166Z   network time: Thu, 02 Apr 2020 13:39:27 GMT
2020-04-02T13:39:28.3182193Z 
2020-04-02T13:39:28.3182193Z 
2020-04-02T13:39:28.3254379Z ##[error]Bash exited with code '1'.
2020-04-02T13:39:28.3266148Z ##[section]Finishing: Run build
2020-04-02T13:39:28.3309742Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-02T13:39:28.3314210Z Task         : Get sources
2020-04-02T13:39:28.3314494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T13:39:28.3314753Z Version      : 1.0.0
2020-04-02T13:39:28.3314950Z Author       : Microsoft
2020-04-02T13:39:28.3314950Z Author       : Microsoft
2020-04-02T13:39:28.3315251Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T13:39:28.3315581Z ==============================================================================
2020-04-02T13:39:28.6293639Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T13:39:28.6353353Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-02T13:39:28.6431404Z Cleaning up task key
2020-04-02T13:39:28.6432584Z Start cleaning up orphan processes.
2020-04-02T13:39:28.6594696Z Terminate orphan process: pid (3717) (python)
2020-04-02T13:39:28.6823477Z ##[section]Finishing: Finalize Job
