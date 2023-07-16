plain
2020-03-30T12:11:19.7067885Z ========================== Starting Command Output ===========================
2020-03-30T12:11:19.7070576Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c4af45c-c191-49b2-b8c3-dfd5dd0ba4d6.sh
2020-03-30T12:11:19.7070850Z 
2020-03-30T12:11:19.7074982Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T12:11:19.7097178Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70562/merge to s
2020-03-30T12:11:19.7100123Z Task         : Get sources
2020-03-30T12:11:19.7100373Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T12:11:19.7100615Z Version      : 1.0.0
2020-03-30T12:11:19.7100776Z Author       : Microsoft
---
2020-03-30T12:11:20.9524956Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T12:11:20.9531328Z ##[command]git config gc.auto 0
2020-03-30T12:11:20.9534424Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T12:11:20.9537409Z ##[command]git config --get-all http.proxy
2020-03-30T12:11:20.9544316Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70562/merge:refs/remotes/pull/70562/merge
---
2020-03-30T12:18:25.2507482Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T12:18:26.7981309Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T12:18:28.4103249Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T12:18:29.4278643Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T12:18:38.7028454Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T12:18:41.0744646Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T12:18:45.5988203Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T12:18:49.8600064Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T12:18:59.4571688Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T12:39:50.7834042Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T12:39:52.4161543Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T12:39:54.2694610Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T12:39:56.5154855Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T12:40:05.9319390Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T12:40:09.8014253Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T12:40:14.8528043Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T12:40:20.0068082Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T12:40:29.9309691Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T13:04:35.8718604Z .................................................................................................... 1700/9858
2020-03-30T13:04:39.6783186Z .................................................................................................... 1800/9858
2020-03-30T13:04:48.4012912Z ............................................................................................i....... 1900/9858
2020-03-30T13:04:55.8261692Z .................................................................................................... 2000/9858
2020-03-30T13:05:02.0673080Z ..................................................................................iiiii............. 2100/9858
2020-03-30T13:05:22.2329525Z .................................................................................................... 2300/9858
2020-03-30T13:05:24.3366153Z .................................................................................................... 2400/9858
2020-03-30T13:05:26.3496511Z ..................................................................................................6    |
2020-03-30T13:05:26.3501853Z 7    = note: `#[warn(incomplete_features)]` on by default
---
2020-03-30T13:05:26.3524398Z 
2020-03-30T13:05:26.3527377Z -    |         ^^^^^^^^^
2020-03-30T13:05:26.3530435Z +    |         ^^^^^^^^^ expected `3usize`, found `N`
2020-03-30T13:05:26.3536772Z +    |
2020-03-30T13:05:26.3537239Z +    = note: expected array `[u32; 3]`
2020-03-30T13:05:26.3537630Z +               found array `[u32; _]`
2020-03-30T13:05:26.3538295Z 15 error: aborting due to previous error
2020-03-30T13:05:26.3538635Z 16 
2020-03-30T13:05:26.3538875Z 
2020-03-30T13:05:26.3539956Z - For more information about this error, try `rustc --explain E0730`.
---
2020-03-30T13:08:12.9596711Z .................................................................................................... 4900/9858
2020-03-30T13:08:17.4564605Z ........................................................i...............i........................... 5000/9858
2020-03-30T13:08:25.1833226Z .................................................................................................... 5100/9858
2020-03-30T13:08:32.7651033Z .................................................................................................... 5200/9858
2020-03-30T13:08:37.8083216Z .i.................................................................................................. 5300/9858
2020-03-30T13:08:48.2282487Z .......................................................................................ii.ii........ 5400/9858
2020-03-30T13:08:51.7726825Z i...i............................................................................................... 5500/9858
2020-03-30T13:09:00.0341951Z ................................i................................................................... 5700/9858
2020-03-30T13:09:09.7128774Z ..................................................ii....................................i........... 5800/9858
2020-03-30T13:09:16.9625791Z .................................................................................................... 5900/9858
2020-03-30T13:09:21.9445120Z .................................................................................................... 6000/9858
2020-03-30T13:09:21.9445120Z .................................................................................................... 6000/9858
2020-03-30T13:09:30.6600275Z ..................................................................................ii...i..ii........ 6100/9858
2020-03-30T13:09:50.0321637Z .................................................................................................... 6300/9858
2020-03-30T13:09:56.9949563Z .................................................................................................... 6400/9858
2020-03-30T13:10:03.9529842Z .................................................................................................... 6500/9858
2020-03-30T13:10:03.9529842Z .................................................................................................... 6500/9858
2020-03-30T13:10:18.2977820Z ............i..ii................................................................................... 6600/9858
2020-03-30T13:10:39.0948273Z .................................................................................................... 6800/9858
2020-03-30T13:10:41.2260096Z ............i....................................................................................... 6900/9858
2020-03-30T13:10:43.4167167Z .................................................................................................... 7000/9858
2020-03-30T13:10:45.6537883Z .................................................i.................................................. 7100/9858
---
2020-03-30T13:12:26.1263733Z .................................................................................................... 7800/9858
2020-03-30T13:12:31.1739010Z .................................................................................................... 7900/9858
2020-03-30T13:12:36.8539816Z .................................................................................................... 8000/9858
2020-03-30T13:12:44.5286435Z .........i.......................................................................................... 8100/9858
2020-03-30T13:12:52.5646295Z ..........................................................iiiiiiiiii.i.............................. 8200/9858
2020-03-30T13:13:06.6106062Z ..i......i.......................................................................................... 8400/9858
2020-03-30T13:13:11.5109986Z .................................................................................................... 8500/9858
2020-03-30T13:13:24.2414565Z .................................................................................................... 8600/9858
2020-03-30T13:13:34.3341735Z .................................................................................................... 8700/9858
---
2020-03-30T13:15:30.9138395Z diff of stderr:
2020-03-30T13:15:30.9138803Z 
2020-03-30T13:15:30.9139121Z 
2020-03-30T13:15:30.9139569Z The actual stderr differed from the expected stderr.
2020-03-30T13:15:30.9140774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0730/E0730.stderr
2020-03-30T13:15:30.9141811Z To update references, rerun the tests and pass the `--bless` flag
2020-03-30T13:15:30.9143233Z To only update this specific test, also pass `--test-args error-codes/E0730.rs`
2020-03-30T13:15:30.9144023Z error: 1 errors occurred comparing output.
2020-03-30T13:15:30.9144380Z status: exit code: 1
2020-03-30T13:15:30.9144380Z status: exit code: 1
2020-03-30T13:15:30.9154267Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0730.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0730" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0730/auxiliary"
2020-03-30T13:15:30.9156305Z ------------------------------------------
2020-03-30T13:15:30.9156657Z 
2020-03-30T13:15:30.9157150Z ------------------------------------------
2020-03-30T13:15:30.9157654Z stderr:
---
2020-03-30T13:15:30.9166386Z 
2020-03-30T13:15:30.9166662Z error[E0308]: mismatched types
2020-03-30T13:15:30.9167287Z   --> /checkout/src/test/ui/error-codes/E0730.rs:6:9
2020-03-30T13:15:30.9167660Z    |
2020-03-30T13:15:30.9168661Z LL |         [1, 2, 3] => true, //~ ERROR cannot pattern-match on an array without a fixed length
2020-03-30T13:15:30.9169379Z    |         ^^^^^^^^^ expected `3usize`, found `N`
2020-03-30T13:15:30.9169704Z    |
2020-03-30T13:15:30.9170026Z    = note: expected array `[u32; 3]`
2020-03-30T13:15:30.9170387Z               found array `[u32; _]`
2020-03-30T13:15:30.9171307Z error: aborting due to previous error
2020-03-30T13:15:30.9171565Z 
2020-03-30T13:15:30.9172098Z For more information about this error, try `rustc --explain E0308`.
2020-03-30T13:15:30.9172481Z 
---
2020-03-30T13:15:30.9185430Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-30T13:15:30.9185985Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-30T13:15:30.9186217Z 
2020-03-30T13:15:30.9186305Z 
2020-03-30T13:15:30.9190402Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-30T13:15:30.9193506Z 
2020-03-30T13:15:30.9193596Z 
2020-03-30T13:15:30.9202463Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-30T13:15:30.9202859Z Build completed unsuccessfully in 1:00:31
2020-03-30T13:15:30.9202859Z Build completed unsuccessfully in 1:00:31
2020-03-30T13:15:30.9257027Z == clock drift check ==
2020-03-30T13:15:30.9277718Z   local time: Mon Mar 30 13:15:30 UTC 2020
2020-03-30T13:15:30.9573285Z   network time: Mon, 30 Mar 2020 13:15:30 GMT
2020-03-30T13:15:30.9577849Z == end clock drift check ==
2020-03-30T13:15:31.3645677Z 
2020-03-30T13:15:31.3729879Z ##[error]Bash exited with code '1'.
2020-03-30T13:15:31.3746102Z ##[section]Finishing: Run build
2020-03-30T13:15:31.3791295Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70562/merge to s
2020-03-30T13:15:31.3795964Z Task         : Get sources
2020-03-30T13:15:31.3796263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T13:15:31.3796541Z Version      : 1.0.0
2020-03-30T13:15:31.3796766Z Author       : Microsoft
2020-03-30T13:15:31.3796766Z Author       : Microsoft
2020-03-30T13:15:31.3797076Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T13:15:31.3797433Z ==============================================================================
2020-03-30T13:15:31.7397186Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T13:15:31.7449228Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70562/merge to s
2020-03-30T13:15:31.7544517Z Cleaning up task key
2020-03-30T13:15:31.7546100Z Start cleaning up orphan processes.
2020-03-30T13:15:31.7926298Z Terminate orphan process: pid (4060) (python)
2020-03-30T13:15:31.7981999Z ##[section]Finishing: Finalize Job
