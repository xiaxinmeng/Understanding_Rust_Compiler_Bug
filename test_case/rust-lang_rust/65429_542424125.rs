plain
2019-10-15T20:21:03.1488524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T20:21:03.1671675Z ##[command]git config gc.auto 0
2019-10-15T20:21:03.1743964Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T20:21:03.1793398Z ##[command]git config --get-all http.proxy
2019-10-15T20:21:03.1932192Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65429/merge:refs/remotes/pull/65429/merge
---
2019-10-15T21:17:20.5952466Z .................................................................................................... 1600/9182
2019-10-15T21:17:25.6887674Z .................................................................................................... 1700/9182
2019-10-15T21:17:38.0159867Z ............................i...............i....................................................... 1800/9182
2019-10-15T21:17:45.2319154Z .................................................................................................... 1900/9182
2019-10-15T21:17:58.9616966Z ...................iiiii............................................................................ 2000/9182
2019-10-15T21:18:08.7487811Z .................................................................................................... 2200/9182
2019-10-15T21:18:11.1177078Z .................................................................................................... 2300/9182
2019-10-15T21:18:16.1428978Z .................................................................................................... 2400/9182
2019-10-15T21:18:37.1156179Z .................................................................................................... 2500/9182
---
2019-10-15T21:21:23.3682644Z ...........................i...............i........................................................ 4800/9182
2019-10-15T21:21:34.4603034Z .................................................................................................... 4900/9182
2019-10-15T21:21:40.5257269Z .................................................................................................... 5000/9182
2019-10-15T21:21:49.2734875Z .................................................................................................... 5100/9182
2019-10-15T21:21:56.6177316Z ...........................ii.ii.................................................................... 5200/9182
2019-10-15T21:22:05.4463176Z .................................................................................................... 5400/9182
2019-10-15T21:22:15.5953465Z .............................................................................................i...... 5500/9182
2019-10-15T21:22:23.3203422Z .................................................................................................... 5600/9182
2019-10-15T21:22:27.7232803Z .................................................................................................... 5700/9182
2019-10-15T21:22:27.7232803Z .................................................................................................... 5700/9182
2019-10-15T21:22:37.9892270Z ..........................................................................................ii...i..ii 5800/9182
2019-10-15T21:23:01.5948103Z .................................................................................................... 6000/9182
2019-10-15T21:23:10.5562879Z .................................................................................................... 6100/9182
2019-10-15T21:23:18.4354780Z .................................................................................................i.. 6200/9182
2019-10-15T21:23:31.9661059Z ii.................................................................................................. 6300/9182
---
2019-10-15T21:28:10.5026288Z  finished in 4.994
2019-10-15T21:28:10.5195258Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:28:10.6704083Z 
2019-10-15T21:28:10.6704699Z running 153 tests
2019-10-15T21:28:13.6341743Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-15T21:28:15.5029197Z i..iiii..............i.........iii.i.........ii......
2019-10-15T21:28:15.5030180Z 
2019-10-15T21:28:15.5034510Z  finished in 4.983
2019-10-15T21:28:15.5206526Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:28:15.6724679Z 
---
2019-10-15T21:28:17.5871762Z  finished in 2.066
2019-10-15T21:28:17.6045334Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:28:17.7570340Z 
2019-10-15T21:28:17.7570510Z running 9 tests
2019-10-15T21:28:17.7571627Z iiiiiiiii
2019-10-15T21:28:17.7571908Z 
2019-10-15T21:28:17.7571941Z  finished in 0.149
2019-10-15T21:28:17.7718001Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:28:17.9236967Z 
---
2019-10-15T21:28:34.6763388Z  finished in 16.904
2019-10-15T21:28:34.6939904Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:28:34.8491887Z 
2019-10-15T21:28:34.8492071Z running 123 tests
2019-10-15T21:28:57.2751166Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-15T21:29:01.5264681Z i.i.i......iii.i.....ii
2019-10-15T21:29:01.5265937Z 
2019-10-15T21:29:01.5269164Z  finished in 26.833
2019-10-15T21:29:01.5280429Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:29:01.5281255Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-15T21:41:01.9602267Z 
2019-10-15T21:41:01.9602987Z    Doc-tests core
2019-10-15T21:41:06.9003303Z 
2019-10-15T21:41:06.9004320Z running 2405 tests
2019-10-15T21:41:17.8448413Z ......iiiii......................................................................................... 100/2405
2019-10-15T21:41:28.2291064Z ...............................................................................ii................... 200/2405
2019-10-15T21:41:52.9746847Z .i.................................................................................................. 400/2405
2019-10-15T21:41:52.9746847Z .i.................................................................................................. 400/2405
2019-10-15T21:42:03.1030146Z ................................................i..i.................iiii........................... 500/2405
2019-10-15T21:42:22.6692412Z .................................................................................................... 700/2405
2019-10-15T21:42:32.6014308Z .................................................................................................... 800/2405
2019-10-15T21:42:42.4745878Z .................................................................................................... 900/2405
2019-10-15T21:42:52.3535645Z .................................................................................................... 1000/2405
---
2019-10-15T21:46:51.0841674Z 
2019-10-15T21:46:51.0842463Z running 995 tests
2019-10-15T21:47:10.4344226Z i................................................................................................... 100/995
2019-10-15T21:47:20.8143080Z .................................................................................................... 200/995
2019-10-15T21:47:28.3166372Z ...................iii......i......i...i......i..................................................... 300/995
2019-10-15T21:47:33.4581771Z .................................................................................................... 400/995
2019-10-15T21:47:40.2619533Z ......................................i..i.................................ii....................... 500/995
2019-10-15T21:47:53.6014446Z .................................................................................................... 700/995
2019-10-15T21:47:53.6014446Z .................................................................................................... 700/995
2019-10-15T21:48:00.6991279Z .....................iiii........................................................................... 800/995
2019-10-15T21:48:14.7529714Z .................................................................................................... 900/995
2019-10-15T21:48:21.6994039Z ...........................................iiii................................................
2019-10-15T21:48:21.6996152Z 
2019-10-15T21:48:21.7084873Z  finished in 180.049
2019-10-15T21:48:21.7099215Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T21:48:21.8972864Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-15T22:01:58.9367086Z     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-15T22:01:58.9399565Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-15T22:01:58.9697065Z     Checking hashbrown v0.6.1
2019-10-15T22:01:59.6418365Z  Documenting std v0.0.0 (/checkout/src/libstd)
2019-10-15T22:02:04.0627848Z error: `[std::fs::write]` cannot be resolved, ignoring it...
2019-10-15T22:02:04.0628688Z    --> src/libstd/fs.rs:32:65
2019-10-15T22:02:04.0629038Z     |
2019-10-15T22:02:04.0629662Z 32  | /// Creates a new file and write bytes to it (you can also use [`std::fs::write`]):
2019-10-15T22:02:04.0630529Z     |
2019-10-15T22:02:04.0630973Z note: lint level defined here
2019-10-15T22:02:04.0631474Z    --> src/libstd/lib.rs:211:9
2019-10-15T22:02:04.0632265Z     |
2019-10-15T22:02:04.0632265Z     |
2019-10-15T22:02:04.0632827Z 211 | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2019-10-15T22:02:04.0634066Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-10-15T22:02:04.0634280Z 
2019-10-15T22:02:04.0634280Z 
2019-10-15T22:02:04.0634682Z error: `[std::fs::read]` cannot be resolved, ignoring it...
2019-10-15T22:02:04.0635394Z   --> src/libstd/fs.rs:45:70
2019-10-15T22:02:04.0635741Z    |
2019-10-15T22:02:04.0636149Z 45 | /// Read the contents of a file into a [`String`] (you can also use [`std::fs::read`]):
2019-10-15T22:02:04.0636977Z    |
2019-10-15T22:02:04.0637395Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-10-15T22:02:04.0637537Z 
2019-10-15T22:02:04.3579280Z error: aborting due to 2 previous errors
2019-10-15T22:02:04.3579280Z error: aborting due to 2 previous errors
2019-10-15T22:02:04.3579375Z 
2019-10-15T22:02:04.3974487Z error: Could not document `std`.
2019-10-15T22:02:04.3974580Z 
2019-10-15T22:02:04.3974630Z Caused by:
2019-10-15T22:02:04.3978042Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --color always --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.40.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-3551d60d1d65c68a.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-2937c7e63d68f3b1.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-942928a82284b8e4.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-3a009331d0964bed.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-67faf12ddd846db2.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-d66f4f2f54eece34.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-c1a3cc2ad9e0a1c3.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-7c8d78f989a1a1ce.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-1d2173391b029d77.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-3152adc362a03acb.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0725c4ee8cd1fcb5.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-5f5dfedc27dfc645.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-60d8f5dbea090760.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-1d9f57a3c7d844c7.rmeta` (exit code: 1)
2019-10-15T22:02:04.3993059Z 
2019-10-15T22:02:04.3993059Z 
2019-10-15T22:02:04.3994585Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.40.0" "--index-page" "/checkout/src/doc/index.md"
2019-10-15T22:02:04.3994750Z 
2019-10-15T22:02:04.3994782Z 
2019-10-15T22:02:04.3999419Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-15T22:02:04.3999476Z Build completed unsuccessfully in 1:33:54
2019-10-15T22:02:04.3999476Z Build completed unsuccessfully in 1:33:54
2019-10-15T22:02:04.4049318Z == clock drift check ==
2019-10-15T22:02:04.4067525Z   local time: Tue Oct 15 22:02:04 UTC 2019
2019-10-15T22:02:04.6846998Z   network time: Tue, 15 Oct 2019 22:02:04 GMT
2019-10-15T22:02:04.6847069Z == end clock drift check ==
2019-10-15T22:02:09.9345860Z ##[error]Bash exited with code '1'.
2019-10-15T22:02:09.9385310Z ##[section]Starting: Checkout
2019-10-15T22:02:09.9387018Z ==============================================================================
2019-10-15T22:02:09.9387070Z Task         : Get sources
2019-10-15T22:02:09.9387105Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
