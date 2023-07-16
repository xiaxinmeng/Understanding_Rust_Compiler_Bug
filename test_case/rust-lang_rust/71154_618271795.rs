plain
2020-04-23T07:10:01.3089681Z ========================== Starting Command Output ===========================
2020-04-23T07:10:01.3097257Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/06c6e8c1-7959-43d9-adfd-88fcd04533f8.sh
2020-04-23T07:10:01.3097677Z 
2020-04-23T07:10:01.3101497Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T07:10:01.3123417Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-23T07:10:01.3127288Z Task         : Get sources
2020-04-23T07:10:01.3127582Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T07:10:01.3127855Z Version      : 1.0.0
2020-04-23T07:10:01.3128043Z Author       : Microsoft
---
2020-04-23T07:10:02.6299908Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T07:10:02.6309031Z ##[command]git config gc.auto 0
2020-04-23T07:10:02.6314315Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T07:10:02.6321669Z ##[command]git config --get-all http.proxy
2020-04-23T07:10:02.6330789Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-23T07:13:07.1549965Z  ---> 318032b5f0e2
2020-04-23T07:13:07.1550879Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T07:13:07.1551474Z  ---> Using cache
2020-04-23T07:13:07.1551789Z  ---> d44a858fd1ce
2020-04-23T07:13:07.1552729Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T07:13:07.1555594Z  ---> 58b910f50f5a
2020-04-23T07:13:07.1555939Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T07:13:07.1556420Z  ---> Using cache
2020-04-23T07:13:07.1556889Z  ---> ee7702aadba1
---
2020-04-23T07:13:08.1764312Z Looks like docker image is the same as before, not uploading
2020-04-23T07:13:13.8990702Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T07:13:13.9266952Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T07:13:13.9297800Z == clock drift check ==
2020-04-23T07:13:13.9307848Z   local time: Thu Apr 23 07:13:13 UTC 2020
2020-04-23T07:13:14.2235231Z   network time: Thu, 23 Apr 2020 07:13:14 GMT
2020-04-23T07:13:14.2258926Z Starting sccache server...
2020-04-23T07:13:14.3116441Z configure: processing command line
2020-04-23T07:13:14.3116923Z configure: 
2020-04-23T07:13:14.3117823Z configure: rust.dist-src        := False
---
2020-04-23T07:18:28.9142477Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T07:18:30.3657286Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T07:18:31.9768100Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T07:18:32.6750124Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T07:18:41.9866652Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T07:18:43.9495675Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T07:18:48.3051326Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T07:18:52.4419256Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T07:19:02.1781402Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T07:40:24.2376399Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T07:40:25.7683731Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T07:40:27.5385106Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T07:40:27.9655496Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T07:40:38.0366735Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T07:40:40.1362864Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T07:40:44.6595020Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T07:40:49.0239047Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T07:40:59.2058057Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T08:02:46.7352998Z .................................................................................................... 1800/9924
2020-04-23T08:02:53.5760816Z .................................................................................................... 1900/9924
2020-04-23T08:03:02.1863505Z ............i....................................................................................... 2000/9924
2020-04-23T08:03:08.4152188Z .................................................................................................... 2100/9924
2020-04-23T08:03:20.8904275Z ..iiiii............................................................................................. 2200/9924
2020-04-23T08:03:29.4508364Z .................................................................................................... 2400/9924
2020-04-23T08:03:31.6751725Z .................................................................................................... 2500/9924
2020-04-23T08:03:36.9666449Z .................................................................................................... 2600/9924
2020-04-23T08:03:54.6903940Z .................................................................................................... 2700/9924
---
2020-04-23T08:06:28.7117537Z .................................................................................................... 5100/9924
2020-04-23T08:06:35.9757925Z .................................................................................................... 5200/9924
2020-04-23T08:06:40.6109817Z .........................i.......................................................................... 5300/9924
2020-04-23T08:06:49.7520575Z ...............i.................................................................................... 5400/9924
2020-04-23T08:06:55.3364554Z ...............ii.ii........i...i................................................................... 5500/9924
2020-04-23T08:07:02.8191009Z ..............................................................i..................................... 5700/9924
2020-04-23T08:07:11.0250945Z ...............................................................................................ii... 5800/9924
2020-04-23T08:07:17.4881114Z ..................................i................................................................. 5900/9924
2020-04-23T08:07:22.8518622Z .................................................................................................... 6000/9924
2020-04-23T08:07:22.8518622Z .................................................................................................... 6000/9924
2020-04-23T08:07:32.6404417Z .................................................................................................... 6100/9924
2020-04-23T08:07:42.1683300Z ............................ii...i..ii...........i.................................................. 6200/9924
2020-04-23T08:07:58.2412264Z .................................................................................................... 6400/9924
2020-04-23T08:08:02.3693410Z .................................................................................................... 6500/9924
2020-04-23T08:08:02.3693410Z .................................................................................................... 6500/9924
2020-04-23T08:08:08.0475914Z ..........................................................i..ii..................................... 6600/9924
2020-04-23T08:08:30.4491098Z .................................................................................................... 6800/9924
2020-04-23T08:08:34.7787424Z ...........................................................i........................................ 6900/9924
2020-04-23T08:08:36.9227684Z .................................................................................................... 7000/9924
2020-04-23T08:08:39.0574510Z ...................................................................................................i 7100/9924
---
2020-04-23T08:10:13.2963648Z .................................................................................................... 7900/9924
2020-04-23T08:10:18.5862829Z .................................................................................................... 8000/9924
2020-04-23T08:10:24.9389196Z ..................................................................i................................. 8100/9924
2020-04-23T08:10:34.7276904Z .................................................................................................... 8200/9924
2020-04-23T08:10:40.0913688Z ...............iiiiii.iiiii.i....................................................................... 8300/9924
2020-04-23T08:10:53.5552728Z .................................................................................................... 8500/9924
2020-04-23T08:10:59.2092033Z .................................................................................................... 8600/9924
2020-04-23T08:11:13.1414318Z .................................................................................................... 8700/9924
2020-04-23T08:11:19.7114762Z .................................................................................................... 8800/9924
---
2020-04-23T08:13:30.1586894Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-23T08:13:30.1772988Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:13:30.4026158Z 
2020-04-23T08:13:30.4027099Z running 186 tests
2020-04-23T08:13:33.2682574Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-23T08:13:35.7851779Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-23T08:13:35.7854865Z 
2020-04-23T08:13:35.7860158Z  finished in 5.608
2020-04-23T08:13:35.7864895Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-23T08:13:35.8062690Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T08:13:37.8716414Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-23T08:13:37.8905659Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:13:38.0445542Z 
2020-04-23T08:13:38.0445873Z running 9 tests
2020-04-23T08:13:38.0446931Z iiiiiiiii
2020-04-23T08:13:38.0447904Z 
2020-04-23T08:13:38.0448046Z  finished in 0.154
2020-04-23T08:13:38.0455766Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-23T08:13:38.0649728Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T08:13:57.5572818Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-23T08:13:57.5802821Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:13:57.7734491Z 
2020-04-23T08:13:57.7735082Z running 115 tests
2020-04-23T08:14:10.8266192Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-23T08:14:12.5135195Z ...iiii.....ii.
2020-04-23T08:14:12.5136310Z 
2020-04-23T08:14:12.5139543Z  finished in 14.933
2020-04-23T08:14:12.5144278Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-23T08:14:12.5147431Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T08:25:11.7809214Z 
2020-04-23T08:25:11.7810392Z    Doc-tests core
2020-04-23T08:25:16.4150517Z 
2020-04-23T08:25:16.4151454Z running 2499 tests
2020-04-23T08:25:24.9701915Z ......iiiii......................................................................................... 100/2499
2020-04-23T08:25:33.3109280Z ......................................................................................ii............ 200/2499
2020-04-23T08:25:52.3553837Z ......................i............................................................................. 400/2499
2020-04-23T08:26:01.6763759Z ............................................................................i..i..................ii 500/2499
2020-04-23T08:26:08.7410435Z ii.................................................................................................. 600/2499
2020-04-23T08:26:16.6375262Z .................................................................................................... 700/2499
---
2020-04-23T08:29:59.7364320Z 
2020-04-23T08:29:59.7364753Z running 1020 tests
2020-04-23T08:30:15.6195282Z i................................................................................................... 100/1020
2020-04-23T08:30:25.2985234Z .................................................................................................... 200/1020
2020-04-23T08:30:32.3607185Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-23T08:30:36.9855755Z .................................................................................................... 400/1020
2020-04-23T08:30:43.3076523Z ....................................................i....i......................................ii.. 500/1020
2020-04-23T08:30:55.2710207Z .................................................................................................... 700/1020
2020-04-23T08:30:55.2710207Z .................................................................................................... 700/1020
2020-04-23T08:31:02.1417037Z ..............................................iiii.................................................. 800/1020
2020-04-23T08:31:14.9791466Z .................................................................................................... 900/1020
2020-04-23T08:31:20.8642686Z ....................................................................iiii............................ 1000/1020
2020-04-23T08:31:22.1572611Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-23T08:31:22.1573197Z 
2020-04-23T08:31:22.1675848Z  finished in 152.847
2020-04-23T08:31:22.1683375Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-23T08:34:18.3285296Z 
2020-04-23T08:34:18.3285644Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-23T08:34:18.3285897Z 
2020-04-23T08:34:18.3348708Z  finished in 0.935
2020-04-23T08:34:18.3353992Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-23T08:34:18.3373800Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:34:18.5196973Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T08:34:19.4894973Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-c219e6ea1cb84f52
2020-04-23T08:34:19.4923709Z 
2020-04-23T08:34:19.4924168Z running 0 tests
2020-04-23T08:34:19.4924511Z 
---
2020-04-23T08:47:17.9553723Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9554466Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9555198Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9555911Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9556659Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9558096Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9558833Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9559536Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-23T08:47:17.9560367Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-23T08:48:16.3816462Z  finished in 38.601
2020-04-23T08:48:16.4177205Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:48:16.6434037Z 
2020-04-23T08:48:16.6434397Z running 211 tests
2020-04-23T08:48:44.6657544Z ......................i...ii.......................................................................i 100/211
2020-04-23T08:49:20.1839450Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-23T08:49:25.4642572Z .......ii..
2020-04-23T08:49:25.4643982Z 
2020-04-23T08:49:25.4649658Z  finished in 69.047
2020-04-23T08:49:25.4655957Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-23T08:49:25.4660905Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-23T08:49:38.6491211Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-23T08:49:38.6687222Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-23T08:49:38.8155864Z  finished in 4.130
2020-04-23T08:49:38.8166847Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-23T08:49:39.5940711Z Checking "alias-1" ... OK
2020-04-23T08:49:39.6571608Z Checking "alias-2" ... OK
2020-04-23T08:49:39.7225172Z Checking "alias-3" ... OK
2020-04-23T08:49:39.8094908Z Checking "alias" ... OK
2020-04-23T08:49:39.9204499Z Checking "basic" ... OK
2020-04-23T08:49:40.0088431Z Checking "deduplication" ... OK
2020-04-23T08:49:40.0624373Z Checking "enum-option" ... OK
2020-04-23T08:49:40.1317209Z Checking "filter-crate" ... OK
2020-04-23T08:49:40.1939444Z Checking "fn-forget" ... OK
2020-04-23T08:49:40.2800894Z Checking "from_u" ... OK
2020-04-23T08:49:40.3660901Z Checking "keyword" ... OK
2020-04-23T08:49:40.9595272Z Checking "macro-check" ... OK
2020-04-23T08:49:40.9603812Z Checking "macro-print" ... OK
2020-04-23T08:49:40.9604218Z Checking "multi-query" ... OK
2020-04-23T08:49:40.9604416Z Checking "never" ... OK
2020-04-23T08:49:40.9604615Z Checking "quoted" ... OK
2020-04-23T08:49:40.9605005Z Checking "return-specific-literal" ... OK
2020-04-23T08:49:40.9605395Z Checking "return-specific" ... OK
2020-04-23T08:49:40.9605780Z Checking "should-fail" ... OK
2020-04-23T08:49:40.9606160Z Checking "string-from_ut" ... OK
2020-04-23T08:49:40.9606511Z Checking "struct-vec" ... OK
2020-04-23T08:49:41.0599690Z Checking "vec-new" ... OK
2020-04-23T08:49:41.0946345Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:49:41.2594737Z 
2020-04-23T08:49:41.2595630Z running 6 tests
2020-04-23T08:49:46.1714325Z ......
---
2020-04-23T08:50:47.2870912Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-23T08:50:47.3065851Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:50:47.4690075Z 
2020-04-23T08:50:47.4690376Z running 13 tests
2020-04-23T08:50:47.8945499Z .iiiiiii.iii.
2020-04-23T08:50:47.8946566Z 
2020-04-23T08:50:47.8951069Z  finished in 0.588
2020-04-23T08:50:47.9017872Z Build completed successfully in 1:35:52
2020-04-23T08:50:47.9017872Z Build completed successfully in 1:35:52
2020-04-23T08:50:47.9027294Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-23T08:50:49.3227877Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T08:50:50.1839111Z     Finished release [optimized] target(s) in 0.85s
2020-04-23T08:50:50.1948292Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-23T08:50:50.2050874Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T08:51:57.1555691Z running 90 tests
2020-04-23T08:52:05.0132566Z ............................F.............................................................
2020-04-23T08:52:05.0135577Z failures:
2020-04-23T08:52:05.0135700Z 
2020-04-23T08:52:05.0136256Z ---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
2020-04-23T08:52:05.0136686Z 21           _9 = const main::promoted[0];    // bb0[4]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T08:52:05.0137097Z 22                                            // ty::Const
2020-04-23T08:52:05.0137410Z 23                                            // + ty: &[u32; 3]
2020-04-23T08:52:05.0138435Z -                                            // + val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), [], Some(promoted[0]))
2020-04-23T08:52:05.0139103Z +                                            // + val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), None, [], Some(promoted[0]))
2020-04-23T08:52:05.0139612Z 25                                            // mir::Constant
2020-04-23T08:52:05.0139966Z 26                                            // + span: $DIR/slice_len.rs:5:6: 5:19
2020-04-23T08:52:05.0140922Z -                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), [], Some(promoted[0])) }
2020-04-23T08:52:05.0141741Z +                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), None, [], Some(promoted[0])) }
2020-04-23T08:52:05.0142392Z 28           _4 = _9;                         // bb0[5]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T08:52:05.0142873Z 29           _3 = _4;                         // bb0[6]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T08:52:05.0143340Z 30           _2 = move _3 as &[u32] (Pointer(Unsize)); // bb0[7]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T08:52:05.0143803Z 
2020-04-23T08:52:05.0144673Z thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-23T08:52:05.0145494Z 
2020-04-23T08:52:05.0145587Z 
2020-04-23T08:52:05.0145717Z failures:
2020-04-23T08:52:05.0146091Z     [mir-opt] mir-opt/const_prop/slice_len.rs
2020-04-23T08:52:05.0146091Z     [mir-opt] mir-opt/const_prop/slice_len.rs
2020-04-23T08:52:05.0146265Z 
2020-04-23T08:52:05.0146819Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-23T08:52:05.0147060Z 
2020-04-23T08:52:05.0147546Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T08:52:05.0149952Z 
2020-04-23T08:52:05.0150060Z 
2020-04-23T08:52:05.0153885Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T08:52:05.0156328Z 
2020-04-23T08:52:05.0156435Z 
2020-04-23T08:52:05.0156435Z 
2020-04-23T08:52:05.0157033Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-23T08:52:05.0227046Z == clock drift check ==
2020-04-23T08:52:05.0227046Z == clock drift check ==
2020-04-23T08:52:05.0245903Z   local time: Thu Apr 23 08:52:05 UTC 2020
2020-04-23T08:52:05.3202513Z   network time: Thu, 23 Apr 2020 08:52:05 GMT
2020-04-23T08:52:07.7211555Z 
2020-04-23T08:52:07.7211555Z 
2020-04-23T08:52:07.7313922Z ##[error]Bash exited with code '1'.
2020-04-23T08:52:07.7327783Z ##[section]Finishing: Run build
2020-04-23T08:52:07.7374792Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-23T08:52:07.7379805Z Task         : Get sources
2020-04-23T08:52:07.7381929Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T08:52:07.7382245Z Version      : 1.0.0
2020-04-23T08:52:07.7382495Z Author       : Microsoft
2020-04-23T08:52:07.7382495Z Author       : Microsoft
2020-04-23T08:52:07.7383964Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T08:52:07.7385242Z ==============================================================================
2020-04-23T08:52:08.0792389Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T08:52:08.0845401Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-23T08:52:08.0939977Z Cleaning up task key
2020-04-23T08:52:08.0941226Z Start cleaning up orphan processes.
2020-04-23T08:52:08.1125921Z Terminate orphan process: pid (3843) (python)
2020-04-23T08:52:08.1402663Z ##[section]Finishing: Finalize Job
