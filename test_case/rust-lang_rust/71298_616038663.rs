plain
2020-04-19T03:40:45.6401439Z ========================== Starting Command Output ===========================
2020-04-19T03:40:45.6406656Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/707e8cb9-a6a9-4d6e-bbc5-74dc9b51e3fd.sh
2020-04-19T03:40:45.6407179Z 
2020-04-19T03:40:45.6412205Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T03:40:45.6432249Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-19T03:40:45.6435956Z Task         : Get sources
2020-04-19T03:40:45.6436258Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T03:40:45.6436572Z Version      : 1.0.0
2020-04-19T03:40:45.6436773Z Author       : Microsoft
---
2020-04-19T03:40:46.8110133Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T03:40:46.8116844Z ##[command]git config gc.auto 0
2020-04-19T03:40:46.8120118Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T03:40:46.8123056Z ##[command]git config --get-all http.proxy
2020-04-19T03:40:46.8129688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71298/merge:refs/remotes/pull/71298/merge
---
2020-04-19T03:43:48.9506851Z  ---> 318032b5f0e2
2020-04-19T03:43:48.9509070Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T03:43:48.9511164Z  ---> Using cache
2020-04-19T03:43:48.9517274Z  ---> d44a858fd1ce
2020-04-19T03:43:48.9518578Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T03:43:48.9519877Z  ---> 58b910f50f5a
2020-04-19T03:43:48.9520140Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T03:43:48.9520557Z  ---> Using cache
2020-04-19T03:43:48.9520940Z  ---> ee7702aadba1
---
2020-04-19T03:43:49.0883230Z Looks like docker image is the same as before, not uploading
2020-04-19T03:43:56.5927977Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T03:43:56.6240495Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T03:43:56.6270647Z == clock drift check ==
2020-04-19T03:43:56.6279130Z   local time: Sun Apr 19 03:43:56 UTC 2020
2020-04-19T03:43:56.7258148Z   network time: Sun, 19 Apr 2020 03:43:56 GMT
2020-04-19T03:43:56.7280863Z Starting sccache server...
2020-04-19T03:43:56.8219675Z configure: processing command line
2020-04-19T03:43:56.8220277Z configure: 
2020-04-19T03:43:56.8221314Z configure: rust.dist-src        := False
---
2020-04-19T03:49:11.4506907Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T03:49:12.9933789Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T03:49:14.5970939Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T03:49:16.3276264Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T03:49:24.8504668Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T03:49:28.3303204Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T03:49:32.8097314Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T03:49:37.0843906Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T03:49:45.7247775Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T04:12:17.7515940Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T04:12:19.3394267Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T04:12:21.1201556Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T04:12:22.0714759Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T04:12:31.8284843Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T04:12:34.9484721Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T04:12:39.6468119Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T04:12:44.2378347Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T04:12:54.3902830Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T04:35:40.9382275Z .................................................................................................... 1700/9907
2020-04-19T04:35:44.8702112Z .................................................................................................... 1800/9907
2020-04-19T04:35:53.3460287Z ..................................................................................................i. 1900/9907
2020-04-19T04:36:00.8869537Z .................................................................................................... 2000/9907
2020-04-19T04:36:06.8150718Z ........................................................................................iiiii....... 2100/9907
2020-04-19T04:36:25.5662566Z .................................................................................................... 2300/9907
2020-04-19T04:36:27.7679957Z .................................................................................................... 2400/9907
2020-04-19T04:36:29.9159950Z .................................................................................................... 2500/9907
2020-04-19T04:36:35.5988243Z .................................................................................................... 2600/9907
---
2020-04-19T04:39:22.6836030Z ................................................................i...............i................... 5000/9907
2020-04-19T04:39:30.2013865Z .................................................................................................... 5100/9907
2020-04-19T04:39:36.7964590Z .................................................................................................... 5200/9907
2020-04-19T04:39:41.9607768Z ..........i......................................................................................... 5300/9907
2020-04-19T04:39:51.5816469Z i................................................................................................... 5400/9907
2020-04-19T04:39:56.3937040Z ii.ii........i...i.................................................................................. 5500/9907
2020-04-19T04:40:04.1102533Z ...............................................i.................................................... 5700/9907
2020-04-19T04:40:12.8357763Z ...............................................................................ii................... 5800/9907
2020-04-19T04:40:19.6701235Z ..................i................................................................................. 5900/9907
2020-04-19T04:40:24.9387273Z .................................................................................................... 6000/9907
2020-04-19T04:40:24.9387273Z .................................................................................................... 6000/9907
2020-04-19T04:40:35.2594502Z .................................................................................................... 6100/9907
2020-04-19T04:40:45.4247406Z ............ii...i..ii...........i.................................................................. 6200/9907
2020-04-19T04:41:00.2968815Z .................................................................................................... 6400/9907
2020-04-19T04:41:06.6931333Z .................................................................................................... 6500/9907
2020-04-19T04:41:06.6931333Z .................................................................................................... 6500/9907
2020-04-19T04:41:18.8803019Z ..........................................i..ii..................................................... 6600/9907
2020-04-19T04:41:39.7824866Z .................................................................................................... 6800/9907
2020-04-19T04:41:41.8287409Z ...........................................i........................................................ 6900/9907
2020-04-19T04:41:43.8034500Z .................................................................................................... 7000/9907
2020-04-19T04:41:45.8489855Z ...................................................................................i................ 7100/9907
---
2020-04-19T04:43:15.9564528Z .................................................................................................... 7800/9907
2020-04-19T04:43:20.5776331Z .................................................................................................... 7900/9907
2020-04-19T04:43:26.9666475Z .................................................................................................... 8000/9907
2020-04-19T04:43:32.6090232Z .................................................i.................................................. 8100/9907
2020-04-19T04:43:42.2183702Z ..................................................................................................ii 8200/9907
2020-04-19T04:43:47.4136632Z iiii.iiiii.i........................................................................................ 8300/9907
2020-04-19T04:44:00.5254683Z .................................................................................................... 8500/9907
2020-04-19T04:44:08.4273989Z .................................................................................................... 8600/9907
2020-04-19T04:44:21.6675626Z .................................................................................................... 8700/9907
2020-04-19T04:44:28.1465011Z .................................................................................................... 8800/9907
---
2020-04-19T04:46:36.3418369Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-19T04:46:36.3581305Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T04:46:36.5502924Z 
2020-04-19T04:46:36.5503284Z running 186 tests
2020-04-19T04:46:39.2954328Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-19T04:46:41.7932895Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-19T04:46:41.7937780Z 
2020-04-19T04:46:41.7938141Z  finished in 5.435
2020-04-19T04:46:41.7948438Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-19T04:46:41.8156613Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T04:46:43.9887787Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-19T04:46:44.0046723Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T04:46:44.1461918Z 
2020-04-19T04:46:44.1465026Z running 9 tests
2020-04-19T04:46:44.1465979Z iiiiiiiii
2020-04-19T04:46:44.1466967Z 
2020-04-19T04:46:44.1470736Z  finished in 0.142
2020-04-19T04:46:44.1545723Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-19T04:46:44.1640216Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T04:47:03.7860615Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-19T04:47:03.8050346Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T04:47:03.9839302Z 
2020-04-19T04:47:03.9840181Z running 115 tests
2020-04-19T04:47:17.4768062Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-19T04:47:19.2345920Z ...iiii.....ii.
2020-04-19T04:47:19.2346948Z 
2020-04-19T04:47:19.2347526Z  finished in 15.429
2020-04-19T04:47:19.2394414Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-19T04:47:19.2395048Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T04:59:08.3288290Z 
2020-04-19T04:59:08.3289191Z    Doc-tests core
2020-04-19T04:59:13.0194334Z 
2020-04-19T04:59:13.0195101Z running 2496 tests
2020-04-19T04:59:21.6056102Z ......iiiii......................................................................................... 100/2496
2020-04-19T04:59:29.7889184Z .....................................................................................ii............. 200/2496
2020-04-19T04:59:49.9380757Z ....................i............................................................................... 400/2496
2020-04-19T04:59:49.9380757Z ....................i............................................................................... 400/2496
2020-04-19T04:59:59.0833545Z ..........................................................................i..i..................iiii 500/2496
2020-04-19T05:00:15.7768411Z .................................................................................................... 700/2496
2020-04-19T05:00:24.7960244Z .................................................................................................... 800/2496
2020-04-19T05:00:34.1154128Z .................................................................................................... 900/2496
2020-04-19T05:00:42.9581965Z .................................................................................................... 1000/2496
---
2020-04-19T05:04:14.0438394Z 
2020-04-19T05:04:14.0438771Z running 1020 tests
2020-04-19T05:04:30.4859318Z i................................................................................................... 100/1020
2020-04-19T05:04:40.1448312Z .................................................................................................... 200/1020
2020-04-19T05:04:47.2868849Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-19T05:04:51.7460500Z .................................................................................................... 400/1020
2020-04-19T05:04:57.9283737Z ....................................................i....i......................................ii.. 500/1020
2020-04-19T05:05:09.8910433Z .................................................................................................... 700/1020
2020-04-19T05:05:09.8910433Z .................................................................................................... 700/1020
2020-04-19T05:05:16.5116635Z ...............................................iiii................................................. 800/1020
2020-04-19T05:05:29.6163766Z .................................................................................................... 900/1020
2020-04-19T05:05:35.4957310Z ....................................................................iiii............................ 1000/1020
2020-04-19T05:05:36.8005877Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-19T05:05:36.8006129Z 
2020-04-19T05:05:36.8126810Z  finished in 155.569
2020-04-19T05:05:36.8134124Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-19T05:08:35.8855196Z 
2020-04-19T05:08:35.8855509Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T05:08:35.8855783Z 
2020-04-19T05:08:35.8919608Z  finished in 0.998
2020-04-19T05:08:35.8920846Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-19T05:08:35.8939138Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T05:08:36.1056800Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T05:08:37.5739980Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-e28c5a74b82483a5
2020-04-19T05:08:37.5740280Z 
2020-04-19T05:08:37.5740423Z running 0 tests
2020-04-19T05:08:37.5740527Z 
---
2020-04-19T05:22:22.4114256Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4115051Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4115913Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4116745Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4117576Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4119242Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4120082Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4120884Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-19T05:22:22.4121897Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-19T05:23:22.1265065Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-19T05:23:22.1626321Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T05:23:22.3907198Z 
2020-04-19T05:23:22.3907866Z running 211 tests
2020-04-19T05:23:51.7353243Z ......................i...ii.......................................................................i 100/211
2020-04-19T05:24:28.4934003Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-19T05:24:30.3146439Z .......ii..
2020-04-19T05:24:30.3149087Z 
2020-04-19T05:24:30.3150317Z  finished in 68.152
2020-04-19T05:24:30.3157846Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-19T05:24:30.3168888Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-19T05:24:44.0934590Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-19T05:24:44.1116033Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-19T05:24:44.2518720Z  finished in 4.340
2020-04-19T05:24:44.2530169Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-19T05:24:45.1317355Z Checking "alias-1" ... OK
2020-04-19T05:24:45.2033935Z Checking "alias-2" ... OK
2020-04-19T05:24:45.2608952Z Checking "alias-3" ... OK
2020-04-19T05:24:45.3211627Z Checking "alias" ... OK
2020-04-19T05:24:45.3903377Z Checking "basic" ... OK
2020-04-19T05:24:45.4601657Z Checking "deduplication" ... OK
2020-04-19T05:24:45.5053051Z Checking "enum-option" ... OK
2020-04-19T05:24:45.5654649Z Checking "filter-crate" ... OK
2020-04-19T05:24:45.6249665Z Checking "fn-forget" ... OK
2020-04-19T05:24:45.7403264Z Checking "from_u" ... OK
2020-04-19T05:24:45.8201614Z Checking "keyword" ... OK
2020-04-19T05:24:45.8634683Z Checking "macro-check" ... OK
2020-04-19T05:24:45.8950649Z Checking "macro-print" ... OK
2020-04-19T05:24:45.9760393Z Checking "multi-query" ... OK
2020-04-19T05:24:46.0129228Z Checking "never" ... OK
2020-04-19T05:24:46.0301830Z Checking "quoted" ... OK
2020-04-19T05:24:46.0618942Z Checking "return-specific-literal" ... OK
2020-04-19T05:24:46.1480072Z Checking "return-specific" ... OK
2020-04-19T05:24:46.1885397Z Checking "should-fail" ... OK
2020-04-19T05:24:46.2611303Z Checking "string-from_ut" ... OK
2020-04-19T05:24:46.3141180Z Checking "struct-vec" ... OK
2020-04-19T05:24:46.4164657Z Checking "vec-new" ... OK
2020-04-19T05:24:46.4385784Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T05:24:46.6120016Z 
2020-04-19T05:24:46.6120411Z running 6 tests
2020-04-19T05:24:51.9026612Z ......
---
2020-04-19T05:25:57.8525204Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-19T05:25:57.8804432Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T05:25:58.0235789Z 
2020-04-19T05:25:58.0236469Z running 13 tests
2020-04-19T05:25:58.5014480Z .iiiiiii.iii.
2020-04-19T05:25:58.5016274Z 
2020-04-19T05:25:58.5021341Z  finished in 0.623
2020-04-19T05:25:58.5086152Z Build completed successfully in 1:40:27
2020-04-19T05:25:58.5086152Z Build completed successfully in 1:40:27
2020-04-19T05:25:58.5095850Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-19T05:25:59.9081316Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-19T05:26:00.1637146Z     Finished release [optimized] target(s) in 0.25s
2020-04-19T05:26:00.1732638Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-19T05:26:00.1863660Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-19T05:27:17.8623556Z ..............................F........................................F...................thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T05:27:17.8625595Z 
2020-04-19T05:27:17.8625729Z failures:
2020-04-19T05:27:17.8625834Z 
2020-04-19T05:27:17.8626381Z ---- [mir-opt] mir-opt/const_prop/usize_literal_propagation.rs stdout ----
2020-04-19T05:27:17.8626831Z 17           _1 = const 0usize;               // bb0[1]: scope 1 at $DIR/usize_literal_propagation.rs:4:17: 4:18
2020-04-19T05:27:17.8627330Z 18                                            // ty::Const
2020-04-19T05:27:17.8627615Z 19                                            // + ty: usize
2020-04-19T05:27:17.8628165Z -                                            // + val: Value(Scalar(0x0000000000000000))
2020-04-19T05:27:17.8628544Z +                                            // + val: Value(Scalar(0x00000000))
2020-04-19T05:27:17.8628858Z 21                                            // mir::Constant
2020-04-19T05:27:17.8629221Z 22                                            // + span: $DIR/usize_literal_propagation.rs:4:17: 4:18
2020-04-19T05:27:17.8629967Z -                                            // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000000)) }
2020-04-19T05:27:17.8630476Z +                                            // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-19T05:27:17.8630972Z 24           StorageLive(_2);                 // bb0[2]: scope 2 at $DIR/usize_literal_propagation.rs:5:9: 5:46
2020-04-19T05:27:17.8631494Z 25           StorageLive(_3);                 // bb0[3]: scope 2 at $DIR/usize_literal_propagation.rs:5:44: 5:45
2020-04-19T05:27:17.8632222Z 26 -         _3 = _1;                         // bb0[4]: scope 2 at $DIR/usize_literal_propagation.rs:5:44: 5:45
2020-04-19T05:27:17.8632504Z 
2020-04-19T05:27:17.8632820Z 27 +         _3 = const 0usize;               // bb0[4]: scope 2 at $DIR/usize_literal_propagation.rs:5:44: 5:45
2020-04-19T05:27:17.8633205Z 28 +                                          // ty::Const
2020-04-19T05:27:17.8633472Z 29 +                                          // + ty: usize
2020-04-19T05:27:17.8634017Z - +                                          // + val: Value(Scalar(0x0000000000000000))
2020-04-19T05:27:17.8634382Z + +                                          // + val: Value(Scalar(0x00000000))
2020-04-19T05:27:17.8634688Z 31 +                                          // mir::Constant
2020-04-19T05:27:17.8635068Z 32 +                                          // + span: $DIR/usize_literal_propagation.rs:5:44: 5:45
2020-04-19T05:27:17.8635789Z - +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000000)) }
2020-04-19T05:27:17.8636276Z + +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-19T05:27:17.8637121Z 34           _2 = const std::intrinsics::transmute::<usize, &i32>(move _3) -> bb1; // bb0[5]: scope 2 at $DIR/usize_literal_propagation.rs:5:9: 5:46
2020-04-19T05:27:17.8637580Z 35                                            // ty::Const
2020-04-19T05:27:17.8638321Z 36                                            // + ty: unsafe extern "rust-intrinsic" fn(usize) -> &i32 {std::intrinsics::transmute::<usize, &i32>}
2020-04-19T05:27:17.8638703Z 
2020-04-19T05:27:17.8639509Z thread '[mir-opt] mir-opt/const_prop/usize_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/usize_literal_propagation/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-19T05:27:17.8640390Z 
2020-04-19T05:27:17.8640776Z ---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
2020-04-19T05:27:17.8640776Z ---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
2020-04-19T05:27:17.8641393Z 31           _3 = const 0isize;               // bb0[4]: scope 1 at $DIR/simplify-arm-identity.rs:19:9: 19:20
2020-04-19T05:27:17.8641836Z 32                                            // ty::Const
2020-04-19T05:27:17.8642098Z 33                                            // + ty: isize
2020-04-19T05:27:17.8642667Z -                                            // + val: Value(Scalar(0x0000000000000000))
2020-04-19T05:27:17.8643020Z +                                            // + val: Value(Scalar(0x00000000))
2020-04-19T05:27:17.8643337Z 35                                            // mir::Constant
2020-04-19T05:27:17.8644034Z 36                                            // + span: $DIR/simplify-arm-identity.rs:19:9: 19:20
2020-04-19T05:27:17.8644793Z -                                            // + literal: Const { ty: isize, val: Value(Scalar(0x0000000000000000)) }
2020-04-19T05:27:17.8645345Z +                                            // + literal: Const { ty: isize, val: Value(Scalar(0x00000000)) }
2020-04-19T05:27:17.8646107Z 38           StorageLive(_4);                 // bb0[5]: scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:19
2020-04-19T05:27:17.8646874Z 39           _4 = const 0u8;                  // bb0[6]: scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:19
2020-04-19T05:27:17.8647377Z 40                                            // ty::Const
2020-04-19T05:27:17.8647540Z 
2020-04-19T05:27:17.8648294Z thread '[mir-opt] mir-opt/simplify-arm-identity.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify-arm-identity/rustc.main.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-19T05:27:17.8648819Z 
2020-04-19T05:27:17.8648930Z failures:
2020-04-19T05:27:17.8649305Z     [mir-opt] mir-opt/const_prop/usize_literal_propagation.rs
2020-04-19T05:27:17.8649730Z     [mir-opt] mir-opt/simplify-arm-identity.rs
2020-04-19T05:27:17.8649730Z     [mir-opt] mir-opt/simplify-arm-identity.rs
2020-04-19T05:27:17.8649873Z 
2020-04-19T05:27:17.8650288Z test result: FAILED. 89 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-19T05:27:17.8650511Z 
2020-04-19T05:27:17.8654376Z 
2020-04-19T05:27:17.8654519Z 
2020-04-19T05:27:17.8658420Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T05:27:17.8660780Z 
2020-04-19T05:27:17.8660985Z 
2020-04-19T05:27:17.8660985Z 
2020-04-19T05:27:17.8661779Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-19T05:27:17.8686239Z == clock drift check ==
2020-04-19T05:27:17.8706377Z   local time: Sun Apr 19 05:27:17 UTC 2020
2020-04-19T05:27:17.8706377Z   local time: Sun Apr 19 05:27:17 UTC 2020
2020-04-19T05:27:18.0677285Z   network time: Sun, 19 Apr 2020 05:27:18 GMT
2020-04-19T05:27:20.4442808Z 
2020-04-19T05:27:20.4442808Z 
2020-04-19T05:27:20.4562942Z ##[error]Bash exited with code '1'.
2020-04-19T05:27:20.4582891Z ##[section]Finishing: Run build
2020-04-19T05:27:20.4695811Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-19T05:27:20.4703006Z Task         : Get sources
2020-04-19T05:27:20.4703362Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T05:27:20.4703742Z Version      : 1.0.0
2020-04-19T05:27:20.4703980Z Author       : Microsoft
2020-04-19T05:27:20.4703980Z Author       : Microsoft
2020-04-19T05:27:20.4704350Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T05:27:20.4704792Z ==============================================================================
2020-04-19T05:27:20.8885960Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T05:27:20.8936905Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-19T05:27:20.9053242Z Cleaning up task key
2020-04-19T05:27:20.9054633Z Start cleaning up orphan processes.
2020-04-19T05:27:20.9281323Z Terminate orphan process: pid (3953) (python)
2020-04-19T05:27:21.0545354Z ##[section]Finishing: Finalize Job
