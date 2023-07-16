plain
2020-03-27T07:23:18.5061128Z ========================== Starting Command Output ===========================
2020-03-27T07:23:18.5063954Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/48664675-af9b-411d-8162-9451ca6634c0.sh
2020-03-27T07:23:18.5064251Z 
2020-03-27T07:23:18.5069301Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T07:23:18.5090725Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70459/merge to s
2020-03-27T07:23:18.5103263Z Task         : Get sources
2020-03-27T07:23:18.5103619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T07:23:18.5103941Z Version      : 1.0.0
2020-03-27T07:23:18.5104160Z Author       : Microsoft
---
2020-03-27T07:23:19.5013900Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T07:23:19.5023644Z ##[command]git config gc.auto 0
2020-03-27T07:23:19.5029463Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T07:23:19.5038883Z ##[command]git config --get-all http.proxy
2020-03-27T07:23:19.5045863Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70459/merge:refs/remotes/pull/70459/merge
---
2020-03-27T07:30:51.3341097Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T07:30:53.0401775Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T07:30:54.3402560Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T07:31:04.1367617Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T07:31:06.3603728Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T07:31:07.8220075Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T07:31:16.9288636Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T07:31:51.7590164Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T07:32:29.0809289Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:34:26.9453259Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T07:55:20.5547865Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T07:55:21.9721395Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T07:55:25.2153017Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T07:55:33.4118421Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T07:55:42.7897911Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T07:55:44.7269022Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T07:55:55.9568493Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T07:56:44.0173716Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T07:57:26.5858700Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:59:58.9612501Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T08:25:29.0119257Z .................................................................................................... 1700/9849
2020-03-27T08:25:35.5357588Z .................................................................................................... 1800/9849
2020-03-27T08:25:52.5723913Z .........................................................................................i.......... 1900/9849
2020-03-27T08:26:03.7790974Z .................................................................................................... 2000/9849
2020-03-27T08:26:15.2625800Z ...............................................................................iiiii................ 2100/9849
2020-03-27T08:26:51.4487793Z .................................................................................................... 2300/9849
2020-03-27T08:26:54.6759496Z .................................................................................................... 2400/9849
2020-03-27T08:26:58.3301284Z .................................................................................................... 2500/9849
2020-03-27T08:27:12.8537886Z .................................................................................................... 2600/9849
---
2020-03-27T08:32:04.3802361Z .....................................................i...............i.............................. 5000/9849
2020-03-27T08:32:17.5142316Z .................................................................................................... 5100/9849
2020-03-27T08:32:28.4478200Z ..................................................................................................i. 5200/9849
2020-03-27T08:32:37.6578938Z .................................................................................................... 5300/9849
2020-03-27T08:32:56.8449003Z ...................................................................................ii.ii........i... 5400/9849
2020-03-27T08:33:02.9034290Z i................................................................................................... 5500/9849
2020-03-27T08:33:18.5109576Z ............................i....................................................................... 5700/9849
2020-03-27T08:33:33.6401641Z .............................................ii....................................i................ 5800/9849
2020-03-27T08:33:45.9598166Z .................................................................................................... 5900/9849
2020-03-27T08:33:55.2668957Z .................................................................................................... 6000/9849
2020-03-27T08:33:55.2668957Z .................................................................................................... 6000/9849
2020-03-27T08:34:13.4849657Z .............................................................................ii...i..ii...........i. 6100/9849
2020-03-27T08:34:47.3095765Z .................................................................................................... 6300/9849
2020-03-27T08:34:58.7650914Z .................................................................................................... 6400/9849
2020-03-27T08:35:07.1125404Z .................................................................................................... 6500/9849
2020-03-27T08:35:07.1125404Z .................................................................................................... 6500/9849
2020-03-27T08:35:27.8920224Z .......i..ii........................................................................................ 6600/9849
2020-03-27T08:36:01.7028383Z .................................................................................................... 6800/9849
2020-03-27T08:36:04.9886586Z .......i............................................................................................ 6900/9849
2020-03-27T08:36:08.0749980Z .................................................................................................... 7000/9849
2020-03-27T08:36:11.8265194Z ...........................................i........................................................ 7100/9849
---
2020-03-27T08:39:00.5130822Z .................................................................................................... 7700/9849
2020-03-27T08:39:09.5808180Z .................................................................................................... 7800/9849
2020-03-27T08:39:18.2526128Z .................................................................................................... 7900/9849
2020-03-27T08:39:31.0009986Z .................................................................................................... 8000/9849
2020-03-27T08:39:44.0997199Z i................................................................................................... 8100/9849
2020-03-27T08:39:57.5045821Z .................................................iiiiiiiiii.i....................................... 8200/9849
2020-03-27T08:40:21.7963528Z i................................................................................................... 8400/9849
2020-03-27T08:40:30.8829023Z .................................................................................................... 8500/9849
2020-03-27T08:40:55.0106091Z .................................................................................................... 8600/9849
2020-03-27T08:41:11.6415519Z .................................................................................................... 8700/9849
---
2020-03-27T08:45:20.9937496Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-27T08:45:21.0106805Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T08:45:21.2116137Z 
2020-03-27T08:45:21.2116484Z running 183 tests
2020-03-27T08:45:25.3125881Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-27T08:45:29.6114347Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-27T08:45:29.6118346Z 
2020-03-27T08:45:29.6121645Z  finished in 8.601
2020-03-27T08:45:29.6128128Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-27T08:45:29.6308121Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-27T08:45:32.4671242Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-27T08:45:32.4883844Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T08:45:32.6468305Z 
2020-03-27T08:45:32.6469263Z running 9 tests
2020-03-27T08:45:32.6471625Z iiiiiiiii
2020-03-27T08:45:32.6472681Z 
2020-03-27T08:45:32.6477208Z  finished in 0.159
2020-03-27T08:45:32.6483822Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-27T08:45:32.6675317Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-27T08:46:05.0917721Z failures:
2020-03-27T08:46:05.0917938Z 
2020-03-27T08:46:05.0922499Z ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
2020-03-27T08:46:05.0922766Z 
2020-03-27T08:46:05.0923055Z error in revision `rpass3`: compilation failed!
2020-03-27T08:46:05.0923348Z status: exit code: 1
2020-03-27T08:46:05.0926045Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
2020-03-27T08:46:05.0928187Z ------------------------------------------
2020-03-27T08:46:05.0928612Z 
2020-03-27T08:46:05.0929062Z ------------------------------------------
2020-03-27T08:46:05.0929291Z stderr:
2020-03-27T08:46:05.0929291Z stderr:
2020-03-27T08:46:05.0929716Z ------------------------------------------
2020-03-27T08:46:05.0930278Z error: CGU-reuse for `main-some_mod` is `PreLto` but should be `No`
2020-03-27T08:46:05.0931204Z    |
2020-03-27T08:46:05.0931204Z    |
2020-03-27T08:46:05.0931785Z LL | #![rustc_partition_codegened(module="main-some_mod", cfg="rpass3")]
2020-03-27T08:46:05.0932458Z 
2020-03-27T08:46:05.0932665Z error: aborting due to previous error
2020-03-27T08:46:05.0932853Z 
2020-03-27T08:46:05.0933087Z 
---
2020-03-27T08:46:05.0936022Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T08:46:05.0936491Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T08:46:05.0936757Z 
2020-03-27T08:46:05.0937298Z 
2020-03-27T08:46:05.0941649Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T08:46:05.0944788Z 
2020-03-27T08:46:05.0944903Z 
2020-03-27T08:46:05.0997252Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T08:46:05.0997711Z Build completed unsuccessfully in 1:18:58
2020-03-27T08:46:05.0997711Z Build completed unsuccessfully in 1:18:58
2020-03-27T08:46:05.1007466Z == clock drift check ==
2020-03-27T08:46:05.1026674Z   local time: Fri Mar 27 08:46:05 UTC 2020
2020-03-27T08:46:05.2726540Z   network time: Fri, 27 Mar 2020 08:46:05 GMT
2020-03-27T08:46:05.2726946Z == end clock drift check ==
2020-03-27T08:46:06.6477648Z 
2020-03-27T08:46:06.6569559Z ##[error]Bash exited with code '1'.
2020-03-27T08:46:06.6590022Z ##[section]Finishing: Run build
2020-03-27T08:46:06.6651906Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70459/merge to s
2020-03-27T08:46:06.6658689Z Task         : Get sources
2020-03-27T08:46:06.6659106Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T08:46:06.6659466Z Version      : 1.0.0
2020-03-27T08:46:06.6659913Z Author       : Microsoft
2020-03-27T08:46:06.6659913Z Author       : Microsoft
2020-03-27T08:46:06.6660342Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T08:46:06.6661138Z ==============================================================================
2020-03-27T08:46:07.0190197Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T08:46:07.0242794Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70459/merge to s
2020-03-27T08:46:07.0368799Z Cleaning up task key
2020-03-27T08:46:07.0370195Z Start cleaning up orphan processes.
2020-03-27T08:46:07.0583022Z Terminate orphan process: pid (4221) (python)
2020-03-27T08:46:07.0888497Z ##[section]Finishing: Finalize Job
