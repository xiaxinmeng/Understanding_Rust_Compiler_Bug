plain
2020-03-31T18:44:50.9034189Z ========================== Starting Command Output ===========================
2020-03-31T18:44:50.9037749Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e7cc0360-b5f5-4278-bc75-fa91e44ca3c4.sh
2020-03-31T18:44:50.9038029Z 
2020-03-31T18:44:50.9042367Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T18:44:50.9060639Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T18:44:50.9063882Z Task         : Get sources
2020-03-31T18:44:50.9064196Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T18:44:50.9064504Z Version      : 1.0.0
2020-03-31T18:44:50.9064708Z Author       : Microsoft
---
2020-03-31T18:44:52.0957197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T18:44:52.0967417Z ##[command]git config gc.auto 0
2020-03-31T18:44:52.0973914Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T18:44:52.0979830Z ##[command]git config --get-all http.proxy
2020-03-31T18:44:52.0986996Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70612/merge:refs/remotes/pull/70612/merge
---
2020-03-31T18:52:41.0938021Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T18:52:42.4488377Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T18:52:43.9693418Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T18:52:44.0037596Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T18:52:53.2647093Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T18:52:54.7855183Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T18:52:58.9040778Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T18:53:02.8485791Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T18:53:11.8381032Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T19:14:19.2820709Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T19:14:20.9652775Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T19:14:22.9447865Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T19:14:23.8106408Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T19:14:34.5098472Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T19:14:36.8275534Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T19:14:41.9694689Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T19:14:47.1581862Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T19:14:58.2813524Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T19:39:18.6652731Z .................................................................................................... 1700/9861
2020-03-31T19:39:22.5209152Z .................................................................................................... 1800/9861
2020-03-31T19:39:30.9527904Z ..............................................................................................i..... 1900/9861
2020-03-31T19:39:38.3719369Z .................................................................................................... 2000/9861
2020-03-31T19:39:44.4224749Z ....................................................................................iiiii........... 2100/9861
2020-03-31T19:40:04.4455604Z .................................................................................................... 2300/9861
2020-03-31T19:40:06.5865709Z .................................................................................................... 2400/9861
2020-03-31T19:40:08.7754914Z .................................................................................................... 2500/9861
2020-03-31T19:40:14.5648778Z .................................................................................................... 2600/9861
---
2020-03-31T19:42:59.5363868Z ..........................................................i...............i......................... 5000/9861
2020-03-31T19:43:06.9065825Z .................................................................................................... 5100/9861
2020-03-31T19:43:14.2260710Z .................................................................................................... 5200/9861
2020-03-31T19:43:19.0889695Z ...i................................................................................................ 5300/9861
2020-03-31T19:43:29.0167586Z .........................................................................................ii.ii...... 5400/9861
2020-03-31T19:43:33.0023863Z ..i...i............................................................................................. 5500/9861
2020-03-31T19:43:41.4217193Z ..................................i................................................................. 5700/9861
2020-03-31T19:43:50.8793315Z ....................................................ii....................................i......... 5800/9861
2020-03-31T19:43:58.0823251Z .................................................................................................... 5900/9861
2020-03-31T19:44:02.7281398Z .................................................................................................... 6000/9861
2020-03-31T19:44:02.7281398Z .................................................................................................... 6000/9861
2020-03-31T19:44:11.6747297Z ....................................................................................ii...i..ii...... 6100/9861
2020-03-31T19:44:31.8444679Z .................................................................................................... 6300/9861
2020-03-31T19:44:35.7853957Z .................................................................................................... 6400/9861
2020-03-31T19:44:38.7608730Z .................................................................................................... 6500/9861
2020-03-31T19:44:38.7608730Z .................................................................................................... 6500/9861
2020-03-31T19:44:50.7811266Z ..............i..ii................................................................................. 6600/9861
2020-03-31T19:45:10.5172864Z .................................................................................................... 6800/9861
2020-03-31T19:45:12.5623998Z ..............i..................................................................................... 6900/9861
2020-03-31T19:45:14.6231687Z .................................................................................................... 7000/9861
2020-03-31T19:45:16.8002420Z ....................................................i............................................... 7100/9861
---
2020-03-31T19:46:51.9594265Z .................................................................................................... 7800/9861
2020-03-31T19:46:56.8684832Z .................................................................................................... 7900/9861
2020-03-31T19:47:01.9672493Z .................................................................................................... 8000/9861
2020-03-31T19:47:10.0463337Z ............i....................................................................................... 8100/9861
2020-03-31T19:47:18.0049422Z .............................................................iiiiiiiiii.i........................... 8200/9861
2020-03-31T19:47:32.0045935Z .....i......i....................................................................................... 8400/9861
2020-03-31T19:47:36.6809714Z .................................................................................................... 8500/9861
2020-03-31T19:47:47.8185949Z .................................................................................................... 8600/9861
2020-03-31T19:47:57.9769714Z .................................................................................................... 8700/9861
---
2020-03-31T19:50:13.2966738Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-31T19:50:13.3157054Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T19:50:13.5051209Z 
2020-03-31T19:50:13.5051525Z running 183 tests
2020-03-31T19:50:16.1344879Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-31T19:50:18.6227951Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-31T19:50:18.6231723Z 
2020-03-31T19:50:18.6232321Z  finished in 5.306
2020-03-31T19:50:18.6237919Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-31T19:50:18.6425185Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T19:50:20.6449304Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-31T19:50:20.6645611Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T19:50:20.8217960Z 
2020-03-31T19:50:20.8219410Z running 9 tests
2020-03-31T19:50:20.8220519Z iiiiiiiii
2020-03-31T19:50:20.8221485Z 
2020-03-31T19:50:20.8221699Z  finished in 0.157
2020-03-31T19:50:20.8225525Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-31T19:50:20.8418944Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T19:50:40.1629274Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-31T19:50:40.1848120Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T19:50:40.3634260Z 
2020-03-31T19:50:40.3634564Z running 115 tests
2020-03-31T19:50:53.2222224Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-31T19:50:54.8383491Z ...iiii.....ii.
2020-03-31T19:50:54.8384751Z 
2020-03-31T19:50:54.8387466Z  finished in 14.653
2020-03-31T19:50:54.8394309Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-31T19:50:54.8398354Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T20:02:39.8318828Z 
2020-03-31T20:02:39.8320560Z    Doc-tests core
2020-03-31T20:02:44.1407052Z 
2020-03-31T20:02:44.1407933Z running 2489 tests
2020-03-31T20:02:52.8537216Z ......iiiii......................................................................................... 100/2489
2020-03-31T20:03:01.3110488Z .....................................................................................ii............. 200/2489
2020-03-31T20:03:21.1272241Z ....................i............................................................................... 400/2489
2020-03-31T20:03:21.1272241Z ....................i............................................................................... 400/2489
2020-03-31T20:03:30.4741219Z ..........................................................................i..i..................iiii 500/2489
2020-03-31T20:03:45.8550761Z .................................................................................................... 700/2489
2020-03-31T20:03:53.7841864Z .................................................................................................... 800/2489
2020-03-31T20:04:01.8290634Z .................................................................................................... 900/2489
2020-03-31T20:04:09.9658598Z .................................................................................................... 1000/2489
---
2020-03-31T20:07:21.0411657Z .................................................................................................... 500/763
2020-03-31T20:07:21.0777635Z .....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-31T20:07:21.0789358Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-03-31T20:07:21.0794388Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-03-31T20:07:21.0812341Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-03-31T20:07:21.5500417Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-31T20:07:21.5513126Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-03-31T20:07:21.5524831Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libstd/sync/mpsc/mod.rs:2034:21
2020-03-31T20:07:21.5547347Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-31T20:07:23.6154947Z .........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-31T20:07:23.6155761Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-03-31T20:07:23.6156571Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-03-31T20:07:23.6162152Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-03-31T20:07:32.5563958Z 
2020-03-31T20:07:32.5564372Z running 1019 tests
2020-03-31T20:07:49.1326058Z i................................................................................................... 100/1019
2020-03-31T20:07:58.7400817Z .................................................................................................... 200/1019
2020-03-31T20:08:05.8328066Z ..................iii......i......i...i......i...................................................... 300/1019
2020-03-31T20:08:17.0204744Z ...................................................i....i......................................ii... 500/1019
2020-03-31T20:08:24.4524955Z .................................................................................................... 600/1019
2020-03-31T20:08:29.2137264Z .................................................................................................... 700/1019
2020-03-31T20:08:29.2137264Z .................................................................................................... 700/1019
2020-03-31T20:08:36.0206605Z .............................................iiii................................................... 800/1019
2020-03-31T20:08:49.3059298Z .................................................................................................... 900/1019
2020-03-31T20:08:55.3200527Z ...................................................................iiii............................. 1000/1019
2020-03-31T20:08:56.5519717Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-31T20:08:56.5520975Z 
2020-03-31T20:08:56.5619935Z  finished in 158.165
2020-03-31T20:08:56.5622324Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-31T20:11:54.7732057Z 
2020-03-31T20:11:54.7732510Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-31T20:11:54.7732918Z 
2020-03-31T20:11:54.7790726Z  finished in 0.898
2020-03-31T20:11:54.7791847Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-31T20:11:54.7807253Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T20:11:54.9583976Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T20:11:55.8865221Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-8f684ad7079c0468
2020-03-31T20:11:55.8901421Z 
2020-03-31T20:11:55.8902028Z running 0 tests
2020-03-31T20:11:55.8902400Z 
---
2020-03-31T20:25:18.7836763Z     Checking backtrace v0.3.46
2020-03-31T20:25:20.7274688Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-03-31T20:25:20.7276791Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-31T20:25:21.4678962Z  Documenting std v0.0.0 (/checkout/src/libstd)
2020-03-31T20:25:25.8142943Z error: `[write_vectored]` cannot be resolved, ignoring it.
2020-03-31T20:25:25.8146934Z     --> src/libstd/io/mod.rs:1382:45
2020-03-31T20:25:25.8162049Z      |
2020-03-31T20:25:25.8163837Z 1382 |     /// This method will continuously call [`write_vectored`] until there is no
2020-03-31T20:25:25.8167644Z      |
2020-03-31T20:25:25.8168954Z note: the lint level is defined here
2020-03-31T20:25:25.8170381Z     --> src/libstd/lib.rs:213:9
2020-03-31T20:25:25.8171591Z      |
2020-03-31T20:25:25.8171591Z      |
2020-03-31T20:25:25.8173267Z 213  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2020-03-31T20:25:25.8176921Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-03-31T20:25:25.8177893Z 
2020-03-31T20:25:25.8177893Z 
2020-03-31T20:25:25.8179268Z error: `[write_vectored]` cannot be resolved, ignoring it.
2020-03-31T20:25:25.8180779Z     --> src/libstd/io/mod.rs:1389:63
2020-03-31T20:25:25.8182002Z      |
2020-03-31T20:25:25.8183517Z 1389 |     /// If the buffer contains no data, this will never call [`write_vectored`].
2020-03-31T20:25:25.8186698Z      |
2020-03-31T20:25:25.8188621Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-03-31T20:25:25.8190032Z 
2020-03-31T20:25:26.1237854Z error: aborting due to 2 previous errors
2020-03-31T20:25:26.1237854Z error: aborting due to 2 previous errors
2020-03-31T20:25:26.1238412Z 
2020-03-31T20:25:26.1471891Z error: Could not document `std`.
2020-03-31T20:25:26.1472165Z 
2020-03-31T20:25:26.1472338Z Caused by:
2020-03-31T20:25:26.1479592Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages -Z unstable-options --resource-suffix 1.44.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c4955f7c11866375.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-fe732e07cc6b018b.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-b0a312d64aa824d2.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-66b1433fa626bbbd.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-f3b6809857109b0f.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-144a4d778798e366.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-53781145e5aa08ab.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-d145b2001095eff1.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-77e11d514de0b2c0.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-f83cf9ea6ff98591.rmeta` (exit code: 1)
2020-03-31T20:25:26.1501017Z 
2020-03-31T20:25:26.1501017Z 
2020-03-31T20:25:26.1503068Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "-Z" "unstable-options" "--resource-suffix" "1.44.0" "--index-page" "/checkout/src/doc/index.md"
2020-03-31T20:25:26.1504355Z 
2020-03-31T20:25:26.1504461Z 
2020-03-31T20:25:26.1507953Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T20:25:26.1509217Z Build completed unsuccessfully in 1:36:04
2020-03-31T20:25:26.1509217Z Build completed unsuccessfully in 1:36:04
2020-03-31T20:25:26.1562262Z == clock drift check ==
2020-03-31T20:25:26.1580474Z   local time: Tue Mar 31 20:25:26 UTC 2020
2020-03-31T20:25:26.2265488Z   network time: Tue, 31 Mar 2020 20:25:26 GMT
2020-03-31T20:25:26.2266082Z == end clock drift check ==
2020-03-31T20:25:29.3001702Z 
2020-03-31T20:25:29.3079109Z ##[error]Bash exited with code '1'.
2020-03-31T20:25:29.3095038Z ##[section]Finishing: Run build
2020-03-31T20:25:29.3141799Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T20:25:29.3146446Z Task         : Get sources
2020-03-31T20:25:29.3146775Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T20:25:29.3147203Z Version      : 1.0.0
2020-03-31T20:25:29.3147428Z Author       : Microsoft
2020-03-31T20:25:29.3147428Z Author       : Microsoft
2020-03-31T20:25:29.3147934Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T20:25:29.3148615Z ==============================================================================
2020-03-31T20:25:29.6429694Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T20:25:29.6474319Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T20:25:29.6559927Z Cleaning up task key
2020-03-31T20:25:29.6561226Z Start cleaning up orphan processes.
2020-03-31T20:25:29.6735315Z Terminate orphan process: pid (3481) (python)
2020-03-31T20:25:29.6972941Z ##[section]Finishing: Finalize Job
