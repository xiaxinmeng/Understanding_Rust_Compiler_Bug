plain
2020-04-14T01:41:31.5634859Z ========================== Starting Command Output ===========================
2020-04-14T01:41:31.5637397Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/31c673d9-6f28-45a0-b98b-8f814c9246eb.sh
2020-04-14T01:41:31.5637624Z 
2020-04-14T01:41:31.5641335Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T01:41:31.5661331Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70755/merge to s
2020-04-14T01:41:31.5664673Z Task         : Get sources
2020-04-14T01:41:31.5664939Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T01:41:31.5665197Z Version      : 1.0.0
2020-04-14T01:41:31.5665372Z Author       : Microsoft
---
2020-04-14T01:41:32.7785198Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T01:41:32.7795317Z ##[command]git config gc.auto 0
2020-04-14T01:41:32.7801716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T01:41:32.7807535Z ##[command]git config --get-all http.proxy
2020-04-14T01:41:32.7819200Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70755/merge:refs/remotes/pull/70755/merge
---
2020-04-14T01:43:41.7836441Z  ---> f58a2bb1e753
2020-04-14T01:43:41.7837662Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T01:43:41.7845181Z  ---> Using cache
2020-04-14T01:43:41.7845601Z  ---> d079cc6b6db8
2020-04-14T01:43:41.7846517Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T01:43:41.7847484Z  ---> 4183ca46ee56
2020-04-14T01:43:41.7847698Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T01:43:41.7852283Z  ---> Using cache
2020-04-14T01:43:41.7852618Z  ---> 69e7f8a2a2fb
---
2020-04-14T01:43:41.8208184Z Looks like docker image is the same as before, not uploading
2020-04-14T01:43:48.3204617Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:43:48.3209870Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:43:48.3210338Z == clock drift check ==
2020-04-14T01:43:48.3210679Z   local time: Tue Apr 14 01:43:48 UTC 2020
2020-04-14T01:43:48.3211062Z   network time: Tue, 14 Apr 2020 01:43:48 GMT
2020-04-14T01:43:48.3211659Z Starting sccache server...
2020-04-14T01:43:48.3211969Z configure: processing command line
2020-04-14T01:43:48.3212239Z configure: 
2020-04-14T01:43:48.3212772Z configure: rust.dist-src        := False
---
2020-04-14T01:49:21.0091866Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T01:49:22.6096363Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T01:49:24.3321430Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T01:49:26.4527008Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T01:49:35.3425231Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T01:49:38.3099554Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T01:49:43.0340229Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T01:49:47.4364230Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T01:49:57.2068027Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T02:13:58.4674125Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T02:14:00.3819433Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T02:14:02.5005718Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T02:14:05.0654308Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T02:14:15.7345925Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T02:14:19.7696950Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T02:14:25.4397329Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T02:14:31.2687586Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T02:14:42.3500379Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T02:41:50.2449961Z .................................................................................................... 1700/9891
2020-04-14T02:41:54.6350199Z .................................................................................................... 1800/9891
2020-04-14T02:42:03.4285607Z .................................................................................................... 1900/9891
2020-04-14T02:42:11.7476368Z ....i............................................................................................... 2000/9891
2020-04-14T02:42:18.2266481Z ..............................................................................................iiiii. 2100/9891
2020-04-14T02:42:40.6992697Z .................................................................................................... 2300/9891
2020-04-14T02:42:43.1091462Z .................................................................................................... 2400/9891
2020-04-14T02:42:45.4193590Z .................................................................................................... 2500/9891
2020-04-14T02:42:51.6135028Z .................................................................................................... 2600/9891
---
2020-04-14T02:45:59.2778536Z .................................................................................................... 5100/9891
2020-04-14T02:46:07.1989155Z .................................................................................................... 5200/9891
2020-04-14T02:46:12.4709660Z ..............i..................................................................................... 5300/9891
2020-04-14T02:46:22.6037979Z ....i............................................................................................... 5400/9891
2020-04-14T02:46:28.2520505Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-14T02:46:36.6583731Z ..................................................i................................................. 5700/9891
2020-04-14T02:46:47.4696290Z ......................................................................ii............................ 5800/9891
2020-04-14T02:46:54.5255991Z .........i.......................................................................................... 5900/9891
2020-04-14T02:47:00.5019370Z .................................................................................................... 6000/9891
2020-04-14T02:47:00.5019370Z .................................................................................................... 6000/9891
2020-04-14T02:47:11.3060581Z .................................................................................................... 6100/9891
2020-04-14T02:47:22.9270855Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-14T02:47:38.9256106Z .................................................................................................... 6400/9891
2020-04-14T02:47:45.4663224Z .................................................................................................... 6500/9891
2020-04-14T02:47:45.4663224Z .................................................................................................... 6500/9891
2020-04-14T02:48:02.4525681Z .................................i..ii.............................................................. 6600/9891
2020-04-14T02:48:24.7828872Z .................................................................................................... 6800/9891
2020-04-14T02:48:27.0177341Z .................................i.................................................................. 6900/9891
2020-04-14T02:48:29.3446187Z .................................................................................................... 7000/9891
2020-04-14T02:48:31.6931920Z ........................................................................i........................... 7100/9891
---
2020-04-14T02:50:14.1602335Z .................................................................................................... 7800/9891
2020-04-14T02:50:18.3358999Z .................................................................................................... 7900/9891
2020-04-14T02:50:25.0776218Z .................................................................................................... 8000/9891
2020-04-14T02:50:31.7615997Z ......................................i............................................................. 8100/9891
2020-04-14T02:50:41.2646960Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-14T02:50:57.4807898Z ................................i......i............................................................ 8400/9891
2020-04-14T02:51:00.9631374Z .................................................................................................... 8500/9891
2020-04-14T02:51:11.5409002Z .................................................................................................... 8600/9891
2020-04-14T02:51:24.8984533Z .................................................................................................... 8700/9891
---
2020-04-14T02:53:55.2055305Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-14T02:53:55.2254031Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T02:53:55.4272072Z 
2020-04-14T02:53:55.4272534Z running 185 tests
2020-04-14T02:53:58.2372669Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-14T02:54:00.9737988Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-14T02:54:00.9742911Z 
2020-04-14T02:54:00.9743051Z  finished in 5.749
2020-04-14T02:54:00.9776618Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-14T02:54:00.9967519Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T02:54:03.1974882Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-14T02:54:03.2152694Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T02:54:03.3711865Z 
2020-04-14T02:54:03.3712422Z running 9 tests
2020-04-14T02:54:03.3714146Z iiiiiiiii
2020-04-14T02:54:03.3716409Z 
2020-04-14T02:54:03.3716786Z  finished in 0.155
2020-04-14T02:54:03.3721221Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-14T02:54:03.3933336Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T02:54:24.5227143Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-14T02:54:24.5449048Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T02:54:24.7374079Z 
2020-04-14T02:54:24.7374512Z running 115 tests
2020-04-14T02:54:38.7329219Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-14T02:54:40.5740882Z ...iiii.....ii.
2020-04-14T02:54:40.5748929Z 
2020-04-14T02:54:40.5753318Z  finished in 16.030
2020-04-14T02:54:40.5758548Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T02:54:40.5762003Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T03:07:49.3579756Z 
2020-04-14T03:07:49.3583647Z    Doc-tests core
2020-04-14T03:07:54.1995352Z 
2020-04-14T03:07:54.1999983Z running 2494 tests
2020-04-14T03:08:03.7579393Z ......iiiii......................................................................................... 100/2494
2020-04-14T03:08:13.4066089Z .....................................................................................ii............. 200/2494
2020-04-14T03:08:35.1585630Z ....................i............................................................................... 400/2494
2020-04-14T03:08:35.1585630Z ....................i............................................................................... 400/2494
2020-04-14T03:08:45.6281116Z ..........................................................................i..i..................iiii 500/2494
2020-04-14T03:09:02.7519573Z .................................................................................................... 700/2494
2020-04-14T03:09:12.2044568Z .................................................................................................... 800/2494
2020-04-14T03:09:20.9339607Z .................................................................................................... 900/2494
2020-04-14T03:09:29.6437018Z .................................................................................................... 1000/2494
---
2020-04-14T03:13:09.9017138Z 
2020-04-14T03:13:09.9017468Z running 1020 tests
2020-04-14T03:13:27.9837408Z i................................................................................................... 100/1020
2020-04-14T03:13:38.4841188Z .................................................................................................... 200/1020
2020-04-14T03:13:46.3017701Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-14T03:13:51.1606968Z .................................................................................................... 400/1020
2020-04-14T03:13:57.9086007Z ....................................................i....i......................................ii.. 500/1020
2020-04-14T03:14:10.7638669Z .................................................................................................... 700/1020
2020-04-14T03:14:10.7638669Z .................................................................................................... 700/1020
2020-04-14T03:14:17.8605949Z ..............................................iiii.................................................. 800/1020
2020-04-14T03:14:31.8091508Z .................................................................................................... 900/1020
2020-04-14T03:14:37.9706242Z ....................................................................iiii............................ 1000/1020
2020-04-14T03:14:39.2990749Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-14T03:14:39.2990982Z 
2020-04-14T03:14:39.3125862Z  finished in 169.783
2020-04-14T03:14:39.3126809Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T03:17:58.0118681Z 
2020-04-14T03:17:58.0118891Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T03:17:58.0119080Z 
2020-04-14T03:17:58.0184731Z  finished in 0.986
2020-04-14T03:17:58.0190594Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-14T03:17:58.0209877Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T03:17:58.2215128Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T03:17:59.2817273Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-1f93b529f1817c7e
2020-04-14T03:17:59.2846061Z 
2020-04-14T03:17:59.2846308Z running 0 tests
2020-04-14T03:17:59.2846425Z 
---
2020-04-14T03:33:04.4286668Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4287294Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4287929Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4288577Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4289221Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4290516Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4291343Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4292378Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T03:33:04.4293290Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-14T03:34:08.9998809Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T03:34:09.0366323Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T03:34:09.2522353Z 
2020-04-14T03:34:09.2523214Z running 211 tests
2020-04-14T03:34:41.4874397Z ......................i...ii.......................................................................i 100/211
2020-04-14T03:35:19.5582765Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-14T03:35:24.6026628Z .......ii..
2020-04-14T03:35:24.6028674Z 
2020-04-14T03:35:24.6036293Z  finished in 75.567
2020-04-14T03:35:24.6043275Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-14T03:35:24.6049099Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-14T03:36:55.0872459Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-14T03:36:55.1073499Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T03:36:55.2657456Z 
2020-04-14T03:36:55.2658746Z running 13 tests
2020-04-14T03:36:55.6999843Z .iiiiiii.iii.
2020-04-14T03:36:55.7001994Z 
2020-04-14T03:36:55.7008897Z  finished in 0.593
2020-04-14T03:36:55.7066994Z Build completed successfully in 1:51:25
2020-04-14T03:36:55.7066994Z Build completed successfully in 1:51:25
2020-04-14T03:36:55.7076264Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-14T03:36:57.0798968Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T03:36:57.5696394Z     Finished release [optimized] target(s) in 0.49s
2020-04-14T03:36:57.5797901Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-14T03:36:57.5950444Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T03:38:24.3150649Z 
2020-04-14T03:38:24.3151414Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-14T03:38:24.3151866Z 24 }
2020-04-14T03:38:24.3152196Z 25 
2020-04-14T03:38:24.3152579Z 26 alloc0 (static: FOO, size: 4, align: 4) {
2020-04-14T03:38:24.3153486Z -     ╾alloc10+0╼                                     │ ╾──╼
2020-04-14T03:38:24.3154763Z +     ╾alloc9+0─╼                                     │ ╾──╼
2020-04-14T03:38:24.3155576Z 29 
2020-04-14T03:38:24.3155576Z 29 
2020-04-14T03:38:24.3156453Z - alloc10 (size: 168, align: 1) {
2020-04-14T03:38:24.3158112Z + alloc9 (size: 168, align: 1) {
2020-04-14T03:38:24.3159106Z 31     0x00 │ ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab │ ................
2020-04-14T03:38:24.3160018Z -     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾alloc5+0─╼ │ ............╾──╼
2020-04-14T03:38:24.3160860Z +     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾alloc4+0─╼ │ ............╾──╼
2020-04-14T03:38:24.3161651Z 33     0x20 │ 01 ef cd ab 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3163221Z 35     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3163631Z 
2020-04-14T03:38:24.3164278Z 36     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3165091Z 37     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3165091Z 37     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3165866Z 38     0x70 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3166659Z -     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾alloc7+0─╼ 00 00 │ ..........╾──╼..
2020-04-14T03:38:24.3167465Z -     0x90 │ ╾alloc8+99╼ 00 00 00 00 00 00 00 00 00 00 00 00 │ ╾──╼............
2020-04-14T03:38:24.3168241Z +     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾alloc6+0─╼ 00 00 │ ..........╾──╼..
2020-04-14T03:38:24.3169621Z +     0x90 │ ╾alloc7+99╼ 00 00 00 00 00 00 00 00 00 00 00 00 │ ╾──╼............
2020-04-14T03:38:24.3170427Z 41     0xa0 │ 00 00 00 00 00 00 00 00                         │ ........
2020-04-14T03:38:24.3171019Z 43 
2020-04-14T03:38:24.3171215Z 
2020-04-14T03:38:24.3171215Z 
2020-04-14T03:38:24.3171677Z - alloc5 (size: 4, align: 4) {
2020-04-14T03:38:24.3172022Z + alloc4 (size: 4, align: 4) {
2020-04-14T03:38:24.3172633Z 45     2a 00 00 00                                     │ *...
2020-04-14T03:38:24.3173210Z 47 
2020-04-14T03:38:24.3173402Z 
2020-04-14T03:38:24.3173402Z 
2020-04-14T03:38:24.3173995Z - alloc7 (fn: main)
2020-04-14T03:38:24.3174289Z + alloc6 (fn: main)
2020-04-14T03:38:24.3174522Z 49 
2020-04-14T03:38:24.3174967Z - alloc8 (size: 100, align: 1) {
2020-04-14T03:38:24.3175320Z + alloc7 (size: 100, align: 1) {
2020-04-14T03:38:24.3176563Z 52     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3177204Z 53     0x20 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
2020-04-14T03:38:24.3177517Z 
2020-04-14T03:38:24.3177517Z 
2020-04-14T03:38:24.3178415Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-14T03:38:24.3179395Z 
2020-04-14T03:38:24.3179893Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2020-04-14T03:38:24.3180394Z 3 fn main() -> () {
2020-04-14T03:38:24.3180850Z 4     let mut _0: ();                      // return place in scope 0 at $DIR/optimizes_into_variable.rs:11:11: 11:11
2020-04-14T03:38:24.3180850Z 4     let mut _0: ();                      // return place in scope 0 at $DIR/optimizes_into_variable.rs:11:11: 11:11
2020-04-14T03:38:24.3181778Z 5     let _1: i32;                         // in scope 0 at $DIR/optimizes_into_variable.rs:12:9: 12:10
2020-04-14T03:38:24.3182612Z -     let mut _3: [i32; 6];                // in scope 0 at $DIR/optimizes_into_variable.rs:13:13: 13:31
2020-04-14T03:38:24.3183452Z 8         debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
2020-04-14T03:38:24.3183452Z 8         debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
2020-04-14T03:38:24.3184095Z 9         let _2: i32;                     // in scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10
2020-04-14T03:38:24.3184713Z 10         scope 2 {
2020-04-14T03:38:24.3185291Z 11             debug y => _2;               // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
2020-04-14T03:38:24.3185291Z 11             debug y => _2;               // in scope 2 at $DIR/optimizes_into_variable.rs:13:9: 13:10
2020-04-14T03:38:24.3186063Z -             let _4: u32;                 // in scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3186821Z +             let _3: u32;                 // in scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3187229Z 13             scope 3 {
2020-04-14T03:38:24.3187866Z -                 debug z => _4;           // in scope 3 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3188434Z +                 debug z => _3;           // in scope 3 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3189081Z 16         }
2020-04-14T03:38:24.3190010Z 17     }
2020-04-14T03:38:24.3190229Z 
2020-04-14T03:38:24.3190229Z 
2020-04-14T03:38:24.3190762Z 26                                          // + span: $DIR/optimizes_into_variable.rs:12:13: 12:18
2020-04-14T03:38:24.3191220Z 27                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000004)) }
2020-04-14T03:38:24.3192944Z 28         StorageLive(_2);                 // bb0[2]: scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10
2020-04-14T03:38:24.3195545Z -         StorageLive(_3);                 // bb0[3]: scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:31
2020-04-14T03:38:24.3196732Z -         _3 = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32, const 5i32]; // bb0[4]: scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:31
2020-04-14T03:38:24.3197302Z +         _2 = const 3i32;                 // bb0[3]: scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
2020-04-14T03:38:24.3197673Z 31                                          // ty::Const
2020-04-14T03:38:24.3197950Z 32                                          // + ty: i32
2020-04-14T03:38:24.3198799Z -                                          // + val: Value(Scalar(0x00000000))
2020-04-14T03:38:24.3199453Z -                                          // mir::Constant
2020-04-14T03:38:24.3200381Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:14: 13:15
2020-04-14T03:38:24.3201267Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
2020-04-14T03:38:24.3202058Z -                                          // ty::Const
2020-04-14T03:38:24.3202522Z -                                          // + ty: i32
2020-04-14T03:38:24.3203204Z -                                          // + val: Value(Scalar(0x00000001))
2020-04-14T03:38:24.3203733Z -                                          // mir::Constant
2020-04-14T03:38:24.3204482Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:17: 13:18
2020-04-14T03:38:24.3205515Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
2020-04-14T03:38:24.3206247Z -                                          // ty::Const
2020-04-14T03:38:24.3206696Z -                                          // + ty: i32
2020-04-14T03:38:24.3207208Z -                                          // + val: Value(Scalar(0x00000002))
2020-04-14T03:38:24.3207741Z -                                          // mir::Constant
2020-04-14T03:38:24.3208579Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:20: 13:21
2020-04-14T03:38:24.3209301Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
2020-04-14T03:38:24.3209882Z -                                          // ty::Const
2020-04-14T03:38:24.3210321Z -                                          // + ty: i32
2020-04-14T03:38:24.3210637Z 51                                          // + val: Value(Scalar(0x00000003))
2020-04-14T03:38:24.3211267Z 52                                          // mir::Constant
2020-04-14T03:38:24.3211935Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:23: 13:24
2020-04-14T03:38:24.3212688Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000003)) }
2020-04-14T03:38:24.3213289Z -                                          // ty::Const
2020-04-14T03:38:24.3213759Z -                                          // + ty: i32
2020-04-14T03:38:24.3214292Z -                                          // + val: Value(Scalar(0x00000004))
2020-04-14T03:38:24.3215290Z -                                          // mir::Constant
2020-04-14T03:38:24.3216180Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:26: 13:27
2020-04-14T03:38:24.3217083Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000004)) }
2020-04-14T03:38:24.3217609Z -                                          // ty::Const
2020-04-14T03:38:24.3218017Z -                                          // + ty: i32
2020-04-14T03:38:24.3218662Z -                                          // + val: Value(Scalar(0x00000005))
2020-04-14T03:38:24.3219820Z -                                          // mir::Constant
2020-04-14T03:38:24.3221239Z -                                          // + span: $DIR/optimizes_into_variable.rs:13:29: 13:30
2020-04-14T03:38:24.3222650Z -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000005)) }
2020-04-14T03:38:24.3224225Z -         _2 = const 3i32;                 // bb0[5]: scope 1 at $DIR/optimizes_into_variable.rs:13:13: 13:34
2020-04-14T03:38:24.3226567Z -                                          // ty::Const
2020-04-14T03:38:24.3227043Z -                                          // + ty: i32
2020-04-14T03:38:24.3227531Z -                                          // + val: Value(Scalar(0x00000003))
2020-04-14T03:38:24.3228007Z -                                          // mir::Constant
2020-04-14T03:38:24.3228398Z 72                                          // + span: $DIR/optimizes_into_variable.rs:13:13: 13:34
2020-04-14T03:38:24.3228857Z 73                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000003)) }
2020-04-14T03:38:24.3232210Z -         StorageDead(_3);                 // bb0[6]: scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
2020-04-14T03:38:24.3233434Z -         StorageLive(_4);                 // bb0[7]: scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3234991Z -         _4 = const 42u32;                // bb0[8]: scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
2020-04-14T03:38:24.3235504Z +         StorageLive(_3);                 // bb0[4]: scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
2020-04-14T03:38:24.3236026Z +         _3 = const 42u32;                // bb0[5]: scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
2020-04-14T03:38:24.3236429Z 77                                          // ty::Const
2020-04-14T03:38:24.3236710Z 78                                          // + ty: u32
2020-04-14T03:38:24.3237348Z 79                                          // + val: Value(Scalar(0x0000002a))
2020-04-14T03:38:24.3239092Z 80                                          // mir::Constant
2020-04-14T03:38:24.3239092Z 80                                          // mir::Constant
2020-04-14T03:38:24.3243787Z 81                                          // + span: $DIR/optimizes_into_variable.rs:14:13: 14:38
2020-04-14T03:38:24.3244832Z 82                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002a)) }
2020-04-14T03:38:24.3246694Z -         StorageDead(_4);                 // bb0[9]: scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3247696Z -         StorageDead(_2);                 // bb0[10]: scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3262110Z -         StorageDead(_1);                 // bb0[11]: scope 0 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3263083Z -         return;                          // bb0[12]: scope 0 at $DIR/optimizes_into_variable.rs:15:2: 15:2
2020-04-14T03:38:24.3263812Z +         StorageDead(_3);                 // bb0[6]: scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3264348Z +         StorageDead(_2);                 // bb0[7]: scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3264908Z +         StorageDead(_1);                 // bb0[8]: scope 0 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-14T03:38:24.3265440Z +         return;                          // bb0[9]: scope 0 at $DIR/optimizes_into_variable.rs:15:2: 15:2
2020-04-14T03:38:24.3265916Z 88 }
2020-04-14T03:38:24.3266039Z 89 
2020-04-14T03:38:24.3266141Z 
2020-04-14T03:38:24.3266141Z 
2020-04-14T03:38:24.3267290Z thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable/32bit/rustc.main.SimplifyLocals.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-14T03:38:24.3267878Z 
2020-04-14T03:38:24.3267999Z failures:
2020-04-14T03:38:24.3268379Z     [mir-opt] mir-opt/const_allocation3.rs
2020-04-14T03:38:24.3268808Z     [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
2020-04-14T03:38:24.3268808Z     [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
2020-04-14T03:38:24.3268989Z 
2020-04-14T03:38:24.3270081Z test result: FAILED. 88 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T03:38:24.3270316Z 
2020-04-14T03:38:24.3270801Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-14T03:38:24.3271023Z 
2020-04-14T03:38:24.3271128Z 
2020-04-14T03:38:24.3275412Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-14T03:38:24.3277769Z 
2020-04-14T03:38:24.3277856Z 
2020-04-14T03:38:24.3277856Z 
2020-04-14T03:38:24.3278419Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-14T03:38:24.3279208Z == clock drift check ==
2020-04-14T03:38:24.3279208Z == clock drift check ==
2020-04-14T03:38:24.3279423Z   local time: Tue Apr 14 03:38:24 UTC 2020
2020-04-14T03:38:24.6226949Z   network time: Tue, 14 Apr 2020 03:38:24 GMT
2020-04-14T03:38:26.9387998Z 
2020-04-14T03:38:26.9387998Z 
2020-04-14T03:38:26.9468527Z ##[error]Bash exited with code '1'.
2020-04-14T03:38:26.9483122Z ##[section]Finishing: Run build
2020-04-14T03:38:26.9534813Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70755/merge to s
2020-04-14T03:38:26.9539847Z Task         : Get sources
2020-04-14T03:38:26.9540134Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T03:38:26.9540414Z Version      : 1.0.0
2020-04-14T03:38:26.9540602Z Author       : Microsoft
2020-04-14T03:38:26.9540602Z Author       : Microsoft
2020-04-14T03:38:26.9540896Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T03:38:26.9541260Z ==============================================================================
2020-04-14T03:38:27.3275924Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T03:38:27.3325946Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70755/merge to s
2020-04-14T03:38:27.3421137Z Cleaning up task key
2020-04-14T03:38:27.3422654Z Start cleaning up orphan processes.
2020-04-14T03:38:27.3623834Z Terminate orphan process: pid (7317) (python)
2020-04-14T03:38:27.3843315Z ##[section]Finishing: Finalize Job
