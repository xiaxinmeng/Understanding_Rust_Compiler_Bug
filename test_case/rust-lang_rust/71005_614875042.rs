plain
2020-04-16T18:28:29.8275137Z ========================== Starting Command Output ===========================
2020-04-16T18:28:29.8277703Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9b24ad6e-f0b9-4831-b36d-e1e36b9b7d40.sh
2020-04-16T18:28:29.8277907Z 
2020-04-16T18:28:29.8283026Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T18:28:29.8308835Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71005/merge to s
2020-04-16T18:28:29.8311890Z Task         : Get sources
2020-04-16T18:28:29.8312122Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T18:28:29.8312398Z Version      : 1.0.0
2020-04-16T18:28:29.8312554Z Author       : Microsoft
---
2020-04-16T18:28:30.8413627Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T18:28:30.8417968Z ##[command]git config gc.auto 0
2020-04-16T18:28:30.8420943Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T18:28:30.8423689Z ##[command]git config --get-all http.proxy
2020-04-16T18:28:30.8429673Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71005/merge:refs/remotes/pull/71005/merge
---
2020-04-16T18:31:34.9527329Z  ---> f58a2bb1e753
2020-04-16T18:31:34.9528085Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-16T18:31:34.9529647Z  ---> Using cache
2020-04-16T18:31:34.9530131Z  ---> d079cc6b6db8
2020-04-16T18:31:34.9532048Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-16T18:31:34.9532994Z  ---> 4183ca46ee56
2020-04-16T18:31:34.9533189Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-16T18:31:34.9533488Z  ---> Using cache
2020-04-16T18:31:34.9533767Z  ---> 69e7f8a2a2fb
---
2020-04-16T18:31:34.9829712Z Looks like docker image is the same as before, not uploading
2020-04-16T18:31:42.6089460Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T18:31:42.6350054Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T18:31:42.6370642Z == clock drift check ==
2020-04-16T18:31:42.6381421Z   local time: Thu Apr 16 18:31:42 UTC 2020
2020-04-16T18:31:42.8317199Z   network time: Thu, 16 Apr 2020 18:31:42 GMT
2020-04-16T18:31:42.8348813Z Starting sccache server...
2020-04-16T18:31:42.9143715Z configure: processing command line
2020-04-16T18:31:42.9144199Z configure: 
2020-04-16T18:31:42.9145349Z configure: rust.dist-src        := False
---
2020-04-16T18:37:04.4303057Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T18:37:05.9330392Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T18:37:07.5181610Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T18:37:09.6300297Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T18:37:17.6815570Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T18:37:21.4242807Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T18:37:26.0122830Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T18:37:30.4633988Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T18:37:38.9941068Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T19:00:28.5619208Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T19:00:30.3737459Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T19:00:32.4426354Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T19:00:35.0919070Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T19:00:45.3448341Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T19:00:49.5237427Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T19:00:54.8397674Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T19:01:00.3394923Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T19:01:10.6952930Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T19:27:38.1300637Z .................................................................................................... 1700/9897
2020-04-16T19:27:42.6131257Z .................................................................................................... 1800/9897
2020-04-16T19:27:51.2801468Z .................................................................................................... 1900/9897
2020-04-16T19:27:59.7824960Z ......i............................................................................................. 2000/9897
2020-04-16T19:28:06.2886017Z ................................................................................................iiii 2100/9897
2020-04-16T19:28:21.8565361Z i................................................................................................... 2200/9897
2020-04-16T19:28:30.2521008Z .................................................................................................... 2400/9897
2020-04-16T19:28:32.4553495Z .................................................................................................... 2500/9897
2020-04-16T19:28:38.2226819Z .................................................................................................... 2600/9897
2020-04-16T19:28:58.6648604Z .................................................................................................... 2700/9897
---
2020-04-16T19:31:41.9953091Z .................................................................................................... 5100/9897
2020-04-16T19:31:50.0865081Z .................................................................................................... 5200/9897
2020-04-16T19:31:54.7041353Z .................i.................................................................................. 5300/9897
2020-04-16T19:32:04.9968068Z .......i............................................................................................ 5400/9897
2020-04-16T19:32:10.4667777Z .......ii.ii........i...i........................................................................... 5500/9897
2020-04-16T19:32:18.0835867Z .....................................................i.............................................. 5700/9897
2020-04-16T19:32:28.2806985Z .........................................................................ii......................... 5800/9897
2020-04-16T19:32:35.0431651Z ............i....................................................................................... 5900/9897
2020-04-16T19:32:40.4035001Z .................................................................................................... 6000/9897
2020-04-16T19:32:40.4035001Z .................................................................................................... 6000/9897
2020-04-16T19:32:51.1078751Z .................................................................................................... 6100/9897
2020-04-16T19:33:02.3432980Z ......ii...i..ii...........i........................................................................ 6200/9897
2020-04-16T19:33:17.9902941Z .................................................................................................... 6400/9897
2020-04-16T19:33:24.3920878Z .................................................................................................... 6500/9897
2020-04-16T19:33:24.3920878Z .................................................................................................... 6500/9897
2020-04-16T19:33:39.4489207Z ....................................i..ii........................................................... 6600/9897
2020-04-16T19:34:01.1283716Z .................................................................................................... 6800/9897
2020-04-16T19:34:03.1269345Z ....................................i............................................................... 6900/9897
2020-04-16T19:34:05.2242907Z .................................................................................................... 7000/9897
2020-04-16T19:34:07.2617696Z ............................................................................i....................... 7100/9897
---
2020-04-16T19:35:46.6216973Z .................................................................................................... 7800/9897
2020-04-16T19:35:50.6896819Z .................................................................................................... 7900/9897
2020-04-16T19:35:57.1043425Z .................................................................................................... 8000/9897
2020-04-16T19:36:03.2830840Z ...........................................i........................................................ 8100/9897
2020-04-16T19:36:12.6824681Z ..........................................................................................iiiiii.iii 8200/9897
2020-04-16T19:36:18.9224283Z ii.i................................................................................................ 8300/9897
2020-04-16T19:36:31.7601533Z .................................................................................................... 8500/9897
2020-04-16T19:36:41.1936496Z .................................................................................................... 8600/9897
2020-04-16T19:36:54.5138125Z .................................................................................................... 8700/9897
2020-04-16T19:37:01.1681126Z .................................................................................................... 8800/9897
---
2020-04-16T19:39:19.5828794Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-16T19:39:19.6019255Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T19:39:19.7874688Z 
2020-04-16T19:39:19.7875376Z running 185 tests
2020-04-16T19:39:22.2879962Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-16T19:39:24.7697658Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-16T19:39:24.7703260Z 
2020-04-16T19:39:24.7708343Z  finished in 5.169
2020-04-16T19:39:24.7714853Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-16T19:39:24.7885656Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T19:39:26.7524012Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-16T19:39:26.7699118Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T19:39:26.9202736Z 
2020-04-16T19:39:26.9204019Z running 9 tests
2020-04-16T19:39:26.9209872Z iiiiiiiii
2020-04-16T19:39:26.9210712Z 
2020-04-16T19:39:26.9210832Z  finished in 0.150
2020-04-16T19:39:26.9212699Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-16T19:39:26.9374877Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T19:39:45.8909272Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-16T19:39:45.9088944Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T19:39:46.0960090Z 
2020-04-16T19:39:46.0960340Z running 115 tests
2020-04-16T19:39:58.7022453Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-16T19:40:00.3620926Z ...iiii.....ii.
2020-04-16T19:40:00.3625509Z 
2020-04-16T19:40:00.3668884Z  finished in 14.454
2020-04-16T19:40:00.3669761Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T19:40:00.3670682Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-16T19:52:37.1550437Z 
2020-04-16T19:52:37.1551232Z    Doc-tests core
2020-04-16T19:52:41.8151115Z 
2020-04-16T19:52:41.8152292Z running 2496 tests
2020-04-16T19:52:50.5019334Z ......iiiii......................................................................................... 100/2496
2020-04-16T19:52:58.8062348Z .....................................................................................ii............. 200/2496
2020-04-16T19:53:18.3250298Z ....................i............................................................................... 400/2496
2020-04-16T19:53:18.3250298Z ....................i............................................................................... 400/2496
2020-04-16T19:53:27.7833961Z ..........................................................................i..i..................iiii 500/2496
2020-04-16T19:53:43.8808010Z .................................................................................................... 700/2496
2020-04-16T19:53:52.3550353Z .................................................................................................... 800/2496
2020-04-16T19:54:00.9211359Z .................................................................................................... 900/2496
2020-04-16T19:54:10.2747838Z .................................................................................................... 1000/2496
---
2020-04-16T19:57:56.8958108Z 
2020-04-16T19:57:56.8958417Z running 1020 tests
2020-04-16T19:58:15.0151021Z i................................................................................................... 100/1020
2020-04-16T19:58:25.3998843Z .................................................................................................... 200/1020
2020-04-16T19:58:33.3386470Z ...................i.ii.....i......i...i......i..................................................... 300/1020
2020-04-16T19:58:38.3419957Z .................................................................................................... 400/1020
2020-04-16T19:58:45.0481416Z ....................................................i....i......................................ii.. 500/1020
2020-04-16T19:58:57.9432685Z .................................................................................................... 700/1020
2020-04-16T19:58:57.9432685Z .................................................................................................... 700/1020
2020-04-16T19:59:05.4114269Z ..............................................iiii.................................................. 800/1020
2020-04-16T19:59:19.7976999Z .................................................................................................... 900/1020
2020-04-16T19:59:25.8774759Z ....................................................................iiii............................ 1000/1020
2020-04-16T19:59:27.2358424Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-16T19:59:27.2358797Z 
2020-04-16T19:59:27.2461642Z  finished in 174.533
2020-04-16T19:59:27.2468093Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-16T20:02:55.5912938Z 
2020-04-16T20:02:55.5913309Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T20:02:55.5913590Z 
2020-04-16T20:02:55.5972704Z  finished in 0.991
2020-04-16T20:02:55.5974020Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-16T20:02:55.5992088Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T20:02:55.7916993Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T20:02:56.8391174Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-45a917947856ee22
2020-04-16T20:02:56.8414239Z 
2020-04-16T20:02:56.8415560Z running 0 tests
2020-04-16T20:02:56.8415688Z 
---
2020-04-16T20:17:37.6765232Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6766033Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6766658Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6767285Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6767932Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6769381Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6770230Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6770854Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-16T20:17:37.6771736Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-16T20:18:36.7294245Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-16T20:18:36.7295396Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T20:18:36.8194543Z 
2020-04-16T20:18:36.8194990Z running 211 tests
2020-04-16T20:19:07.1919310Z ......................i...ii.......................................................................i 100/211
2020-04-16T20:19:43.3405915Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-16T20:19:48.3549040Z .......ii..
2020-04-16T20:19:48.3552065Z 
2020-04-16T20:19:48.3556168Z  finished in 71.744
2020-04-16T20:19:48.3561043Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-16T20:19:48.3566428Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-16T20:20:01.3669539Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-16T20:20:01.3828990Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-16T20:20:01.5184492Z  finished in 4.012
2020-04-16T20:20:01.5189389Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-16T20:20:02.4232068Z Checking "alias-1" ... OK
2020-04-16T20:20:02.4846528Z Checking "alias-2" ... OK
2020-04-16T20:20:02.5438244Z Checking "alias-3" ... OK
2020-04-16T20:20:02.6097950Z Checking "alias" ... OK
2020-04-16T20:20:02.6834324Z Checking "basic" ... OK
2020-04-16T20:20:02.7562649Z Checking "deduplication" ... OK
2020-04-16T20:20:02.8072332Z Checking "enum-option" ... OK
2020-04-16T20:20:02.8763439Z Checking "filter-crate" ... OK
2020-04-16T20:20:02.9357585Z Checking "fn-forget" ... OK
2020-04-16T20:20:03.0351305Z Checking "from_u" ... OK
2020-04-16T20:20:03.1229660Z Checking "keyword" ... OK
2020-04-16T20:20:03.1668434Z Checking "macro-check" ... OK
2020-04-16T20:20:03.2014123Z Checking "macro-print" ... OK
2020-04-16T20:20:03.2747755Z Checking "multi-query" ... OK
2020-04-16T20:20:03.3025535Z Checking "never" ... OK
2020-04-16T20:20:03.3192663Z Checking "quoted" ... OK
2020-04-16T20:20:03.3423372Z Checking "return-specific-literal" ... OK
2020-04-16T20:20:03.4105266Z Checking "return-specific" ... OK
2020-04-16T20:20:03.4474747Z Checking "should-fail" ... OK
2020-04-16T20:20:03.5141168Z Checking "string-from_ut" ... OK
2020-04-16T20:20:03.5799273Z Checking "struct-vec" ... OK
2020-04-16T20:20:03.6966139Z Checking "vec-new" ... OK
2020-04-16T20:20:03.7236418Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T20:20:03.8862148Z 
2020-04-16T20:20:03.8865133Z running 6 tests
2020-04-16T20:20:09.1472544Z ......
---
2020-04-16T20:21:13.6253660Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-16T20:21:13.6416437Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T20:21:13.7927401Z 
2020-04-16T20:21:13.7927732Z running 13 tests
2020-04-16T20:21:14.4581658Z .iiiiiii.iii.
2020-04-16T20:21:14.4583653Z 
2020-04-16T20:21:14.4587837Z  finished in 0.816
2020-04-16T20:21:14.4635539Z Build completed successfully in 1:47:49
2020-04-16T20:21:14.4635539Z Build completed successfully in 1:47:49
2020-04-16T20:21:14.4642696Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T20:21:15.8360385Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-16T20:21:16.0592550Z     Finished release [optimized] target(s) in 0.21s
2020-04-16T20:21:16.0693685Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-16T20:21:16.0765122Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-16T20:22:39.7477479Z 
2020-04-16T20:22:39.7477878Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-16T20:22:39.7478070Z 30 }
2020-04-16T20:22:39.7478174Z 31 
2020-04-16T20:22:39.7478353Z 32 alloc0 (static: FOO, size: 8, align: 4) {
2020-04-16T20:22:39.7479075Z -     ╾alloc24+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-16T20:22:39.7479545Z +     ╾alloc25+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-16T20:22:39.7479851Z 35 
2020-04-16T20:22:39.7479851Z 35 
2020-04-16T20:22:39.7480164Z - alloc24 (size: 48, align: 4) {
2020-04-16T20:22:39.7480627Z -     0x00 │ 00 00 00 00 __ __ __ __ ╾alloc9+0─╼ 00 00 00 00 │ ....░░░░╾──╼....
2020-04-16T20:22:39.7481153Z -     0x10 │ 00 00 00 00 __ __ __ __ ╾alloc14+0╼ 02 00 00 00 │ ....░░░░╾──╼....
2020-04-16T20:22:39.7482108Z -     0x20 │ 01 00 00 00 2a 00 00 00 ╾alloc22+0╼ 03 00 00 00 │ ....*...╾──╼....
2020-04-16T20:22:39.7482381Z + alloc25 (size: 48, align: 4) {
2020-04-16T20:22:39.7482910Z +     0x00 │ 00 00 00 00 __ __ __ __ ╾alloc10+0╼ 00 00 00 00 │ ....░░░░╾──╼....
2020-04-16T20:22:39.7483409Z +     0x10 │ 00 00 00 00 __ __ __ __ ╾alloc15+0╼ 02 00 00 00 │ ....░░░░╾──╼....
2020-04-16T20:22:39.7483899Z +     0x20 │ 01 00 00 00 2a 00 00 00 ╾alloc23+0╼ 03 00 00 00 │ ....*...╾──╼....
2020-04-16T20:22:39.7484212Z 41 
2020-04-16T20:22:39.7484212Z 41 
2020-04-16T20:22:39.7484506Z - alloc9 (size: 0, align: 4) {}
2020-04-16T20:22:39.7484723Z + alloc10 (size: 0, align: 4) {}
2020-04-16T20:22:39.7484870Z 43 
2020-04-16T20:22:39.7485790Z - alloc14 (size: 8, align: 4) {
2020-04-16T20:22:39.7487605Z -     ╾alloc12+0╼ ╾alloc13+0╼                         │ ╾──╼╾──╼
2020-04-16T20:22:39.7488012Z + alloc15 (size: 8, align: 4) {
2020-04-16T20:22:39.7488532Z +     ╾alloc13+0╼ ╾alloc14+0╼                         │ ╾──╼╾──╼
2020-04-16T20:22:39.7490161Z 47 
2020-04-16T20:22:39.7490161Z 47 
2020-04-16T20:22:39.7490540Z - alloc12 (size: 1, align: 1) {
2020-04-16T20:22:39.7490745Z + alloc13 (size: 1, align: 1) {
2020-04-16T20:22:39.7491835Z 50 }
2020-04-16T20:22:39.7491937Z 51 
2020-04-16T20:22:39.7492023Z 
2020-04-16T20:22:39.7492023Z 
2020-04-16T20:22:39.7492537Z - alloc13 (size: 1, align: 1) {
2020-04-16T20:22:39.7492733Z + alloc14 (size: 1, align: 1) {
2020-04-16T20:22:39.7493823Z 54 }
2020-04-16T20:22:39.7493924Z 55 
2020-04-16T20:22:39.7494007Z 
2020-04-16T20:22:39.7494007Z 
2020-04-16T20:22:39.7494324Z - alloc22 (size: 12, align: 4) {
2020-04-16T20:22:39.7494770Z -     ╾alloc18+3╼ ╾alloc19+0╼ ╾alloc21+2╼             │ ╾──╼╾──╼╾──╼
2020-04-16T20:22:39.7495012Z + alloc23 (size: 12, align: 4) {
2020-04-16T20:22:39.7495619Z +     ╾alloc19+3╼ ╾alloc20+0╼ ╾alloc22+2╼             │ ╾──╼╾──╼╾──╼
2020-04-16T20:22:39.7496075Z 59 
2020-04-16T20:22:39.7496075Z 59 
2020-04-16T20:22:39.7496414Z - alloc18 (size: 4, align: 1) {
2020-04-16T20:22:39.7496640Z + alloc19 (size: 4, align: 1) {
2020-04-16T20:22:39.7497738Z 61     2a 45 15 6f                                     │ *E.o
2020-04-16T20:22:39.7498058Z 63 
2020-04-16T20:22:39.7498146Z 
2020-04-16T20:22:39.7498146Z 
2020-04-16T20:22:39.7498661Z - alloc19 (size: 1, align: 1) {
2020-04-16T20:22:39.7499046Z + alloc20 (size: 1, align: 1) {
2020-04-16T20:22:39.7499981Z 66 }
2020-04-16T20:22:39.7501868Z 67 
2020-04-16T20:22:39.7501981Z 
2020-04-16T20:22:39.7501981Z 
2020-04-16T20:22:39.7502354Z - alloc21 (size: 4, align: 1) {
2020-04-16T20:22:39.7502552Z + alloc22 (size: 4, align: 1) {
2020-04-16T20:22:39.7502944Z 69     2a 45 15 6f                                     │ *E.o
2020-04-16T20:22:39.7503239Z 71 
2020-04-16T20:22:39.7503319Z 
2020-04-16T20:22:39.7503319Z 
2020-04-16T20:22:39.7504615Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-16T20:22:39.7506143Z 
2020-04-16T20:22:39.7506249Z 
2020-04-16T20:22:39.7506377Z failures:
2020-04-16T20:22:39.7506937Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-16T20:22:39.7506937Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-16T20:22:39.7507256Z 
2020-04-16T20:22:39.7507750Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-16T20:22:39.7507986Z 
2020-04-16T20:22:39.7508594Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-16T20:22:39.7509021Z 
2020-04-16T20:22:39.7509123Z 
2020-04-16T20:22:39.7512702Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-16T20:22:39.7515123Z 
2020-04-16T20:22:39.7515201Z 
2020-04-16T20:22:39.7515201Z 
2020-04-16T20:22:39.7515741Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-16T20:22:39.7555494Z == clock drift check ==
2020-04-16T20:22:39.7555494Z == clock drift check ==
2020-04-16T20:22:39.7571316Z   local time: Thu Apr 16 20:22:39 UTC 2020
2020-04-16T20:22:39.9540400Z   network time: Thu, 16 Apr 2020 20:22:39 GMT
2020-04-16T20:22:42.2783900Z 
2020-04-16T20:22:42.2783900Z 
2020-04-16T20:22:42.2853375Z ##[error]Bash exited with code '1'.
2020-04-16T20:22:42.2867668Z ##[section]Finishing: Run build
2020-04-16T20:22:42.2914703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71005/merge to s
2020-04-16T20:22:42.2921004Z Task         : Get sources
2020-04-16T20:22:42.2921285Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T20:22:42.2921701Z Version      : 1.0.0
2020-04-16T20:22:42.2922104Z Author       : Microsoft
2020-04-16T20:22:42.2922104Z Author       : Microsoft
2020-04-16T20:22:42.2922449Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T20:22:42.2922772Z ==============================================================================
2020-04-16T20:22:42.6446299Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T20:22:42.6488622Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71005/merge to s
2020-04-16T20:22:42.6574666Z Cleaning up task key
2020-04-16T20:22:42.6575929Z Start cleaning up orphan processes.
2020-04-16T20:22:42.6748790Z Terminate orphan process: pid (3663) (python)
2020-04-16T20:22:42.7200658Z ##[section]Finishing: Finalize Job
