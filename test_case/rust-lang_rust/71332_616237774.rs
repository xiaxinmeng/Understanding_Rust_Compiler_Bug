plain
2020-04-19T21:05:20.7388464Z ========================== Starting Command Output ===========================
2020-04-19T21:05:20.7412261Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f07634e-500c-4833-b930-6bb40b1d38a2.sh
2020-04-19T21:05:20.7648224Z 
2020-04-19T21:05:20.7707455Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T21:05:20.7725914Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71332/merge to s
2020-04-19T21:05:20.7730448Z Task         : Get sources
2020-04-19T21:05:20.7730749Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T21:05:20.7731063Z Version      : 1.0.0
2020-04-19T21:05:20.7731259Z Author       : Microsoft
---
2020-04-19T21:05:22.2238927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T21:05:22.2255496Z ##[command]git config gc.auto 0
2020-04-19T21:05:22.2291401Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T21:05:22.2300516Z ##[command]git config --get-all http.proxy
2020-04-19T21:05:22.2307959Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71332/merge:refs/remotes/pull/71332/merge
---
2020-04-19T21:08:41.0299231Z  ---> 318032b5f0e2
2020-04-19T21:08:41.0300627Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T21:08:41.0302693Z  ---> Using cache
2020-04-19T21:08:41.0303038Z  ---> d44a858fd1ce
2020-04-19T21:08:41.0303928Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T21:08:41.0308550Z  ---> 58b910f50f5a
2020-04-19T21:08:41.0308751Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T21:08:41.0312191Z  ---> Using cache
2020-04-19T21:08:41.0312519Z  ---> ee7702aadba1
---
2020-04-19T21:08:41.1937065Z Looks like docker image is the same as before, not uploading
2020-04-19T21:08:49.7618880Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T21:08:49.7897048Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T21:08:49.7929613Z == clock drift check ==
2020-04-19T21:08:49.7952669Z   local time: Sun Apr 19 21:08:49 UTC 2020
2020-04-19T21:08:49.8579448Z   network time: Sun, 19 Apr 2020 21:08:49 GMT
2020-04-19T21:08:49.8608167Z Starting sccache server...
2020-04-19T21:08:49.9413931Z configure: processing command line
2020-04-19T21:08:49.9414863Z configure: 
2020-04-19T21:08:49.9416169Z configure: rust.dist-src        := False
---
2020-04-19T21:13:43.0861957Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:13:44.4627655Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:13:45.9428031Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:13:46.7992529Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:13:55.2000575Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:13:57.3507722Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:14:01.3948210Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:14:05.3934453Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:14:14.3291157Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T21:34:50.6398211Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:34:52.1462000Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:34:53.5990187Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:34:53.8756008Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:35:03.2615633Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:35:05.6109609Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:35:09.9573479Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:35:13.9529662Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:35:23.7132817Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T21:56:32.8249074Z .................................................................................................... 1700/9908
2020-04-19T21:56:36.6139646Z .................................................................................................... 1800/9908
2020-04-19T21:56:44.5825133Z ...................................................................................................i 1900/9908
2020-04-19T21:56:51.6711022Z .................................................................................................... 2000/9908
2020-04-19T21:56:57.3846677Z .........................................................................................iiiii...... 2100/9908
2020-04-19T21:57:15.4497131Z .................................................................................................... 2300/9908
2020-04-19T21:57:17.5061082Z .................................................................................................... 2400/9908
2020-04-19T21:57:19.6870767Z .................................................................................................... 2500/9908
2020-04-19T21:57:25.3022576Z .................................................................................................... 2600/9908
---
2020-04-19T22:00:06.0579877Z .................................................................i...............i.................. 5000/9908
2020-04-19T22:00:13.6995401Z .................................................................................................... 5100/9908
2020-04-19T22:00:20.1366224Z .................................................................................................... 5200/9908
2020-04-19T22:00:25.3327657Z ...........i........................................................................................ 5300/9908
2020-04-19T22:00:34.7895116Z .i.................................................................................................. 5400/9908
2020-04-19T22:00:39.8731220Z .ii.ii........i...i................................................................................. 5500/9908
2020-04-19T22:00:48.2344263Z ................................................i................................................... 5700/9908
2020-04-19T22:00:56.7118435Z ................................................................................ii.................. 5800/9908
2020-04-19T22:01:03.9066242Z ...................i................................................................................ 5900/9908
2020-04-19T22:01:09.1497180Z .................................................................................................... 6000/9908
2020-04-19T22:01:09.1497180Z .................................................................................................... 6000/9908
2020-04-19T22:01:19.4444151Z .................................................................................................... 6100/9908
2020-04-19T22:01:29.3338692Z .............ii...i..ii...........i................................................................. 6200/9908
2020-04-19T22:01:43.2968588Z .................................................................................................... 6400/9908
2020-04-19T22:01:46.6934677Z .................................................................................................... 6500/9908
2020-04-19T22:01:46.6934677Z .................................................................................................... 6500/9908
2020-04-19T22:01:56.5586907Z ...........................................i..ii.................................................... 6600/9908
2020-04-19T22:02:16.3650058Z .................................................................................................... 6800/9908
2020-04-19T22:02:18.4321488Z ............................................i....................................................... 6900/9908
2020-04-19T22:02:20.4287899Z .................................................................................................... 7000/9908
2020-04-19T22:02:22.4524163Z ....................................................................................i............... 7100/9908
---
2020-04-19T22:03:46.2164056Z .................................................................................................... 7800/9908
2020-04-19T22:03:50.4904530Z .................................................................................................... 7900/9908
2020-04-19T22:03:56.5442577Z .................................................................................................... 8000/9908
2020-04-19T22:04:01.7905456Z ..................................................i................................................. 8100/9908
2020-04-19T22:04:11.1534901Z ...................................................................................................i 8200/9908
2020-04-19T22:04:15.9760371Z iiiii.iiiii.i....................................................................................... 8300/9908
2020-04-19T22:04:28.3906910Z .................................................................................................... 8500/9908
2020-04-19T22:04:35.8062484Z .................................................................................................... 8600/9908
2020-04-19T22:04:48.4028083Z .................................................................................................... 8700/9908
2020-04-19T22:04:54.7391337Z .................................................................................................... 8800/9908
---
2020-04-19T22:07:02.5522042Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-19T22:07:02.5714437Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:07:02.7795202Z 
2020-04-19T22:07:02.7795990Z running 186 tests
2020-04-19T22:07:05.6458398Z iiii......i.............ii.i..........i..............................i.i..................i....i.... 100/186
2020-04-19T22:07:08.1058447Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-19T22:07:08.8429804Z 
2020-04-19T22:07:08.8430439Z  finished in 5.535
2020-04-19T22:07:08.8431393Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-19T22:07:08.8432258Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T22:07:10.2541559Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-19T22:07:10.2723603Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:07:10.4237400Z 
2020-04-19T22:07:10.4237721Z running 9 tests
2020-04-19T22:07:10.4238789Z iiiiiiiii
2020-04-19T22:07:10.4239658Z 
2020-04-19T22:07:10.4244259Z  finished in 0.151
2020-04-19T22:07:10.4244986Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-19T22:07:10.4436233Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T22:07:29.8989140Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-19T22:07:29.9210222Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:07:30.0979602Z 
2020-04-19T22:07:30.0980402Z running 115 tests
2020-04-19T22:07:42.8509396Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-19T22:07:44.4798345Z ...iiii.....ii.
2020-04-19T22:07:44.4803244Z 
2020-04-19T22:07:44.4807048Z  finished in 14.559
2020-04-19T22:07:44.4815342Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-19T22:07:44.4817043Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T22:18:19.2738452Z 
2020-04-19T22:18:19.2739575Z    Doc-tests core
2020-04-19T22:18:23.3646603Z 
2020-04-19T22:18:23.3647517Z running 2497 tests
2020-04-19T22:18:31.1687904Z ......iiiii......................................................................................... 100/2497
2020-04-19T22:18:38.7385846Z .....................................................................................ii............. 200/2497
2020-04-19T22:18:56.4398136Z .....................i.............................................................................. 400/2497
2020-04-19T22:18:56.4398136Z .....................i.............................................................................. 400/2497
2020-04-19T22:19:05.2282016Z ...........................................................................i..i..................iii 500/2497
2020-04-19T22:19:11.9905749Z i................................................................................................... 600/2497
2020-04-19T22:19:27.0602629Z .................................................................................................... 800/2497
2020-04-19T22:19:34.5307171Z .................................................................................................... 900/2497
2020-04-19T22:19:41.9464611Z .................................................................................................... 1000/2497
2020-04-19T22:19:49.2270486Z .................................................................................................... 1100/2497
---
2020-04-19T22:22:46.1832810Z 
2020-04-19T22:22:46.1845188Z running 1020 tests
2020-04-19T22:23:00.8486604Z i................................................................................................... 100/1020
2020-04-19T22:23:10.3035056Z .................................................................................................... 200/1020
2020-04-19T22:23:17.2655092Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-19T22:23:21.8476810Z .................................................................................................... 400/1020
2020-04-19T22:23:28.1488783Z ....................................................i....i......................................ii.. 500/1020
2020-04-19T22:23:40.0197581Z .................................................................................................... 700/1020
2020-04-19T22:23:40.0197581Z .................................................................................................... 700/1020
2020-04-19T22:23:46.5049394Z ..............................................iiii.................................................. 800/1020
2020-04-19T22:23:58.6729690Z .................................................................................................... 900/1020
2020-04-19T22:24:04.1145964Z ....................................................................iiii............................ 1000/1020
2020-04-19T22:24:05.3013949Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-19T22:24:05.3014188Z 
2020-04-19T22:24:05.3074982Z  finished in 142.919
2020-04-19T22:24:05.3088605Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-19T22:26:44.8762712Z 
2020-04-19T22:26:44.8762966Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T22:26:44.8763230Z 
2020-04-19T22:26:44.8830905Z  finished in 0.856
2020-04-19T22:26:44.8842995Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-19T22:26:44.8859990Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:26:45.0542212Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T22:26:45.9495995Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-46fc6f225e780bbc
2020-04-19T22:26:45.9523084Z 
2020-04-19T22:26:45.9523337Z running 0 tests
2020-04-19T22:26:45.9523512Z 
---
2020-04-19T22:38:58.0397875Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0398571Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0399763Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0400746Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0401876Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0403226Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0403892Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0404554Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T22:38:58.0405218Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-19T22:39:53.1483448Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-19T22:39:53.1812357Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:39:53.3871967Z 
2020-04-19T22:39:53.3872496Z running 211 tests
2020-04-19T22:40:19.7674223Z ......................i...ii.......................................................................i 100/211
2020-04-19T22:40:54.5166772Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-19T22:40:55.7498200Z .......ii..
2020-04-19T22:40:55.7500885Z 
2020-04-19T22:40:55.7506924Z  finished in 62.569
2020-04-19T22:40:55.7514358Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-19T22:40:55.7521240Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-19T22:41:07.9500592Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-19T22:41:07.9657332Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-19T22:41:08.1011848Z  finished in 3.694
2020-04-19T22:41:08.1020340Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-19T22:41:08.9573034Z Checking "alias-1" ... OK
2020-04-19T22:41:09.0260542Z Checking "alias-2" ... OK
2020-04-19T22:41:09.0830904Z Checking "alias-3" ... OK
2020-04-19T22:41:09.1462328Z Checking "alias" ... OK
2020-04-19T22:41:09.2210831Z Checking "basic" ... OK
2020-04-19T22:41:09.2937954Z Checking "deduplication" ... OK
2020-04-19T22:41:09.3396497Z Checking "enum-option" ... OK
2020-04-19T22:41:09.4061319Z Checking "filter-crate" ... OK
2020-04-19T22:41:09.4638338Z Checking "fn-forget" ... OK
2020-04-19T22:41:09.5552483Z Checking "from_u" ... OK
2020-04-19T22:41:09.6423539Z Checking "keyword" ... OK
2020-04-19T22:41:09.6852624Z Checking "macro-check" ... OK
2020-04-19T22:41:09.7155811Z Checking "macro-print" ... OK
2020-04-19T22:41:09.7847558Z Checking "multi-query" ... OK
2020-04-19T22:41:09.8101239Z Checking "never" ... OK
2020-04-19T22:41:09.8235760Z Checking "quoted" ... OK
2020-04-19T22:41:09.8441815Z Checking "return-specific-literal" ... OK
2020-04-19T22:41:09.9091792Z Checking "return-specific" ... OK
2020-04-19T22:41:09.9417662Z Checking "should-fail" ... OK
2020-04-19T22:41:09.9970228Z Checking "string-from_ut" ... OK
2020-04-19T22:41:10.0467832Z Checking "struct-vec" ... OK
2020-04-19T22:41:10.1462709Z Checking "vec-new" ... OK
2020-04-19T22:41:10.1717579Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:41:10.3274143Z 
2020-04-19T22:41:10.3275622Z running 6 tests
2020-04-19T22:41:14.6044302Z ......
---
2020-04-19T22:42:11.6391251Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-19T22:42:11.6574484Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:42:11.8034938Z 
2020-04-19T22:42:11.8036320Z running 13 tests
2020-04-19T22:42:12.1725514Z .iiiiiii.iii.
2020-04-19T22:42:12.1726633Z 
2020-04-19T22:42:12.1731360Z  finished in 0.515
2020-04-19T22:42:12.1791827Z Build completed successfully in 1:31:51
2020-04-19T22:42:12.1791827Z Build completed successfully in 1:31:51
2020-04-19T22:42:12.1801158Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-19T22:42:13.3593585Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T22:42:13.5760694Z     Finished release [optimized] target(s) in 0.21s
2020-04-19T22:42:13.5837444Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-19T22:42:13.5941321Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T22:43:14.3528369Z running 90 tests
2020-04-19T22:43:21.3332823Z ........FF................................................................................
2020-04-19T22:43:21.3337571Z failures:
2020-04-19T22:43:21.3337701Z 
2020-04-19T22:43:21.3338237Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-19T22:43:21.3338565Z 14                                          // + val: Value(Scalar(alloc0+0))
2020-04-19T22:43:21.3338903Z 15                                          // mir::Constant
2020-04-19T22:43:21.3339266Z 16                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
2020-04-19T22:43:21.3339628Z +                                          // + user_ty: UserType(0)
2020-04-19T22:43:21.3340133Z 17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc0+0)) }
2020-04-19T22:43:21.3340696Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-19T22:43:21.3341174Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-19T22:43:21.3341437Z 
2020-04-19T22:43:21.3342225Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-19T22:43:21.3343013Z 
2020-04-19T22:43:21.3343322Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-19T22:43:21.3343322Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-19T22:43:21.3343600Z 14                                          // + val: Value(Scalar(alloc0+0))
2020-04-19T22:43:21.3343871Z 15                                          // mir::Constant
2020-04-19T22:43:21.3344173Z 16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
2020-04-19T22:43:21.3344505Z +                                          // + user_ty: UserType(0)
2020-04-19T22:43:21.3344857Z 17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc0+0)) }
2020-04-19T22:43:21.3345291Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-19T22:43:21.3345819Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-19T22:43:21.3346051Z 
2020-04-19T22:43:21.3346788Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-19T22:43:21.3347237Z 
2020-04-19T22:43:21.3347340Z failures:
2020-04-19T22:43:21.3347654Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-19T22:43:21.3347979Z     [mir-opt] mir-opt/const_allocation3.rs
2020-04-19T22:43:21.3347979Z     [mir-opt] mir-opt/const_allocation3.rs
2020-04-19T22:43:21.3348103Z 
2020-04-19T22:43:21.3354961Z test result: FAILED. 88 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T22:43:21.3362269Z 
2020-04-19T22:43:21.3413764Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T22:43:21.3414025Z 
2020-04-19T22:43:21.3414133Z 
2020-04-19T22:43:21.3417732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T22:43:21.3419960Z 
2020-04-19T22:43:21.3420045Z 
2020-04-19T22:43:21.3420045Z 
2020-04-19T22:43:21.3420595Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-19T22:43:21.3444994Z == clock drift check ==
2020-04-19T22:43:21.3467352Z   local time: Sun Apr 19 22:43:21 UTC 2020
2020-04-19T22:43:21.3467352Z   local time: Sun Apr 19 22:43:21 UTC 2020
2020-04-19T22:43:21.4269712Z   network time: Sun, 19 Apr 2020 22:43:21 GMT
2020-04-19T22:43:23.9050378Z 
2020-04-19T22:43:23.9050378Z 
2020-04-19T22:43:23.9127446Z ##[error]Bash exited with code '1'.
2020-04-19T22:43:23.9140503Z ##[section]Finishing: Run build
2020-04-19T22:43:23.9190487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71332/merge to s
2020-04-19T22:43:23.9194357Z Task         : Get sources
2020-04-19T22:43:23.9194590Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T22:43:23.9194788Z Version      : 1.0.0
2020-04-19T22:43:23.9194931Z Author       : Microsoft
2020-04-19T22:43:23.9194931Z Author       : Microsoft
2020-04-19T22:43:23.9195173Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T22:43:23.9195420Z ==============================================================================
2020-04-19T22:43:24.2309838Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T22:43:24.2352339Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71332/merge to s
2020-04-19T22:43:24.2436341Z Cleaning up task key
2020-04-19T22:43:24.2437832Z Start cleaning up orphan processes.
2020-04-19T22:43:24.2655137Z Terminate orphan process: pid (3748) (python)
2020-04-19T22:43:24.3131561Z ##[section]Finishing: Finalize Job
