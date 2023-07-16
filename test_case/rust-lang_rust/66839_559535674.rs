plain
2019-11-28T14:21:17.8910430Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T14:21:17.8926157Z ##[command]git config gc.auto 0
2019-11-28T14:21:17.8928491Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T14:21:17.8931887Z ##[command]git config --get-all http.proxy
2019-11-28T14:21:17.8934909Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66839/merge:refs/remotes/pull/66839/merge
---
2019-11-28T14:50:33.1530066Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-28T14:50:33.3079075Z    Compiling backtrace v0.3.40
2019-11-28T14:50:33.8524522Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-11-28T14:50:34.0492935Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-28T14:50:53.7900806Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T14:50:53.7900956Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T14:50:53.7901056Z This program is free software; you may redistribute it under the terms of
2019-11-28T14:50:53.7901297Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T14:50:53.7901356Z This program has absolutely no warranty.
2019-11-28T14:50:53.7901643Z 
2019-11-28T14:50:53.9717872Z    Compiling rustc-std-workspace-std v1.99.0 (/checkout/src/tools/rustc-std-workspace-std)
2019-11-28T14:50:53.9751771Z    Compiling term v0.0.0 (/checkout/src/libterm)
2019-11-28T14:50:54.0231012Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-11-28T14:50:54.0231012Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-11-28T14:50:59.0934500Z    Compiling unicode-width v0.1.6
2019-11-28T14:50:59.2029049Z    Compiling getopts v0.2.21
2019-11-28T14:51:08.0367216Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-11-28T14:51:20.1803238Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T14:51:20.1804730Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T14:51:20.1805160Z This program is free software; you may redistribute it under the terms of
2019-11-28T14:51:20.1805445Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T14:51:20.1805686Z This program has absolutely no warranty.
2019-11-28T14:51:20.1806536Z 
2019-11-28T14:51:20.2737046Z     Finished release [optimized] target(s) in 1m 16s
2019-11-28T14:51:20.2889028Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-28T14:51:20.2910965Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-28T14:53:24.5175160Z    Compiling rustc-rayon v0.3.0
2019-11-28T14:53:29.2148474Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-11-28T14:53:33.2642497Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-11-28T14:53:35.9232978Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-28T14:53:48.2851903Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T14:53:48.2852050Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T14:53:48.2852112Z This program is free software; you may redistribute it under the terms of
2019-11-28T14:53:48.2859553Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T14:53:48.2859611Z This program has absolutely no warranty.
2019-11-28T14:53:48.2865759Z 
2019-11-28T14:53:48.7478744Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-28T14:53:50.6093385Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-28T14:54:15.3845868Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
---
2019-11-28T15:12:40.9390704Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-11-28T15:13:08.1215926Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-11-28T15:13:23.3379616Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-11-28T15:13:56.5690804Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-11-28T15:15:43.3950094Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:15:43.3951009Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:15:43.3951238Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:15:43.3951448Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:15:43.3951617Z This program has absolutely no warranty.
2019-11-28T15:15:43.3952260Z 
2019-11-28T15:15:51.6535860Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2019-11-28T15:15:51.9025577Z     Finished release [optimized] target(s) in 24m 31s
2019-11-28T15:15:51.9663695Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-28T15:15:51.9663695Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-28T15:15:51.9706849Z Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
2019-11-28T15:15:52.4256704Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-11-28T15:15:52.4261902Z    Compiling cc v1.0.47
2019-11-28T15:15:54.3371128Z    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
2019-11-28T15:15:58.7395980Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2019-11-28T15:17:15.0529433Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:17:15.0530178Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:17:15.0530358Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:17:15.0530566Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:17:15.0530712Z This program has absolutely no warranty.
2019-11-28T15:17:15.0534864Z 
2019-11-28T15:17:16.2878179Z     Finished release [optimized] target(s) in 1m 24s
2019-11-28T15:17:16.2964027Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2019-11-28T15:17:16.2986632Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-28T15:20:55.4105936Z .................................................................................................... 1600/9303
2019-11-28T15:21:00.3541057Z .................................................................................................... 1700/9303
2019-11-28T15:21:13.1597802Z ...................................i................................................................ 1800/9303
2019-11-28T15:21:21.0123743Z .................................................................................................... 1900/9303
2019-11-28T15:21:34.8669197Z ....................iiiii........................................................................... 2000/9303
2019-11-28T15:21:45.0995109Z .................................................................................................... 2200/9303
2019-11-28T15:21:47.7232698Z .................................................................................................... 2300/9303
2019-11-28T15:21:52.7233703Z .................................................................................................... 2400/9303
2019-11-28T15:22:14.2969248Z .................................................................................................... 2500/9303
---
2019-11-28T15:24:57.5716183Z .....................i...............i.............................................................. 4800/9303
2019-11-28T15:25:08.2097814Z .................................................................................................... 4900/9303
2019-11-28T15:25:14.2310141Z .................................................................................................... 5000/9303
2019-11-28T15:25:22.8601372Z .................................................................................................... 5100/9303
2019-11-28T15:25:30.7299408Z ..........................ii.ii...........i......................................................... 5200/9303
2019-11-28T15:25:40.4769373Z .................................................................................................... 5400/9303
2019-11-28T15:25:51.9562943Z .................................................................................................... 5500/9303
2019-11-28T15:25:58.9732788Z ........i........................................................................................... 5600/9303
2019-11-28T15:26:05.5073325Z .................................................................................................... 5700/9303
2019-11-28T15:26:05.5073325Z .................................................................................................... 5700/9303
2019-11-28T15:26:16.3284918Z ..............................................................................................ii...i 5800/9303
2019-11-28T15:26:29.1804671Z ..ii...........i.................................................................................... 5900/9303
2019-11-28T15:26:46.5921314Z .................................................................................................... 6100/9303
2019-11-28T15:26:50.4161352Z .................................................................................................... 6200/9303
2019-11-28T15:26:50.4161352Z .................................................................................................... 6200/9303
2019-11-28T15:27:04.5453703Z .................i..ii.............................................................................. 6300/9303
2019-11-28T15:27:24.1585721Z .....................................................................................i.............. 6500/9303
2019-11-28T15:27:26.5978744Z .................................................................................................... 6600/9303
2019-11-28T15:27:28.9073463Z ............................................................................i....................... 6700/9303
2019-11-28T15:27:31.7442095Z .................................................................................................... 6800/9303
---
2019-11-28T15:32:24.7345020Z failures:
2019-11-28T15:32:24.7384642Z 
2019-11-28T15:32:24.7385361Z ---- [ui] ui/proc-macro/no-missing-docs.rs stdout ----
2019-11-28T15:32:24.7385609Z normalized stderr:
2019-11-28T15:32:24.7386515Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:32:24.7386788Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:32:24.7387251Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:32:24.7387422Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:32:24.7387612Z This program has absolutely no warranty.
2019-11-28T15:32:24.7387892Z 
2019-11-28T15:32:24.7388029Z 
2019-11-28T15:32:24.7388192Z 
2019-11-28T15:32:24.7388328Z 
2019-11-28T15:32:24.7388328Z 
2019-11-28T15:32:24.7388487Z The actual stderr differed from the expected stderr.
2019-11-28T15:32:24.7389038Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/no-missing-docs.stderr
2019-11-28T15:32:24.7389664Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T15:32:24.7390461Z To only update this specific test, also pass `--test-args proc-macro/no-missing-docs.rs`
2019-11-28T15:32:24.7390803Z error: 1 errors occurred comparing output.
2019-11-28T15:32:24.7390957Z status: exit code: 0
2019-11-28T15:32:24.7390957Z status: exit code: 0
2019-11-28T15:32:24.7391958Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/no-missing-docs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-missing-docs/auxiliary" "-A" "unused"
2019-11-28T15:32:24.7392668Z ------------------------------------------
2019-11-28T15:32:24.7392866Z 
2019-11-28T15:32:24.7393262Z ------------------------------------------
2019-11-28T15:32:24.7393464Z stderr:
2019-11-28T15:32:24.7393464Z stderr:
2019-11-28T15:32:24.7393794Z ------------------------------------------
2019-11-28T15:32:24.7394271Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:32:24.7394479Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:32:24.7394634Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:32:24.7394798Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:32:24.7394940Z This program has absolutely no warranty.
2019-11-28T15:32:24.7395200Z 
2019-11-28T15:32:24.7395322Z 
2019-11-28T15:32:24.7395666Z ------------------------------------------
2019-11-28T15:32:24.7396907Z 
2019-11-28T15:32:24.7396907Z 
2019-11-28T15:32:24.7397119Z 
2019-11-28T15:32:24.7397626Z ---- [ui] ui/rust-2018/proc-macro-crate-in-paths.rs stdout ----
2019-11-28T15:32:24.7397845Z normalized stderr:
2019-11-28T15:32:24.7398370Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:32:24.7398588Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:32:24.7398769Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:32:24.7398961Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:32:24.7399124Z This program has absolutely no warranty.
2019-11-28T15:32:24.7399421Z 
2019-11-28T15:32:24.7399721Z 
2019-11-28T15:32:24.7400009Z 
2019-11-28T15:32:24.7400127Z 
2019-11-28T15:32:24.7400127Z 
2019-11-28T15:32:24.7400285Z The actual stderr differed from the expected stderr.
2019-11-28T15:32:24.7400735Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths/proc-macro-crate-in-paths.stderr
2019-11-28T15:32:24.7401160Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T15:32:24.7401600Z To only update this specific test, also pass `--test-args rust-2018/proc-macro-crate-in-paths.rs`
2019-11-28T15:32:24.7402116Z error: 1 errors occurred comparing output.
2019-11-28T15:32:24.7403096Z status: exit code: 0
2019-11-28T15:32:24.7403096Z status: exit code: 0
2019-11-28T15:32:24.7403880Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/proc-macro-crate-in-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/proc-macro-crate-in-paths/auxiliary" "-A" "unused"
2019-11-28T15:32:24.7404198Z ------------------------------------------
2019-11-28T15:32:24.7404245Z 
2019-11-28T15:32:24.7404439Z ------------------------------------------
2019-11-28T15:32:24.7404480Z stderr:
2019-11-28T15:32:24.7404480Z stderr:
2019-11-28T15:32:24.7404858Z ------------------------------------------
2019-11-28T15:32:24.7405316Z warning: Linker version inspection failed to parse: `"cc" "cc" "-v" "-Xlinker" "--version"`, output: GNU ld (GNU Binutils for Ubuntu) 2.26.1
2019-11-28T15:32:24.7405534Z Copyright (C) 2015 Free Software Foundation, Inc.
2019-11-28T15:32:24.7405600Z This program is free software; you may redistribute it under the terms of
2019-11-28T15:32:24.7405649Z the GNU General Public License version 3 or (at your option) a later version.
2019-11-28T15:32:24.7405694Z This program has absolutely no warranty.
2019-11-28T15:32:24.7405759Z 
2019-11-28T15:32:24.7405781Z 
2019-11-28T15:32:24.7406561Z ------------------------------------------
2019-11-28T15:32:24.7406601Z 
---
2019-11-28T15:32:24.7425420Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-28T15:32:24.7425491Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-28T15:32:24.7445002Z 
2019-11-28T15:32:24.7445090Z 
2019-11-28T15:32:24.7447312Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-28T15:32:24.7447601Z 
2019-11-28T15:32:24.7447631Z 
2019-11-28T15:32:24.7453359Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-28T15:32:24.7453426Z Build completed unsuccessfully in 1:05:03
2019-11-28T15:32:24.7453426Z Build completed unsuccessfully in 1:05:03
2019-11-28T15:32:24.7507186Z == clock drift check ==
2019-11-28T15:32:24.7521016Z   local time: Thu Nov 28 15:32:24 UTC 2019
2019-11-28T15:32:25.0302445Z   network time: Thu, 28 Nov 2019 15:32:25 GMT
2019-11-28T15:32:25.0302597Z == end clock drift check ==
2019-11-28T15:32:25.8527677Z 
2019-11-28T15:32:25.8659128Z ##[error]Bash exited with code '1'.
2019-11-28T15:32:25.8695006Z ##[section]Starting: Checkout
2019-11-28T15:32:25.8697295Z ==============================================================================
2019-11-28T15:32:25.8697354Z Task         : Get sources
2019-11-28T15:32:25.8697417Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
