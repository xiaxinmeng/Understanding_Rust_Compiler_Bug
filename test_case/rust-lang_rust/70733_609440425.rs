plain
2020-04-05T14:50:46.9572827Z ========================== Starting Command Output ===========================
2020-04-05T14:50:46.9575295Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c698e9aa-da8b-4d51-a138-236529c0d78e.sh
2020-04-05T14:50:46.9575573Z 
2020-04-05T14:50:46.9579975Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T14:50:46.9601081Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-05T14:50:46.9604559Z Task         : Get sources
2020-04-05T14:50:46.9604868Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T14:50:46.9605188Z Version      : 1.0.0
2020-04-05T14:50:46.9605391Z Author       : Microsoft
---
2020-04-05T14:50:48.2251213Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T14:50:48.2257044Z ##[command]git config gc.auto 0
2020-04-05T14:50:48.2260805Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T14:50:48.2264249Z ##[command]git config --get-all http.proxy
2020-04-05T14:50:48.2272074Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70733/merge:refs/remotes/pull/70733/merge
---
2020-04-05T14:52:49.0570093Z Looks like docker image is the same as before, not uploading
2020-04-05T14:52:56.4217063Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T14:52:56.4482927Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T14:52:56.4508220Z == clock drift check ==
2020-04-05T14:52:56.4517765Z   local time: Sun Apr  5 14:52:56 UTC 2020
2020-04-05T14:52:56.7206104Z   network time: Sun, 05 Apr 2020 14:52:56 GMT
2020-04-05T14:52:56.7219504Z Starting sccache server...
2020-04-05T14:52:56.8052318Z configure: processing command line
2020-04-05T14:52:56.8053800Z configure: 
2020-04-05T14:52:56.8055726Z configure: rust.dist-src        := False
---
2020-04-05T14:57:49.5773236Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T14:57:50.9668085Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T14:57:52.3670940Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T14:57:52.4671405Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T14:58:01.9313930Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T14:58:03.5188420Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T14:58:07.6719920Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T14:58:11.5809152Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T14:58:20.8681801Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T15:19:41.6761601Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T15:19:43.4043645Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T15:19:45.3778158Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T15:19:46.0163461Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T15:19:57.4614580Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T15:19:59.2869822Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T15:20:04.4719395Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T15:20:09.7637264Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T15:20:21.5475110Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T15:45:01.8341245Z .................................................................................................... 1700/9876
2020-04-05T15:45:05.7975087Z .................................................................................................... 1800/9876
2020-04-05T15:45:14.5288023Z ................................................................................................i... 1900/9876
2020-04-05T15:45:22.2297601Z .................................................................................................... 2000/9876
2020-04-05T15:45:28.4833158Z ......................................................................................iiiii......... 2100/9876
2020-04-05T15:45:48.9540198Z .................................................................................................... 2300/9876
2020-04-05T15:45:51.0973627Z .................................................................................................... 2400/9876
2020-04-05T15:45:53.3152948Z .................................................................................................... 2500/9876
2020-04-05T15:45:59.1949756Z .................................................................................................... 2600/9876
---
2020-04-05T15:48:47.8291033Z ............................................................i...............i....................... 5000/9876
2020-04-05T15:48:54.9904236Z .................................................................................................... 5100/9876
2020-04-05T15:49:02.4316919Z .................................................................................................... 5200/9876
2020-04-05T15:49:07.3927483Z .....i.............................................................................................. 5300/9876
2020-04-05T15:49:17.0720238Z ..............................................................................................ii.ii. 5400/9876
2020-04-05T15:49:21.6900263Z .......i...i........................................................................................ 5500/9876
2020-04-05T15:49:30.1437079Z .......................................i............................................................ 5700/9876
2020-04-05T15:49:30.1437079Z .......................................i............................................................ 5700/9876
2020-04-05T15:49:39.5068621Z ...........................................................ii.....................................i. 5800/9876
2020-04-05T15:49:51.5507902Z .................................................................................................... 6000/9876
2020-04-05T15:49:51.5507902Z .................................................................................................... 6000/9876
2020-04-05T15:50:00.9723308Z ............................................................................................ii...i.. 6100/9876
2020-04-05T15:50:12.6102639Z ii...........i...................................................................................... 6200/9876
2020-04-05T15:50:26.0127267Z .................................................................................................... 6400/9876
2020-04-05T15:50:28.6851966Z .................................................................................................... 6500/9876
2020-04-05T15:50:28.6851966Z .................................................................................................... 6500/9876
2020-04-05T15:50:40.8334726Z ......................i..ii......................................................................... 6600/9876
2020-04-05T15:51:00.6828619Z .................................................................................................... 6800/9876
2020-04-05T15:51:02.7341343Z ......................i............................................................................. 6900/9876
2020-04-05T15:51:04.7401382Z .................................................................................................... 7000/9876
2020-04-05T15:51:06.8840401Z .............................................................i...................................... 7100/9876
---
2020-04-05T15:52:42.8580470Z .................................................................................................... 7800/9876
2020-04-05T15:52:47.1939100Z .................................................................................................... 7900/9876
2020-04-05T15:52:52.9871837Z .................................................................................................... 8000/9876
2020-04-05T15:53:00.6884242Z .........................i.......................................................................... 8100/9876
2020-04-05T15:53:08.5538301Z ..........................................................................iiiiiiiiii.i.............. 8200/9876
2020-04-05T15:53:23.7446687Z ..................i......i.......................................................................... 8400/9876
2020-04-05T15:53:28.3532494Z .................................................................................................... 8500/9876
2020-04-05T15:53:38.9576880Z .................................................................................................... 8600/9876
2020-04-05T15:53:50.7355468Z .................................................................................................... 8700/9876
---
2020-04-05T15:56:07.6838488Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-05T15:56:07.7025965Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T15:56:07.9002117Z 
2020-04-05T15:56:07.9002439Z running 185 tests
2020-04-05T15:56:10.5335348Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-05T15:56:13.0559586Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-05T15:56:13.0570696Z 
2020-04-05T15:56:13.0574334Z  finished in 5.352
2020-04-05T15:56:13.0577147Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-05T15:56:13.0761316Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T15:56:15.0890194Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-05T15:56:15.1083690Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T15:56:15.2607159Z 
2020-04-05T15:56:15.2608124Z running 9 tests
2020-04-05T15:56:15.2609557Z iiiiiiiii
2020-04-05T15:56:15.2612500Z 
2020-04-05T15:56:15.2613728Z  finished in 0.153
2020-04-05T15:56:15.2617692Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-05T15:56:15.2807436Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T15:56:34.4240262Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-05T15:56:34.4466460Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T15:56:34.6319569Z 
2020-04-05T15:56:34.6319964Z running 115 tests
2020-04-05T15:56:48.4030972Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-05T15:56:50.1349757Z ...iiii.....ii.
2020-04-05T15:56:50.1351030Z 
2020-04-05T15:56:50.1357041Z  finished in 15.689
2020-04-05T15:56:50.1361978Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-05T15:56:50.1365394Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T16:07:42.5868570Z ---- sync.rs - sync::Arc<T>::decr_strong_count (line 799) stdout ----
2020-04-05T16:07:42.5869188Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-04-05T16:07:42.5869827Z   --> sync.rs:807:1
2020-04-05T16:07:42.5870123Z    |
2020-04-05T16:07:42.5870423Z 11 | Arc::decr_strong_count(ptr);
2020-04-05T16:07:42.5870872Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-04-05T16:07:42.5871853Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-04-05T16:07:42.5872261Z 
2020-04-05T16:07:42.5872662Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-04-05T16:07:42.5873253Z   --> sync.rs:811:12
2020-04-05T16:07:42.5873253Z   --> sync.rs:811:12
2020-04-05T16:07:42.5873563Z    |
2020-04-05T16:07:42.5873865Z 15 | let five = Arc::from_raw(ptr);
2020-04-05T16:07:42.5874261Z    |            ^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-04-05T16:07:42.5875243Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-04-05T16:07:42.5875636Z 
2020-04-05T16:07:42.5875952Z error: aborting due to 2 previous errors
2020-04-05T16:07:42.5876227Z 
2020-04-05T16:07:42.5876227Z 
2020-04-05T16:07:42.5876779Z For more information about this error, try `rustc --explain E0133`.
2020-04-05T16:07:42.5877654Z Couldn't compile the test.
2020-04-05T16:07:42.5878307Z ---- sync.rs - sync::Arc<T>::incr_strong_count (line 765) stdout ----
2020-04-05T16:07:42.5878849Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-04-05T16:07:42.5879428Z   --> sync.rs:773:1
2020-04-05T16:07:42.5879733Z    |
2020-04-05T16:07:42.5880419Z 11 | Arc::incr_strong_count(ptr);
2020-04-05T16:07:42.5881193Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-04-05T16:07:42.5882313Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-04-05T16:07:42.5882701Z 
2020-04-05T16:07:42.5883411Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2020-04-05T16:07:42.5883959Z   --> sync.rs:777:12
2020-04-05T16:07:42.5883959Z   --> sync.rs:777:12
2020-04-05T16:07:42.5884236Z    |
2020-04-05T16:07:42.5884533Z 15 | let five = Arc::from_raw(ptr);
2020-04-05T16:07:42.5884941Z    |            ^^^^^^^^^^^^^^^^^^ call to unsafe function
2020-04-05T16:07:42.5885762Z    = note: consult the function's documentation for information on how to avoid undefined behavior
2020-04-05T16:07:42.5886142Z 
2020-04-05T16:07:42.5886558Z error: aborting due to 2 previous errors
2020-04-05T16:07:42.5886730Z 
---
2020-04-05T16:07:42.5965590Z 
2020-04-05T16:07:43.2320922Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-05T16:07:43.2327391Z Build completed unsuccessfully in 1:13:19
2020-04-05T16:07:43.2358572Z == clock drift check ==
2020-04-05T16:07:43.2359011Z   local time: Sun Apr  5 16:07:42 UTC 2020
2020-04-05T16:07:43.2359439Z   network time: Sun, 05 Apr 2020 16:07:42 GMT
2020-04-05T16:07:43.2359987Z 
2020-04-05T16:07:43.2359987Z 
2020-04-05T16:07:43.2392595Z ##[error]Bash exited with code '1'.
2020-04-05T16:07:43.2409877Z ##[section]Finishing: Run build
2020-04-05T16:07:43.2455423Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-05T16:07:43.2460352Z Task         : Get sources
2020-04-05T16:07:43.2460714Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T16:07:43.2461028Z Version      : 1.0.0
2020-04-05T16:07:43.2461249Z Author       : Microsoft
2020-04-05T16:07:43.2461249Z Author       : Microsoft
2020-04-05T16:07:43.2461617Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T16:07:43.2462025Z ==============================================================================
2020-04-05T16:07:43.5712779Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T16:07:43.5717146Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-05T16:07:43.5800636Z Cleaning up task key
2020-04-05T16:07:43.5801868Z Start cleaning up orphan processes.
2020-04-05T16:07:43.5972216Z Terminate orphan process: pid (5346) (python)
2020-04-05T16:07:43.6203651Z ##[section]Finishing: Finalize Job
