plain
2019-11-29T12:18:41.6008774Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T12:18:41.6024318Z ##[command]git config gc.auto 0
2019-11-29T12:18:41.6027744Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T12:18:41.6031400Z ##[command]git config --get-all http.proxy
2019-11-29T12:18:41.6037314Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66839/merge:refs/remotes/pull/66839/merge
---
2019-11-29T12:49:15.1706587Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-29T12:49:15.3292875Z    Compiling backtrace v0.3.40
2019-11-29T12:49:15.9480437Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-11-29T12:49:16.1266984Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-29T12:49:36.6478120Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T12:49:36.6478553Z 
2019-11-29T12:49:36.6479088Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T12:49:36.6479687Z 
2019-11-29T12:49:36.6480284Z note: For more information, see ***/issues/61539
2019-11-29T12:49:36.8396358Z    Compiling rustc-std-workspace-std v1.99.0 (/checkout/src/tools/rustc-std-workspace-std)
2019-11-29T12:49:36.8426849Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-11-29T12:49:36.8907564Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-11-29T12:49:42.2200387Z    Compiling unicode-width v0.1.6
2019-11-29T12:49:42.2200387Z    Compiling unicode-width v0.1.6
2019-11-29T12:49:42.3225054Z    Compiling getopts v0.2.21
2019-11-29T12:49:51.4529225Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-11-29T12:50:03.9019335Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T12:50:03.9020289Z 
2019-11-29T12:50:03.9021049Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T12:50:03.9021390Z 
2019-11-29T12:50:03.9027048Z note: For more information, see ***/issues/61539
2019-11-29T12:50:03.9971396Z     Finished release [optimized] target(s) in 1m 19s
2019-11-29T12:50:04.0115556Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-29T12:50:04.0136877Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-29T12:50:04.6035150Z    Compiling cfg-if v0.1.8
---
2019-11-29T12:52:13.3913595Z    Compiling rustc-rayon v0.3.0
2019-11-29T12:52:19.1824165Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-11-29T12:52:22.5339540Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-11-29T12:52:24.9729125Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-29T12:52:38.0120284Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T12:52:38.0125807Z 
2019-11-29T12:52:38.0131610Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T12:52:38.0133626Z 
2019-11-29T12:52:38.0134421Z note: For more information, see ***/issues/61539
2019-11-29T12:52:38.4963409Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-29T12:52:40.4381536Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-29T12:53:06.0891572Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-29T12:53:58.3155827Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2019-11-29T13:12:00.2726898Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-11-29T13:12:16.1494540Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-11-29T13:12:42.4987207Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-11-29T13:13:35.8217795Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-11-29T13:15:16.9457816Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:15:16.9461458Z 
2019-11-29T13:15:16.9465872Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:15:16.9469986Z 
2019-11-29T13:15:16.9470838Z note: For more information, see ***/issues/61539
2019-11-29T13:15:25.3497258Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2019-11-29T13:15:25.6122414Z     Finished release [optimized] target(s) in 25m 21s
2019-11-29T13:15:25.6666759Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-29T13:15:25.6764172Z Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
2019-11-29T13:15:25.6764172Z Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
2019-11-29T13:15:26.0364259Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-11-29T13:15:26.0370949Z    Compiling cc v1.0.47
2019-11-29T13:15:28.0549546Z    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
2019-11-29T13:15:32.6172584Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-11-29T13:16:50.3135483Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:16:50.3140129Z 
2019-11-29T13:16:50.3141772Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:16:50.3142123Z 
2019-11-29T13:16:50.3142790Z note: For more information, see ***/issues/61539
2019-11-29T13:16:51.6406368Z     Finished release [optimized] target(s) in 1m 25s
2019-11-29T13:16:51.6509468Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2019-11-29T13:16:51.6547440Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-29T13:16:51.6548490Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-29T13:20:36.3676520Z .................................................................................................... 1600/9306
2019-11-29T13:20:41.3047599Z .................................................................................................... 1700/9306
2019-11-29T13:20:54.5834832Z ....................................i............................................................... 1800/9306
2019-11-29T13:21:02.5703720Z .................................................................................................... 1900/9306
2019-11-29T13:21:16.8428556Z .....................iiiii.......................................................................... 2000/9306
2019-11-29T13:21:27.4851742Z .................................................................................................... 2200/9306
2019-11-29T13:21:30.1215223Z .................................................................................................... 2300/9306
2019-11-29T13:21:35.1403784Z .................................................................................................... 2400/9306
2019-11-29T13:21:57.6455848Z .................................................................................................... 2500/9306
---
2019-11-29T13:24:45.7214808Z ......................i...............i............................................................. 4800/9306
2019-11-29T13:24:56.6047215Z .................................................................................................... 4900/9306
2019-11-29T13:25:02.7011037Z .................................................................................................... 5000/9306
2019-11-29T13:25:11.2243942Z .................................................................................................... 5100/9306
2019-11-29T13:25:19.1808689Z ...........................ii.ii...........i........................................................ 5200/9306
2019-11-29T13:25:29.1188824Z .................................................................................................... 5400/9306
2019-11-29T13:25:40.1236712Z .................................................................................................... 5500/9306
2019-11-29T13:25:47.5721476Z .........i.......................................................................................... 5600/9306
2019-11-29T13:25:54.2980193Z .................................................................................................... 5700/9306
2019-11-29T13:25:54.2980193Z .................................................................................................... 5700/9306
2019-11-29T13:26:05.5202758Z ...............................................................................................ii... 5800/9306
2019-11-29T13:26:18.6432869Z i...ii..........i................................................................................... 5900/9306
2019-11-29T13:26:37.6266916Z .................................................................................................... 6100/9306
2019-11-29T13:26:45.7117305Z .................................................................................................... 6200/9306
2019-11-29T13:26:45.7117305Z .................................................................................................... 6200/9306
2019-11-29T13:27:00.3793659Z ..................i..ii............................................................................. 6300/9306
2019-11-29T13:27:20.9427866Z ......................................................................................i............. 6500/9306
2019-11-29T13:27:23.3851599Z .................................................................................................... 6600/9306
2019-11-29T13:27:25.7884622Z .............................................................................i...................... 6700/9306
2019-11-29T13:27:28.6131222Z .................................................................................................... 6800/9306
---
2019-11-29T13:32:30.8985788Z failures:
2019-11-29T13:32:30.9032804Z 
2019-11-29T13:32:30.9034017Z ---- [ui] ui/proc-macro/no-missing-docs.rs stdout ----
2019-11-29T13:32:30.9034265Z normalized stderr:
2019-11-29T13:32:30.9034463Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:32:30.9034612Z 
2019-11-29T13:32:30.9034793Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:32:30.9035099Z 
2019-11-29T13:32:30.9035580Z note: For more information, see ***/issues/61539
2019-11-29T13:32:30.9035849Z 
2019-11-29T13:32:30.9035975Z 
2019-11-29T13:32:30.9036080Z 
2019-11-29T13:32:30.9036206Z The actual stderr differed from the expected stderr.
2019-11-29T13:32:30.9036206Z The actual stderr differed from the expected stderr.
2019-11-29T13:32:30.9036631Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/no-missing-docs.stderr
2019-11-29T13:32:30.9037009Z To update references, rerun the tests and pass the `--bless` flag
2019-11-29T13:32:30.9037430Z To only update this specific test, also pass `--test-args proc-macro/no-missing-docs.rs`
2019-11-29T13:32:30.9037751Z error: 1 errors occurred comparing output.
2019-11-29T13:32:30.9037881Z status: exit code: 0
2019-11-29T13:32:30.9037881Z status: exit code: 0
2019-11-29T13:32:30.9038877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/no-missing-docs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/auxiliary" "-A" "unused"
2019-11-29T13:32:30.9039539Z ------------------------------------------
2019-11-29T13:32:30.9039694Z 
2019-11-29T13:32:30.9040046Z ------------------------------------------
2019-11-29T13:32:30.9040211Z stderr:
2019-11-29T13:32:30.9040211Z stderr:
2019-11-29T13:32:30.9040709Z ------------------------------------------
2019-11-29T13:32:30.9041804Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:32:30.9041993Z 
2019-11-29T13:32:30.9042165Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:32:30.9042319Z 
2019-11-29T13:32:30.9042852Z note: For more information, see ***/issues/61539
2019-11-29T13:32:30.9043717Z 
2019-11-29T13:32:30.9044218Z ------------------------------------------
2019-11-29T13:32:30.9045316Z 
2019-11-29T13:32:30.9045354Z 
2019-11-29T13:32:30.9045354Z 
2019-11-29T13:32:30.9045659Z ---- [ui] ui/rust-2018/proc-macro-crate-in-paths.rs stdout ----
2019-11-29T13:32:30.9045705Z normalized stderr:
2019-11-29T13:32:30.9045752Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:32:30.9045799Z 
2019-11-29T13:32:30.9045842Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:32:30.9045881Z 
2019-11-29T13:32:30.9046158Z note: For more information, see ***/issues/61539
2019-11-29T13:32:30.9046210Z 
2019-11-29T13:32:30.9046231Z 
2019-11-29T13:32:30.9046253Z 
2019-11-29T13:32:30.9046317Z The actual stderr differed from the expected stderr.
2019-11-29T13:32:30.9046317Z The actual stderr differed from the expected stderr.
2019-11-29T13:32:30.9046619Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths/proc-macro-crate-in-paths.stderr
2019-11-29T13:32:30.9046990Z To update references, rerun the tests and pass the `--bless` flag
2019-11-29T13:32:30.9047272Z To only update this specific test, also pass `--test-args rust-2018/proc-macro-crate-in-paths.rs`
2019-11-29T13:32:30.9047344Z error: 1 errors occurred comparing output.
2019-11-29T13:32:30.9047402Z status: exit code: 0
2019-11-29T13:32:30.9047402Z status: exit code: 0
2019-11-29T13:32:30.9048088Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/proc-macro-crate-in-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths/auxiliary" "-A" "unused"
2019-11-29T13:32:30.9048398Z ------------------------------------------
2019-11-29T13:32:30.9048427Z 
2019-11-29T13:32:30.9048626Z ------------------------------------------
2019-11-29T13:32:30.9048685Z stderr:
2019-11-29T13:32:30.9048685Z stderr:
2019-11-29T13:32:30.9048885Z ------------------------------------------
2019-11-29T13:32:30.9048935Z warning: Using linker `GNU ld (GNU Binutils for Ubuntu) 2.26.1` with Rust dynamic libraries has known bugs.
2019-11-29T13:32:30.9048985Z 
2019-11-29T13:32:30.9049029Z note: Consider upgrading to GNU ld version 2.29 or newer, or using a different linker.
2019-11-29T13:32:30.9049065Z 
2019-11-29T13:32:30.9049338Z note: For more information, see ***/issues/61539
2019-11-29T13:32:30.9049391Z 
2019-11-29T13:32:30.9049721Z ------------------------------------------
2019-11-29T13:32:30.9049778Z 
2019-11-29T13:32:30.9049801Z 
---
2019-11-29T13:32:30.9069996Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-29T13:32:30.9070069Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-29T13:32:30.9084244Z 
2019-11-29T13:32:30.9084420Z 
2019-11-29T13:32:30.9089423Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-29T13:32:30.9089664Z 
2019-11-29T13:32:30.9089689Z 
2019-11-29T13:32:30.9100644Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-29T13:32:30.9101161Z Build completed unsuccessfully in 1:07:35
2019-11-29T13:32:30.9101161Z Build completed unsuccessfully in 1:07:35
2019-11-29T13:32:30.9158013Z == clock drift check ==
2019-11-29T13:32:30.9173999Z   local time: Fri Nov 29 13:32:30 UTC 2019
2019-11-29T13:32:31.1963427Z   network time: Fri, 29 Nov 2019 13:32:31 GMT
2019-11-29T13:32:31.1968168Z == end clock drift check ==
2019-11-29T13:32:32.0637991Z 
2019-11-29T13:32:32.0749734Z ##[error]Bash exited with code '1'.
2019-11-29T13:32:32.0790285Z ##[section]Starting: Checkout
2019-11-29T13:32:32.0793206Z ==============================================================================
2019-11-29T13:32:32.0793269Z Task         : Get sources
2019-11-29T13:32:32.0793335Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
