plain
2020-04-22T22:19:50.3191471Z ========================== Starting Command Output ===========================
2020-04-22T22:19:50.3193836Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8054b005-f902-49cd-8c51-642ad4041d46.sh
2020-04-22T22:19:50.3194083Z 
2020-04-22T22:19:50.3197771Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T22:19:50.3231246Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-22T22:19:50.3234417Z Task         : Get sources
2020-04-22T22:19:50.3234695Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T22:19:50.3234963Z Version      : 1.0.0
2020-04-22T22:19:50.3235167Z Author       : Microsoft
---
2020-04-22T22:19:51.3157305Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T22:19:51.3165230Z ##[command]git config gc.auto 0
2020-04-22T22:19:51.3170776Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T22:19:51.3175563Z ##[command]git config --get-all http.proxy
2020-04-22T22:19:51.3184206Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-22T22:22:41.9493700Z  ---> 318032b5f0e2
2020-04-22T22:22:41.9494891Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T22:22:41.9499078Z  ---> Using cache
2020-04-22T22:22:41.9499712Z  ---> d44a858fd1ce
2020-04-22T22:22:41.9500826Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T22:22:41.9505138Z  ---> 58b910f50f5a
2020-04-22T22:22:41.9505486Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T22:22:41.9506208Z  ---> Using cache
2020-04-22T22:22:41.9507891Z  ---> ee7702aadba1
---
2020-04-22T22:22:41.9849533Z Looks like docker image is the same as before, not uploading
2020-04-22T22:22:49.9326427Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T22:22:49.9541202Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T22:22:49.9569237Z == clock drift check ==
2020-04-22T22:22:49.9581004Z   local time: Wed Apr 22 22:22:49 UTC 2020
2020-04-22T22:22:50.0568052Z   network time: Wed, 22 Apr 2020 22:22:50 GMT
2020-04-22T22:22:50.0590176Z Starting sccache server...
2020-04-22T22:22:50.1419721Z configure: processing command line
2020-04-22T22:22:50.1420302Z configure: 
2020-04-22T22:22:50.1421443Z configure: rust.dist-src        := False
---
2020-04-22T22:27:51.9657120Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T22:27:53.3648884Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T22:27:54.8838933Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T22:27:56.1283090Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T22:28:04.5718756Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T22:28:07.0860394Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T22:28:11.3001980Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T22:28:15.2975404Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T22:28:24.3433563Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T22:49:39.0035188Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T22:49:40.5522088Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T22:49:42.1668002Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T22:49:42.3149831Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T22:49:52.4943316Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T22:49:54.4719133Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T22:49:58.9501338Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T22:50:03.1467751Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T22:50:13.5105683Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T23:12:12.9838167Z .................................................................................................... 1800/9923
2020-04-22T23:12:20.0873958Z .................................................................................................... 1900/9923
2020-04-22T23:12:29.0878791Z ............i....................................................................................... 2000/9923
2020-04-22T23:12:35.4897057Z .................................................................................................... 2100/9923
2020-04-22T23:12:48.1021768Z ..iiiii............................................................................................. 2200/9923
2020-04-22T23:12:56.8252445Z .................................................................................................... 2400/9923
2020-04-22T23:12:59.1307092Z .................................................................................................... 2500/9923
2020-04-22T23:13:04.4075720Z .................................................................................................... 2600/9923
2020-04-22T23:13:22.3661240Z .................................................................................................... 2700/9923
---
2020-04-22T23:15:59.0563736Z .................................................................................................... 5100/9923
2020-04-22T23:16:06.2473019Z .................................................................................................... 5200/9923
2020-04-22T23:16:10.7013569Z .........................i.......................................................................... 5300/9923
2020-04-22T23:16:19.8340965Z ...............i.................................................................................... 5400/9923
2020-04-22T23:16:25.3753812Z ...............ii.ii........i...i................................................................... 5500/9923
2020-04-22T23:16:32.7368707Z ..............................................................i..................................... 5700/9923
2020-04-22T23:16:40.9783044Z ...............................................................................................ii... 5800/9923
2020-04-22T23:16:47.4953405Z ..................................i................................................................. 5900/9923
2020-04-22T23:16:52.9497073Z .................................................................................................... 6000/9923
2020-04-22T23:16:52.9497073Z .................................................................................................... 6000/9923
2020-04-22T23:17:02.8286931Z .................................................................................................... 6100/9923
2020-04-22T23:17:12.3861384Z ............................ii...i..ii...........i.................................................. 6200/9923
2020-04-22T23:17:28.7769538Z .................................................................................................... 6400/9923
2020-04-22T23:17:32.2506190Z .................................................................................................... 6500/9923
2020-04-22T23:17:32.2506190Z .................................................................................................... 6500/9923
2020-04-22T23:17:38.1658552Z ..........................................................i..ii..................................... 6600/9923
2020-04-22T23:18:01.5663373Z .................................................................................................... 6800/9923
2020-04-22T23:18:05.9154790Z ...........................................................i........................................ 6900/9923
2020-04-22T23:18:07.9673001Z .................................................................................................... 7000/9923
2020-04-22T23:18:10.0638342Z ...................................................................................................i 7100/9923
---
2020-04-22T23:19:43.1364006Z .................................................................................................... 7900/9923
2020-04-22T23:19:48.3599008Z .................................................................................................... 8000/9923
2020-04-22T23:19:54.5782548Z .................................................................i.................................. 8100/9923
2020-04-22T23:20:04.2140444Z .................................................................................................... 8200/9923
2020-04-22T23:20:09.7135313Z ..............iiiiii.iiiii.i........................................................................ 8300/9923
2020-04-22T23:20:23.5156595Z .................................................................................................... 8500/9923
2020-04-22T23:20:29.4480356Z .................................................................................................... 8600/9923
2020-04-22T23:20:44.0307295Z .................................................................................................... 8700/9923
2020-04-22T23:20:50.9865250Z .................................................................................................... 8800/9923
---
2020-04-22T23:23:02.9905121Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-22T23:23:03.0111347Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:23:03.2111433Z 
2020-04-22T23:23:03.2111760Z running 186 tests
2020-04-22T23:23:06.4799988Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-22T23:23:08.6419882Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-22T23:23:08.6422887Z 
2020-04-22T23:23:08.6428036Z  finished in 5.631
2020-04-22T23:23:08.6434118Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-22T23:23:08.6635508Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-22T23:23:10.7592120Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-22T23:23:10.7795589Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:23:10.9403096Z 
2020-04-22T23:23:10.9405643Z running 9 tests
2020-04-22T23:23:10.9408453Z iiiiiiiii
2020-04-22T23:23:10.9410052Z 
2020-04-22T23:23:10.9412660Z  finished in 0.161
2020-04-22T23:23:10.9415194Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-22T23:23:10.9608621Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-22T23:23:30.9555322Z ) not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-22T23:23:30.9780428Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:23:31.1642892Z 
2020-04-22T23:23:31.1643246Z running 115 tests
2020-04-22T23:23:44.9345638Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-22T23:23:46.6439829Z ...iiii.....ii.
2020-04-22T23:23:46.6442072Z 
2020-04-22T23:23:46.6448499Z  finished in 15.667
2020-04-22T23:23:46.6453649Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-22T23:23:46.6457917Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-22T23:34:57.0455630Z 
2020-04-22T23:34:57.0457012Z    Doc-tests core
2020-04-22T23:35:01.5917530Z 
2020-04-22T23:35:01.5925003Z running 2499 tests
2020-04-22T23:35:10.0213408Z ......iiiii......................................................................................... 100/2499
2020-04-22T23:35:18.4230996Z ......................................................................................ii............ 200/2499
2020-04-22T23:35:37.5077700Z ......................i............................................................................. 400/2499
2020-04-22T23:35:46.7561287Z ............................................................................i..i..................ii 500/2499
2020-04-22T23:35:53.7905710Z ii.................................................................................................. 600/2499
2020-04-22T23:36:01.7688108Z .................................................................................................... 700/2499
---
2020-04-22T23:39:36.2923569Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:695:13
2020-04-22T23:39:36.2924410Z ..thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-22T23:39:36.2924881Z   left: `1`,
2020-04-22T23:39:36.2925491Z  right: `2`', src/libstd/sync/mutex.rs:658:13
2020-04-22T23:39:36.2926412Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:789:13
2020-04-22T23:39:36.2927872Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:765:13
2020-04-22T23:39:36.2928843Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:701:13
2020-04-22T23:39:36.2929696Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:629:13
2020-04-22T23:39:36.2930523Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
2020-04-22T23:39:36.2931369Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:603:13
2020-04-22T23:39:36.2932544Z .thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:616:13
2020-04-22T23:39:37.3834121Z ....................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-04-22T23:39:37.9916673Z ...............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-04-22T23:39:37.9918497Z thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1692:13
2020-04-22T23:39:37.9919411Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
---
2020-04-22T23:39:44.3473718Z 
2020-04-22T23:39:44.3475693Z running 1020 tests
2020-04-22T23:40:00.1291699Z i................................................................................................... 100/1020
2020-04-22T23:40:09.6952654Z .................................................................................................... 200/1020
2020-04-22T23:40:16.8651754Z ...................ii.i.....i......i...i......i..................................................... 300/1020
2020-04-22T23:40:21.4261473Z .................................................................................................... 400/1020
2020-04-22T23:40:27.6768259Z ....................................................i....i......................................ii.. 500/1020
2020-04-22T23:40:39.6383387Z .................................................................................................... 700/1020
2020-04-22T23:40:39.6383387Z .................................................................................................... 700/1020
2020-04-22T23:40:46.2144554Z ..............................................iiii.................................................. 800/1020
2020-04-22T23:40:58.7944604Z .................................................................................................... 900/1020
2020-04-22T23:41:04.4942826Z ....................................................................iiii............................ 1000/1020
2020-04-22T23:41:05.7017024Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-22T23:41:05.7017248Z 
2020-04-22T23:41:05.7117818Z  finished in 152.025
2020-04-22T23:41:05.7119087Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-22T23:43:57.2718594Z 
2020-04-22T23:43:57.2718873Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-22T23:43:57.2719090Z 
2020-04-22T23:43:57.2786793Z  finished in 0.927
2020-04-22T23:43:57.2806501Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-22T23:43:57.2818452Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:43:57.4652831Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T23:43:58.4185912Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-76a541b09e642f01
2020-04-22T23:43:58.4216642Z 
2020-04-22T23:43:58.4216908Z running 0 tests
2020-04-22T23:43:58.4217061Z 
---
2020-04-22T23:56:59.1947576Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1948270Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1949008Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1949720Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1950435Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1951890Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1952618Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1953313Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T23:56:59.1954131Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-22T23:57:58.9696684Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-22T23:57:59.0074502Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:57:59.2196812Z 
2020-04-22T23:57:59.2197231Z running 211 tests
2020-04-22T23:58:27.6850142Z ......................i...ii.......................................................................i 100/211
2020-04-22T23:59:04.6984814Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-22T23:59:10.2760448Z .......ii..
2020-04-22T23:59:10.2761478Z 
2020-04-22T23:59:10.2789451Z  finished in 71.269
2020-04-22T23:59:10.2790286Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
2020-04-22T23:59:10.2792398Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-04-22T23:59:23.4876842Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-22T23:59:23.5076113Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-22T23:59:23.6542082Z  finished in 4.075
2020-04-22T23:59:23.6550432Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-22T23:59:24.4152122Z Checking "alias-1" ... OK
2020-04-22T23:59:24.4837296Z Checking "alias-2" ... OK
2020-04-22T23:59:24.5413787Z Checking "alias-3" ... OK
2020-04-22T23:59:24.6175355Z Checking "alias" ... OK
2020-04-22T23:59:24.7119968Z Checking "basic" ... OK
2020-04-22T23:59:24.7993797Z Checking "deduplication" ... OK
2020-04-22T23:59:24.8532541Z Checking "enum-option" ... OK
2020-04-22T23:59:24.9216643Z Checking "filter-crate" ... OK
2020-04-22T23:59:24.9813557Z Checking "fn-forget" ... OK
2020-04-22T23:59:25.0672772Z Checking "from_u" ... OK
2020-04-22T23:59:25.1433863Z Checking "keyword" ... OK
2020-04-22T23:59:25.1910070Z Checking "macro-check" ... OK
2020-04-22T23:59:25.2256068Z Checking "macro-print" ... OK
2020-04-22T23:59:25.3054973Z Checking "multi-query" ... OK
2020-04-22T23:59:25.3348938Z Checking "never" ... OK
2020-04-22T23:59:25.3515398Z Checking "quoted" ... OK
2020-04-22T23:59:25.3886871Z Checking "return-specific-literal" ... OK
2020-04-22T23:59:25.4788162Z Checking "return-specific" ... OK
2020-04-22T23:59:25.5485287Z Checking "should-fail" ... OK
2020-04-22T23:59:25.6598221Z Checking "string-from_ut" ... OK
2020-04-22T23:59:25.7148238Z Checking "struct-vec" ... OK
2020-04-22T23:59:25.8278528Z Checking "vec-new" ... OK
2020-04-22T23:59:25.8538551Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T23:59:26.0092217Z 
2020-04-22T23:59:26.0092640Z running 6 tests
2020-04-22T23:59:31.3509854Z ......
---
2020-04-23T00:00:33.0617980Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-23T00:00:33.0821402Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T00:00:33.2376609Z 
2020-04-23T00:00:33.2376954Z running 13 tests
2020-04-23T00:00:33.6313674Z .iiiiiii.iii.
2020-04-23T00:00:33.6317890Z 
2020-04-23T00:00:33.6320395Z  finished in 0.550
2020-04-23T00:00:33.6383924Z Build completed successfully in 1:36:09
2020-04-23T00:00:33.6383924Z Build completed successfully in 1:36:09
2020-04-23T00:00:33.6396374Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-23T00:00:34.9684846Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T00:00:35.2015616Z     Finished release [optimized] target(s) in 0.22s
2020-04-23T00:00:35.2109840Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-23T00:00:35.2215133Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T00:01:42.3662607Z running 90 tests
2020-04-23T00:01:49.7393567Z ............................F.............................................................
2020-04-23T00:01:49.7395946Z failures:
2020-04-23T00:01:49.7396143Z 
2020-04-23T00:01:49.7396840Z ---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
2020-04-23T00:01:49.7397352Z 21           _9 = const main::promoted[0];    // bb0[4]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T00:01:49.7398324Z 22                                            // ty::Const
2020-04-23T00:01:49.7398622Z 23                                            // + ty: &[u32; 3]
2020-04-23T00:01:49.7399448Z -                                            // + val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), [], Some(promoted[0]))
2020-04-23T00:01:49.7400056Z +                                            // + val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), None, [], Some(promoted[0]))
2020-04-23T00:01:49.7400536Z 25                                            // mir::Constant
2020-04-23T00:01:49.7400876Z 26                                            // + span: $DIR/slice_len.rs:5:6: 5:19
2020-04-23T00:01:49.7401725Z -                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), [], Some(promoted[0])) }
2020-04-23T00:01:49.7402511Z +                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(DefId(0:3 ~ slice_len[317d]::main[0]), None, [], Some(promoted[0])) }
2020-04-23T00:01:49.7403122Z 28           _4 = _9;                         // bb0[5]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T00:01:49.7403572Z 29           _3 = _4;                         // bb0[6]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T00:01:49.7404019Z 30           _2 = move _3 as &[u32] (Pointer(Unsize)); // bb0[7]: scope 0 at $DIR/slice_len.rs:5:6: 5:19
2020-04-23T00:01:49.7404286Z 
2020-04-23T00:01:49.7405294Z thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-23T00:01:49.7406088Z 
2020-04-23T00:01:49.7406197Z 
2020-04-23T00:01:49.7406323Z failures:
2020-04-23T00:01:49.7406704Z     [mir-opt] mir-opt/const_prop/slice_len.rs
2020-04-23T00:01:49.7406704Z     [mir-opt] mir-opt/const_prop/slice_len.rs
2020-04-23T00:01:49.7406861Z 
2020-04-23T00:01:49.7407342Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-23T00:01:49.7407572Z 
2020-04-23T00:01:49.7410395Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T00:01:49.7411005Z 
2020-04-23T00:01:49.7411096Z 
2020-04-23T00:01:49.7414769Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T00:01:49.7418584Z 
2020-04-23T00:01:49.7418678Z 
2020-04-23T00:01:49.7418678Z 
2020-04-23T00:01:49.7422555Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-23T00:01:49.7476862Z == clock drift check ==
2020-04-23T00:01:49.7476862Z == clock drift check ==
2020-04-23T00:01:49.7503577Z   local time: Thu Apr 23 00:01:49 UTC 2020
2020-04-23T00:01:50.0499161Z   network time: Thu, 23 Apr 2020 00:01:50 GMT
2020-04-23T00:01:52.3755266Z 
2020-04-23T00:01:52.3755266Z 
2020-04-23T00:01:52.3822266Z ##[error]Bash exited with code '1'.
2020-04-23T00:01:52.3836146Z ##[section]Finishing: Run build
2020-04-23T00:01:52.3889215Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-23T00:01:52.3894269Z Task         : Get sources
2020-04-23T00:01:52.3894597Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T00:01:52.3894884Z Version      : 1.0.0
2020-04-23T00:01:52.3895103Z Author       : Microsoft
2020-04-23T00:01:52.3895103Z Author       : Microsoft
2020-04-23T00:01:52.3895426Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T00:01:52.3895789Z ==============================================================================
2020-04-23T00:01:52.7387787Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T00:01:52.7393188Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-23T00:01:52.7485350Z Cleaning up task key
2020-04-23T00:01:52.7486627Z Start cleaning up orphan processes.
2020-04-23T00:01:52.7668170Z Terminate orphan process: pid (4612) (python)
2020-04-23T00:01:52.8582836Z ##[section]Finishing: Finalize Job
