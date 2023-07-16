plain
2020-04-17T19:52:26.7395263Z ========================== Starting Command Output ===========================
2020-04-17T19:52:26.7397613Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4c464d21-8e73-4c13-a619-50cb55da92dc.sh
2020-04-17T19:52:26.7397862Z 
2020-04-17T19:52:26.7401421Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T19:52:26.7419804Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-17T19:52:26.7422852Z Task         : Get sources
2020-04-17T19:52:26.7423145Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T19:52:26.7423419Z Version      : 1.0.0
2020-04-17T19:52:26.7423604Z Author       : Microsoft
---
2020-04-17T19:52:27.7348981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T19:52:27.7355192Z ##[command]git config gc.auto 0
2020-04-17T19:52:27.7364907Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T19:52:27.7368707Z ##[command]git config --get-all http.proxy
2020-04-17T19:52:27.7375818Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-17T19:55:32.5495396Z  ---> f58a2bb1e753
2020-04-17T19:55:32.5499113Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-17T19:55:32.5499830Z  ---> Using cache
2020-04-17T19:55:32.5500153Z  ---> d079cc6b6db8
2020-04-17T19:55:32.5501054Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-17T19:55:32.5506556Z  ---> 4183ca46ee56
2020-04-17T19:55:32.5506784Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-17T19:55:32.5509562Z  ---> Using cache
2020-04-17T19:55:32.5510003Z  ---> 69e7f8a2a2fb
---
2020-04-17T19:55:32.5914563Z Looks like docker image is the same as before, not uploading
2020-04-17T19:55:39.8901257Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T19:55:39.9148005Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T19:55:39.9180851Z == clock drift check ==
2020-04-17T19:55:39.9189847Z   local time: Fri Apr 17 19:55:39 UTC 2020
2020-04-17T19:55:40.0113645Z   network time: Fri, 17 Apr 2020 19:55:40 GMT
2020-04-17T19:55:40.0135209Z Starting sccache server...
2020-04-17T19:55:40.0964055Z configure: processing command line
2020-04-17T19:55:40.0964706Z configure: 
2020-04-17T19:55:40.0966360Z configure: rust.dist-src        := False
---
2020-04-17T20:00:45.6716775Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T20:00:47.0635025Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T20:00:48.5674938Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T20:00:49.5756790Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T20:00:58.1267218Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T20:01:00.3275060Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T20:01:04.5621816Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T20:01:08.5670188Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T20:01:17.6551686Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T20:22:55.7479438Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T20:22:57.4262303Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T20:22:59.3517948Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T20:23:00.7067185Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T20:23:11.3104803Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T20:23:13.9010602Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T20:23:18.9954343Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T20:23:24.2272478Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T20:23:35.3362596Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T20:48:27.2044011Z .................................................................................................... 1700/9903
2020-04-17T20:48:31.9733681Z .................................................................................................... 1800/9903
2020-04-17T20:48:40.1378972Z .................................................................................................... 1900/9903
2020-04-17T20:48:49.0253516Z .........i.......................................................................................... 2000/9903
2020-04-17T20:48:55.6618523Z ...................................................................................................i 2100/9903
2020-04-17T20:49:10.0411731Z iiii................................................................................................ 2200/9903
2020-04-17T20:49:18.9802246Z .................................................................................................... 2400/9903
2020-04-17T20:49:21.2691276Z .................................................................................................... 2500/9903
2020-04-17T20:49:26.8151651Z .................................................................................................... 2600/9903
2020-04-17T20:49:45.9371653Z .................................................................................................... 2700/9903
---
2020-04-17T20:52:24.2334544Z .................................................................................................... 5100/9903
2020-04-17T20:52:31.8434413Z .................................................................................................... 5200/9903
2020-04-17T20:52:36.3801654Z .....................i.............................................................................. 5300/9903
2020-04-17T20:52:46.2876774Z ...........i........................................................................................ 5400/9903
2020-04-17T20:52:51.8924523Z ...........ii.ii........i...i....................................................................... 5500/9903
2020-04-17T20:52:59.4912200Z .........................................................i.......................................... 5700/9903
2020-04-17T20:53:08.6912689Z .............................................................................ii..................... 5800/9903
2020-04-17T20:53:15.5419402Z ................i................................................................................... 5900/9903
2020-04-17T20:53:21.0020682Z .................................................................................................... 6000/9903
2020-04-17T20:53:21.0020682Z .................................................................................................... 6000/9903
2020-04-17T20:53:31.4110532Z .................................................................................................... 6100/9903
2020-04-17T20:53:41.7559695Z ..........ii...i..ii...........i.................................................................... 6200/9903
2020-04-17T20:53:55.5905580Z .................................................................................................... 6400/9903
2020-04-17T20:53:59.0225080Z .................................................................................................... 6500/9903
2020-04-17T20:53:59.0225080Z .................................................................................................... 6500/9903
2020-04-17T20:54:10.4209721Z ........................................i..ii....................................................... 6600/9903
2020-04-17T20:54:31.7614217Z .................................................................................................... 6800/9903
2020-04-17T20:54:33.7922860Z .........................................i.......................................................... 6900/9903
2020-04-17T20:54:35.7971120Z .................................................................................................... 7000/9903
2020-04-17T20:54:37.8763565Z .................................................................................i.................. 7100/9903
---
2020-04-17T20:56:11.6723245Z .................................................................................................... 7800/9903
2020-04-17T20:56:15.9332051Z .................................................................................................... 7900/9903
2020-04-17T20:56:22.3326670Z .................................................................................................... 8000/9903
2020-04-17T20:56:28.2048254Z ...............................................i.................................................... 8100/9903
2020-04-17T20:56:37.8489543Z ...............................................................................................iiiii 8200/9903
2020-04-17T20:56:43.4852808Z i.iiiii..i.......................................................................................... 8300/9903
2020-04-17T20:56:56.3335350Z .................................................................................................... 8500/9903
2020-04-17T20:57:04.4379920Z .................................................................................................... 8600/9903
2020-04-17T20:57:18.1778925Z .................................................................................................... 8700/9903
2020-04-17T20:57:24.7437263Z .................................................................................................... 8800/9903
---
2020-04-17T20:59:39.7324923Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-17T20:59:39.7508979Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T20:59:39.9492417Z 
2020-04-17T20:59:39.9496855Z running 185 tests
2020-04-17T20:59:42.6258059Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-17T20:59:45.1707665Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-17T20:59:45.1710130Z 
2020-04-17T20:59:45.1715388Z  finished in 5.420
2020-04-17T20:59:45.1733882Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-17T20:59:45.1936884Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T20:59:47.2750373Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-17T20:59:47.2923493Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T20:59:47.4437692Z 
2020-04-17T20:59:47.4438080Z running 9 tests
2020-04-17T20:59:47.4439187Z iiiiiiiii
2020-04-17T20:59:47.4440271Z 
2020-04-17T20:59:47.4440435Z  finished in 0.150
2020-04-17T20:59:47.4499475Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-17T20:59:47.4631825Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T21:00:07.5106706Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-17T21:00:07.5325971Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:00:07.7151786Z 
2020-04-17T21:00:07.7152091Z running 115 tests
2020-04-17T21:00:20.6311664Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-17T21:00:22.3375007Z ...iiii.....ii.
2020-04-17T21:00:22.3376902Z 
2020-04-17T21:00:22.3382642Z  finished in 14.805
2020-04-17T21:00:22.3388165Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T21:00:22.3392163Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T21:12:18.8464672Z 
2020-04-17T21:12:18.8466144Z    Doc-tests core
2020-04-17T21:12:23.3142132Z 
2020-04-17T21:12:23.3143216Z running 2496 tests
2020-04-17T21:12:32.0599819Z ......iiiii......................................................................................... 100/2496
2020-04-17T21:12:40.4687633Z .....................................................................................ii............. 200/2496
2020-04-17T21:12:59.8880934Z ....................i............................................................................... 400/2496
2020-04-17T21:12:59.8880934Z ....................i............................................................................... 400/2496
2020-04-17T21:13:09.6450929Z ..........................................................................i..i..................iiii 500/2496
2020-04-17T21:13:25.9388039Z .................................................................................................... 700/2496
2020-04-17T21:13:34.4017039Z .................................................................................................... 800/2496
2020-04-17T21:13:42.7136765Z .................................................................................................... 900/2496
2020-04-17T21:13:50.9316867Z .................................................................................................... 1000/2496
---
2020-04-17T21:17:16.1355440Z 
2020-04-17T21:17:16.1355784Z running 1020 tests
2020-04-17T21:17:32.5401128Z i................................................................................................... 100/1020
2020-04-17T21:17:42.2631276Z .................................................................................................... 200/1020
2020-04-17T21:17:49.5675541Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-17T21:17:54.1329500Z .................................................................................................... 400/1020
2020-04-17T21:18:00.4644859Z ....................................................i....i......................................ii.. 500/1020
2020-04-17T21:18:12.3946610Z .................................................................................................... 700/1020
2020-04-17T21:18:12.3946610Z .................................................................................................... 700/1020
2020-04-17T21:18:19.1457860Z ..............................................iiii.................................................. 800/1020
2020-04-17T21:18:32.2334128Z .................................................................................................... 900/1020
2020-04-17T21:18:38.8973089Z ....................................................................iiii............................ 1000/1020
2020-04-17T21:18:39.2436555Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-17T21:18:39.2436771Z 
2020-04-17T21:18:39.2549933Z  finished in 156.909
2020-04-17T21:18:39.2557865Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-17T21:21:40.0013522Z 
2020-04-17T21:21:40.0013810Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-17T21:21:40.0014054Z 
2020-04-17T21:21:40.0083956Z  finished in 0.932
2020-04-17T21:21:40.0096368Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-17T21:21:40.0138924Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:21:40.1972036Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T21:21:41.1529809Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-cb468390e53b0165
2020-04-17T21:21:41.1559474Z 
2020-04-17T21:21:41.1559698Z running 0 tests
2020-04-17T21:21:41.1559808Z 
---
2020-04-17T21:35:28.8717553Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8718265Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8719059Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8719782Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8720517Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8721958Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8722669Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8723367Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T21:35:28.8724112Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-17T21:36:29.6915274Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T21:36:29.7266558Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:36:29.9375595Z 
2020-04-17T21:36:29.9375935Z running 211 tests
2020-04-17T21:37:00.3751910Z ......................i...ii.......................................................................i 100/211
2020-04-17T21:37:40.2354876Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-17T21:37:41.4546162Z .......ii..
2020-04-17T21:37:41.4548347Z 
2020-04-17T21:37:41.4552444Z  finished in 71.728
2020-04-17T21:37:41.4558469Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-17T21:37:41.4563371Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-17T21:37:55.1352151Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-17T21:37:55.1534908Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-17T21:37:55.3044644Z  finished in 4.261
2020-04-17T21:37:55.3052588Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-17T21:37:56.2194428Z Checking "alias-1" ... OK
2020-04-17T21:37:56.2868199Z Checking "alias-2" ... OK
2020-04-17T21:37:56.3466182Z Checking "alias-3" ... OK
2020-04-17T21:37:56.4125537Z Checking "alias" ... OK
2020-04-17T21:37:56.4901972Z Checking "basic" ... OK
2020-04-17T21:37:56.5683457Z Checking "deduplication" ... OK
2020-04-17T21:37:56.6196677Z Checking "enum-option" ... OK
2020-04-17T21:37:56.6882881Z Checking "filter-crate" ... OK
2020-04-17T21:37:56.7551347Z Checking "fn-forget" ... OK
2020-04-17T21:37:56.8601751Z Checking "from_u" ... OK
2020-04-17T21:37:56.9477046Z Checking "keyword" ... OK
2020-04-17T21:37:56.9938405Z Checking "macro-check" ... OK
2020-04-17T21:37:57.0271697Z Checking "macro-print" ... OK
2020-04-17T21:37:57.1016303Z Checking "multi-query" ... OK
2020-04-17T21:37:57.1293583Z Checking "never" ... OK
2020-04-17T21:37:57.1448048Z Checking "quoted" ... OK
2020-04-17T21:37:57.1719222Z Checking "return-specific-literal" ... OK
2020-04-17T21:37:57.2437439Z Checking "return-specific" ... OK
2020-04-17T21:37:57.2800895Z Checking "should-fail" ... OK
2020-04-17T21:37:57.3470654Z Checking "string-from_ut" ... OK
2020-04-17T21:37:57.4078552Z Checking "struct-vec" ... OK
2020-04-17T21:37:57.5194015Z Checking "vec-new" ... OK
2020-04-17T21:37:57.5448811Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:37:57.7024809Z 
2020-04-17T21:37:57.7026212Z running 6 tests
2020-04-17T21:38:03.4644562Z ......
---
2020-04-17T21:39:04.7636087Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-17T21:39:04.7831587Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:39:04.9372793Z 
2020-04-17T21:39:04.9374057Z running 13 tests
2020-04-17T21:39:05.3467687Z .iiiiiii.iii.
2020-04-17T21:39:05.3468673Z 
2020-04-17T21:39:05.3472162Z  finished in 0.563
2020-04-17T21:39:05.3529825Z Build completed successfully in 1:41:45
2020-04-17T21:39:05.3529825Z Build completed successfully in 1:41:45
2020-04-17T21:39:05.3540012Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-17T21:39:06.6888487Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T21:39:06.9224116Z     Finished release [optimized] target(s) in 0.22s
2020-04-17T21:39:06.9324260Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-17T21:39:06.9432051Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T21:40:27.0314193Z 
2020-04-17T21:40:27.0314801Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-17T21:40:27.0315021Z 30 }
2020-04-17T21:40:27.0315143Z 31 
2020-04-17T21:40:27.0315353Z 32 alloc0 (static: FOO, size: 8, align: 4) {
2020-04-17T21:40:27.0315904Z -     ╾alloc24+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-17T21:40:27.0316404Z +     ╾alloc20+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-17T21:40:27.0316745Z 35 
2020-04-17T21:40:27.0316745Z 35 
2020-04-17T21:40:27.0318816Z - alloc24 (size: 48, align: 4) {
2020-04-17T21:40:27.0320898Z -     0x00 │ 00 00 00 00 __ __ __ __ ╾alloc9+0─╼ 00 00 00 00 │ ....░░░░╾──╼....
2020-04-17T21:40:27.0321508Z -     0x10 │ 00 00 00 00 __ __ __ __ ╾alloc14+0╼ 02 00 00 00 │ ....░░░░╾──╼....
2020-04-17T21:40:27.0322072Z -     0x20 │ 01 00 00 00 2a 00 00 00 ╾alloc22+0╼ 03 00 00 00 │ ....*...╾──╼....
2020-04-17T21:40:27.0322361Z + alloc20 (size: 48, align: 4) {
2020-04-17T21:40:27.0322889Z +     0x00 │ 00 00 00 00 __ __ __ __ ╾alloc4+0─╼ 00 00 00 00 │ ....░░░░╾──╼....
2020-04-17T21:40:27.0323451Z +     0x10 │ 00 00 00 00 __ __ __ __ ╾alloc9+0─╼ 02 00 00 00 │ ....░░░░╾──╼....
2020-04-17T21:40:27.0324008Z +     0x20 │ 01 00 00 00 2a 00 00 00 ╾alloc18+0╼ 03 00 00 00 │ ....*...╾──╼....
2020-04-17T21:40:27.0325912Z 41 
2020-04-17T21:40:27.0325912Z 41 
2020-04-17T21:40:27.0326333Z - alloc9 (size: 0, align: 4) {}
2020-04-17T21:40:27.0326579Z + alloc4 (size: 0, align: 4) {}
2020-04-17T21:40:27.0326751Z 43 
2020-04-17T21:40:27.0327872Z - alloc14 (size: 8, align: 4) {
2020-04-17T21:40:27.0328411Z -     ╾alloc12+0╼ ╾alloc13+0╼                         │ ╾──╼╾──╼
2020-04-17T21:40:27.0331064Z + alloc9 (size: 8, align: 4) {
2020-04-17T21:40:27.0331676Z +     ╾alloc7+0─╼ ╾alloc8+0─╼                         │ ╾──╼╾──╼
2020-04-17T21:40:27.0332026Z 47 
2020-04-17T21:40:27.0332026Z 47 
2020-04-17T21:40:27.0332363Z - alloc12 (size: 1, align: 1) {
2020-04-17T21:40:27.0332589Z + alloc7 (size: 1, align: 1) {
2020-04-17T21:40:27.0333245Z 50 }
2020-04-17T21:40:27.0333362Z 51 
2020-04-17T21:40:27.0333474Z 
2020-04-17T21:40:27.0333474Z 
2020-04-17T21:40:27.0333804Z - alloc13 (size: 1, align: 1) {
2020-04-17T21:40:27.0334029Z + alloc8 (size: 1, align: 1) {
2020-04-17T21:40:27.0334703Z 54 }
2020-04-17T21:40:27.0334821Z 55 
2020-04-17T21:40:27.0334916Z 
2020-04-17T21:40:27.0334916Z 
2020-04-17T21:40:27.0335261Z - alloc22 (size: 12, align: 4) {
2020-04-17T21:40:27.0335768Z -     ╾alloc18+3╼ ╾alloc19+0╼ ╾alloc21+2╼             │ ╾──╼╾──╼╾──╼
2020-04-17T21:40:27.0336039Z + alloc18 (size: 12, align: 4) {
2020-04-17T21:40:27.0336544Z +     ╾alloc14+3╼ ╾alloc15+0╼ ╾alloc17+2╼             │ ╾──╼╾──╼╾──╼
2020-04-17T21:40:27.0336876Z 59 
2020-04-17T21:40:27.0336876Z 59 
2020-04-17T21:40:27.0337408Z - alloc18 (size: 4, align: 1) {
2020-04-17T21:40:27.0338575Z + alloc14 (size: 4, align: 1) {
2020-04-17T21:40:27.0339062Z 61     2a 45 15 6f                                     │ *E.o
2020-04-17T21:40:27.0339404Z 63 
2020-04-17T21:40:27.0339500Z 
2020-04-17T21:40:27.0339500Z 
2020-04-17T21:40:27.0339828Z - alloc19 (size: 1, align: 1) {
2020-04-17T21:40:27.0340053Z + alloc15 (size: 1, align: 1) {
2020-04-17T21:40:27.0340724Z 66 }
2020-04-17T21:40:27.0340839Z 67 
2020-04-17T21:40:27.0340948Z 
2020-04-17T21:40:27.0340948Z 
2020-04-17T21:40:27.0341276Z - alloc21 (size: 4, align: 1) {
2020-04-17T21:40:27.0341512Z + alloc17 (size: 4, align: 1) {
2020-04-17T21:40:27.0341969Z 69     2a 45 15 6f                                     │ *E.o
2020-04-17T21:40:27.0342589Z 71 
2020-04-17T21:40:27.0343188Z 
2020-04-17T21:40:27.0343188Z 
2020-04-17T21:40:27.0344115Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-17T21:40:27.0344909Z 
2020-04-17T21:40:27.0345013Z 
2020-04-17T21:40:27.0345139Z failures:
2020-04-17T21:40:27.0345501Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-17T21:40:27.0345501Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-17T21:40:27.0345722Z 
2020-04-17T21:40:27.0346206Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-17T21:40:27.0346432Z 
2020-04-17T21:40:27.0346892Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-17T21:40:27.0349747Z 
2020-04-17T21:40:27.0350150Z 
2020-04-17T21:40:27.0354152Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-17T21:40:27.0356650Z 
2020-04-17T21:40:27.0356742Z 
2020-04-17T21:40:27.0356742Z 
2020-04-17T21:40:27.0357369Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-17T21:40:27.0407632Z == clock drift check ==
2020-04-17T21:40:27.0407632Z == clock drift check ==
2020-04-17T21:40:27.0425307Z   local time: Fri Apr 17 21:40:27 UTC 2020
2020-04-17T21:40:27.3355832Z   network time: Fri, 17 Apr 2020 21:40:27 GMT
2020-04-17T21:40:29.8923613Z 
2020-04-17T21:40:29.8923613Z 
2020-04-17T21:40:29.8996481Z ##[error]Bash exited with code '1'.
2020-04-17T21:40:29.9010037Z ##[section]Finishing: Run build
2020-04-17T21:40:29.9056771Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-17T21:40:29.9062065Z Task         : Get sources
2020-04-17T21:40:29.9062381Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T21:40:29.9062661Z Version      : 1.0.0
2020-04-17T21:40:29.9062881Z Author       : Microsoft
2020-04-17T21:40:29.9062881Z Author       : Microsoft
2020-04-17T21:40:29.9063207Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T21:40:29.9063578Z ==============================================================================
2020-04-17T21:40:30.2291043Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T21:40:30.2354935Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-17T21:40:30.2449869Z Cleaning up task key
2020-04-17T21:40:30.2451091Z Start cleaning up orphan processes.
2020-04-17T21:40:30.2631746Z Terminate orphan process: pid (3591) (python)
2020-04-17T21:40:30.2835228Z ##[section]Finishing: Finalize Job
