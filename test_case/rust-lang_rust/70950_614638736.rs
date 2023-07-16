plain
2020-04-16T11:04:59.5094139Z ========================== Starting Command Output ===========================
2020-04-16T11:04:59.5099613Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13cfddc9-63fa-46f0-965a-1cc93ec58b00.sh
2020-04-16T11:04:59.5100085Z 
2020-04-16T11:04:59.5105202Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T11:04:59.5141094Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-16T11:04:59.5144327Z Task         : Get sources
2020-04-16T11:04:59.5144593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T11:04:59.5144848Z Version      : 1.0.0
2020-04-16T11:04:59.5145041Z Author       : Microsoft
---
2020-04-16T11:05:01.1647371Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T11:05:01.2174095Z ##[command]git config gc.auto 0
2020-04-16T11:05:01.2214273Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T11:05:01.2220201Z ##[command]git config --get-all http.proxy
2020-04-16T11:05:01.2228431Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70950/merge:refs/remotes/pull/70950/merge
---
2020-04-16T11:07:07.0432207Z  ---> f58a2bb1e753
2020-04-16T11:07:07.0433272Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-16T11:07:07.0436677Z  ---> Using cache
2020-04-16T11:07:07.0437099Z  ---> d079cc6b6db8
2020-04-16T11:07:07.0438128Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-16T11:07:07.0444004Z  ---> 4183ca46ee56
2020-04-16T11:07:07.0444234Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-16T11:07:07.0447718Z  ---> Using cache
2020-04-16T11:07:07.0448152Z  ---> 69e7f8a2a2fb
---
2020-04-16T11:07:07.0977816Z Looks like docker image is the same as before, not uploading
2020-04-16T11:07:15.1835588Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T11:07:15.2139882Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T11:07:15.2176408Z == clock drift check ==
2020-04-16T11:07:15.2187644Z   local time: Thu Apr 16 11:07:15 UTC 2020
2020-04-16T11:07:15.5160874Z   network time: Thu, 16 Apr 2020 11:07:15 GMT
2020-04-16T11:07:15.5188878Z Starting sccache server...
2020-04-16T11:07:15.6130829Z configure: processing command line
2020-04-16T11:07:15.6131074Z configure: 
2020-04-16T11:07:15.6132206Z configure: rust.dist-src        := False
---
2020-04-16T11:12:44.9024422Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T11:12:46.8208547Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T11:12:48.1632003Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T11:12:50.1981973Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T11:12:58.8325451Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T11:13:02.5406944Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T11:13:07.3449910Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T11:13:11.8672412Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T11:13:21.5737732Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T11:36:58.8764786Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T11:37:00.7168139Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T11:37:02.7774865Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T11:37:05.8352160Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T11:37:15.4381774Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T11:37:20.9829644Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T11:37:26.4093758Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T11:37:32.0780812Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T11:37:41.6719610Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T12:04:40.3499216Z .................................................................................................... 1700/9899
2020-04-16T12:04:45.0642426Z .................................................................................................... 1800/9899
2020-04-16T12:04:54.2491076Z .................................................................................................... 1900/9899
2020-04-16T12:05:03.0699904Z ......i............................................................................................. 2000/9899
2020-04-16T12:05:09.9136289Z ................................................................................................iiii 2100/9899
2020-04-16T12:05:25.5810836Z i................................................................................................... 2200/9899
2020-04-16T12:05:34.3000300Z .................................................................................................... 2400/9899
2020-04-16T12:05:36.7161521Z .................................................................................................... 2500/9899
2020-04-16T12:05:42.7067685Z .................................................................................................... 2600/9899
2020-04-16T12:06:02.7274867Z .................................................................................................... 2700/9899
---
2020-04-16T12:08:47.2350717Z .................................................................................................... 5100/9899
2020-04-16T12:08:55.4260289Z .................................................................................................... 5200/9899
2020-04-16T12:09:00.1636811Z .................i.................................................................................. 5300/9899
2020-04-16T12:09:11.2696497Z .......i............................................................................................ 5400/9899
2020-04-16T12:09:16.8125643Z .......ii.ii........i...i........................................................................... 5500/9899
2020-04-16T12:09:24.5192615Z .....................................................i.............................................. 5700/9899
2020-04-16T12:09:34.4819356Z .........................................................................ii......................... 5800/9899
2020-04-16T12:09:41.3011578Z ............i....................................................................................... 5900/9899
2020-04-16T12:09:46.7573447Z .................................................................................................... 6000/9899
2020-04-16T12:09:46.7573447Z .................................................................................................... 6000/9899
2020-04-16T12:09:57.6453459Z .................................................................................................... 6100/9899
2020-04-16T12:10:08.9821203Z ......ii...i..ii...........i........................................................................ 6200/9899
2020-04-16T12:10:24.5585852Z .................................................................................................... 6400/9899
2020-04-16T12:10:31.1761459Z .................................................................................................... 6500/9899
2020-04-16T12:10:31.1761459Z .................................................................................................... 6500/9899
2020-04-16T12:10:47.0082716Z ......................................i..ii......................................................... 6600/9899
2020-04-16T12:11:09.3074395Z .................................................................................................... 6800/9899
2020-04-16T12:11:11.3705417Z ......................................i............................................................. 6900/9899
2020-04-16T12:11:13.4670505Z .................................................................................................... 7000/9899
2020-04-16T12:11:15.6197334Z ..............................................................................i..................... 7100/9899
---
2020-04-16T12:12:57.8755942Z .................................................................................................... 7800/9899
2020-04-16T12:13:02.3074341Z .................................................................................................... 7900/9899
2020-04-16T12:13:09.1423702Z .................................................................................................... 8000/9899
2020-04-16T12:13:15.4795720Z ............................................i....................................................... 8100/9899
2020-04-16T12:13:25.6923120Z ............................................................................................iiiiii.i 8200/9899
2020-04-16T12:13:31.7722477Z iiii.i.............................................................................................. 8300/9899
2020-04-16T12:13:45.1554748Z .................................................................................................... 8500/9899
2020-04-16T12:13:54.5567974Z .................................................................................................... 8600/9899
2020-04-16T12:14:08.3662503Z .................................................................................................... 8700/9899
2020-04-16T12:14:15.3391454Z .................................................................................................... 8800/9899
---
2020-04-16T12:16:41.3003039Z st/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-16T12:16:41.3184644Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:16:41.5156511Z 
2020-04-16T12:16:41.5156763Z running 185 tests
2020-04-16T12:16:44.3097509Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-16T12:16:47.0066786Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-16T12:16:47.0070385Z 
2020-04-16T12:16:47.0075925Z  finished in 5.689
2020-04-16T12:16:47.0082065Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-16T12:16:47.0246753Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T12:16:49.1946779Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-16T12:16:49.2116841Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:16:49.3637663Z 
2020-04-16T12:16:49.3638106Z running 9 tests
2020-04-16T12:16:49.3639469Z iiiiiiiii
2020-04-16T12:16:49.3640481Z 
2020-04-16T12:16:49.3644835Z  finished in 0.152
2020-04-16T12:16:49.3645637Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-16T12:16:49.3805131Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T12:17:10.5770093Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-16T12:17:10.5967745Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:17:10.7729817Z 
2020-04-16T12:17:10.7730041Z running 115 tests
2020-04-16T12:17:24.9049969Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-16T12:17:26.6975339Z ...iiii.....ii.
2020-04-16T12:17:26.6976709Z 
2020-04-16T12:17:26.6981368Z  finished in 16.101
2020-04-16T12:17:26.6994116Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-16T12:17:26.6995199Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T12:31:05.6911306Z 
2020-04-16T12:31:05.6912782Z    Doc-tests core
2020-04-16T12:31:10.8554832Z 
2020-04-16T12:31:10.8555967Z running 2496 tests
2020-04-16T12:31:20.4942302Z ......iiiii......................................................................................... 100/2496
2020-04-16T12:31:29.7412447Z .....................................................................................ii............. 200/2496
2020-04-16T12:31:52.3934074Z ....................i............................................................................... 400/2496
2020-04-16T12:31:52.3934074Z ....................i............................................................................... 400/2496
2020-04-16T12:32:03.0135657Z ..........................................................................i..i..................iiii 500/2496
2020-04-16T12:32:20.3373922Z .................................................................................................... 700/2496
2020-04-16T12:32:29.2034900Z .................................................................................................... 800/2496
2020-04-16T12:32:37.9735645Z .................................................................................................... 900/2496
2020-04-16T12:32:46.6729868Z .................................................................................................... 1000/2496
---
2020-04-16T12:36:19.8005399Z .................................................................................................... 500/764
2020-04-16T12:36:19.8493349Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-16T12:36:19.8517045Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-16T12:36:19.8522725Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-16T12:36:19.8541158Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-16T12:36:20.1018297Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-16T12:36:20.1070404Z ....thread '<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:..21....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-16T12:36:20.1311243Z ............ 600/764
2020-04-16T12:36:22.1724020Z ............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-16T12:36:22.1724857Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-16T12:36:22.1735783Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
---
2020-04-16T12:36:31.2716896Z 
2020-04-16T12:36:31.2717936Z running 1020 tests
2020-04-16T12:36:49.4142593Z i................................................................................................... 100/1020
2020-04-16T12:36:59.9223226Z .................................................................................................... 200/1020
2020-04-16T12:37:07.7294625Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-16T12:37:12.7697498Z .................................................................................................... 400/1020
2020-04-16T12:37:19.5305974Z ....................................................i....i......................................ii.. 500/1020
2020-04-16T12:37:32.6486899Z .................................................................................................... 700/1020
2020-04-16T12:37:32.6486899Z .................................................................................................... 700/1020
2020-04-16T12:37:39.8782082Z ..............................................iiii.................................................. 800/1020
2020-04-16T12:37:54.2428188Z .................................................................................................... 900/1020
2020-04-16T12:38:00.4778453Z ....................................................................iiii............................ 1000/1020
2020-04-16T12:38:01.8782719Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-16T12:38:01.8783173Z 
2020-04-16T12:38:01.8910115Z  finished in 172.924
2020-04-16T12:38:01.8913418Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-16T12:41:31.8966465Z 
2020-04-16T12:41:31.8966798Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T12:41:31.8967155Z 
2020-04-16T12:41:31.9021232Z  finished in 1.053
2020-04-16T12:41:31.9022419Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-16T12:41:31.9034160Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:41:32.0940277Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T12:41:33.2427401Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-f7467bb7b4d568b0
2020-04-16T12:41:33.2456276Z 
2020-04-16T12:41:33.2456722Z running 0 tests
2020-04-16T12:41:33.2457028Z 
---
2020-04-16T12:57:13.6711898Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6712754Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6713597Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6714419Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6715260Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6716908Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6717754Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6718566Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T12:57:13.6719394Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-16T12:58:19.3585396Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-16T12:58:19.3977121Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:58:19.6126188Z 
2020-04-16T12:58:19.6127432Z running 211 tests
2020-04-16T12:58:53.3676176Z ......................i...ii.......................................................................i 100/211
2020-04-16T12:59:34.9936313Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-16T12:59:39.6728690Z .......ii..
2020-04-16T12:59:39.6730867Z 
2020-04-16T12:59:39.6734555Z  finished in 80.276
2020-04-16T12:59:39.6739660Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-16T12:59:39.6744253Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-16T12:59:54.5646659Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-16T12:59:54.5826902Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-16T12:59:54.7516513Z  finished in 4.650
2020-04-16T12:59:54.7525643Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-16T12:59:55.7211183Z Checking "alias-1" ... OK
2020-04-16T12:59:55.7898746Z Checking "alias-2" ... OK
2020-04-16T12:59:55.8516621Z Checking "alias-3" ... OK
2020-04-16T12:59:55.9200085Z Checking "alias" ... OK
2020-04-16T12:59:55.9980258Z Checking "basic" ... OK
2020-04-16T12:59:56.0753891Z Checking "deduplication" ... OK
2020-04-16T12:59:56.1280575Z Checking "enum-option" ... OK
2020-04-16T12:59:56.1984149Z Checking "filter-crate" ... OK
2020-04-16T12:59:56.2618397Z Checking "fn-forget" ... OK
2020-04-16T12:59:56.3673564Z Checking "from_u" ... OK
2020-04-16T12:59:56.4602450Z Checking "keyword" ... OK
2020-04-16T12:59:56.5145229Z Checking "macro-check" ... OK
2020-04-16T12:59:56.5506504Z Checking "macro-print" ... OK
2020-04-16T12:59:56.6303424Z Checking "multi-query" ... OK
2020-04-16T12:59:56.6588110Z Checking "never" ... OK
2020-04-16T12:59:56.6792585Z Checking "quoted" ... OK
2020-04-16T12:59:56.7055985Z Checking "return-specific-literal" ... OK
2020-04-16T12:59:56.7779295Z Checking "return-specific" ... OK
2020-04-16T12:59:56.8143574Z Checking "should-fail" ... OK
2020-04-16T12:59:56.8875639Z Checking "string-from_ut" ... OK
2020-04-16T12:59:56.9401340Z Checking "struct-vec" ... OK
2020-04-16T12:59:57.0612780Z Checking "vec-new" ... OK
2020-04-16T12:59:57.0856404Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T12:59:57.2537366Z 
2020-04-16T12:59:57.2537879Z running 6 tests
2020-04-16T13:00:03.1927450Z ......
---
2020-04-16T13:01:15.6294396Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-16T13:01:15.6502316Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T13:01:15.8294965Z 
2020-04-16T13:01:15.8295217Z running 13 tests
2020-04-16T13:01:16.6367004Z .iiiiiii.iii.
2020-04-16T13:01:16.6368703Z 
2020-04-16T13:01:16.6372011Z  finished in 0.987
2020-04-16T13:01:16.6439263Z Build completed successfully in 1:52:20
2020-04-16T13:01:16.6439263Z Build completed successfully in 1:52:20
2020-04-16T13:01:16.6449458Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T13:01:18.2699546Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T13:01:18.5076065Z     Finished release [optimized] target(s) in 0.23s
2020-04-16T13:01:18.5164228Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-16T13:01:18.5271383Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T13:02:50.0544101Z failures:
2020-04-16T13:02:50.0552599Z 
2020-04-16T13:02:50.0554294Z ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
2020-04-16T13:02:50.0554625Z 7 | Inferred Region Values
2020-04-16T13:02:50.0555364Z 8 | '_#0r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#0r, '_#1r}
2020-04-16T13:02:50.0556461Z 9 | '_#1r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#1r}
2020-04-16T13:02:50.0560060Z - | '_#2r | U0 | {bb2[0..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0560675Z - | '_#3r | U0 | {bb2[1..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0561334Z - | '_#4r | U0 | {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0561816Z + | '_#2r | U0 | {}
2020-04-16T13:02:50.0562260Z + | '_#3r | U0 | {bb2[0..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0562748Z + | '_#4r | U0 | {bb2[1..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0563259Z + | '_#5r | U0 | {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0563635Z 14 | Inference Constraints
2020-04-16T13:02:50.0563635Z 14 | Inference Constraints
2020-04-16T13:02:50.0564306Z 15 | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
2020-04-16T13:02:50.0564613Z 
2020-04-16T13:02:50.0565408Z 16 | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
2020-04-16T13:02:50.0566037Z - | '_#2r live at {bb2[0]}
2020-04-16T13:02:50.0566440Z - | '_#3r live at {bb2[1..=3]}
2020-04-16T13:02:50.0566900Z - | '_#4r live at {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0567416Z - | '_#2r: '_#3r due to Assignment at Single(bb2[0])
2020-04-16T13:02:50.0567919Z - | '_#3r: '_#4r due to Assignment at Single(bb2[3])
2020-04-16T13:02:50.0568346Z + | '_#3r live at {bb2[0]}
2020-04-16T13:02:50.0568760Z + | '_#4r live at {bb2[1..=3]}
2020-04-16T13:02:50.0569220Z + | '_#5r live at {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-16T13:02:50.0569714Z + | '_#3r: '_#4r due to Assignment at Single(bb2[0])
2020-04-16T13:02:50.0570230Z + | '_#4r: '_#5r due to Assignment at Single(bb2[3])
2020-04-16T13:02:50.0570784Z 23 fn main() -> () {
2020-04-16T13:02:50.0571457Z -     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:14:11: 14:11
2020-04-16T13:02:50.0571457Z -     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:14:11: 14:11
2020-04-16T13:02:50.0572424Z -     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-16T13:02:50.0573298Z -     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-16T13:02:50.0574108Z -     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-16T13:02:50.0574902Z -     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-16T13:02:50.0575691Z -     let mut _7: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-16T13:02:50.0576491Z -     let _8: bool;                        // in scope 0 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-16T13:02:50.0577277Z -     let mut _9: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-16T13:02:50.0578068Z -     let _10: bool;                       // in scope 0 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0578918Z +     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11
2020-04-16T13:02:50.0579851Z +     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-16T13:02:50.0580736Z +     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-16T13:02:50.0581525Z +     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-16T13:02:50.0582317Z +     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-16T13:02:50.0583122Z +     let mut _7: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-16T13:02:50.0583911Z +     let _8: bool;                        // in scope 0 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0584849Z +     let mut _9: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-16T13:02:50.0585716Z +     let _10: bool;                       // in scope 0 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-16T13:02:50.0586674Z -         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-16T13:02:50.0586674Z -         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-16T13:02:50.0587458Z -         let _2: &'_#3r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-16T13:02:50.0588235Z +         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-16T13:02:50.0589018Z +         let _2: &'_#4r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-16T13:02:50.0590050Z -             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-16T13:02:50.0590050Z -             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-16T13:02:50.0590877Z -             let _6: &'_#4r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-16T13:02:50.0591647Z +             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-16T13:02:50.0592415Z +             let _6: &'_#5r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-16T13:02:50.0592786Z 39             scope 3 {
2020-04-16T13:02:50.0593392Z -                 debug q => _6;           // in scope 3 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-16T13:02:50.0594142Z +                 debug q => _6;           // in scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-16T13:02:50.0594670Z 42         }
2020-04-16T13:02:50.0594827Z 43     }
2020-04-16T13:02:50.0594945Z 
2020-04-16T13:02:50.0595088Z 44 
2020-04-16T13:02:50.0595088Z 44 
2020-04-16T13:02:50.0595246Z 45     bb0: {
2020-04-16T13:02:50.0595941Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-16T13:02:50.0599998Z -         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:15:17: 15:26
2020-04-16T13:02:50.0601816Z +         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-16T13:02:50.0603262Z +         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:17:17: 17:26
2020-04-16T13:02:50.0604565Z 48                                          // ty::Const
2020-04-16T13:02:50.0604965Z 49                                          // + ty: usize
2020-04-16T13:02:50.0605357Z 50                                          // + val: Value(Scalar(0x00000001))
2020-04-16T13:02:50.0605844Z 51                                          // mir::Constant
2020-04-16T13:02:50.0605844Z 51                                          // mir::Constant
2020-04-16T13:02:50.0606638Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:18: 15:19
2020-04-16T13:02:50.0607723Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:18: 17:19
2020-04-16T13:02:50.0608341Z 53                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
2020-04-16T13:02:50.0608813Z 54                                          // ty::Const
2020-04-16T13:02:50.0609132Z 55                                          // + ty: usize
2020-04-16T13:02:50.0609339Z 
2020-04-16T13:02:50.0609627Z 56                                          // + val: Value(Scalar(0x00000002))
2020-04-16T13:02:50.0610016Z 57                                          // mir::Constant
2020-04-16T13:02:50.0610963Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:21: 15:22
2020-04-16T13:02:50.0619362Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:21: 17:22
2020-04-16T13:02:50.0619989Z 59                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-16T13:02:50.0620450Z 60                                          // ty::Const
2020-04-16T13:02:50.0620804Z 61                                          // + ty: usize
2020-04-16T13:02:50.0621012Z 
2020-04-16T13:02:50.0621289Z 62                                          // + val: Value(Scalar(0x00000003))
2020-04-16T13:02:50.0621703Z 63                                          // mir::Constant
2020-04-16T13:02:50.0622449Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:24: 15:25
2020-04-16T13:02:50.0623335Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:24: 17:25
2020-04-16T13:02:50.0624101Z 65                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000003)) }
2020-04-16T13:02:50.0625352Z -         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-16T13:02:50.0626273Z -         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-16T13:02:50.0627188Z -         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-16T13:02:50.0628158Z -         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-16T13:02:50.0629112Z +         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-16T13:02:50.0629994Z +         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-16T13:02:50.0630879Z +         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-16T13:02:50.0631878Z +         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-16T13:02:50.0632390Z 70                                          // ty::Const
2020-04-16T13:02:50.0632715Z 71                                          // + ty: usize
2020-04-16T13:02:50.0633095Z 72                                          // + val: Value(Scalar(0x00000000))
2020-04-16T13:02:50.0633591Z 73                                          // mir::Constant
2020-04-16T13:02:50.0633591Z 73                                          // mir::Constant
2020-04-16T13:02:50.0634303Z -                                          // + span: $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-16T13:02:50.0635113Z +                                          // + span: $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-16T13:02:50.0635697Z 75                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-16T13:02:50.0636608Z -         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-16T13:02:50.0637492Z -         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-16T13:02:50.0638683Z -         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-16T13:02:50.0639818Z +         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-16T13:02:50.0640708Z +         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-16T13:02:50.0649478Z +         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-16T13:02:50.0650269Z 80 
2020-04-16T13:02:50.0650269Z 80 
2020-04-16T13:02:50.0650852Z 81     bb1 (cleanup): {
2020-04-16T13:02:50.0651010Z 
2020-04-16T13:02:50.0651764Z -         resume;                          // bb1[0]: scope 0 at $DIR/region-subtyping-basic.rs:14:1: 23:2
2020-04-16T13:02:50.0652658Z +         resume;                          // bb1[0]: scope 0 at $DIR/region-subtyping-basic.rs:16:1: 25:2
2020-04-16T13:02:50.0653189Z 84 
2020-04-16T13:02:50.0653346Z 85     bb2: {
2020-04-16T13:02:50.0653613Z 
2020-04-16T13:02:50.0653613Z 
2020-04-16T13:02:50.0655076Z -         _2 = &'_#2r _1[_3];              // bb2[0]: scope 1 at $DIR/region-subtyping-basic.rs:16:13: 16:18
2020-04-16T13:02:50.0656702Z -         FakeRead(ForLet, _2);            // bb2[1]: scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-16T13:02:50.0657613Z -         StorageLive(_6);                 // bb2[2]: scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-16T13:02:50.0658944Z -         _6 = _2;                         // bb2[3]: scope 2 at $DIR/region-subtyping-basic.rs:17:13: 17:14
2020-04-16T13:02:50.0659961Z -         FakeRead(ForLet, _6);            // bb2[4]: scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-16T13:02:50.0660837Z -         StorageLive(_7);                 // bb2[5]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-16T13:02:50.0661944Z -         _7 = const Const(Value(Scalar(0x01)): bool); // bb2[6]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-16T13:02:50.0662917Z +         _2 = &'_#3r _1[_3];              // bb2[0]: scope 1 at $DIR/region-subtyping-basic.rs:18:13: 18:18
2020-04-16T13:02:50.0663790Z +         FakeRead(ForLet, _2);            // bb2[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-16T13:02:50.0664694Z +         StorageLive(_6);                 // bb2[2]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-16T13:02:50.0665576Z +         _6 = _2;                         // bb2[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
2020-04-16T13:02:50.0666497Z +         FakeRead(ForLet, _6);            // bb2[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-16T13:02:50.0667395Z +         StorageLive(_7);                 // bb2[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-16T13:02:50.0668348Z +         _7 = const Const(Value(Scalar(0x01)): bool); // bb2[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-16T13:02:50.0668888Z 93                                          // ty::Const
2020-04-16T13:02:50.0669216Z 94                                          // + ty: bool
2020-04-16T13:02:50.0669570Z 95                                          // + val: Value(Scalar(0x01))
2020-04-16T13:02:50.0670066Z 96                                          // mir::Constant
2020-04-16T13:02:50.0670066Z 96                                          // mir::Constant
2020-04-16T13:02:50.0670778Z -                                          // + span: $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-16T13:02:50.0671605Z +                                          // + span: $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-16T13:02:50.0672178Z 98                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
2020-04-16T13:02:50.0673236Z -         FakeRead(ForMatchedPlace, _7);   // bb2[7]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-16T13:02:50.0674338Z -         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb3]; // bb2[8]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-16T13:02:50.0675344Z +         FakeRead(ForMatchedPlace, _7);   // bb2[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-16T13:02:50.0676385Z +         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb3]; // bb2[8]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-16T13:02:50.0677047Z 102 
2020-04-16T13:02:50.0677210Z 103     bb3: {
2020-04-16T13:02:50.0677521Z 
2020-04-16T13:02:50.0677521Z 
2020-04-16T13:02:50.0678264Z -         falseEdges -> [real: bb5, imaginary: bb4]; // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-16T13:02:50.0679444Z +         falseEdges -> [real: bb5, imaginary: bb4]; // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-16T13:02:50.0680040Z 106 
2020-04-16T13:02:50.0680199Z 107     bb4: {
2020-04-16T13:02:50.0680332Z 
2020-04-16T13:02:50.0680332Z 
2020-04-16T13:02:50.0680999Z -         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0682931Z -         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0684135Z +         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-16T13:02:50.0685723Z +         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-16T13:02:50.0686492Z 110                                          // ty::Const
2020-04-16T13:02:50.0687199Z 111                                          // + ty: fn(usize) -> bool {use_x}
2020-04-16T13:02:50.0687609Z 112                                          // + val: Value(Scalar(<ZST>))
2020-04-16T13:02:50.0688109Z 113                                          // mir::Constant
2020-04-16T13:02:50.0688109Z 113                                          // mir::Constant
2020-04-16T13:02:50.0688825Z -                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
2020-04-16T13:02:50.0689815Z +                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
2020-04-16T13:02:50.0690810Z 115                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
2020-04-16T13:02:50.0691340Z 116                                          // ty::Const
2020-04-16T13:02:50.0691671Z 117                                          // + ty: usize
2020-04-16T13:02:50.0691900Z 
2020-04-16T13:02:50.0692168Z 118                                          // + val: Value(Scalar(0x00000016))
2020-04-16T13:02:50.0692553Z 119                                          // mir::Constant
2020-04-16T13:02:50.0693430Z -                                          // + span: $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-16T13:02:50.0694284Z +                                          // + span: $DIR/region-subtyping-basic.rs:23:15: 23:17
2020-04-16T13:02:50.0694875Z 121                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000016)) }
2020-04-16T13:02:50.0695419Z 123 
2020-04-16T13:02:50.0695528Z 
2020-04-16T13:02:50.0695681Z 124     bb5: {
2020-04-16T13:02:50.0695681Z 124     bb5: {
2020-04-16T13:02:50.0696380Z -         StorageLive(_8);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-16T13:02:50.0697283Z -         StorageLive(_9);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-16T13:02:50.0698346Z -         _9 = (*_6);                      // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-16T13:02:50.0699553Z -         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb6, unwind: bb1]; // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-16T13:02:50.0700616Z +         StorageLive(_8);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0701664Z +         StorageLive(_9);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-16T13:02:50.0702591Z +         _9 = (*_6);                      // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-16T13:02:50.0703738Z +         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb6, unwind: bb1]; // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-16T13:02:50.0704714Z 129                                          // ty::Const
2020-04-16T13:02:50.0705385Z 130                                          // + ty: fn(usize) -> bool {use_x}
2020-04-16T13:02:50.0705817Z 131                                          // + val: Value(Scalar(<ZST>))
2020-04-16T13:02:50.0706319Z 132                                          // mir::Constant
2020-04-16T13:02:50.0706319Z 132                                          // mir::Constant
2020-04-16T13:02:50.0707012Z -                                          // + span: $DIR/region-subtyping-basic.rs:19:9: 19:14
2020-04-16T13:02:50.0707841Z +                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
2020-04-16T13:02:50.0708740Z 134                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
2020-04-16T13:02:50.0709322Z 136 
2020-04-16T13:02:50.0709431Z 
2020-04-16T13:02:50.0709682Z 137     bb6: {
2020-04-16T13:02:50.0709682Z 137     bb6: {
2020-04-16T13:02:50.0710411Z -         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:19:17: 19:18
2020-04-16T13:02:50.0711341Z -         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:19:18: 19:19
2020-04-16T13:02:50.0712277Z -         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-16T13:02:50.0713238Z +         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
2020-04-16T13:02:50.0714142Z +         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
2020-04-16T13:02:50.0715131Z +         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
2020-04-16T13:02:50.0715672Z 141                                          // ty::Const
2020-04-16T13:02:50.0715990Z 142                                          // + ty: ()
2020-04-16T13:02:50.0716349Z 143                                          // + val: Value(Scalar(<ZST>))
2020-04-16T13:02:50.0716845Z 144                                          // mir::Constant
2020-04-16T13:02:50.0716845Z 144                                          // mir::Constant
2020-04-16T13:02:50.0717553Z -                                          // + span: $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-16T13:02:50.0718395Z +                                          // + span: $DIR/region-subtyping-basic.rs:20:13: 22:6
2020-04-16T13:02:50.0718972Z 146                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T13:02:50.0719835Z -         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-16T13:02:50.0720721Z +         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-16T13:02:50.0722321Z 149 
2020-04-16T13:02:50.0722510Z 150     bb7: {
2020-04-16T13:02:50.0722654Z 
2020-04-16T13:02:50.0722654Z 
2020-04-16T13:02:50.0723394Z -         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
2020-04-16T13:02:50.0724351Z -         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-16T13:02:50.0725314Z +         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
2020-04-16T13:02:50.0726250Z +         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
2020-04-16T13:02:50.0726782Z 153                                          // ty::Const
2020-04-16T13:02:50.0727092Z 154                                          // + ty: ()
2020-04-16T13:02:50.0727460Z 155                                          // + val: Value(Scalar(<ZST>))
2020-04-16T13:02:50.0727946Z 156                                          // mir::Constant
2020-04-16T13:02:50.0727946Z 156                                          // mir::Constant
2020-04-16T13:02:50.0728648Z -                                          // + span: $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-16T13:02:50.0729649Z +                                          // + span: $DIR/region-subtyping-basic.rs:22:12: 24:6
2020-04-16T13:02:50.0730230Z 158                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-16T13:02:50.0731340Z -         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-16T13:02:50.0732249Z +         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-16T13:02:50.0732783Z 161 
2020-04-16T13:02:50.0732962Z 162     bb8: {
2020-04-16T13:02:50.0733096Z 
2020-04-16T13:02:50.0733096Z 
2020-04-16T13:02:50.0733732Z -         StorageDead(_6);                 // bb8[0]: scope 2 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-16T13:02:50.0734730Z -         StorageDead(_3);                 // bb8[1]: scope 1 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-16T13:02:50.0735680Z -         StorageDead(_2);                 // bb8[2]: scope 1 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-16T13:02:50.0736562Z -         StorageDead(_1);                 // bb8[3]: scope 0 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-16T13:02:50.0737446Z -         StorageDead(_7);                 // bb8[4]: scope 0 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-16T13:02:50.0738318Z -         return;                          // bb8[5]: scope 0 at $DIR/region-subtyping-basic.rs:23:2: 23:2
2020-04-16T13:02:50.0739204Z +         StorageDead(_6);                 // bb8[0]: scope 2 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-16T13:02:50.0740080Z +         StorageDead(_3);                 // bb8[1]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-16T13:02:50.0740952Z +         StorageDead(_2);                 // bb8[2]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-16T13:02:50.0741819Z +         StorageDead(_1);                 // bb8[3]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-16T13:02:50.0742714Z +         StorageDead(_7);                 // bb8[4]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-16T13:02:50.0743904Z +         return;                          // bb8[5]: scope 0 at $DIR/region-subtyping-basic.rs:25:2: 25:2
2020-04-16T13:02:50.0744449Z 170 }
2020-04-16T13:02:50.0744585Z 171 
2020-04-16T13:02:50.0744694Z 
2020-04-16T13:02:50.0744694Z 
2020-04-16T13:02:50.0745774Z thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region-subtyping-basic/32bit/rustc.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T13:02:50.0746709Z 
2020-04-16T13:02:50.0746833Z 
2020-04-16T13:02:50.0746973Z failures:
2020-04-16T13:02:50.0747429Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-16T13:02:50.0747429Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-16T13:02:50.0747629Z 
2020-04-16T13:02:50.0748265Z test result: FAILEDthread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-16T13:02:50.0748708Z . 88 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T13:02:50.0748915Z 
2020-04-16T13:02:50.0749032Z 
2020-04-16T13:02:50.0749133Z 
2020-04-16T13:02:50.0762134Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-16T13:02:50.0765145Z 
2020-04-16T13:02:50.0765251Z 
2020-04-16T13:02:50.0765251Z 
2020-04-16T13:02:50.0766169Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T13:02:50.0766916Z == clock drift check ==
2020-04-16T13:02:50.0766916Z == clock drift check ==
2020-04-16T13:02:50.0767169Z   local time: Thu Apr 16 13:02:50 UTC 2020
2020-04-16T13:02:50.4149934Z   network time: Thu, 16 Apr 2020 13:02:50 GMT
2020-04-16T13:02:52.7565849Z 
2020-04-16T13:02:52.7565849Z 
2020-04-16T13:02:52.7642912Z ##[error]Bash exited with code '1'.
2020-04-16T13:02:52.7662034Z ##[section]Finishing: Run build
2020-04-16T13:02:52.7706874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-16T13:02:52.7712707Z Task         : Get sources
2020-04-16T13:02:52.7713008Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T13:02:52.7713302Z Version      : 1.0.0
2020-04-16T13:02:52.7713501Z Author       : Microsoft
2020-04-16T13:02:52.7713501Z Author       : Microsoft
2020-04-16T13:02:52.7713814Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T13:02:52.7714196Z ==============================================================================
2020-04-16T13:02:53.1404991Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T13:02:53.1445257Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-16T13:02:53.1531566Z Cleaning up task key
2020-04-16T13:02:53.1532691Z Start cleaning up orphan processes.
2020-04-16T13:02:53.1716953Z Terminate orphan process: pid (3462) (python)
2020-04-16T13:02:53.1978855Z ##[section]Finishing: Finalize Job
