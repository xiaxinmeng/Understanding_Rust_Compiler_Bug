plain
2020-04-16T14:06:03.1077450Z ========================== Starting Command Output ===========================
2020-04-16T14:06:03.1079856Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad9c5950-ed3f-400a-924f-2e8b8aa91e84.sh
2020-04-16T14:06:03.1080090Z 
2020-04-16T14:06:03.1084108Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T14:06:03.1103746Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71200/merge to s
2020-04-16T14:06:03.1106939Z Task         : Get sources
2020-04-16T14:06:03.1107215Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T14:06:03.1107486Z Version      : 1.0.0
2020-04-16T14:06:03.1107668Z Author       : Microsoft
---
2020-04-16T14:06:04.1217501Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T14:06:04.1224812Z ##[command]git config gc.auto 0
2020-04-16T14:06:04.1230307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T14:06:04.1235683Z ##[command]git config --get-all http.proxy
2020-04-16T14:06:04.1243452Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71200/merge:refs/remotes/pull/71200/merge
---
2020-04-16T14:08:22.9216824Z  ---> f58a2bb1e753
2020-04-16T14:08:22.9217860Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-16T14:08:22.9218684Z  ---> Using cache
2020-04-16T14:08:22.9219229Z  ---> d079cc6b6db8
2020-04-16T14:08:22.9220453Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-16T14:08:22.9221944Z  ---> 4183ca46ee56
2020-04-16T14:08:22.9222312Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-16T14:08:22.9222798Z  ---> Using cache
2020-04-16T14:08:22.9223182Z  ---> 69e7f8a2a2fb
---
2020-04-16T14:08:23.6507687Z Looks like docker image is the same as before, not uploading
2020-04-16T14:08:30.7202188Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T14:08:30.7434217Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T14:08:30.7462416Z == clock drift check ==
2020-04-16T14:08:30.7471863Z   local time: Thu Apr 16 14:08:30 UTC 2020
2020-04-16T14:08:30.8072068Z   network time: Thu, 16 Apr 2020 14:08:30 GMT
2020-04-16T14:08:30.8095736Z Starting sccache server...
2020-04-16T14:08:30.8949734Z configure: processing command line
2020-04-16T14:08:30.8950328Z configure: 
2020-04-16T14:08:30.8951539Z configure: rust.dist-src        := False
---
2020-04-16T14:13:26.6160274Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T14:13:27.9694482Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T14:13:29.5776479Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T14:13:29.6474099Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T14:13:38.9931218Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T14:13:40.5136419Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T14:13:44.6348312Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T14:13:48.5553234Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T14:13:57.7077717Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T14:35:13.1866515Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T14:35:14.8693634Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T14:35:16.7592554Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T14:35:17.9015768Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T14:35:28.5273207Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T14:35:30.8224167Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T14:35:35.9017974Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T14:35:41.0489350Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T14:35:52.1019374Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T15:00:31.8500519Z .................................................................................................... 1700/9897
2020-04-16T15:00:36.1436521Z .................................................................................................... 1800/9897
2020-04-16T15:00:44.1391012Z .................................................................................................... 1900/9897
2020-04-16T15:00:52.0578881Z ......i............................................................................................. 2000/9897
2020-04-16T15:00:58.1797202Z ................................................................................................iiii 2100/9897
2020-04-16T15:01:12.3669572Z i................................................................................................... 2200/9897
2020-04-16T15:01:20.6980008Z .................................................................................................... 2400/9897
2020-04-16T15:01:22.4248097Z .................................................................................................... 2500/9897
2020-04-16T15:01:27.9251720Z .................................................................................................... 2600/9897
2020-04-16T15:01:46.5682634Z .................................................................................................... 2700/9897
---
2020-04-16T15:04:24.9203539Z .................................................................................................... 5100/9897
2020-04-16T15:04:32.7607557Z .................................................................................................... 5200/9897
2020-04-16T15:04:37.0004485Z .................i.................................................................................. 5300/9897
2020-04-16T15:04:47.1083209Z .......i............................................................................................ 5400/9897
2020-04-16T15:04:52.5115459Z .......ii.ii........i...i........................................................................... 5500/9897
2020-04-16T15:05:00.1846037Z .....................................................i.............................................. 5700/9897
2020-04-16T15:05:09.6658801Z .........................................................................ii......................... 5800/9897
2020-04-16T15:05:16.2660447Z ............i....................................................................................... 5900/9897
2020-04-16T15:05:21.4701209Z .................................................................................................... 6000/9897
2020-04-16T15:05:21.4701209Z .................................................................................................... 6000/9897
2020-04-16T15:05:31.9960776Z .................................................................................................... 6100/9897
2020-04-16T15:05:42.5802608Z ......ii...i..ii...........i........................................................................ 6200/9897
2020-04-16T15:05:57.8718506Z .................................................................................................... 6400/9897
2020-04-16T15:06:01.7353538Z .................................................................................................... 6500/9897
2020-04-16T15:06:01.7353538Z .................................................................................................... 6500/9897
2020-04-16T15:06:13.1715736Z ....................................i..ii........................................................... 6600/9897
2020-04-16T15:06:34.0275571Z .................................................................................................... 6800/9897
2020-04-16T15:06:35.9969804Z ....................................i............................................................... 6900/9897
2020-04-16T15:06:38.0589580Z .................................................................................................... 7000/9897
2020-04-16T15:06:40.3113835Z ............................................................................i....................... 7100/9897
---
2020-04-16T15:08:16.5530048Z .................................................................................................... 7800/9897
2020-04-16T15:08:20.8933576Z .................................................................................................... 7900/9897
2020-04-16T15:08:27.6211896Z .................................................................................................... 8000/9897
2020-04-16T15:08:34.0201757Z ..........................................i......................................................... 8100/9897
2020-04-16T15:08:43.9465301Z ..........................................................................................iiiiii.iii 8200/9897
2020-04-16T15:08:50.1679801Z ii.i................................................................................................ 8300/9897
2020-04-16T15:09:03.0830336Z .................................................................................................... 8500/9897
2020-04-16T15:09:13.1284370Z .................................................................................................... 8600/9897
2020-04-16T15:09:26.6569080Z .................................................................................................... 8700/9897
2020-04-16T15:09:33.1388867Z .................................................................................................... 8800/9897
---
2020-04-16T15:11:46.7707061Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-16T15:11:46.7893754Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:11:46.9869848Z 
2020-04-16T15:11:46.9870209Z running 185 tests
2020-04-16T15:11:49.5841420Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-16T15:11:52.0349965Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-16T15:11:52.0352708Z 
2020-04-16T15:11:52.0358731Z  finished in 5.246
2020-04-16T15:11:52.0364301Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-16T15:11:52.0547625Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T15:11:54.0902547Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-16T15:11:54.1093398Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:11:54.2616766Z 
2020-04-16T15:11:54.2617277Z running 9 tests
2020-04-16T15:11:54.2618474Z iiiiiiiii
2020-04-16T15:11:54.2624459Z 
2020-04-16T15:11:54.2626202Z  finished in 0.153
2020-04-16T15:11:54.2633583Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-16T15:11:54.2925903Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T15:12:13.8634701Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-16T15:12:13.8866943Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:12:14.0694814Z 
2020-04-16T15:12:14.0695742Z running 115 tests
2020-04-16T15:12:26.9072705Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-16T15:12:28.6942394Z ...iiii.....ii.
2020-04-16T15:12:28.6943563Z 
2020-04-16T15:12:28.6947371Z  finished in 14.808
2020-04-16T15:12:28.6954369Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-16T15:12:28.6955089Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T15:24:14.3051617Z 
2020-04-16T15:24:14.3052547Z    Doc-tests core
2020-04-16T15:24:18.6096616Z 
2020-04-16T15:24:18.6097047Z running 2496 tests
2020-04-16T15:24:27.1399472Z ......iiiii......................................................................................... 100/2496
2020-04-16T15:24:35.4606683Z .....................................................................................ii............. 200/2496
2020-04-16T15:24:54.6517435Z ....................i............................................................................... 400/2496
2020-04-16T15:24:54.6517435Z ....................i............................................................................... 400/2496
2020-04-16T15:25:03.7904912Z ..........................................................................i..i..................iiii 500/2496
2020-04-16T15:25:19.0092141Z .................................................................................................... 700/2496
2020-04-16T15:25:26.9275664Z .................................................................................................... 800/2496
2020-04-16T15:25:34.7716372Z .................................................................................................... 900/2496
2020-04-16T15:25:42.6235478Z .................................................................................................... 1000/2496
---
2020-04-16T15:28:50.6819852Z .................................................................................................... 500/764
2020-04-16T15:28:50.7111984Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-16T15:28:50.7123936Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-16T15:28:50.7129903Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-16T15:28:50.7152857Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-16T15:28:51.0388394Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-16T15:28:51.0413780Z ....thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError..', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-16T15:28:51.0440665Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-16T15:28:53.0880924Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-16T15:28:53.0888479Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-16T15:28:53.0894861Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-16T15:28:53.0897739Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-16T15:29:02.0731603Z 
2020-04-16T15:29:02.0731968Z running 1020 tests
2020-04-16T15:29:18.0247320Z i................................................................................................... 100/1020
2020-04-16T15:29:27.4710721Z .................................................................................................... 200/1020
2020-04-16T15:29:34.4741060Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-16T15:29:38.9215228Z .................................................................................................... 400/1020
2020-04-16T15:29:45.0559511Z ....................................................i....i......................................ii.. 500/1020
2020-04-16T15:29:56.6741334Z .................................................................................................... 700/1020
2020-04-16T15:29:56.6741334Z .................................................................................................... 700/1020
2020-04-16T15:30:03.1375285Z ..............................................iiii.................................................. 800/1020
2020-04-16T15:30:15.8991553Z .................................................................................................... 900/1020
2020-04-16T15:30:21.4754890Z ....................................................................iiii............................ 1000/1020
2020-04-16T15:30:22.6523998Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-16T15:30:22.6528065Z 
2020-04-16T15:30:22.6634096Z  finished in 152.748
2020-04-16T15:30:22.6640117Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-16T15:33:18.0618531Z running 0 tests
2020-04-16T15:33:18.0618672Z 
2020-04-16T15:33:18.0618947Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T15:33:18.0619196Z 
2020-04-16T15:33:18.0794931Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-16T15:33:18.0796247Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:33:18.0796247Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:33:18.2455522Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T15:33:19.1641961Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-4852cc3c53f0787b
2020-04-16T15:33:19.1672804Z 
2020-04-16T15:33:19.1673057Z running 0 tests
2020-04-16T15:33:19.1673169Z 
---
2020-04-16T15:46:50.7264625Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7265349Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7266080Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7266811Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7267556Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7269018Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7269762Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7270721Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T15:46:50.7271484Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-16T15:47:49.9504430Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-16T15:47:49.9870195Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:47:50.2153069Z 
2020-04-16T15:47:50.2153374Z running 211 tests
2020-04-16T15:48:19.9315775Z ......................i...ii.......................................................................i 100/211
2020-04-16T15:48:54.4420633Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-16T15:48:59.3251912Z .......ii..
2020-04-16T15:48:59.3258072Z 
2020-04-16T15:48:59.3264528Z  finished in 69.339
2020-04-16T15:48:59.3273936Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-16T15:48:59.3282269Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-16T15:49:12.3359239Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-16T15:49:12.3524746Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-16T15:49:12.4977664Z  finished in 4.028
2020-04-16T15:49:12.4986568Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-16T15:49:13.3968825Z Checking "alias-1" ... OK
2020-04-16T15:49:13.4742344Z Checking "alias-2" ... OK
2020-04-16T15:49:13.5312899Z Checking "alias-3" ... OK
2020-04-16T15:49:13.5945014Z Checking "alias" ... OK
2020-04-16T15:49:13.6692329Z Checking "basic" ... OK
2020-04-16T15:49:13.7436705Z Checking "deduplication" ... OK
2020-04-16T15:49:13.7918187Z Checking "enum-option" ... OK
2020-04-16T15:49:13.8627644Z Checking "filter-crate" ... OK
2020-04-16T15:49:13.9232501Z Checking "fn-forget" ... OK
2020-04-16T15:49:14.0254401Z Checking "from_u" ... OK
2020-04-16T15:49:14.1147290Z Checking "keyword" ... OK
2020-04-16T15:49:14.1585743Z Checking "macro-check" ... OK
2020-04-16T15:49:14.1911087Z Checking "macro-print" ... OK
2020-04-16T15:49:14.2613596Z Checking "multi-query" ... OK
2020-04-16T15:49:14.2910268Z Checking "never" ... OK
2020-04-16T15:49:14.3078284Z Checking "quoted" ... OK
2020-04-16T15:49:14.3363950Z Checking "return-specific-literal" ... OK
2020-04-16T15:49:14.4352222Z Checking "return-specific" ... OK
2020-04-16T15:49:14.4672568Z Checking "should-fail" ... OK
2020-04-16T15:49:14.5297223Z Checking "string-from_ut" ... OK
2020-04-16T15:49:14.5861395Z Checking "struct-vec" ... OK
2020-04-16T15:49:14.6947674Z Checking "vec-new" ... OK
2020-04-16T15:49:14.7192349Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:49:14.8801059Z 
2020-04-16T15:49:14.8801382Z running 6 tests
2020-04-16T15:49:20.3405978Z ......
---
2020-04-16T15:50:20.1773283Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-16T15:50:20.1964645Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:50:20.3433033Z 
2020-04-16T15:50:20.3433392Z running 13 tests
2020-04-16T15:50:20.9623679Z .iiiiiii.iii.
2020-04-16T15:50:20.9624750Z 
2020-04-16T15:50:20.9628675Z  finished in 0.766
2020-04-16T15:50:20.9692798Z Build completed successfully in 1:40:17
2020-04-16T15:50:20.9692798Z Build completed successfully in 1:40:17
2020-04-16T15:50:20.9701941Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T15:50:22.2167562Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T15:50:22.4387082Z     Finished release [optimized] target(s) in 0.21s
2020-04-16T15:50:22.4465465Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-16T15:50:22.4574133Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T15:51:32.3963445Z     Finished release [optimized] target(s) in 0.19s
2020-04-16T15:51:32.4205052Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> armv5te-unknown-linux-gnueabi)
2020-04-16T15:51:32.5997790Z 
2020-04-16T15:51:32.5998071Z running 89 tests
2020-04-16T15:51:39.6855129Z .F...F.FFF.F.....F...F....F.F................F.......F.........F.....F.......F.........F.
2020-04-16T15:51:39.6858583Z 
2020-04-16T15:51:39.6902752Z ---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
2020-04-16T15:51:39.6903250Z 1 // MIR for `main` after SimplifyCfg-elaborate-drops
2020-04-16T15:51:39.6903436Z 2 
2020-04-16T15:51:39.6903436Z 2 
2020-04-16T15:51:39.6903716Z 3 fn main() -> () {
2020-04-16T15:51:39.6904285Z -     let mut _0: ();                      // return place in scope 0 at $DIR/array-index-is-temporary.rs:12:11: 12:11
2020-04-16T15:51:39.6905014Z -     let mut _1: [u32; 3];                // in scope 0 at $DIR/array-index-is-temporary.rs:13:9: 13:14
2020-04-16T15:51:39.6905689Z -     let mut _4: &mut usize;              // in scope 0 at $DIR/array-index-is-temporary.rs:15:25: 15:31
2020-04-16T15:51:39.6906372Z -     let mut _5: u32;                     // in scope 0 at $DIR/array-index-is-temporary.rs:16:12: 16:29
2020-04-16T15:51:39.6907034Z -     let mut _6: *mut usize;              // in scope 0 at $DIR/array-index-is-temporary.rs:16:25: 16:26
2020-04-16T15:51:39.6907875Z -     let _7: usize;                       // in scope 0 at $DIR/array-index-is-temporary.rs:16:7: 16:8
2020-04-16T15:51:39.6908595Z -     let mut _8: usize;                   // in scope 0 at $DIR/array-index-is-temporary.rs:16:5: 16:9
2020-04-16T15:51:39.6909299Z -     let mut _9: bool;                    // in scope 0 at $DIR/array-index-is-temporary.rs:16:5: 16:9
2020-04-16T15:51:39.6909634Z +     let mut _0: ();
2020-04-16T15:51:39.6909851Z +     let mut _1: [u32; 3];
2020-04-16T15:51:39.6910283Z +     let mut _5: u32;
2020-04-16T15:51:39.6910503Z +     let mut _6: *mut usize;
2020-04-16T15:51:39.6932560Z +     let _7: usize;
2020-04-16T15:51:39.6932756Z +     let mut _8: usize;
2020-04-16T15:51:39.6932756Z +     let mut _8: usize;
2020-04-16T15:51:39.6933113Z +     let mut _9: bool;
2020-04-16T15:51:39.6933282Z 12     scope 1 {
2020-04-16T15:51:39.6940695Z -         debug x => _1;                   // in scope 1 at $DIR/array-index-is-temporary.rs:13:9: 13:14
2020-04-16T15:51:39.6941375Z -         let mut _2: usize;               // in scope 1 at $DIR/array-index-is-temporary.rs:14:9: 14:14
2020-04-16T15:51:39.6941688Z +         debug x => _1;
2020-04-16T15:51:39.6941874Z +         let mut _2: usize;
2020-04-16T15:51:39.6942075Z 15         scope 2 {
2020-04-16T15:51:39.6942588Z -             debug y => _2;               // in scope 2 at $DIR/array-index-is-temporary.rs:14:9: 14:14
2020-04-16T15:51:39.6943513Z -             let _3: *mut usize as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 2 at $DIR/array-index-is-temporary.rs:15:9: 15:10
2020-04-16T15:51:39.6944020Z +             debug y => _2;
2020-04-16T15:51:39.6944325Z +             let _3: *mut usize as UserTypeProjection { base: UserType(0), projs: [] };
2020-04-16T15:51:39.6944621Z 18             scope 3 {
2020-04-16T15:51:39.6945164Z -                 debug z => _3;           // in scope 3 at $DIR/array-index-is-temporary.rs:15:9: 15:10
2020-04-16T15:51:39.6945474Z +                 debug z => _3;
2020-04-16T15:51:39.6945846Z 21                 }
2020-04-16T15:51:39.6946000Z 22             }
2020-04-16T15:51:39.6946111Z 
2020-04-16T15:51:39.6946242Z 24     }
2020-04-16T15:51:39.6946242Z 24     }
2020-04-16T15:51:39.6946361Z 25 
2020-04-16T15:51:39.6946493Z 26     bb0: {
2020-04-16T15:51:39.6947064Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/array-index-is-temporary.rs:13:9: 13:14
2020-04-16T15:51:39.6947872Z -         _1 = [const 42u32, const 43u32, const 44u32]; // bb0[1]: scope 0 at $DIR/array-index-is-temporary.rs:13:17: 13:29
2020-04-16T15:51:39.6948232Z +         StorageLive(_1);
2020-04-16T15:51:39.6948457Z +         _1 = [const 42u32, const 43u32, const 44u32];
2020-04-16T15:51:39.6948725Z 29                                          // ty::Const
2020-04-16T15:51:39.6948987Z 30                                          // + ty: u32
2020-04-16T15:51:39.6949303Z 31                                          // + val: Value(Scalar(0x0000002a))
2020-04-16T15:51:39.6949704Z 44                                          // mir::Constant
2020-04-16T15:51:39.6949704Z 44                                          // mir::Constant
2020-04-16T15:51:39.6950300Z 45                                          // + span: $DIR/array-index-is-temporary.rs:13:26: 13:28
2020-04-16T15:51:39.6951051Z 46                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002c)) }
2020-04-16T15:51:39.6951818Z -         StorageLive(_2);                 // bb0[2]: scope 1 at $DIR/array-index-is-temporary.rs:14:9: 14:14
2020-04-16T15:51:39.6952564Z -         _2 = const 1usize;               // bb0[3]: scope 1 at $DIR/array-index-is-temporary.rs:14:17: 14:18
2020-04-16T15:51:39.6952926Z +         StorageLive(_2);
2020-04-16T15:51:39.6953335Z 49                                          // ty::Const
2020-04-16T15:51:39.6953612Z 50                                          // + ty: usize
2020-04-16T15:51:39.6953612Z 50                                          // + ty: usize
2020-04-16T15:51:39.6953922Z 51                                          // + val: Value(Scalar(0x00000001))
2020-04-16T15:51:39.6954334Z 52                                          // mir::Constant
2020-04-16T15:51:39.6954334Z 52                                          // mir::Constant
2020-04-16T15:51:39.6954917Z 53                                          // + span: $DIR/array-index-is-temporary.rs:14:17: 14:18
2020-04-16T15:51:39.6955413Z 54                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
2020-04-16T15:51:39.6956150Z -         StorageLive(_3);                 // bb0[4]: scope 2 at $DIR/array-index-is-temporary.rs:15:9: 15:10
2020-04-16T15:51:39.6956899Z -         StorageLive(_4);                 // bb0[5]: scope 2 at $DIR/array-index-is-temporary.rs:15:25: 15:31
2020-04-16T15:51:39.6957640Z -         _4 = &mut _2;                    // bb0[6]: scope 2 at $DIR/array-index-is-temporary.rs:15:25: 15:31
2020-04-16T15:51:39.6958393Z -         _3 = &raw mut (*_4);             // bb0[7]: scope 2 at $DIR/array-index-is-temporary.rs:15:25: 15:31
2020-04-16T15:51:39.6959127Z -         StorageDead(_4);                 // bb0[8]: scope 2 at $DIR/array-index-is-temporary.rs:15:31: 15:32
2020-04-16T15:51:39.6959874Z -         StorageLive(_5);                 // bb0[9]: scope 3 at $DIR/array-index-is-temporary.rs:16:12: 16:29
2020-04-16T15:51:39.6960615Z -         StorageLive(_6);                 // bb0[10]: scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
2020-04-16T15:51:39.6961361Z -         _6 = _3;                         // bb0[11]: scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
2020-04-16T15:51:39.6962212Z -         _5 = const foo(move _6) -> bb1;  // bb0[12]: scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
2020-04-16T15:51:39.6962619Z +         StorageLive(_3);
2020-04-16T15:51:39.6962791Z +         StorageLive(_4);
2020-04-16T15:51:39.6962978Z +         _4 = &mut _2;
2020-04-16T15:51:39.6963155Z +         _3 = &raw mut (*_4);
2020-04-16T15:51:39.6963335Z +         StorageDead(_4);
2020-04-16T15:51:39.6963524Z +         StorageLive(_5);
2020-04-16T15:51:39.6963697Z +         StorageLive(_6);
2020-04-16T15:51:39.6963859Z +         _6 = _3;
2020-04-16T15:51:39.6964236Z +         _5 = const foo(move _6) -> bb1;
2020-04-16T15:51:39.6964486Z 64                                          // ty::Const
2020-04-16T15:51:39.6965008Z 65                                          // + ty: unsafe fn(*mut usize) -> u32 {foo}
2020-04-16T15:51:39.6965572Z 66                                          // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.6965904Z 70     }
2020-04-16T15:51:39.6966034Z 71 
2020-04-16T15:51:39.6966196Z 72     bb1: {
2020-04-16T15:51:39.6966196Z 72     bb1: {
2020-04-16T15:51:39.6966805Z -         StorageDead(_6);                 // bb1[0]: scope 4 at $DIR/array-index-is-temporary.rs:16:26: 16:27
2020-04-16T15:51:39.6967598Z -         StorageLive(_7);                 // bb1[1]: scope 3 at $DIR/array-index-is-temporary.rs:16:7: 16:8
2020-04-16T15:51:39.6968398Z -         _7 = _2;                         // bb1[2]: scope 3 at $DIR/array-index-is-temporary.rs:16:7: 16:8
2020-04-16T15:51:39.6969179Z -         _8 = Len(_1);                    // bb1[3]: scope 3 at $DIR/array-index-is-temporary.rs:16:5: 16:9
2020-04-16T15:51:39.6970214Z -         _9 = Lt(_7, _8);                 // bb1[4]: scope 3 at $DIR/array-index-is-temporary.rs:16:5: 16:9
2020-04-16T15:51:39.6971209Z -         assert(move _9, "index out of bounds: the len is {} but the index is {}", move _8, _7) -> bb2; // bb1[5]: scope 3 at $DIR/array-index-is-temporary.rs:16:5: 16:9
2020-04-16T15:51:39.6971704Z +         StorageDead(_6);
2020-04-16T15:51:39.6971915Z +         StorageLive(_7);
2020-04-16T15:51:39.6972095Z +         _7 = _2;
2020-04-16T15:51:39.6972268Z +         _8 = Len(_1);
2020-04-16T15:51:39.6972583Z +         _9 = Lt(_7, _8);
2020-04-16T15:51:39.6973178Z +         assert(move _9, "index out of bounds: the len is {} but the index is {}", move _8, _7) -> bb2;
2020-04-16T15:51:39.6973607Z 80 
2020-04-16T15:51:39.6973765Z 81     bb2: {
2020-04-16T15:51:39.6973882Z 
2020-04-16T15:51:39.6973882Z 
2020-04-16T15:51:39.6974461Z -         _1[_7] = move _5;                // bb2[0]: scope 3 at $DIR/array-index-is-temporary.rs:16:5: 16:29
2020-04-16T15:51:39.6975272Z -         StorageDead(_5);                 // bb2[1]: scope 3 at $DIR/array-index-is-temporary.rs:16:28: 16:29
2020-04-16T15:51:39.6976071Z -         StorageDead(_7);                 // bb2[2]: scope 3 at $DIR/array-index-is-temporary.rs:16:29: 16:30
2020-04-16T15:51:39.6976863Z -         _0 = const ();                   // bb2[3]: scope 0 at $DIR/array-index-is-temporary.rs:12:11: 17:2
2020-04-16T15:51:39.6977263Z +         _1[_7] = move _5;
2020-04-16T15:51:39.6977449Z +         StorageDead(_5);
2020-04-16T15:51:39.6977637Z +         StorageDead(_7);
2020-04-16T15:51:39.6977840Z +         _0 = const ();
2020-04-16T15:51:39.6978081Z 86                                          // ty::Const
2020-04-16T15:51:39.6978485Z 87                                          // + ty: ()
2020-04-16T15:51:39.6978815Z 88                                          // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.6979238Z 89                                          // mir::Constant
2020-04-16T15:51:39.6979238Z 89                                          // mir::Constant
2020-04-16T15:51:39.6979882Z 90                                          // + span: $DIR/array-index-is-temporary.rs:12:11: 17:2
2020-04-16T15:51:39.6980540Z 91                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T15:51:39.6981317Z -         StorageDead(_3);                 // bb2[4]: scope 2 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-16T15:51:39.6982377Z -         StorageDead(_2);                 // bb2[5]: scope 1 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-16T15:51:39.6983196Z -         StorageDead(_1);                 // bb2[6]: scope 0 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-16T15:51:39.6983971Z -         return;                          // bb2[7]: scope 0 at $DIR/array-index-is-temporary.rs:17:2: 17:2
2020-04-16T15:51:39.6984353Z +         StorageDead(_3);
2020-04-16T15:51:39.6984533Z +         StorageDead(_2);
2020-04-16T15:51:39.6984717Z +         StorageDead(_1);
2020-04-16T15:51:39.6985058Z 96     }
2020-04-16T15:51:39.6985186Z 97 }
2020-04-16T15:51:39.6985305Z 98 
2020-04-16T15:51:39.6985421Z 
2020-04-16T15:51:39.6985421Z 
2020-04-16T15:51:39.6986469Z thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array-index-is-temporary/32bit/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.6987362Z 
2020-04-16T15:51:39.6987765Z ---- [mir-opt] mir-opt/combine_array_len.rs stdout ----
2020-04-16T15:51:39.6987765Z ---- [mir-opt] mir-opt/combine_array_len.rs stdout ----
2020-04-16T15:51:39.6988023Z 2 + // MIR for `norm2` after InstCombine
2020-04-16T15:51:39.6988217Z 3   
2020-04-16T15:51:39.6988705Z 4   fn norm2(_1: [f32; 2]) -> f32 {
2020-04-16T15:51:39.6989274Z -       debug x => _1;                       // in scope 0 at $DIR/combine_array_len.rs:4:10: 4:11
2020-04-16T15:51:39.6989996Z -       let mut _0: f32;                     // return place in scope 0 at $DIR/combine_array_len.rs:4:26: 4:29
2020-04-16T15:51:39.6990871Z -       let _2: f32;                         // in scope 0 at $DIR/combine_array_len.rs:5:9: 5:10
2020-04-16T15:51:39.6991571Z -       let _3: usize;                       // in scope 0 at $DIR/combine_array_len.rs:5:15: 5:16
2020-04-16T15:51:39.6992408Z -       let mut _4: usize;                   // in scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.6993713Z -       let mut _5: bool;                    // in scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.6998088Z -       let _7: usize;                       // in scope 0 at $DIR/combine_array_len.rs:6:15: 6:16
2020-04-16T15:51:39.6998783Z -       let mut _8: usize;                   // in scope 0 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.6999459Z -       let mut _9: bool;                    // in scope 0 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7000149Z -       let mut _10: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:5: 7:8
2020-04-16T15:51:39.7000822Z -       let mut _11: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:5: 7:6
2020-04-16T15:51:39.7001491Z -       let mut _12: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:7: 7:8
2020-04-16T15:51:39.7002196Z -       let mut _13: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:11: 7:14
2020-04-16T15:51:39.7002885Z -       let mut _14: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:11: 7:12
2020-04-16T15:51:39.7003582Z -       let mut _15: f32;                    // in scope 0 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7004088Z +       let mut _0: f32;
2020-04-16T15:51:39.7004283Z +       let _2: f32;
2020-04-16T15:51:39.7004489Z +       let _3: usize;
2020-04-16T15:51:39.7004690Z +       let mut _4: usize;
---
2020-04-16T15:51:39.7006570Z +       let mut _13: f32;
2020-04-16T15:51:39.7006945Z +       let mut _14: f32;
2020-04-16T15:51:39.7007151Z +       let mut _15: f32;
2020-04-16T15:51:39.7007354Z 20       scope 1 {
2020-04-16T15:51:39.7007911Z -           debug a => _2;                   // in scope 1 at $DIR/combine_array_len.rs:5:9: 5:10
2020-04-16T15:51:39.7008576Z -           let _6: f32;                     // in scope 1 at $DIR/combine_array_len.rs:6:9: 6:10
2020-04-16T15:51:39.7008914Z +           debug a => _2;
2020-04-16T15:51:39.7009110Z +           let _6: f32;
2020-04-16T15:51:39.7009974Z -               debug b => _6;               // in scope 2 at $DIR/combine_array_len.rs:6:9: 6:10
2020-04-16T15:51:39.7010312Z +               debug b => _6;
2020-04-16T15:51:39.7010490Z 25           }
2020-04-16T15:51:39.7010637Z 26       }
2020-04-16T15:51:39.7010637Z 26       }
2020-04-16T15:51:39.7010786Z 27   
2020-04-16T15:51:39.7010888Z 
2020-04-16T15:51:39.7011028Z 28       bb0: {
2020-04-16T15:51:39.7011644Z -           StorageLive(_2);                 // bb0[0]: scope 0 at $DIR/combine_array_len.rs:5:9: 5:10
2020-04-16T15:51:39.7012398Z -           StorageLive(_3);                 // bb0[1]: scope 0 at $DIR/combine_array_len.rs:5:15: 5:16
2020-04-16T15:51:39.7014787Z -           _3 = const 0usize;               // bb0[2]: scope 0 at $DIR/combine_array_len.rs:5:15: 5:16
2020-04-16T15:51:39.7015172Z +           StorageLive(_2);
2020-04-16T15:51:39.7015369Z +           StorageLive(_3);
2020-04-16T15:51:39.7016188Z 32                                            // ty::Const
2020-04-16T15:51:39.7016712Z 33                                            // + ty: usize
2020-04-16T15:51:39.7016712Z 33                                            // + ty: usize
2020-04-16T15:51:39.7017059Z 34                                            // + val: Value(Scalar(0x00000000))
2020-04-16T15:51:39.7017516Z 35                                            // mir::Constant
2020-04-16T15:51:39.7017516Z 35                                            // mir::Constant
2020-04-16T15:51:39.7017897Z 36                                            // + span: $DIR/combine_array_len.rs:5:15: 5:16
2020-04-16T15:51:39.7018412Z 37                                            // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-16T15:51:39.7019252Z - -         _4 = Len(_1);                    // bb0[3]: scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7020002Z - +         _4 = const 2usize;               // bb0[3]: scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7030781Z + -         _4 = Len(_1);
2020-04-16T15:51:39.7030994Z + +         _4 = const 2usize;
2020-04-16T15:51:39.7031246Z 40 +                                          // ty::Const
2020-04-16T15:51:39.7031559Z 41 +                                          // + ty: usize
2020-04-16T15:51:39.7031897Z 42 +                                          // + val: Value(Scalar(0x00000002))
2020-04-16T15:51:39.7032330Z 43 +                                          // mir::Constant
2020-04-16T15:51:39.7032330Z 43 +                                          // mir::Constant
2020-04-16T15:51:39.7032739Z 44 +                                          // + span: $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7033246Z 45 +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-16T15:51:39.7034048Z -           _5 = Lt(_3, _4);                 // bb0[4]: scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7036236Z -           assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // bb0[5]: scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7036824Z +           _5 = Lt(_3, _4);
2020-04-16T15:51:39.7037430Z +           assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1;
2020-04-16T15:51:39.7037877Z 49   
2020-04-16T15:51:39.7038043Z 50       bb1: {
2020-04-16T15:51:39.7038168Z 
2020-04-16T15:51:39.7038168Z 
2020-04-16T15:51:39.7038721Z -           _2 = _1[_3];                     // bb1[0]: scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
2020-04-16T15:51:39.7039635Z -           StorageDead(_3);                 // bb1[1]: scope 0 at $DIR/combine_array_len.rs:5:17: 5:18
2020-04-16T15:51:39.7040472Z -           StorageLive(_6);                 // bb1[2]: scope 1 at $DIR/combine_array_len.rs:6:9: 6:10
2020-04-16T15:51:39.7041226Z -           StorageLive(_7);                 // bb1[3]: scope 1 at $DIR/combine_array_len.rs:6:15: 6:16
2020-04-16T15:51:39.7041984Z -           _7 = const 1usize;               // bb1[4]: scope 1 at $DIR/combine_array_len.rs:6:15: 6:16
2020-04-16T15:51:39.7042334Z +           _2 = _1[_3];
2020-04-16T15:51:39.7042521Z +           StorageDead(_3);
2020-04-16T15:51:39.7042733Z +           StorageLive(_6);
2020-04-16T15:51:39.7042927Z +           StorageLive(_7);
2020-04-16T15:51:39.7043376Z 56                                            // ty::Const
2020-04-16T15:51:39.7043679Z 57                                            // + ty: usize
2020-04-16T15:51:39.7043679Z 57                                            // + ty: usize
2020-04-16T15:51:39.7044025Z 58                                            // + val: Value(Scalar(0x00000001))
2020-04-16T15:51:39.7044477Z 59                                            // mir::Constant
2020-04-16T15:51:39.7044477Z 59                                            // mir::Constant
2020-04-16T15:51:39.7044858Z 60                                            // + span: $DIR/combine_array_len.rs:6:15: 6:16
2020-04-16T15:51:39.7045534Z 61                                            // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
2020-04-16T15:51:39.7046338Z - -         _8 = Len(_1);                    // bb1[5]: scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7047087Z - +         _8 = const 2usize;               // bb1[5]: scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7047594Z + -         _8 = Len(_1);
2020-04-16T15:51:39.7047803Z + +         _8 = const 2usize;
2020-04-16T15:51:39.7048054Z 64 +                                          // ty::Const
2020-04-16T15:51:39.7048347Z 65 +                                          // + ty: usize
2020-04-16T15:51:39.7048793Z 66 +                                          // + val: Value(Scalar(0x00000002))
2020-04-16T15:51:39.7049234Z 67 +                                          // mir::Constant
2020-04-16T15:51:39.7049234Z 67 +                                          // mir::Constant
2020-04-16T15:51:39.7049636Z 68 +                                          // + span: $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7050141Z 69 +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-16T15:51:39.7050918Z -           _9 = Lt(_7, _8);                 // bb1[6]: scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7051898Z -           assert(move _9, "index out of bounds: the len is {} but the index is {}", move _8, _7) -> bb2; // bb1[7]: scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7052365Z +           _9 = Lt(_7, _8);
2020-04-16T15:51:39.7052932Z +           assert(move _9, "index out of bounds: the len is {} but the index is {}", move _8, _7) -> bb2;
2020-04-16T15:51:39.7053404Z 73   
2020-04-16T15:51:39.7053553Z 74       bb2: {
2020-04-16T15:51:39.7053691Z 
2020-04-16T15:51:39.7053691Z 
2020-04-16T15:51:39.7054243Z -           _6 = _1[_7];                     // bb2[0]: scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
2020-04-16T15:51:39.7055140Z -           StorageDead(_7);                 // bb2[1]: scope 1 at $DIR/combine_array_len.rs:6:17: 6:18
2020-04-16T15:51:39.7055910Z -           StorageLive(_10);                // bb2[2]: scope 2 at $DIR/combine_array_len.rs:7:5: 7:8
2020-04-16T15:51:39.7056640Z -           StorageLive(_11);                // bb2[3]: scope 2 at $DIR/combine_array_len.rs:7:5: 7:6
2020-04-16T15:51:39.7057506Z -           _11 = _2;                        // bb2[4]: scope 2 at $DIR/combine_array_len.rs:7:5: 7:6
2020-04-16T15:51:39.7058269Z -           StorageLive(_12);                // bb2[5]: scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
2020-04-16T15:51:39.7059288Z -           _12 = _2;                        // bb2[6]: scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
2020-04-16T15:51:39.7060147Z -           _10 = Mul(move _11, move _12);   // bb2[7]: scope 2 at $DIR/combine_array_len.rs:7:5: 7:8
2020-04-16T15:51:39.7060907Z -           StorageDead(_12);                // bb2[8]: scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
2020-04-16T15:51:39.7061776Z -           StorageDead(_11);                // bb2[9]: scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
2020-04-16T15:51:39.7062541Z -           StorageLive(_13);                // bb2[10]: scope 2 at $DIR/combine_array_len.rs:7:11: 7:14
2020-04-16T15:51:39.7063285Z -           StorageLive(_14);                // bb2[11]: scope 2 at $DIR/combine_array_len.rs:7:11: 7:12
2020-04-16T15:51:39.7064028Z -           _14 = _6;                        // bb2[12]: scope 2 at $DIR/combine_array_len.rs:7:11: 7:12
2020-04-16T15:51:39.7064794Z -           StorageLive(_15);                // bb2[13]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7065540Z -           _15 = _6;                        // bb2[14]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7066432Z -           _13 = Mul(move _14, move _15);   // bb2[15]: scope 2 at $DIR/combine_array_len.rs:7:11: 7:14
2020-04-16T15:51:39.7067212Z -           StorageDead(_15);                // bb2[16]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7067965Z -           StorageDead(_14);                // bb2[17]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7068703Z -           _0 = Add(move _10, move _13);    // bb2[18]: scope 2 at $DIR/combine_array_len.rs:7:5: 7:14
2020-04-16T15:51:39.7069459Z -           StorageDead(_13);                // bb2[19]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7070211Z -           StorageDead(_10);                // bb2[20]: scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
2020-04-16T15:51:39.7071093Z -           StorageDead(_6);                 // bb2[21]: scope 1 at $DIR/combine_array_len.rs:8:1: 8:2
2020-04-16T15:51:39.7071843Z -           StorageDead(_2);                 // bb2[22]: scope 0 at $DIR/combine_array_len.rs:8:1: 8:2
2020-04-16T15:51:39.7072577Z -           return;                          // bb2[23]: scope 0 at $DIR/combine_array_len.rs:8:2: 8:2
2020-04-16T15:51:39.7072940Z +           _6 = _1[_7];
2020-04-16T15:51:39.7073130Z +           StorageDead(_7);
2020-04-16T15:51:39.7073327Z +           StorageLive(_10);
2020-04-16T15:51:39.7073540Z +           StorageLive(_11);
2020-04-16T15:51:39.7073725Z +           _11 = _2;
2020-04-16T15:51:39.7073911Z +           StorageLive(_12);
2020-04-16T15:51:39.7074112Z +           _12 = _2;
2020-04-16T15:51:39.7074320Z +           _10 = Mul(move _11, move _12);
2020-04-16T15:51:39.7074537Z +           StorageDead(_12);
2020-04-16T15:51:39.7074736Z +           StorageDead(_11);
2020-04-16T15:51:39.7074948Z +           StorageLive(_13);
2020-04-16T15:51:39.7075144Z +           StorageLive(_14);
2020-04-16T15:51:39.7075326Z +           _14 = _6;
2020-04-16T15:51:39.7075527Z +           StorageLive(_15);
2020-04-16T15:51:39.7075721Z +           _15 = _6;
2020-04-16T15:51:39.7075924Z +           _13 = Mul(move _14, move _15);
2020-04-16T15:51:39.7076154Z +           StorageDead(_15);
2020-04-16T15:51:39.7076350Z +           StorageDead(_14);
2020-04-16T15:51:39.7076565Z +           _0 = Add(move _10, move _13);
2020-04-16T15:51:39.7076795Z +           StorageDead(_13);
2020-04-16T15:51:39.7076991Z +           StorageDead(_10);
2020-04-16T15:51:39.7077187Z +           StorageDead(_6);
2020-04-16T15:51:39.7077397Z +           StorageDead(_2);
2020-04-16T15:51:39.7077733Z 99       }
2020-04-16T15:51:39.7077870Z 100   }
2020-04-16T15:51:39.7078016Z 101   
2020-04-16T15:51:39.7078117Z 
2020-04-16T15:51:39.7078117Z 
2020-04-16T15:51:39.7078956Z thread '[mir-opt] mir-opt/combine_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/combine_array_len/32bit/rustc.norm2.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.7079943Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-04-16T15:51:39.7080193Z 1 // MIR for `main` after ConstProp
2020-04-16T15:51:39.7080378Z 2 
2020-04-16T15:51:39.7080676Z 3 fn main() -> () {
2020-04-16T15:51:39.7080676Z 3 fn main() -> () {
2020-04-16T15:51:39.7081243Z -     let mut _0: ();                      // return place in scope 0 at $DIR/const_allocation.rs:7:11: 7:11
2020-04-16T15:51:39.7081975Z -     let _1: &[(std::option::Option<i32>, &[&str])]; // in scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7082707Z -     let mut _2: &&[(std::option::Option<i32>, &[&str])]; // in scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7083057Z +     let mut _0: ();
2020-04-16T15:51:39.7083323Z +     let _1: &[(std::option::Option<i32>, &[&str])];
2020-04-16T15:51:39.7083639Z +     let mut _2: &&[(std::option::Option<i32>, &[&str])];
2020-04-16T15:51:39.7084011Z 8     bb0: {
2020-04-16T15:51:39.7084011Z 8     bb0: {
2020-04-16T15:51:39.7084579Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7085303Z -         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7086151Z -         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&str])]}; // bb0[2]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7086558Z +         StorageLive(_1);
2020-04-16T15:51:39.7086742Z +         StorageLive(_2);
2020-04-16T15:51:39.7087046Z +         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&str])]};
2020-04-16T15:51:39.7087386Z 12                                          // ty::Const
2020-04-16T15:51:39.7087766Z 13                                          // + ty: &&[(std::option::Option<i32>, &[&str])]
2020-04-16T15:51:39.7088207Z 14                                          // + val: Value(Scalar(alloc0+0))
2020-04-16T15:51:39.7088629Z 15                                          // mir::Constant
2020-04-16T15:51:39.7088629Z 15                                          // mir::Constant
2020-04-16T15:51:39.7089024Z 16                                          // + span: $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7089624Z 17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&str])], val: Value(Scalar(alloc0+0)) }
2020-04-16T15:51:39.7090481Z -         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-16T15:51:39.7091212Z -         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-16T15:51:39.7091936Z -         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-16T15:51:39.7092676Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation.rs:7:11: 9:2
2020-04-16T15:51:39.7093011Z +         _1 = (*_2);
2020-04-16T15:51:39.7093190Z +         StorageDead(_2);
2020-04-16T15:51:39.7093395Z +         StorageDead(_1);
2020-04-16T15:51:39.7093580Z +         _0 = const ();
2020-04-16T15:51:39.7093991Z 22                                          // ty::Const
2020-04-16T15:51:39.7094274Z 23                                          // + ty: ()
2020-04-16T15:51:39.7094606Z 24                                          // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.7095019Z 25                                          // mir::Constant
2020-04-16T15:51:39.7095019Z 25                                          // mir::Constant
2020-04-16T15:51:39.7095408Z 26                                          // + span: $DIR/const_allocation.rs:7:11: 9:2
2020-04-16T15:51:39.7095880Z 27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T15:51:39.7096619Z -         return;                          // bb0[7]: scope 0 at $DIR/const_allocation.rs:9:2: 9:2
2020-04-16T15:51:39.7097107Z 29     }
2020-04-16T15:51:39.7097231Z 30 }
2020-04-16T15:51:39.7097368Z 31 
2020-04-16T15:51:39.7097463Z 
2020-04-16T15:51:39.7097463Z 
2020-04-16T15:51:39.7098481Z thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.7099415Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-16T15:51:39.7099666Z 1 // MIR for `main` after ConstProp
2020-04-16T15:51:39.7099850Z 2 
2020-04-16T15:51:39.7100148Z 3 fn main() -> () {
2020-04-16T15:51:39.7100148Z 3 fn main() -> () {
2020-04-16T15:51:39.7100843Z -     let mut _0: ();                      // return place in scope 0 at $DIR/const_allocation2.rs:4:11: 4:11
2020-04-16T15:51:39.7101602Z -     let _1: &[(std::option::Option<i32>, &[&u8])]; // in scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7102337Z -     let mut _2: &&[(std::option::Option<i32>, &[&u8])]; // in scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7102685Z +     let mut _0: ();
2020-04-16T15:51:39.7102951Z +     let _1: &[(std::option::Option<i32>, &[&u8])];
2020-04-16T15:51:39.7103267Z +     let mut _2: &&[(std::option::Option<i32>, &[&u8])];
2020-04-16T15:51:39.7103644Z 8     bb0: {
2020-04-16T15:51:39.7103644Z 8     bb0: {
2020-04-16T15:51:39.7104207Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7104931Z -         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7105774Z -         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&u8])]}; // bb0[2]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7106183Z +         StorageLive(_1);
2020-04-16T15:51:39.7106369Z +         StorageLive(_2);
2020-04-16T15:51:39.7106670Z +         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&u8])]};
2020-04-16T15:51:39.7107008Z 12                                          // ty::Const
2020-04-16T15:51:39.7107380Z 13                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]
2020-04-16T15:51:39.7107823Z 14                                          // + val: Value(Scalar(alloc0+0))
2020-04-16T15:51:39.7108246Z 15                                          // mir::Constant
2020-04-16T15:51:39.7108246Z 15                                          // mir::Constant
2020-04-16T15:51:39.7108633Z 16                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7109218Z 17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc0+0)) }
2020-04-16T15:51:39.7110072Z -         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-16T15:51:39.7110990Z -         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-16T15:51:39.7111718Z -         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-16T15:51:39.7112442Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
2020-04-16T15:51:39.7112801Z +         _1 = (*_2);
2020-04-16T15:51:39.7112985Z +         StorageDead(_2);
2020-04-16T15:51:39.7113180Z +         StorageDead(_1);
2020-04-16T15:51:39.7113383Z +         _0 = const ();
2020-04-16T15:51:39.7113623Z 22                                          // ty::Const
2020-04-16T15:51:39.7113901Z 23                                          // + ty: ()
2020-04-16T15:51:39.7114235Z 24                                          // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.7114648Z 25                                          // mir::Constant
2020-04-16T15:51:39.7114648Z 25                                          // mir::Constant
2020-04-16T15:51:39.7115035Z 26                                          // + span: $DIR/const_allocation2.rs:4:11: 6:2
2020-04-16T15:51:39.7115514Z 27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T15:51:39.7116247Z -         return;                          // bb0[7]: scope 0 at $DIR/const_allocation2.rs:6:2: 6:2
2020-04-16T15:51:39.7116736Z 29     }
2020-04-16T15:51:39.7116939Z 30 }
2020-04-16T15:51:39.7117123Z 31 
2020-04-16T15:51:39.7117220Z 
2020-04-16T15:51:39.7117220Z 
2020-04-16T15:51:39.7118066Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.7118923Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-16T15:51:39.7119173Z 1 // MIR for `main` after ConstProp
2020-04-16T15:51:39.7119341Z 2 
2020-04-16T15:51:39.7119653Z 3 fn main() -> () {
2020-04-16T15:51:39.7119653Z 3 fn main() -> () {
2020-04-16T15:51:39.7120218Z -     let mut _0: ();                      // return place in scope 0 at $DIR/const_allocation3.rs:4:11: 4:11
2020-04-16T15:51:39.7120899Z -     let _1: &Packed;                     // in scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7121576Z -     let mut _2: &&Packed;                // in scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7121894Z +     let mut _0: ();
2020-04-16T15:51:39.7122091Z +     let _1: &Packed;
2020-04-16T15:51:39.7122307Z +     let mut _2: &&Packed;
2020-04-16T15:51:39.7122606Z 8     bb0: {
2020-04-16T15:51:39.7122606Z 8     bb0: {
2020-04-16T15:51:39.7123188Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7123914Z -         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7124802Z -         _2 = const {alloc0+0: &&Packed}; // bb0[2]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7125180Z +         StorageLive(_1);
2020-04-16T15:51:39.7125369Z +         StorageLive(_2);
2020-04-16T15:51:39.7125597Z +         _2 = const {alloc0+0: &&Packed};
2020-04-16T15:51:39.7125895Z 12                                          // ty::Const
2020-04-16T15:51:39.7126191Z 13                                          // + ty: &&Packed
2020-04-16T15:51:39.7126529Z 14                                          // + val: Value(Scalar(alloc0+0))
2020-04-16T15:51:39.7126975Z 15                                          // mir::Constant
2020-04-16T15:51:39.7126975Z 15                                          // mir::Constant
2020-04-16T15:51:39.7127351Z 16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7127865Z 17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc0+0)) }
2020-04-16T15:51:39.7128634Z -         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-16T15:51:39.7129356Z -         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-16T15:51:39.7130100Z -         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-16T15:51:39.7130829Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
2020-04-16T15:51:39.7131164Z +         _1 = (*_2);
2020-04-16T15:51:39.7131367Z +         StorageDead(_2);
2020-04-16T15:51:39.7131560Z +         StorageDead(_1);
2020-04-16T15:51:39.7131745Z +         _0 = const ();
2020-04-16T15:51:39.7131999Z 22                                          // ty::Const
2020-04-16T15:51:39.7132279Z 23                                          // + ty: ()
2020-04-16T15:51:39.7132590Z 24                                          // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.7133026Z 25                                          // mir::Constant
2020-04-16T15:51:39.7133026Z 25                                          // mir::Constant
2020-04-16T15:51:39.7133398Z 26                                          // + span: $DIR/const_allocation3.rs:4:11: 6:2
2020-04-16T15:51:39.7133891Z 27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T15:51:39.7134609Z -         return;                          // bb0[7]: scope 0 at $DIR/const_allocation3.rs:6:2: 6:2
2020-04-16T15:51:39.7135099Z 29     }
2020-04-16T15:51:39.7135227Z 30 }
2020-04-16T15:51:39.7135444Z 31 
2020-04-16T15:51:39.7135543Z 
2020-04-16T15:51:39.7135543Z 
2020-04-16T15:51:39.7136406Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.7137256Z ---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
2020-04-16T15:51:39.7137533Z 2 + // MIR for `main` after ConstProp
2020-04-16T15:51:39.7137703Z 3   
2020-04-16T15:51:39.7138006Z 4   fn main() -> () {
2020-04-16T15:51:39.7138006Z 4   fn main() -> () {
2020-04-16T15:51:39.7138577Z -       let mut _0: ();                      // return place in scope 0 at $DIR/array_index.rs:4:11: 4:11
2020-04-16T15:51:39.7139347Z -       let _1: u32 as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 0 at $DIR/array_index.rs:5:9: 5:10
2020-04-16T15:51:39.7140069Z -       let mut _2: [u32; 4];                // in scope 0 at $DIR/array_index.rs:5:18: 5:30
2020-04-16T15:51:39.7140745Z -       let _3: usize;                       // in scope 0 at $DIR/array_index.rs:5:31: 5:32
2020-04-16T15:51:39.7141394Z -       let mut _4: usize;                   // in scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7142056Z -       let mut _5: bool;                    // in scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7142373Z +       let mut _0: ();
2020-04-16T15:51:39.7142673Z +       let _1: u32 as UserTypeProjection { base: UserType(0), projs: [] };
2020-04-16T15:51:39.7142981Z +       let mut _2: [u32; 4];
2020-04-16T15:51:39.7143400Z +       let mut _4: usize;
2020-04-16T15:51:39.7143606Z +       let mut _5: bool;
2020-04-16T15:51:39.7143807Z 11       scope 1 {
2020-04-16T15:51:39.7144315Z -           debug x => _1;                   // in scope 1 at $DIR/array_index.rs:5:9: 5:10
2020-04-16T15:51:39.7144315Z -           debug x => _1;                   // in scope 1 at $DIR/array_index.rs:5:9: 5:10
2020-04-16T15:51:39.7144611Z +           debug x => _1;
2020-04-16T15:51:39.7144791Z 13       }
2020-04-16T15:51:39.7144924Z 14   
2020-04-16T15:51:39.7145079Z 15       bb0: {
2020-04-16T15:51:39.7145200Z 
2020-04-16T15:51:39.7145742Z -           StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/array_index.rs:5:9: 5:10
2020-04-16T15:51:39.7146443Z -           StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/array_index.rs:5:18: 5:30
2020-04-16T15:51:39.7147222Z -           _2 = [const 0u32, const 1u32, const 2u32, const 3u32]; // bb0[2]: scope 0 at $DIR/array_index.rs:5:18: 5:30
2020-04-16T15:51:39.7147586Z +           StorageLive(_1);
2020-04-16T15:51:39.7147781Z +           StorageLive(_2);
2020-04-16T15:51:39.7148046Z +           _2 = [const 0u32, const 1u32, const 2u32, const 3u32];
2020-04-16T15:51:39.7148350Z 19                                            // ty::Const
2020-04-16T15:51:39.7148636Z 20                                            // + ty: u32
2020-04-16T15:51:39.7148984Z 21                                            // + val: Value(Scalar(0x00000000))
2020-04-16T15:51:39.7149427Z 40                                            // mir::Constant
2020-04-16T15:51:39.7149427Z 40                                            // mir::Constant
2020-04-16T15:51:39.7149791Z 41                                            // + span: $DIR/array_index.rs:5:28: 5:29
2020-04-16T15:51:39.7150324Z 42                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000003)) }
2020-04-16T15:51:39.7151237Z -           StorageLive(_3);                 // bb0[3]: scope 0 at $DIR/array_index.rs:5:31: 5:32
2020-04-16T15:51:39.7151951Z -           _3 = const 2usize;               // bb0[4]: scope 0 at $DIR/array_index.rs:5:31: 5:32
2020-04-16T15:51:39.7152310Z +           StorageLive(_3);
2020-04-16T15:51:39.7152761Z 45                                            // ty::Const
2020-04-16T15:51:39.7153063Z 46                                            // + ty: usize
2020-04-16T15:51:39.7153063Z 46                                            // + ty: usize
2020-04-16T15:51:39.7153397Z 47                                            // + val: Value(Scalar(0x00000002))
2020-04-16T15:51:39.7153974Z 48                                            // mir::Constant
2020-04-16T15:51:39.7153974Z 48                                            // mir::Constant
2020-04-16T15:51:39.7154343Z 49                                            // + span: $DIR/array_index.rs:5:31: 5:32
2020-04-16T15:51:39.7154821Z 50                                            // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-16T15:51:39.7155589Z -           _4 = const 4usize;               // bb0[5]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7156160Z 52                                            // ty::Const
2020-04-16T15:51:39.7156461Z 53                                            // + ty: usize
2020-04-16T15:51:39.7156461Z 53                                            // + ty: usize
2020-04-16T15:51:39.7156794Z 54                                            // + val: Value(Scalar(0x00000004))
2020-04-16T15:51:39.7157240Z 55                                            // mir::Constant
2020-04-16T15:51:39.7157240Z 55                                            // mir::Constant
2020-04-16T15:51:39.7157615Z 56                                            // + span: $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7158109Z 57                                            // + literal: Const { ty: usize, val: Value(Scalar(0x00000004)) }
2020-04-16T15:51:39.7158864Z - -         _5 = Lt(_3, _4);                 // bb0[6]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7159745Z - -         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // bb0[7]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7160565Z - +         _5 = const true;                 // bb0[6]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7161088Z + -         _5 = Lt(_3, _4);
2020-04-16T15:51:39.7161661Z + -         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1;
2020-04-16T15:51:39.7161999Z + +         _5 = const true;
2020-04-16T15:51:39.7162265Z 61 +                                          // ty::Const
2020-04-16T15:51:39.7162552Z 62 +                                          // + ty: bool
2020-04-16T15:51:39.7162877Z 63 +                                          // + val: Value(Scalar(0x01))
2020-04-16T15:51:39.7163313Z 64 +                                          // mir::Constant
2020-04-16T15:51:39.7163313Z 64 +                                          // mir::Constant
2020-04-16T15:51:39.7163680Z 65 +                                          // + span: $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7164172Z 66 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
2020-04-16T15:51:39.7165078Z - +         assert(const true, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // bb0[7]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7165907Z + +         assert(const true, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1;
2020-04-16T15:51:39.7166311Z 68 +                                          // ty::Const
2020-04-16T15:51:39.7166594Z 69 +                                          // + ty: bool
2020-04-16T15:51:39.7166924Z 70 +                                          // + val: Value(Scalar(0x01))
2020-04-16T15:51:39.7167271Z 74       }
2020-04-16T15:51:39.7167404Z 75   
2020-04-16T15:51:39.7167569Z 76       bb1: {
2020-04-16T15:51:39.7167569Z 76       bb1: {
2020-04-16T15:51:39.7168302Z - -         _1 = _2[_3];                     // bb1[0]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7169016Z - +         _1 = const 2u32;                 // bb1[0]: scope 0 at $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7169520Z + -         _1 = _2[_3];
2020-04-16T15:51:39.7169709Z + +         _1 = const 2u32;
2020-04-16T15:51:39.7169957Z 79 +                                          // ty::Const
2020-04-16T15:51:39.7170259Z 80 +                                          // + ty: u32
2020-04-16T15:51:39.7170588Z 81 +                                          // + val: Value(Scalar(0x00000002))
2020-04-16T15:51:39.7171019Z 82 +                                          // mir::Constant
2020-04-16T15:51:39.7171019Z 82 +                                          // mir::Constant
2020-04-16T15:51:39.7171502Z 83 +                                          // + span: $DIR/array_index.rs:5:18: 5:33
2020-04-16T15:51:39.7171987Z 84 +                                          // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
2020-04-16T15:51:39.7172730Z -           StorageDead(_3);                 // bb1[1]: scope 0 at $DIR/array_index.rs:5:33: 5:34
2020-04-16T15:51:39.7173455Z -           StorageDead(_2);                 // bb1[2]: scope 0 at $DIR/array_index.rs:5:33: 5:34
2020-04-16T15:51:39.7174161Z -           _0 = const ();                   // bb1[3]: scope 0 at $DIR/array_index.rs:4:11: 6:2
2020-04-16T15:51:39.7174505Z +           StorageDead(_3);
2020-04-16T15:51:39.7174825Z +           StorageDead(_2);
2020-04-16T15:51:39.7175016Z +           _0 = const ();
2020-04-16T15:51:39.7175277Z 88                                            // ty::Const
2020-04-16T15:51:39.7175561Z 89                                            // + ty: ()
2020-04-16T15:51:39.7175881Z 90                                            // + val: Value(Scalar(<ZST>))
2020-04-16T15:51:39.7176316Z 91                                            // mir::Constant
2020-04-16T15:51:39.7176316Z 91                                            // mir::Constant
2020-04-16T15:51:39.7176676Z 92                                            // + span: $DIR/array_index.rs:4:11: 6:2
2020-04-16T15:51:39.7177137Z 93                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T15:51:39.7177892Z -           StorageDead(_1);                 // bb1[4]: scope 0 at $DIR/array_index.rs:6:1: 6:2
2020-04-16T15:51:39.7178593Z -           return;                          // bb1[5]: scope 0 at $DIR/array_index.rs:6:2: 6:2
2020-04-16T15:51:39.7178927Z +           StorageDead(_1);
2020-04-16T15:51:39.7179273Z 96       }
2020-04-16T15:51:39.7179408Z 97   }
2020-04-16T15:51:39.7179618Z 98   
2020-04-16T15:51:39.7179717Z 
2020-04-16T15:51:39.7179717Z 
2020-04-16T15:51:39.7180576Z thread '[mir-opt] mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T15:51:39.7181458Z ---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
2020-04-16T15:51:39.7181720Z 2 + // MIR for `main` after ConstProp
2020-04-16T15:51:39.7181892Z 3   
2020-04-16T15:51:39.7182212Z 4   fn main() -> () {
2020-04-16T15:51:39.7182212Z 4   fn main() -> () {
2020-04-16T15:51:39.7182773Z -       let mut _0: ();                      // return place in scope 0 at $DIR/discriminant.rs:5:11: 5:11
2020-04-16T15:51:39.7183447Z -       let _1: i32;                         // in scope 0 at $DIR/discriminant.rs:6:9: 6:10
2020-04-16T15:51:39.7184115Z -       let mut _2: i32;                     // in scope 0 at $DIR/discriminant.rs:6:13: 6:64
2020-04-16T15:51:39.7184783Z -       let mut _3: std::option::Option<bool>; // in scope 0 at $DIR/discriminant.rs:6:34: 6:44
2020-04-16T15:51:39.7185472Z -       let mut _4: isize;                   // in scope 0 at $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7185793Z +       let mut _0: ();
2020-04-16T15:51:39.7185986Z +       let _1: i32;
2020-04-16T15:51:39.7186181Z +       let mut _2: i32;
2020-04-16T15:51:39.7186690Z +       let mut _4: isize;
2020-04-16T15:51:39.7186875Z 10       scope 1 {
2020-04-16T15:51:39.7187405Z -           debug x => _1;                   // in scope 1 at $DIR/discriminant.rs:6:9: 6:10
2020-04-16T15:51:39.7187705Z +           debug x => _1;
2020-04-16T15:51:39.7187705Z +           debug x => _1;
2020-04-16T15:51:39.7187869Z 12       }
2020-04-16T15:51:39.7188017Z 13   
2020-04-16T15:51:39.7188164Z 14       bb0: {
2020-04-16T15:51:39.7188286Z 
2020-04-16T15:51:39.7188814Z -           StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/discriminant.rs:6:9: 6:10
2020-04-16T15:51:39.7189548Z -           StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/discriminant.rs:6:13: 6:64
2020-04-16T15:51:39.7190316Z -           StorageLive(_3);                 // bb0[2]: scope 0 at $DIR/discriminant.rs:6:34: 6:44
2020-04-16T15:51:39.7192096Z - -         _3 = std::option::Option::<bool>::Some(const true,); // bb0[3]: scope 0 at $DIR/discriminant.rs:6:34: 6:44
2020-04-16T15:51:39.7193249Z - +         _3 = const {transmute(0x01): std::option::Option<bool>}; // bb0[3]: scope 0 at $DIR/discriminant.rs:6:34: 6:44
2020-04-16T15:51:39.7193648Z +           StorageLive(_1);
2020-04-16T15:51:39.7193857Z +           StorageLive(_2);
2020-04-16T15:51:39.7194052Z +           StorageLive(_3);
2020-04-16T15:51:39.7194524Z + -         _3 = std::option::Option::<bool>::Some(const true,);
2020-04-16T15:51:39.7194912Z + +         _3 = const {transmute(0x01): std::option::Option<bool>};
2020-04-16T15:51:39.7195247Z 20                                            // ty::Const
2020-04-16T15:51:39.7195724Z 21 -                                          // + ty: bool
2020-04-16T15:51:39.7196094Z 22 +                                          // + ty: std::option::Option<bool>
2020-04-16T15:51:39.7196557Z 24                                            // mir::Constant
2020-04-16T15:51:39.7196557Z 24                                            // mir::Constant
2020-04-16T15:51:39.7197144Z 25 -                                          // + span: $DIR/discriminant.rs:6:39: 6:43
2020-04-16T15:51:39.7197873Z 26 -                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
2020-04-16T15:51:39.7198603Z - -         _4 = discriminant(_3);           // bb0[4]: scope 0 at $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7199421Z - -         switchInt(move _4) -> [1isize: bb2, otherwise: bb1]; // bb0[5]: scope 0 at $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7199995Z + -         _4 = discriminant(_3);
2020-04-16T15:51:39.7200472Z + -         switchInt(move _4) -> [1isize: bb2, otherwise: bb1];
2020-04-16T15:51:39.7200897Z 29 +                                          // + span: $DIR/discriminant.rs:6:34: 6:44
2020-04-16T15:51:39.7201445Z 30 +                                          // + literal: Const { ty: std::option::Option<bool>, val: Value(Scalar(0x01)) }
2020-04-16T15:51:39.7202231Z - +         _4 = const 1isize;               // bb0[4]: scope 0 at $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7202589Z + +         _4 = const 1isize;
2020-04-16T15:51:39.7202839Z 32 +                                          // ty::Const
2020-04-16T15:51:39.7203126Z 33 +                                          // + ty: isize
2020-04-16T15:51:39.7203483Z 34 +                                          // + val: Value(Scalar(0x00000001))
2020-04-16T15:51:39.7203915Z 35 +                                          // mir::Constant
2020-04-16T15:51:39.7203915Z 35 +                                          // mir::Constant
2020-04-16T15:51:39.7204295Z 36 +                                          // + span: $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7204781Z 37 +                                          // + literal: Const { ty: isize, val: Value(Scalar(0x00000001)) }
2020-04-16T15:51:39.7205627Z - +         switchInt(const 1isize) -> [1isize: bb2, otherwise: bb1]; // bb0[5]: scope 0 at $DIR/discriminant.rs:6:21: 6:31
2020-04-16T15:51:39.7206344Z + +         switchInt(const 1isize) -> [1isize: bb2, otherwise: bb1];
2020-04-16T15:51:39.7206686Z 39 +                                          // ty::Const
2020-04-16T15:51:39.7206972Z 40 +                                          // + ty: isize
2020-04-16T15:51:39.7207322Z 41 +                                          // + val: Value(Scalar(0x00000001))
2020-04-16T15:51:39.7207667Z 45       }
2020-04-16T15:51:39.7207800Z 46   
2020-04-16T15:51:39.7207968Z 47       bb1: {
2020-04-16T15:51:39.7207968Z 47       bb1: {
2020-04-16T15:51:39.7208537Z -           _2 = const 10i32;                // bb1[0]: scope 0 at $DIR/discriminant.rs:6:59: 6:61
2020-04-16T15:51:39.7208871Z +           _2 = const 10i32;
2020-04-16T15:51:39.7209132Z 49                                            // ty::Const
2020-04-16T15:51:39.7209413Z 50                                            // + ty: i32
2020-04-16T15:51:39.7209846Z 51                                            // + val: Value(Scalar(0x0000000a))
2020-04-16T15:51:39.7210349Z 52                                            // mir::Constant
2020-04-16T15:51:39.7210349Z 52                                            // mir::Constant
2020-04-16T15:51:39.7210716Z 53                                            // + span: $DIR/discriminant.rs:6:59: 6:61
2020-04-16T15:51:39.7211220Z 54                                            // + literal: Const { ty: i32, val: Value(Scalar(0x0000000a)) }
2020-04-16T15:51:39.7211976Z -           goto -> bb4;                     // bb1[1]: scope 0 at $DIR/discriminant.rs:6:13: 6:64
2020-04-16T15:51:39.7212468Z +           goto -> bb4;
2020-04-16T15:51:39.7212780Z 57   
2020-04-16T15:51:39.7212929Z 58       bb2: {
2020-04-16T15:51:39.7213050Z 
2020-04-16T15:51:39.7213050Z 
2020-04-16T15:51:39.7213723Z - -         switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // bb2[0]: scope 0 at $DIR/discriminant.rs:6:26: 6:30
2020-04-16T15:51:39.7214627Z - +         switchInt(const true) -> [false: bb1, otherwise: bb3]; // bb2[0]: scope 0 at $DIR/discriminant.rs:6:26: 6:30
2020-04-16T15:51:39.7215352Z + -         switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3];
2020-04-16T15:51:39.7215956Z + +         switchInt(const true) -> [false: bb1, otherwise: bb3];
2020-04-16T15:51:39.7216296Z 61 +                                          // ty::Const
2020-04-16T15:51:39.7216598Z 62 +                                          // + ty: bool
2020-04-16T15:51:39.7216919Z 63 +                                          // + val: Value(Scalar(0x01))
2020-04-16T15:51:39.7217258Z 67       }
2020-04-16T15:51:39.7217400Z 68   
2020-04-16T15:51:39.7217544Z 69       bb3: {
2020-04-16T15:51:39.7217544Z 69       bb3: {
2020-04-16T15:51:39.7218111Z -           _2 = const 42i32;                // bb3[0]: scope 0 at $DIR/discriminant.rs:6:47: 6:49
2020-04-16T15:51:39.7218476Z +           _2 = const 42i32;
2020-04-16T15:51:39.7218720Z 71                                            // ty::Const
2020-04-16T15:51:39.7219011Z 72                                            // + ty: i32
2020-04-16T15:51:39.7219367Z 73                                            // + val: Value(Scalar(0x0000002a))
2020-04-16T15:51:39.7219800Z 74                                            // mir::Constant
2020-04-16T15:51:39.7219800Z 74                                            // mir::Constant
2020-04-16T15:51:39.7220167Z 75                                            // + span: $DIR/discriminant.rs:6:47: 6:49
2020-04-16T15:51:39.7220662Z 76                                            // + literal: Const { ty: i32, val: Value(Scalar(0x0000002a)) }
2020-04-16T15:51:39.7221401Z -           goto -> bb4;                     // bb3[1]: scope 0 at $DIR/discriminant.rs:6:13: 6:64
2020-04-16T15:51:39.7221915Z +           goto -> bb4;
2020-04-16T15:51:39.7222214Z 79   
2020-04-16T15:51:39.7222362Z 80       bb4: {
2020-04-16T15:51:39.7222500Z 
2020-04-16T15:51:39.7222500Z 
2020-04-16T15:51:39.7223037Z -           _1 = Add(move _2, const 0i32);   // bb4[0]: scope 0 at $DIR/discriminant.rs:6:13: 6:68
2020-04-16T15:51:39.7223404Z +           _1 = Add(move _2, const 0i32);
2020-04-16T15:51:39.7223679Z 82                                            // ty::Const
---
2020-04-16T15:51:39.7654325Z test result: FAILED. 73 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T15:51:39.7654575Z 
2020-04-16T15:51:39.7654665Z 
2020-04-16T15:51:39.7654754Z 
2020-04-16T15:51:39.7658428Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-16T15:51:39.7661101Z 
2020-04-16T15:51:39.7661207Z 
2020-04-16T15:51:39.7661207Z 
2020-04-16T15:51:39.7661840Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T15:51:39.7662802Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-16T15:51:39.7663106Z == clock drift check ==
2020-04-16T15:51:39.7663106Z == clock drift check ==
2020-04-16T15:51:39.7663331Z   local time: Thu Apr 16 15:51:39 UTC 2020
2020-04-16T15:51:39.8952402Z   network time: Thu, 16 Apr 2020 15:51:39 GMT
2020-04-16T15:51:41.8569113Z 
2020-04-16T15:51:41.8569113Z 
2020-04-16T15:51:41.8979397Z ##[error]Bash exited with code '1'.
2020-04-16T15:51:41.8994448Z ##[section]Finishing: Run build
2020-04-16T15:51:41.9044274Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71200/merge to s
2020-04-16T15:51:41.9049192Z Task         : Get sources
2020-04-16T15:51:41.9049516Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T15:51:41.9049782Z Version      : 1.0.0
2020-04-16T15:51:41.9049978Z Author       : Microsoft
2020-04-16T15:51:41.9049978Z Author       : Microsoft
2020-04-16T15:51:41.9050296Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T15:51:41.9050634Z ==============================================================================
2020-04-16T15:51:42.2733700Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T15:51:42.2781826Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71200/merge to s
2020-04-16T15:51:42.2868814Z Cleaning up task key
2020-04-16T15:51:42.2869987Z Start cleaning up orphan processes.
2020-04-16T15:51:42.3043189Z Terminate orphan process: pid (3347) (python)
2020-04-16T15:51:42.3415030Z ##[section]Finishing: Finalize Job
