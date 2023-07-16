plain
2020-04-09T01:29:32.2850133Z ========================== Starting Command Output ===========================
2020-04-09T01:29:32.2853808Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ae5bb6dc-b093-4d3d-a5d3-3f0da771aa8b.sh
2020-04-09T01:29:32.2854227Z 
2020-04-09T01:29:32.2859771Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T01:29:32.2879265Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70939/merge to s
2020-04-09T01:29:32.2882614Z Task         : Get sources
2020-04-09T01:29:32.2882938Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T01:29:32.2883259Z Version      : 1.0.0
2020-04-09T01:29:32.2883490Z Author       : Microsoft
---
2020-04-09T01:29:34.5632591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T01:29:34.5642970Z ##[command]git config gc.auto 0
2020-04-09T01:29:34.5674685Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T01:29:34.5683442Z ##[command]git config --get-all http.proxy
2020-04-09T01:29:34.5694511Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70939/merge:refs/remotes/pull/70939/merge
---
2020-04-09T01:32:03.6910339Z Looks like docker image is the same as before, not uploading
2020-04-09T01:32:04.4080283Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T01:32:04.4379550Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T01:32:04.4413326Z == clock drift check ==
2020-04-09T01:32:04.4424113Z   local time: Thu Apr  9 01:32:04 UTC 2020
2020-04-09T01:32:04.6130164Z   network time: Thu, 09 Apr 2020 01:32:04 GMT
2020-04-09T01:32:04.6154228Z Starting sccache server...
2020-04-09T01:32:04.7345319Z configure: processing command line
2020-04-09T01:32:04.7347848Z configure: 
2020-04-09T01:32:04.7352251Z configure: rust.dist-src        := False
---
2020-04-09T01:37:25.1324816Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T01:37:26.5759968Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T01:37:28.1968024Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T01:37:28.9370326Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T01:37:38.0756487Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T01:37:40.2840310Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T01:37:44.8501093Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T01:37:49.0122790Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T01:37:58.0504993Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T02:00:18.1257942Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T02:00:19.8638011Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T02:00:21.8990996Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T02:00:23.5632838Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T02:00:34.4385381Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T02:00:37.5645919Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T02:00:42.9154597Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T02:00:48.4042801Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T02:00:59.4304276Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T02:26:57.3460220Z .................................................................................................... 1600/9881
2020-04-09T02:27:03.9981332Z .................................................................................................... 1700/9881
2020-04-09T02:27:08.5693146Z .................................................................................................... 1800/9881
2020-04-09T02:27:18.0993171Z .................................................................................................... 1900/9881
2020-04-09T02:27:26.4824362Z i................................................................................................... 2000/9881
2020-04-09T02:27:33.2999034Z ..........................................................................................iiiii..... 2100/9881
2020-04-09T02:27:54.6413236Z .................................................................................................... 2300/9881
2020-04-09T02:27:56.7952638Z .................................................................................................... 2400/9881
2020-04-09T02:27:59.0066066Z .................................................................................................... 2500/9881
2020-04-09T02:28:05.1005118Z .................................................................................................... 2600/9881
---
2020-04-09T02:31:05.7979212Z ................................................................i...............i................... 5000/9881
2020-04-09T02:31:13.4451417Z .................................................................................................... 5100/9881
2020-04-09T02:31:21.1651873Z .................................................................................................... 5200/9881
2020-04-09T02:31:26.7104269Z .........i.......................................................................................... 5300/9881
2020-04-09T02:31:37.1646835Z ..................................................................................................ii 5400/9881
2020-04-09T02:31:42.1796853Z .ii........i...i.................................................................................... 5500/9881
2020-04-09T02:31:50.6452177Z ...........................................i........................................................ 5700/9881
2020-04-09T02:32:01.0028442Z ...............................................................ii................................... 5800/9881
2020-04-09T02:32:08.2584371Z ..i................................................................................................. 5900/9881
2020-04-09T02:32:13.6841435Z .................................................................................................... 6000/9881
2020-04-09T02:32:13.6841435Z .................................................................................................... 6000/9881
2020-04-09T02:32:23.7355630Z ................................................................................................ii.. 6100/9881
2020-04-09T02:32:35.5095835Z .i..ii...........i.................................................................................. 6200/9881
2020-04-09T02:32:52.0308494Z .................................................................................................... 6400/9881
2020-04-09T02:32:56.3782536Z .................................................................................................... 6500/9881
2020-04-09T02:32:56.3782536Z .................................................................................................... 6500/9881
2020-04-09T02:33:09.0661277Z ..........................i..ii..................................................................... 6600/9881
2020-04-09T02:33:30.2527137Z .................................................................................................... 6800/9881
2020-04-09T02:33:32.3864667Z ..........................i......................................................................... 6900/9881
2020-04-09T02:33:34.4257427Z .................................................................................................... 7000/9881
2020-04-09T02:33:36.6759944Z .................................................................i.................................. 7100/9881
---
2020-04-09T02:35:14.4401411Z .................................................................................................... 7800/9881
2020-04-09T02:35:18.4792233Z .................................................................................................... 7900/9881
2020-04-09T02:35:24.9015993Z .................................................................................................... 8000/9881
2020-04-09T02:35:32.1112478Z ..............................i..................................................................... 8100/9881
2020-04-09T02:35:40.5538979Z ..............................................................................iiiiii.iiii.i......... 8200/9881
2020-04-09T02:35:56.8858824Z .......................i......i..................................................................... 8400/9881
2020-04-09T02:36:01.5422120Z .................................................................................................... 8500/9881
2020-04-09T02:36:13.4638316Z .................................................................................................... 8600/9881
2020-04-09T02:36:27.0079839Z .................................................................................................... 8700/9881
---
2020-04-09T02:38:26.7191339Z 9 error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7191974Z -   --> $DIR/array-len-succ.rs:4:40
2020-04-09T02:38:26.7192615Z +   --> $DIR/issue-61522-array-len-succ.rs:4:40
2020-04-09T02:38:26.7192995Z 11    |
2020-04-09T02:38:26.7193393Z 12 LL | pub struct MyArray<const COUNT: usize>([u8; COUNT + 1]);
2020-04-09T02:38:26.7194222Z 
2020-04-09T02:38:26.7194602Z 15    = note: this may fail depending on what value the parameter takes
2020-04-09T02:38:26.7194979Z 16 
2020-04-09T02:38:26.7195351Z 17 error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7195351Z 17 error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7195970Z -   --> $DIR/array-len-succ.rs:8:24
2020-04-09T02:38:26.7199017Z +   --> $DIR/issue-61522-array-len-succ.rs:8:24
2020-04-09T02:38:26.7199543Z 19    |
2020-04-09T02:38:26.7200104Z 20 LL |     fn inner(&self) -> &[u8; COUNT + 1] {
2020-04-09T02:38:26.7200864Z 
2020-04-09T02:38:26.7201079Z 
2020-04-09T02:38:26.7201417Z The actual stderr differed from the expected stderr.
2020-04-09T02:38:26.7201417Z The actual stderr differed from the expected stderr.
2020-04-09T02:38:26.7202350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61522-array-len-succ/issue-61522-array-len-succ.stderr
2020-04-09T02:38:26.7203214Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T02:38:26.7204045Z To only update this specific test, also pass `--test-args const-generics/issue-61522-array-len-succ.rs`
2020-04-09T02:38:26.7209133Z error: 1 errors occurred comparing output.
2020-04-09T02:38:26.7209404Z status: exit code: 1
2020-04-09T02:38:26.7209404Z status: exit code: 1
2020-04-09T02:38:26.7214119Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-61522-array-len-succ.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61522-array-len-succ" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-61522-array-len-succ/auxiliary"
2020-04-09T02:38:26.7218871Z ------------------------------------------
2020-04-09T02:38:26.7219074Z 
2020-04-09T02:38:26.7219466Z ------------------------------------------
2020-04-09T02:38:26.7219682Z stderr:
---
2020-04-09T02:38:26.7222623Z 
2020-04-09T02:38:26.7222856Z error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7223490Z   --> /checkout/src/test/ui/const-generics/issue-61522-array-len-succ.rs:4:40
2020-04-09T02:38:26.7223780Z    |
2020-04-09T02:38:26.7224047Z LL | pub struct MyArray<const COUNT: usize>([u8; COUNT + 1]);
2020-04-09T02:38:26.7224659Z    |
2020-04-09T02:38:26.7224923Z    = note: this may fail depending on what value the parameter takes
2020-04-09T02:38:26.7225207Z 
2020-04-09T02:38:26.7225452Z error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7225452Z error: constant expression depends on a generic parameter
2020-04-09T02:38:26.7226070Z   --> /checkout/src/test/ui/const-generics/issue-61522-array-len-succ.rs:8:24
2020-04-09T02:38:26.7226357Z    |
2020-04-09T02:38:26.7226778Z LL |     fn inner(&self) -> &[u8; COUNT + 1] {
2020-04-09T02:38:26.7227287Z    |
2020-04-09T02:38:26.7227565Z    = note: this may fail depending on what value the parameter takes
2020-04-09T02:38:26.7227800Z 
2020-04-09T02:38:26.7228000Z error: aborting due to 2 previous errors
---
2020-04-09T02:38:26.7232133Z 5    |            ^^^^^^^^^^^^^^
2020-04-09T02:38:26.7232296Z 
2020-04-09T02:38:26.7232398Z 
2020-04-09T02:38:26.7232631Z The actual stderr differed from the expected stderr.
2020-04-09T02:38:26.7233492Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-66596-impl-trait-for-str-const-arg/issue-66596-impl-trait-for-str-const-arg.stderr
2020-04-09T02:38:26.7234267Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T02:38:26.7234994Z To only update this specific test, also pass `--test-args const-generics/issue-66596-impl-trait-for-str-const-arg.rs`
2020-04-09T02:38:26.7235517Z error: 1 errors occurred comparing output.
2020-04-09T02:38:26.7235849Z status: exit code: 0
2020-04-09T02:38:26.7235849Z status: exit code: 0
2020-04-09T02:38:26.7239923Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-66596-impl-trait-for-str-const-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-66596-impl-trait-for-str-const-arg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-66596-impl-trait-for-str-const-arg/auxiliary"
2020-04-09T02:38:26.7241842Z ------------------------------------------
2020-04-09T02:38:26.7242027Z 
2020-04-09T02:38:26.7242427Z ------------------------------------------
2020-04-09T02:38:26.7242645Z stderr:
---
2020-04-09T02:38:26.7249481Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-09T02:38:26.7249919Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-09T02:38:26.7250181Z 
2020-04-09T02:38:26.7250284Z 
2020-04-09T02:38:26.7254194Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-09T02:38:26.7257064Z 
2020-04-09T02:38:26.7257168Z 
2020-04-09T02:38:26.7257734Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T02:38:26.7258914Z Build completed unsuccessfully in 1:04:37
2020-04-09T02:38:26.7258914Z Build completed unsuccessfully in 1:04:37
2020-04-09T02:38:26.7273365Z == clock drift check ==
2020-04-09T02:38:26.7291483Z   local time: Thu Apr  9 02:38:26 UTC 2020
2020-04-09T02:38:26.9011293Z   network time: Thu, 09 Apr 2020 02:38:26 GMT
2020-04-09T02:38:27.3165708Z 
2020-04-09T02:38:27.3165708Z 
2020-04-09T02:38:27.3232582Z ##[error]Bash exited with code '1'.
2020-04-09T02:38:27.3250142Z ##[section]Finishing: Run build
2020-04-09T02:38:27.3303181Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70939/merge to s
2020-04-09T02:38:27.3308309Z Task         : Get sources
2020-04-09T02:38:27.3308694Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T02:38:27.3309033Z Version      : 1.0.0
2020-04-09T02:38:27.3309271Z Author       : Microsoft
2020-04-09T02:38:27.3309271Z Author       : Microsoft
2020-04-09T02:38:27.3309675Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T02:38:27.3310111Z ==============================================================================
2020-04-09T02:38:27.6752473Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T02:38:27.6796437Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70939/merge to s
2020-04-09T02:38:27.6886688Z Cleaning up task key
2020-04-09T02:38:27.6887999Z Start cleaning up orphan processes.
2020-04-09T02:38:27.7071119Z Terminate orphan process: pid (3433) (python)
2020-04-09T02:38:27.7301065Z ##[section]Finishing: Finalize Job
