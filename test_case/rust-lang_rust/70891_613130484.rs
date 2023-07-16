plain
2020-04-13T20:40:10.5045422Z ========================== Starting Command Output ===========================
2020-04-13T20:40:10.5050271Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e46789c-b435-4c54-bf0d-65d0bfe8da4d.sh
2020-04-13T20:40:10.5050684Z 
2020-04-13T20:40:10.5055404Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T20:40:10.5072353Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-13T20:40:10.5075554Z Task         : Get sources
2020-04-13T20:40:10.5075790Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T20:40:10.5076038Z Version      : 1.0.0
2020-04-13T20:40:10.5076196Z Author       : Microsoft
---
2020-04-13T20:40:11.4946507Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T20:40:11.4953381Z ##[command]git config gc.auto 0
2020-04-13T20:40:11.4957541Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T20:40:11.4961316Z ##[command]git config --get-all http.proxy
2020-04-13T20:40:11.4968144Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70891/merge:refs/remotes/pull/70891/merge
---
2020-04-13T20:43:03.2484158Z  ---> f58a2bb1e753
2020-04-13T20:43:03.2487097Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-13T20:43:03.2487806Z  ---> Using cache
2020-04-13T20:43:03.2490199Z  ---> d079cc6b6db8
2020-04-13T20:43:03.2491107Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-13T20:43:03.2493233Z  ---> 4183ca46ee56
2020-04-13T20:43:03.2493429Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-13T20:43:03.2493760Z  ---> Using cache
2020-04-13T20:43:03.2494077Z  ---> 69e7f8a2a2fb
---
2020-04-13T20:43:03.2925310Z Looks like docker image is the same as before, not uploading
2020-04-13T20:43:10.8018281Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T20:43:10.8309522Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T20:43:10.8338470Z == clock drift check ==
2020-04-13T20:43:10.8347293Z   local time: Mon Apr 13 20:43:10 UTC 2020
2020-04-13T20:43:11.0381351Z   network time: Mon, 13 Apr 2020 20:43:11 GMT
2020-04-13T20:43:11.0411655Z Starting sccache server...
2020-04-13T20:43:11.1266099Z configure: processing command line
2020-04-13T20:43:11.1266417Z configure: 
2020-04-13T20:43:11.1267386Z configure: rust.dist-src        := False
---
2020-04-13T20:48:47.0089227Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T20:48:48.5820659Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T20:48:50.2226513Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T20:48:51.8536073Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T20:49:01.3127234Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T20:49:03.9188025Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T20:49:08.7892263Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T20:49:13.3460829Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T20:49:23.5528341Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T21:12:45.3425183Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T21:12:47.1906601Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T21:12:49.2937668Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T21:12:52.2090480Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T21:13:02.3441635Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T21:13:06.7224637Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T21:13:12.3282814Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T21:13:18.0510050Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T21:13:27.7417217Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T21:39:48.9275468Z .................................................................................................... 1700/9891
2020-04-13T21:39:53.1842675Z .................................................................................................... 1800/9891
2020-04-13T21:40:01.6069242Z .................................................................................................... 1900/9891
2020-04-13T21:40:09.7851353Z ....i............................................................................................... 2000/9891
2020-04-13T21:40:16.0823917Z ..............................................................................................iiiii. 2100/9891
2020-04-13T21:40:37.6313149Z .................................................................................................... 2300/9891
2020-04-13T21:40:39.8523834Z .................................................................................................... 2400/9891
2020-04-13T21:40:42.1976326Z .................................................................................................... 2500/9891
2020-04-13T21:40:48.3158604Z .................................................................................................... 2600/9891
---
2020-04-13T21:43:51.5047275Z .................................................................................................... 5100/9891
2020-04-13T21:43:59.2436413Z .................................................................................................... 5200/9891
2020-04-13T21:44:04.3250524Z ..............i..................................................................................... 5300/9891
2020-04-13T21:44:14.0686871Z ....i............................................................................................... 5400/9891
2020-04-13T21:44:19.1673507Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-13T21:44:26.8304830Z ..................................................i................................................. 5700/9891
2020-04-13T21:44:36.9299948Z ......................................................................ii............................ 5800/9891
2020-04-13T21:44:43.1422669Z .........i.......................................................................................... 5900/9891
2020-04-13T21:44:48.7496951Z .................................................................................................... 6000/9891
2020-04-13T21:44:48.7496951Z .................................................................................................... 6000/9891
2020-04-13T21:44:58.6407183Z .................................................................................................... 6100/9891
2020-04-13T21:45:09.4788802Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-13T21:45:24.7835349Z .................................................................................................... 6400/9891
2020-04-13T21:45:31.1868452Z .................................................................................................... 6500/9891
2020-04-13T21:45:31.1868452Z .................................................................................................... 6500/9891
2020-04-13T21:45:51.1945825Z .................................i..ii.............................................................. 6600/9891
2020-04-13T21:46:13.1904422Z .................................................................................................... 6800/9891
2020-04-13T21:46:15.2150577Z .................................i.................................................................. 6900/9891
2020-04-13T21:46:17.2027475Z .................................................................................................... 7000/9891
2020-04-13T21:46:19.3306481Z ........................................................................i........................... 7100/9891
---
2020-04-13T21:47:57.8887242Z .................................................................................................... 7800/9891
2020-04-13T21:48:02.0149997Z .................................................................................................... 7900/9891
2020-04-13T21:48:08.9484699Z .................................................................................................... 8000/9891
2020-04-13T21:48:15.6284413Z ......................................i............................................................. 8100/9891
2020-04-13T21:48:25.2634141Z .......................................................................................iiiiiiiiiii.i 8200/9891
2020-04-13T21:48:41.4845562Z ................................i......i............................................................ 8400/9891
2020-04-13T21:48:44.9809181Z .................................................................................................... 8500/9891
2020-04-13T21:48:55.8703525Z .................................................................................................... 8600/9891
2020-04-13T21:49:09.8568342Z .................................................................................................... 8700/9891
---
2020-04-13T21:51:37.5376368Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-13T21:51:37.5561013Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T21:51:37.7804415Z 
2020-04-13T21:51:37.7804713Z running 185 tests
2020-04-13T21:51:40.4738547Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-13T21:51:43.1570378Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-13T21:51:43.1573013Z 
2020-04-13T21:51:43.1576114Z  finished in 5.601
2020-04-13T21:51:43.1594390Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-13T21:51:43.1774487Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-13T21:51:45.2023389Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-13T21:51:45.2206667Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T21:51:45.3656778Z 
2020-04-13T21:51:45.3657543Z running 9 tests
2020-04-13T21:51:45.3658566Z iiiiiiiii
2020-04-13T21:51:45.3659412Z 
2020-04-13T21:51:45.3664515Z  finished in 0.146
2020-04-13T21:51:45.3672556Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-13T21:51:45.3831833Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-13T21:52:05.0082451Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-13T21:52:05.0305304Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T21:52:05.2340654Z 
2020-04-13T21:52:05.2342147Z running 115 tests
2020-04-13T21:52:19.2387062Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-13T21:52:21.0309633Z ...iiii.....ii.
2020-04-13T21:52:21.0312505Z 
2020-04-13T21:52:21.0316514Z  finished in 16.001
2020-04-13T21:52:21.0321707Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-13T21:52:21.0326532Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-13T22:05:13.1287598Z 
2020-04-13T22:05:13.1291269Z    Doc-tests core
2020-04-13T22:05:17.8757679Z 
2020-04-13T22:05:17.8758593Z running 2490 tests
2020-04-13T22:05:26.9877031Z ......iiiii......................................................................................... 100/2490
2020-04-13T22:05:36.2674508Z .....................................................................................ii............. 200/2490
2020-04-13T22:05:56.2890127Z ....................i............................................................................... 400/2490
2020-04-13T22:05:56.2890127Z ....................i............................................................................... 400/2490
2020-04-13T22:06:06.1473145Z ..........................................................................i..i..................iiii 500/2490
2020-04-13T22:06:22.2430525Z .................................................................................................... 700/2490
2020-04-13T22:06:30.7427050Z .................................................................................................... 800/2490
2020-04-13T22:06:39.2474212Z .................................................................................................... 900/2490
2020-04-13T22:06:48.1440748Z .................................................................................................... 1000/2490
---
2020-04-13T22:10:11.3948273Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-04-13T22:10:11.3955169Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-13T22:10:11.3985986Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-13T22:10:11.4148735Z .............. 600/764
2020-04-13T22:10:13.4488873Z .......................thread '<unnamed>' panicked at '.explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-13T22:10:13.4496564Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-13T22:10:13.4510415Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-13T22:10:13.4531851Z ....thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-13T22:10:13.4532448Z   left: `1`,
2020-04-13T22:10:13.4533209Z  right: `2`', src/libstd/sync/mutex.rs:657:13
2020-04-13T22:10:13.4642994Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:789:13
---
2020-04-13T22:10:22.5533771Z 
2020-04-13T22:10:22.5534231Z running 1020 tests
2020-04-13T22:10:40.0900285Z i................................................................................................... 100/1020
2020-04-13T22:10:50.2202758Z .................................................................................................... 200/1020
2020-04-13T22:10:57.6838389Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-13T22:11:02.3732017Z .................................................................................................... 400/1020
2020-04-13T22:11:08.8914794Z ....................................................i....i......................................ii.. 500/1020
2020-04-13T22:11:21.4039386Z .................................................................................................... 700/1020
2020-04-13T22:11:21.4039386Z .................................................................................................... 700/1020
2020-04-13T22:11:28.4282488Z ..............................................iiii.................................................. 800/1020
2020-04-13T22:11:42.3466495Z .................................................................................................... 900/1020
2020-04-13T22:11:48.3564979Z ....................................................................iiii............................ 1000/1020
2020-04-13T22:11:49.7720366Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-13T22:11:49.7722616Z 
2020-04-13T22:11:49.7831486Z  finished in 164.859
2020-04-13T22:11:49.7836631Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-13T22:15:08.8415767Z 
2020-04-13T22:15:08.8416027Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-13T22:15:08.8416292Z 
2020-04-13T22:15:08.8481808Z  finished in 1.047
2020-04-13T22:15:08.8483071Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-13T22:15:08.8498077Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T22:15:09.0472086Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T22:15:10.0538224Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-f3c6cc793918c18a
2020-04-13T22:15:10.0564812Z 
2020-04-13T22:15:10.0565043Z running 0 tests
2020-04-13T22:15:10.0565174Z 
---
2020-04-13T22:30:12.9872864Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9873694Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9874532Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9875351Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9876200Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9877874Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9878699Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9879498Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-13T22:30:12.9880347Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-13T22:31:16.3158234Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-13T22:31:16.3501281Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T22:31:16.5586356Z 
2020-04-13T22:31:16.5586658Z running 211 tests
2020-04-13T22:31:49.7061996Z ......................i...ii.......................................................................i 100/211
2020-04-13T22:32:28.0340586Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-13T22:32:32.2500970Z .......ii..
2020-04-13T22:32:32.2502268Z 
2020-04-13T22:32:32.2507073Z  finished in 75.900
2020-04-13T22:32:32.2513791Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-13T22:32:32.2531267Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-13T22:34:02.5176049Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-13T22:34:02.5372322Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T22:34:02.7145111Z 
2020-04-13T22:34:02.7145716Z running 13 tests
2020-04-13T22:34:03.1936847Z .iiiiiii.iii.
2020-04-13T22:34:03.1940459Z 
2020-04-13T22:34:03.1940654Z  finished in 0.657
2020-04-13T22:34:03.2010617Z Build completed successfully in 1:49:01
2020-04-13T22:34:03.2010617Z Build completed successfully in 1:49:01
2020-04-13T22:34:03.2021069Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-13T22:34:04.6361384Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-13T22:34:04.8840148Z     Finished release [optimized] target(s) in 0.24s
2020-04-13T22:34:04.8975166Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-13T22:34:04.9083991Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-13T22:35:23.4479832Z     Finished release [optimized] target(s) in 0.19s
2020-04-13T22:35:23.4737754Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> armv5te-unknown-linux-gnueabi)
2020-04-13T22:35:23.6610846Z 
2020-04-13T22:35:23.6611133Z running 89 tests
2020-04-13T22:35:31.3841967Z F......FF.FF.....F...F....F.F................F...............F..F........................
2020-04-13T22:35:31.3844350Z 
2020-04-13T22:35:31.3852332Z ---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
2020-04-13T22:35:31.3852332Z ---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
2020-04-13T22:35:31.3853302Z 82         _1[_7] = move _5;                // bb2[0]: scope 3 at $DIR/array-index-is-temporary.rs:16:5: 16:29
2020-04-13T22:35:31.3854212Z 83         StorageDead(_5);                 // bb2[1]: scope 3 at $DIR/array-index-is-temporary.rs:16:28: 16:29
2020-04-13T22:35:31.3855118Z 84         StorageDead(_7);                 // bb2[2]: scope 3 at $DIR/array-index-is-temporary.rs:16:29: 16:30
2020-04-13T22:35:31.3856019Z -         _0 = ();                         // bb2[3]: scope 0 at $DIR/array-index-is-temporary.rs:12:11: 17:2
2020-04-13T22:35:31.3856903Z +         _0 = const ();                   // bb2[3]: scope 0 at $DIR/array-index-is-temporary.rs:12:11: 17:2
2020-04-13T22:35:31.3857380Z +                                          // ty::Const
2020-04-13T22:35:31.3857680Z +                                          // + ty: ()
2020-04-13T22:35:31.3858211Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3858581Z +                                          // mir::Constant
2020-04-13T22:35:31.3859333Z +                                          // + span: $DIR/array-index-is-temporary.rs:12:11: 17:2
2020-04-13T22:35:31.3859894Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3860732Z 86         StorageDead(_3);                 // bb2[4]: scope 2 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-13T22:35:31.3861619Z 87         StorageDead(_2);                 // bb2[5]: scope 1 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-13T22:35:31.3862503Z 88         StorageDead(_1);                 // bb2[6]: scope 0 at $DIR/array-index-is-temporary.rs:17:1: 17:2
2020-04-13T22:35:31.3862841Z 
2020-04-13T22:35:31.3863885Z thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array-index-is-temporary/32bit/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3864874Z 
2020-04-13T22:35:31.3865310Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-04-13T22:35:31.3865310Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-04-13T22:35:31.3865767Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-13T22:35:31.3866313Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-13T22:35:31.3866861Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-13T22:35:31.3872978Z -         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation.rs:7:11: 9:2
2020-04-13T22:35:31.3877079Z +         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation.rs:7:11: 9:2
2020-04-13T22:35:31.3877558Z +                                          // ty::Const
2020-04-13T22:35:31.3877858Z +                                          // + ty: ()
2020-04-13T22:35:31.3878195Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3878562Z +                                          // mir::Constant
2020-04-13T22:35:31.3878975Z +                                          // + span: $DIR/const_allocation.rs:7:11: 9:2
2020-04-13T22:35:31.3879662Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3880210Z 22         return;                          // bb0[7]: scope 0 at $DIR/const_allocation.rs:9:2: 9:2
2020-04-13T22:35:31.3880678Z 24 }
2020-04-13T22:35:31.3880783Z 
2020-04-13T22:35:31.3880783Z 
2020-04-13T22:35:31.3881936Z thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3882894Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-13T22:35:31.3882894Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-13T22:35:31.3883357Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-13T22:35:31.3883892Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-13T22:35:31.3884434Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-13T22:35:31.3885262Z -         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
2020-04-13T22:35:31.3885821Z +         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
2020-04-13T22:35:31.3886331Z +                                          // ty::Const
2020-04-13T22:35:31.3886629Z +                                          // + ty: ()
2020-04-13T22:35:31.3887060Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3887440Z +                                          // mir::Constant
2020-04-13T22:35:31.3887847Z +                                          // + span: $DIR/const_allocation2.rs:4:11: 6:2
2020-04-13T22:35:31.3888357Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3888906Z 22         return;                          // bb0[7]: scope 0 at $DIR/const_allocation2.rs:6:2: 6:2
2020-04-13T22:35:31.3889385Z 24 }
2020-04-13T22:35:31.3889486Z 
2020-04-13T22:35:31.3889486Z 
2020-04-13T22:35:31.3890434Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3891385Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-13T22:35:31.3891385Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-13T22:35:31.3891847Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-13T22:35:31.3892385Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-13T22:35:31.3892932Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-13T22:35:31.3893751Z -         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
2020-04-13T22:35:31.3894300Z +         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
2020-04-13T22:35:31.3894720Z +                                          // ty::Const
2020-04-13T22:35:31.3895048Z +                                          // + ty: ()
2020-04-13T22:35:31.3895389Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3895762Z +                                          // mir::Constant
2020-04-13T22:35:31.3896167Z +                                          // + span: $DIR/const_allocation3.rs:4:11: 6:2
2020-04-13T22:35:31.3896674Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3897220Z 22         return;                          // bb0[7]: scope 0 at $DIR/const_allocation3.rs:6:2: 6:2
2020-04-13T22:35:31.3897688Z 24 }
2020-04-13T22:35:31.3897882Z 
2020-04-13T22:35:31.3897882Z 
2020-04-13T22:35:31.3898883Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3899838Z ---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
2020-04-13T22:35:31.3899838Z ---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
2020-04-13T22:35:31.3900331Z 84 +                                          // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
2020-04-13T22:35:31.3900894Z 85           StorageDead(_3);                 // bb1[1]: scope 0 at $DIR/array_index.rs:5:33: 5:34
2020-04-13T22:35:31.3901430Z 86           StorageDead(_2);                 // bb1[2]: scope 0 at $DIR/array_index.rs:5:33: 5:34
2020-04-13T22:35:31.3902238Z -           _0 = ();                         // bb1[3]: scope 0 at $DIR/array_index.rs:4:11: 6:2
2020-04-13T22:35:31.3902753Z +           _0 = const ();                   // bb1[3]: scope 0 at $DIR/array_index.rs:4:11: 6:2
2020-04-13T22:35:31.3903165Z +                                            // ty::Const
2020-04-13T22:35:31.3903483Z +                                            // + ty: ()
2020-04-13T22:35:31.3903827Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3904184Z +                                            // mir::Constant
2020-04-13T22:35:31.3904595Z +                                            // + span: $DIR/array_index.rs:4:11: 6:2
2020-04-13T22:35:31.3905179Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3905733Z 88           StorageDead(_1);                 // bb1[4]: scope 0 at $DIR/array_index.rs:6:1: 6:2
2020-04-13T22:35:31.3906239Z 89           return;                          // bb1[5]: scope 0 at $DIR/array_index.rs:6:2: 6:2
2020-04-13T22:35:31.3906676Z 
2020-04-13T22:35:31.3906676Z 
2020-04-13T22:35:31.3907651Z thread '[mir-opt] mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3921967Z ---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
2020-04-13T22:35:31.3921967Z ---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
2020-04-13T22:35:31.3922543Z 87                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
2020-04-13T22:35:31.3923229Z 88           StorageDead(_2);                 // bb4[1]: scope 0 at $DIR/discriminant.rs:6:67: 6:68
2020-04-13T22:35:31.3923857Z 89           StorageDead(_3);                 // bb4[2]: scope 0 at $DIR/discriminant.rs:6:68: 6:69
2020-04-13T22:35:31.3924815Z -           _0 = ();                         // bb4[3]: scope 0 at $DIR/discriminant.rs:5:11: 7:2
2020-04-13T22:35:31.3925459Z +           _0 = const ();                   // bb4[3]: scope 0 at $DIR/discriminant.rs:5:11: 7:2
2020-04-13T22:35:31.3925971Z +                                            // ty::Const
2020-04-13T22:35:31.3926304Z +                                            // + ty: ()
2020-04-13T22:35:31.3926715Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3927151Z +                                            // mir::Constant
2020-04-13T22:35:31.3927600Z +                                            // + span: $DIR/discriminant.rs:5:11: 7:2
2020-04-13T22:35:31.3928216Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3928870Z 91           StorageDead(_1);                 // bb4[4]: scope 0 at $DIR/discriminant.rs:7:1: 7:2
2020-04-13T22:35:31.3929504Z 92           return;                          // bb4[5]: scope 0 at $DIR/discriminant.rs:7:2: 7:2
2020-04-13T22:35:31.3930039Z 
2020-04-13T22:35:31.3930039Z 
2020-04-13T22:35:31.3931067Z thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3932334Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2020-04-13T22:35:31.3932334Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2020-04-13T22:35:31.3932897Z 171 +                                          // + span: $DIR/optimizes_into_variable.rs:14:13: 14:38
2020-04-13T22:35:31.3933591Z 172 +                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002a)) }
2020-04-13T22:35:31.3934335Z 173           StorageDead(_9);                 // bb2[7]: scope 2 at $DIR/optimizes_into_variable.rs:14:38: 14:39
2020-04-13T22:35:31.3935376Z -           _0 = ();                         // bb2[8]: scope 0 at $DIR/optimizes_into_variable.rs:11:11: 15:2
2020-04-13T22:35:31.3936103Z +           _0 = const ();                   // bb2[8]: scope 0 at $DIR/optimizes_into_variable.rs:11:11: 15:2
2020-04-13T22:35:31.3936659Z +                                            // ty::Const
2020-04-13T22:35:31.3936975Z +                                            // + ty: ()
2020-04-13T22:35:31.3937386Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3937829Z +                                            // mir::Constant
2020-04-13T22:35:31.3938330Z +                                            // + span: $DIR/optimizes_into_variable.rs:11:11: 15:2
2020-04-13T22:35:31.3939080Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3939805Z 175           StorageDead(_8);                 // bb2[9]: scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-13T22:35:31.3940519Z 176           StorageDead(_3);                 // bb2[10]: scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-13T22:35:31.3941213Z 177           StorageDead(_1);                 // bb2[11]: scope 0 at $DIR/optimizes_into_variable.rs:15:1: 15:2
2020-04-13T22:35:31.3941625Z 
2020-04-13T22:35:31.3942729Z thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3943791Z ---- [mir-opt] mir-opt/const_prop/repeat.rs stdout ----
2020-04-13T22:35:31.3943791Z ---- [mir-opt] mir-opt/const_prop/repeat.rs stdout ----
2020-04-13T22:35:31.3944270Z 80           StorageDead(_2);                 // bb1[2]: scope 0 at $DIR/repeat.rs:6:31: 6:32
2020-04-13T22:35:31.3944876Z 81           StorageDead(_4);                 // bb1[3]: scope 0 at $DIR/repeat.rs:6:32: 6:33
2020-04-13T22:35:31.3945495Z 82           StorageDead(_3);                 // bb1[4]: scope 0 at $DIR/repeat.rs:6:32: 6:33
2020-04-13T22:35:31.3946506Z -           _0 = ();                         // bb1[5]: scope 0 at $DIR/repeat.rs:5:11: 7:2
2020-04-13T22:35:31.3947089Z +           _0 = const ();                   // bb1[5]: scope 0 at $DIR/repeat.rs:5:11: 7:2
2020-04-13T22:35:31.3947562Z +                                            // ty::Const
2020-04-13T22:35:31.3948247Z +                                            // + ty: ()
2020-04-13T22:35:31.3948665Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3949087Z +                                            // mir::Constant
2020-04-13T22:35:31.3949556Z +                                            // + span: $DIR/repeat.rs:5:11: 7:2
2020-04-13T22:35:31.3950147Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3950771Z 84           StorageDead(_1);                 // bb1[6]: scope 0 at $DIR/repeat.rs:7:1: 7:2
2020-04-13T22:35:31.3951374Z 85           return;                          // bb1[7]: scope 0 at $DIR/repeat.rs:7:2: 7:2
2020-04-13T22:35:31.3951880Z 
2020-04-13T22:35:31.3951880Z 
2020-04-13T22:35:31.3952925Z thread '[mir-opt] mir-opt/const_prop/repeat.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/repeat/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3954075Z ---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
2020-04-13T22:35:31.3954075Z ---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
2020-04-13T22:35:31.3954578Z 76           StorageDead(_4);                 // bb1[2]: scope 0 at $DIR/slice_len.rs:5:33: 5:34
2020-04-13T22:35:31.3955216Z 77           StorageDead(_2);                 // bb1[3]: scope 0 at $DIR/slice_len.rs:5:33: 5:34
2020-04-13T22:35:31.3955836Z 78           StorageDead(_1);                 // bb1[4]: scope 0 at $DIR/slice_len.rs:5:33: 5:34
2020-04-13T22:35:31.3956759Z -           _0 = ();                         // bb1[5]: scope 0 at $DIR/slice_len.rs:4:11: 6:2
2020-04-13T22:35:31.3957385Z +           _0 = const ();                   // bb1[5]: scope 0 at $DIR/slice_len.rs:4:11: 6:2
2020-04-13T22:35:31.3957873Z +                                            // ty::Const
2020-04-13T22:35:31.3958234Z +                                            // + ty: ()
2020-04-13T22:35:31.3958644Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3959069Z +                                            // mir::Constant
2020-04-13T22:35:31.3959522Z +                                            // + span: $DIR/slice_len.rs:4:11: 6:2
2020-04-13T22:35:31.3960109Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3960825Z 80           return;                          // bb1[6]: scope 0 at $DIR/slice_len.rs:6:2: 6:2
2020-04-13T22:35:31.3961405Z 82   }
2020-04-13T22:35:31.3961525Z 
2020-04-13T22:35:31.3961525Z 
2020-04-13T22:35:31.3962559Z thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len/32bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.3963599Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-13T22:35:31.3963599Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-13T22:35:31.3964402Z 29 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
2020-04-13T22:35:31.3965186Z 30 -                                          // + user_ty: UserType(1)
2020-04-13T22:35:31.3966277Z 31 -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3967102Z + -     }
2020-04-13T22:35:31.3967434Z + - 
2020-04-13T22:35:31.3967830Z + -     bb1 (cleanup): {
2020-04-13T22:35:31.3968592Z + -         resume;                          // bb1[0]: scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
2020-04-13T22:35:31.3969190Z + -     }
2020-04-13T22:35:31.3969513Z + - 
2020-04-13T22:35:31.3969894Z + -     bb2: {
2020-04-13T22:35:31.3975966Z + -         _1 = move _2;                    // bb2[0]: scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
2020-04-13T22:35:31.3976892Z + -         StorageDead(_2);                 // bb2[1]: scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
2020-04-13T22:35:31.3977771Z + -         _0 = const ();                   // bb2[2]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-13T22:35:31.3978315Z 32 +                                          // + span: $SRC_DIR/liballoc/vec.rs:LL:COL
2020-04-13T22:35:31.3978728Z 33 +                                          // + user_ty: UserType(0)
2020-04-13T22:35:31.3980124Z 34 +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
2020-04-13T22:35:31.3981324Z 
2020-04-13T22:35:31.3981692Z 35 +         ((*_4).1: usize) = const 0usize; // bb0[5]: scope 2 at $SRC_DIR/liballoc/vec.rs:LL:COL
2020-04-13T22:35:31.3982574Z - +                                          // ty::Const
2020-04-13T22:35:31.3982894Z +                                            // ty::Const
2020-04-13T22:35:31.3983240Z 37 +                                          // + ty: usize
2020-04-13T22:35:31.3983608Z 38 +                                          // + val: Value(Scalar(0x00000000))
2020-04-13T22:35:31.3983989Z 39 +                                          // mir::Constant
2020-04-13T22:35:31.3984207Z 
2020-04-13T22:35:31.3984560Z 41 +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-13T22:35:31.3985427Z 42 +         _1 = move _2;                    // bb0[6]: scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
2020-04-13T22:35:31.3986307Z 43 +         StorageDead(_2);                 // bb0[7]: scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
2020-04-13T22:35:31.3987167Z - +         _0 = ();                         // bb0[8]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-13T22:35:31.3988182Z + +         _0 = const ();                   // bb0[8]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-13T22:35:31.3988668Z + +                                          // ty::Const
2020-04-13T22:35:31.3988972Z +                                            // + ty: ()
2020-04-13T22:35:31.3989316Z +                                            // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.3989798Z +                                            // mir::Constant
2020-04-13T22:35:31.3990550Z +                                            // + span: $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-13T22:35:31.3991085Z +                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.3991969Z + -         drop(_1) -> [return: bb3, unwind: bb1]; // bb2[3]: scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
2020-04-13T22:35:31.3996983Z 45 +         drop(_1) -> [return: bb2, unwind: bb1]; // bb0[9]: scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
2020-04-13T22:35:31.3997571Z 47   
2020-04-13T22:35:31.3997686Z 
2020-04-13T22:35:31.3997686Z 
2020-04-13T22:35:31.3998064Z -       bb1 (cleanup): {
2020-04-13T22:35:31.3998757Z -           resume;                          // bb1[0]: scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
2020-04-13T22:35:31.3999569Z -   
2020-04-13T22:35:31.3999913Z -       bb2: {
2020-04-13T22:35:31.3999913Z -       bb2: {
2020-04-13T22:35:31.4000588Z - -         _1 = move _2;                    // bb2[0]: scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
2020-04-13T22:35:31.4001436Z - -         StorageDead(_2);                 // bb2[1]: scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
2020-04-13T22:35:31.4002307Z - -         _0 = ();                         // bb2[2]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-13T22:35:31.4003187Z - -         drop(_1) -> [return: bb3, unwind: bb1]; // bb2[3]: scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
2020-04-13T22:35:31.4004088Z - - 
2020-04-13T22:35:31.4004423Z 59 -     bb3: {
2020-04-13T22:35:31.4004423Z 59 -     bb3: {
2020-04-13T22:35:31.4005080Z 60 -         StorageDead(_1);                 // bb3[0]: scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
2020-04-13T22:35:31.4005948Z 61 -         return;                          // bb3[1]: scope 0 at $DIR/inline-into-box-place.rs:9:2: 9:2
2020-04-13T22:35:31.4006570Z - -     }
2020-04-13T22:35:31.4006875Z - - 
2020-04-13T22:35:31.4006875Z - - 
2020-04-13T22:35:31.4007081Z + +     bb1 (cleanup): {
2020-04-13T22:35:31.4007757Z + +         resume;                          // bb1[0]: scope 0 at $DIR/inline-into-box-place.rs:7:1: 9:2
2020-04-13T22:35:31.4008289Z +   
2020-04-13T22:35:31.4008289Z +   
2020-04-13T22:35:31.4008657Z 64 -     bb4 (cleanup): {
2020-04-13T22:35:31.4009649Z 65 -         _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb1; // bb4[0]: scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
2020-04-13T22:35:31.4010750Z 66 -                                          // ty::Const
2020-04-13T22:35:31.4011411Z 69 -                                          // mir::Constant
2020-04-13T22:35:31.4011411Z 69 -                                          // mir::Constant
2020-04-13T22:35:31.4012120Z 70 -                                          // + span: $DIR/inline-into-box-place.rs:8:42: 8:43
2020-04-13T22:35:31.4013303Z 71 -                                          // + literal: Const { ty: unsafe fn(std::ptr::Unique<std::vec::Vec<u32>>) {alloc::alloc::box_free::<std::vec::Vec<u32>>}, val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.4013968Z + +     bb2: {
2020-04-13T22:35:31.4014628Z 72 +         StorageDead(_1);                 // bb2[0]: scope 0 at $DIR/inline-into-box-place.rs:9:1: 9:2
2020-04-13T22:35:31.4015485Z 73 +         return;                          // bb2[1]: scope 0 at $DIR/inline-into-box-place.rs:9:2: 9:2
2020-04-13T22:35:31.4015973Z 
2020-04-13T22:35:31.4015973Z 
2020-04-13T22:35:31.4016957Z thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-into-box-place/32bit/rustc.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.4017971Z ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
2020-04-13T22:35:31.4018237Z 137     bb6: {
2020-04-13T22:35:31.4018237Z 137     bb6: {
2020-04-13T22:35:31.4019014Z 138         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:19:17: 19:18
2020-04-13T22:35:31.4019926Z 139         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:19:18: 19:19
2020-04-13T22:35:31.4020811Z -         _0 = ();                         // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-13T22:35:31.4021749Z +         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-13T22:35:31.4022261Z +                                          // ty::Const
2020-04-13T22:35:31.4022557Z +                                          // + ty: ()
2020-04-13T22:35:31.4022912Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.4024213Z +                                          // mir::Constant
2020-04-13T22:35:31.4025019Z +                                          // + span: $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-13T22:35:31.4025606Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.4026446Z 141         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-13T22:35:31.4026987Z 143 
2020-04-13T22:35:31.4027296Z 
2020-04-13T22:35:31.4027456Z 144     bb7: {
2020-04-13T22:35:31.4027456Z 144     bb7: {
2020-04-13T22:35:31.4028417Z 145         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
2020-04-13T22:35:31.4029331Z -         _0 = ();                         // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-13T22:35:31.4030251Z +         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-13T22:35:31.4030769Z +                                          // ty::Const
2020-04-13T22:35:31.4031073Z +                                          // + ty: ()
2020-04-13T22:35:31.4031415Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.4031798Z +                                          // mir::Constant
2020-04-13T22:35:31.4032488Z +                                          // + span: $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-13T22:35:31.4033024Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.4033869Z 147         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-13T22:35:31.4034518Z 149 
2020-04-13T22:35:31.4034639Z 
2020-04-13T22:35:31.4034639Z 
2020-04-13T22:35:31.4035612Z thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region-subtyping-basic/32bit/rustc.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.4036623Z ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
2020-04-13T22:35:31.4036623Z ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
2020-04-13T22:35:31.4037383Z 61         StorageDead(_6);                 // bb4[0]: scope 1 at $DIR/packed-struct-drop-aligned.rs:7:5: 7:8
2020-04-13T22:35:31.4038256Z 62         (_1.0: Aligned) = move _4;       // bb4[1]: scope 1 at $DIR/packed-struct-drop-aligned.rs:7:5: 7:8
2020-04-13T22:35:31.4039172Z 63         StorageDead(_4);                 // bb4[2]: scope 1 at $DIR/packed-struct-drop-aligned.rs:7:28: 7:29
2020-04-13T22:35:31.4040037Z -         _0 = ();                         // bb4[3]: scope 0 at $DIR/packed-struct-drop-aligned.rs:5:11: 8:2
2020-04-13T22:35:31.4040928Z +         _0 = const ();                   // bb4[3]: scope 0 at $DIR/packed-struct-drop-aligned.rs:5:11: 8:2
2020-04-13T22:35:31.4041388Z +                                          // ty::Const
2020-04-13T22:35:31.4041723Z +                                          // + ty: ()
2020-04-13T22:35:31.4042077Z +                                          // + val: Value(Scalar(<ZST>))
2020-04-13T22:35:31.4042517Z +                                          // mir::Constant
2020-04-13T22:35:31.4043327Z +                                          // + span: $DIR/packed-struct-drop-aligned.rs:5:11: 8:2
2020-04-13T22:35:31.4044787Z +                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-13T22:35:31.4045808Z 65         drop(_1) -> [return: bb2, unwind: bb1]; // bb4[4]: scope 0 at $DIR/packed-struct-drop-aligned.rs:8:1: 8:2
2020-04-13T22:35:31.4046377Z 67 }
2020-04-13T22:35:31.4046482Z 
2020-04-13T22:35:31.4046482Z 
2020-04-13T22:35:31.4047482Z thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/packed-struct-drop-aligned/32bit/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-13T22:35:31.4048183Z 
2020-04-13T22:35:31.4048325Z failures:
2020-04-13T22:35:31.4048758Z     [mir-opt] mir-opt/array-index-is-temporary.rs
2020-04-13T22:35:31.4049253Z     [mir-opt] mir-opt/const_allocation.rs
---
2020-04-13T22:35:31.4055178Z 
2020-04-13T22:35:31.4055706Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-13T22:35:31.4055965Z 
2020-04-13T22:35:31.4056066Z 
2020-04-13T22:35:31.4060112Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-13T22:35:31.4063065Z 
2020-04-13T22:35:31.4063186Z 
2020-04-13T22:35:31.4063186Z 
2020-04-13T22:35:31.4063890Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-13T22:35:31.4064665Z == clock drift check ==
2020-04-13T22:35:31.4065002Z   local time: Mon Apr 13 22:35:31 UTC 2020
2020-04-13T22:35:31.4065002Z   local time: Mon Apr 13 22:35:31 UTC 2020
2020-04-13T22:35:32.3601509Z   network time: Mon, 13 Apr 2020 22:35:31 GMT
2020-04-13T22:35:33.9021292Z 
2020-04-13T22:35:33.9021292Z 
2020-04-13T22:35:33.9112179Z ##[error]Bash exited with code '1'.
2020-04-13T22:35:33.9128451Z ##[section]Finishing: Run build
2020-04-13T22:35:33.9228263Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-13T22:35:33.9234125Z Task         : Get sources
2020-04-13T22:35:33.9234520Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T22:35:33.9234847Z Version      : 1.0.0
2020-04-13T22:35:33.9235084Z Author       : Microsoft
2020-04-13T22:35:33.9235084Z Author       : Microsoft
2020-04-13T22:35:33.9235553Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T22:35:33.9235968Z ==============================================================================
2020-04-13T22:35:34.2600566Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T22:35:34.2648074Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-13T22:35:34.2731394Z Cleaning up task key
2020-04-13T22:35:34.2732523Z Start cleaning up orphan processes.
2020-04-13T22:35:34.2902279Z Terminate orphan process: pid (3412) (python)
2020-04-13T22:35:34.3130392Z ##[section]Finishing: Finalize Job
