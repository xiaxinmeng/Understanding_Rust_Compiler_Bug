plain
2020-04-14T10:07:03.6174753Z ========================== Starting Command Output ===========================
2020-04-14T10:07:03.6178827Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d1cd4dfd-5e67-46cd-a033-077ce9a8b5c7.sh
2020-04-14T10:07:03.6179246Z 
2020-04-14T10:07:03.6183883Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T10:07:03.6209790Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-14T10:07:03.6212834Z Task         : Get sources
2020-04-14T10:07:03.6213122Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T10:07:03.6213385Z Version      : 1.0.0
2020-04-14T10:07:03.6213566Z Author       : Microsoft
---
2020-04-14T10:07:04.6120984Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T10:07:04.6126627Z ##[command]git config gc.auto 0
2020-04-14T10:07:04.6130288Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T10:07:04.6133463Z ##[command]git config --get-all http.proxy
2020-04-14T10:07:04.6139660Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70950/merge:refs/remotes/pull/70950/merge
---
2020-04-14T10:10:30.9004758Z  ---> f58a2bb1e753
2020-04-14T10:10:30.9009548Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T10:10:30.9013170Z  ---> Using cache
2020-04-14T10:10:30.9016281Z  ---> d079cc6b6db8
2020-04-14T10:10:30.9021054Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T10:10:30.9028054Z  ---> 4183ca46ee56
2020-04-14T10:10:30.9031316Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T10:10:30.9034371Z  ---> Using cache
2020-04-14T10:10:30.9037175Z  ---> 69e7f8a2a2fb
---
2020-04-14T10:10:30.9410783Z Looks like docker image is the same as before, not uploading
2020-04-14T10:10:38.2002176Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T10:10:38.2327099Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T10:10:38.2353284Z == clock drift check ==
2020-04-14T10:10:38.2366486Z   local time: Tue Apr 14 10:10:38 UTC 2020
2020-04-14T10:10:38.5464562Z   network time: Tue, 14 Apr 2020 10:10:38 GMT
2020-04-14T10:10:38.5485880Z Starting sccache server...
2020-04-14T10:10:38.6383978Z configure: processing command line
2020-04-14T10:10:38.6384807Z configure: 
2020-04-14T10:10:38.6385849Z configure: rust.dist-src        := False
---
2020-04-14T10:16:18.3394962Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T10:16:19.9751035Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T10:16:21.6860672Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T10:16:23.5459822Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T10:16:33.0540565Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T10:16:36.1055759Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T10:16:41.0805630Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T10:16:45.7195022Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T10:16:55.6807022Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T10:41:49.9040587Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T10:41:51.8849472Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T10:41:53.9792907Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T10:41:57.1268556Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T10:42:07.7423095Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T10:42:12.5561895Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T10:42:18.2932654Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T10:42:24.2947829Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T10:42:35.1983327Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T11:10:29.4925148Z .................................................................................................... 1700/9896
2020-04-14T11:10:34.1837330Z .................................................................................................... 1800/9896
2020-04-14T11:10:43.0511423Z .................................................................................................... 1900/9896
2020-04-14T11:10:51.5151404Z .....i.............................................................................................. 2000/9896
2020-04-14T11:10:58.2228009Z ...............................................................................................iiiii 2100/9896
2020-04-14T11:11:20.7549622Z .................................................................................................... 2300/9896
2020-04-14T11:11:23.0815586Z .................................................................................................... 2400/9896
2020-04-14T11:11:25.4277796Z .................................................................................................... 2500/9896
2020-04-14T11:11:31.2742460Z .................................................................................................... 2600/9896
---
2020-04-14T11:14:40.2214686Z .................................................................................................... 5100/9896
2020-04-14T11:14:48.5010474Z .................................................................................................... 5200/9896
2020-04-14T11:14:53.3385126Z ................i................................................................................... 5300/9896
2020-04-14T11:15:04.1709315Z ......i............................................................................................. 5400/9896
2020-04-14T11:15:09.6907315Z ......ii.ii........i...i............................................................................ 5500/9896
2020-04-14T11:15:17.7033990Z ....................................................i............................................... 5700/9896
2020-04-14T11:15:28.3667672Z ........................................................................ii.......................... 5800/9896
2020-04-14T11:15:35.0096785Z ...........i........................................................................................ 5900/9896
2020-04-14T11:15:40.6747238Z .................................................................................................... 6000/9896
2020-04-14T11:15:40.6747238Z .................................................................................................... 6000/9896
2020-04-14T11:15:51.4042474Z .................................................................................................... 6100/9896
2020-04-14T11:16:02.8710707Z .....ii...i..ii...........i......................................................................... 6200/9896
2020-04-14T11:16:18.7318089Z .................................................................................................... 6400/9896
2020-04-14T11:16:25.4403132Z .................................................................................................... 6500/9896
2020-04-14T11:16:25.4403132Z .................................................................................................... 6500/9896
2020-04-14T11:16:48.1374925Z .....................................i..ii.......................................................... 6600/9896
2020-04-14T11:17:10.2543689Z .................................................................................................... 6800/9896
2020-04-14T11:17:12.3837541Z .....................................i.............................................................. 6900/9896
2020-04-14T11:17:14.5424331Z .................................................................................................... 7000/9896
2020-04-14T11:17:16.6806616Z ............................................................................i....................... 7100/9896
---
2020-04-14T11:19:00.8765967Z .................................................................................................... 7800/9896
2020-04-14T11:19:05.2255650Z .................................................................................................... 7900/9896
2020-04-14T11:19:11.9637565Z .................................................................................................... 8000/9896
2020-04-14T11:19:18.5124340Z ..........................................i......................................................... 8100/9896
2020-04-14T11:19:28.7731059Z ..........................................................................................iiiiii.iii 8200/9896
2020-04-14T11:19:35.3896544Z ii.i................................................................................................ 8300/9896
2020-04-14T11:19:48.9676915Z .................................................................................................... 8500/9896
2020-04-14T11:19:59.1484541Z .................................................................................................... 8600/9896
2020-04-14T11:20:13.1977613Z .................................................................................................... 8700/9896
2020-04-14T11:20:20.2751413Z .................................................................................................... 8800/9896
---
2020-04-14T11:22:48.1830535Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-14T11:22:48.2004731Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T11:22:48.4192094Z 
2020-04-14T11:22:48.4192425Z running 185 tests
2020-04-14T11:22:51.2467470Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-14T11:22:54.0295161Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-14T11:22:54.0297743Z 
2020-04-14T11:22:54.0304634Z  finished in 5.830
2020-04-14T11:22:54.0309728Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-14T11:22:54.0516583Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T11:22:56.1654182Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-14T11:22:56.1838606Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T11:22:56.3404050Z 
2020-04-14T11:22:56.3405538Z running 9 tests
2020-04-14T11:22:56.3406551Z iiiiiiiii
2020-04-14T11:22:56.3407469Z 
2020-04-14T11:22:56.3407610Z  finished in 0.156
2020-04-14T11:22:56.3414429Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-14T11:22:56.3605183Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T11:23:17.7194997Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-14T11:23:17.7419063Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T11:23:17.9387942Z 
2020-04-14T11:23:17.9388284Z running 115 tests
2020-04-14T11:23:32.1955089Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-14T11:23:34.1288054Z ...iiii.....ii.
2020-04-14T11:23:34.1291225Z 
2020-04-14T11:23:34.1344033Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T11:23:34.1344834Z  finished in 16.388
2020-04-14T11:23:34.1345823Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T11:37:16.7627650Z 
2020-04-14T11:37:16.7632061Z    Doc-tests core
2020-04-14T11:37:21.8614161Z 
2020-04-14T11:37:21.8615240Z running 2494 tests
2020-04-14T11:37:31.4834399Z ......iiiii......................................................................................... 100/2494
2020-04-14T11:37:40.8688630Z .....................................................................................ii............. 200/2494
2020-04-14T11:38:02.5258828Z ....................i............................................................................... 400/2494
2020-04-14T11:38:02.5258828Z ....................i............................................................................... 400/2494
2020-04-14T11:38:12.8480937Z ..........................................................................i..i..................iiii 500/2494
2020-04-14T11:38:30.3591397Z .................................................................................................... 700/2494
2020-04-14T11:38:39.7177767Z .................................................................................................... 800/2494
2020-04-14T11:38:48.7808423Z .................................................................................................... 900/2494
2020-04-14T11:38:57.8260544Z .................................................................................................... 1000/2494
---
2020-04-14T11:42:33.0070760Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-14T11:42:33.0076365Z . 300/764
2020-04-14T11:42:33.1102941Z .................................................................................................... 400/764
2020-04-14T11:42:35.2023022Z .................................................................................................... 500/764
2020-04-14T11:42:35.2534623Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-14T11:42:35.2554656Z ....thread '<unnamed>' panicked at 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-14T11:42:35.2567898Z .called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:.21.
2020-04-14T11:42:35.2583302Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-14T11:42:35.5326066Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-14T11:42:35.5364618Z .....thread '.<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError.', src/libstd/sync/mpsc/mod.rs:.2034:21.
2020-04-14T11:42:35.5373229Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-14T11:42:37.5794701Z .........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-14T11:42:37.5795653Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-14T11:42:37.5796503Z ....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-14T11:42:37.5825327Z ...........thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-14T11:42:46.6888077Z 
2020-04-14T11:42:46.6888415Z running 1020 tests
2020-04-14T11:43:05.0234755Z i................................................................................................... 100/1020
2020-04-14T11:43:16.0630139Z .................................................................................................... 200/1020
2020-04-14T11:43:24.0530272Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-14T11:43:29.1090571Z .................................................................................................... 400/1020
2020-04-14T11:43:36.1308152Z ....................................................i....i......................................ii.. 500/1020
2020-04-14T11:43:49.4146719Z .................................................................................................... 700/1020
2020-04-14T11:43:49.4146719Z .................................................................................................... 700/1020
2020-04-14T11:43:56.8858763Z ..............................................iiii.................................................. 800/1020
2020-04-14T11:44:11.2557577Z .................................................................................................... 900/1020
2020-04-14T11:44:17.5902816Z ....................................................................iiii............................ 1000/1020
2020-04-14T11:44:18.9601110Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-14T11:44:18.9601492Z 
2020-04-14T11:44:18.9710920Z  finished in 175.253
2020-04-14T11:44:18.9720776Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-14T11:47:45.7299824Z 
2020-04-14T11:47:45.7300087Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T11:47:45.7300329Z 
2020-04-14T11:47:45.7373631Z  finished in 1.021
2020-04-14T11:47:45.7430743Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-14T11:47:45.7431999Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T11:47:45.9499291Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T11:47:47.0241280Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-efd115a6855c8648
2020-04-14T11:47:47.0271592Z 
2020-04-14T11:47:47.0271825Z running 0 tests
2020-04-14T11:47:47.0271972Z 
---
2020-04-14T12:03:29.1056917Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1057623Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1058748Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1059479Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1060196Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1061664Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1062597Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1063316Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T12:03:29.1064050Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-14T12:04:34.7584225Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T12:04:34.7991169Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T12:04:35.0646696Z 
2020-04-14T12:04:35.0648034Z running 211 tests
2020-04-14T12:05:08.1102437Z ......................i...ii.......................................................................i 100/211
2020-04-14T12:05:48.6446971Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-14T12:05:53.1320174Z .......ii..
2020-04-14T12:05:53.1324226Z 
2020-04-14T12:05:53.1324516Z  finished in 78.333
2020-04-14T12:05:53.1348363Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-14T12:05:53.1350426Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-14T12:07:29.0403115Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-14T12:07:29.0616834Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T12:07:29.2520183Z 
2020-04-14T12:07:29.2524123Z running 13 tests
2020-04-14T12:07:29.7298660Z .iiiiiii.iii.
2020-04-14T12:07:29.7302699Z 
2020-04-14T12:07:29.7303876Z  finished in 0.669
2020-04-14T12:07:29.7367004Z Build completed successfully in 1:55:10
2020-04-14T12:07:29.7367004Z Build completed successfully in 1:55:10
2020-04-14T12:07:29.7377203Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-14T12:07:31.9447193Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T12:07:32.5892527Z     Finished release [optimized] target(s) in 0.63s
2020-04-14T12:07:32.5992743Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-14T12:07:32.6127978Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T12:09:03.3330081Z failures:
2020-04-14T12:09:03.3330202Z 
2020-04-14T12:09:03.3331002Z ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
2020-04-14T12:09:03.3331268Z 7 | Inferred Region Values
2020-04-14T12:09:03.3332072Z 8 | '_#0r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#0r, '_#1r}
2020-04-14T12:09:03.3332819Z 9 | '_#1r | U0 | {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5], '_#1r}
2020-04-14T12:09:03.3333381Z - | '_#2r | U0 | {bb2[0..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3333812Z - | '_#3r | U0 | {bb2[1..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3334235Z - | '_#4r | U0 | {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3334613Z + | '_#2r | U0 | {}
2020-04-14T12:09:03.3334993Z + | '_#3r | U0 | {bb2[0..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3335415Z + | '_#4r | U0 | {bb2[1..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3335852Z + | '_#5r | U0 | {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3336196Z 14 | Inference Constraints
2020-04-14T12:09:03.3336196Z 14 | Inference Constraints
2020-04-14T12:09:03.3336775Z 15 | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
2020-04-14T12:09:03.3337050Z 
2020-04-14T12:09:03.3337574Z 16 | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=8], bb3[0], bb4[0..=1], bb5[0..=3], bb6[0..=3], bb7[0..=2], bb8[0..=5]}
2020-04-14T12:09:03.3338056Z - | '_#2r live at {bb2[0]}
2020-04-14T12:09:03.3338404Z - | '_#3r live at {bb2[1..=3]}
2020-04-14T12:09:03.3338802Z - | '_#4r live at {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3339249Z - | '_#2r: '_#3r due to Assignment at Single(bb2[0])
2020-04-14T12:09:03.3339689Z - | '_#3r: '_#4r due to Assignment at Single(bb2[3])
2020-04-14T12:09:03.3340060Z + | '_#3r live at {bb2[0]}
2020-04-14T12:09:03.3340405Z + | '_#4r live at {bb2[1..=3]}
2020-04-14T12:09:03.3340822Z + | '_#5r live at {bb2[4..=8], bb3[0], bb5[0..=2]}
2020-04-14T12:09:03.3341253Z + | '_#3r: '_#4r due to Assignment at Single(bb2[0])
2020-04-14T12:09:03.3341814Z + | '_#4r: '_#5r due to Assignment at Single(bb2[3])
2020-04-14T12:09:03.3342466Z 23 fn main() -> () {
2020-04-14T12:09:03.3343046Z -     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:14:11: 14:11
2020-04-14T12:09:03.3343046Z -     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:14:11: 14:11
2020-04-14T12:09:03.3343880Z -     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-14T12:09:03.3344754Z -     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-14T12:09:03.3345443Z -     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-14T12:09:03.3346109Z -     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-14T12:09:03.3346766Z -     let mut _7: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-14T12:09:03.3347446Z -     let _8: bool;                        // in scope 0 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-14T12:09:03.3348121Z -     let mut _9: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-14T12:09:03.3349454Z -     let _10: bool;                       // in scope 0 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3350217Z +     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11
2020-04-14T12:09:03.3351044Z +     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-14T12:09:03.3351799Z +     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-14T12:09:03.3352587Z +     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-14T12:09:03.3353366Z +     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-14T12:09:03.3354082Z +     let mut _7: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-14T12:09:03.3354749Z +     let _8: bool;                        // in scope 0 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3355405Z +     let mut _9: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-14T12:09:03.3356083Z +     let _10: bool;                       // in scope 0 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-14T12:09:03.3357016Z -         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-14T12:09:03.3357016Z -         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-14T12:09:03.3357707Z -         let _2: &'_#3r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-14T12:09:03.3358391Z +         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-14T12:09:03.3359074Z +         let _2: &'_#4r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-14T12:09:03.3359951Z -             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-14T12:09:03.3359951Z -             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-14T12:09:03.3360621Z -             let _6: &'_#4r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-14T12:09:03.3361310Z +             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-14T12:09:03.3361985Z +             let _6: &'_#5r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-14T12:09:03.3362329Z 39             scope 3 {
2020-04-14T12:09:03.3362869Z -                 debug q => _6;           // in scope 3 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-14T12:09:03.3363553Z +                 debug q => _6;           // in scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-14T12:09:03.3364032Z 42         }
2020-04-14T12:09:03.3364262Z 43     }
2020-04-14T12:09:03.3364365Z 
2020-04-14T12:09:03.3364490Z 44 
2020-04-14T12:09:03.3364490Z 44 
2020-04-14T12:09:03.3364627Z 45     bb0: {
2020-04-14T12:09:03.3365252Z -         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-14T12:09:03.3366423Z -         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:15:17: 15:26
2020-04-14T12:09:03.3367465Z +         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-14T12:09:03.3368639Z +         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:17:17: 17:26
2020-04-14T12:09:03.3369330Z 48                                          // ty::Const
2020-04-14T12:09:03.3369638Z 49                                          // + ty: usize
2020-04-14T12:09:03.3371161Z 50                                          // + val: Value(Scalar(0x00000001))
2020-04-14T12:09:03.3371640Z 51                                          // mir::Constant
2020-04-14T12:09:03.3371640Z 51                                          // mir::Constant
2020-04-14T12:09:03.3372431Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:18: 15:19
2020-04-14T12:09:03.3374978Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:18: 17:19
2020-04-14T12:09:03.3375552Z 53                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
2020-04-14T12:09:03.3376005Z 54                                          // ty::Const
2020-04-14T12:09:03.3376286Z 55                                          // + ty: usize
2020-04-14T12:09:03.3376473Z 
2020-04-14T12:09:03.3376862Z 56                                          // + val: Value(Scalar(0x00000002))
2020-04-14T12:09:03.3377226Z 57                                          // mir::Constant
2020-04-14T12:09:03.3377997Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:21: 15:22
2020-04-14T12:09:03.3378751Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:21: 17:22
2020-04-14T12:09:03.3379287Z 59                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-14T12:09:03.3379716Z 60                                          // ty::Const
2020-04-14T12:09:03.3380025Z 61                                          // + ty: usize
2020-04-14T12:09:03.3380209Z 
2020-04-14T12:09:03.3380450Z 62                                          // + val: Value(Scalar(0x00000003))
2020-04-14T12:09:03.3380814Z 63                                          // mir::Constant
2020-04-14T12:09:03.3381446Z -                                          // + span: $DIR/region-subtyping-basic.rs:15:24: 15:25
2020-04-14T12:09:03.3382177Z +                                          // + span: $DIR/region-subtyping-basic.rs:17:24: 17:25
2020-04-14T12:09:03.3382735Z 65                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000003)) }
2020-04-14T12:09:03.3383522Z -         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:15:9: 15:14
2020-04-14T12:09:03.3384300Z -         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-14T12:09:03.3385106Z -         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-14T12:09:03.3385977Z -         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-14T12:09:03.3386840Z +         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
2020-04-14T12:09:03.3387628Z +         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-14T12:09:03.3388741Z +         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-14T12:09:03.3389689Z +         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-14T12:09:03.3390182Z 70                                          // ty::Const
2020-04-14T12:09:03.3390471Z 71                                          // + ty: usize
2020-04-14T12:09:03.3390827Z 72                                          // + val: Value(Scalar(0x00000000))
2020-04-14T12:09:03.3391273Z 73                                          // mir::Constant
2020-04-14T12:09:03.3391273Z 73                                          // mir::Constant
2020-04-14T12:09:03.3391927Z -                                          // + span: $DIR/region-subtyping-basic.rs:16:16: 16:17
2020-04-14T12:09:03.3392663Z +                                          // + span: $DIR/region-subtyping-basic.rs:18:16: 18:17
2020-04-14T12:09:03.3393197Z 75                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-14T12:09:03.3394054Z -         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-14T12:09:03.3394860Z -         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-14T12:09:03.3396912Z -         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:16:14: 16:18
2020-04-14T12:09:03.3397953Z +         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-14T12:09:03.3399659Z +         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-14T12:09:03.3400896Z +         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
2020-04-14T12:09:03.3401581Z 80 
2020-04-14T12:09:03.3401581Z 80 
2020-04-14T12:09:03.3401759Z 81     bb1 (cleanup): {
2020-04-14T12:09:03.3401900Z 
2020-04-14T12:09:03.3402498Z -         resume;                          // bb1[0]: scope 0 at $DIR/region-subtyping-basic.rs:14:1: 23:2
2020-04-14T12:09:03.3403294Z +         resume;                          // bb1[0]: scope 0 at $DIR/region-subtyping-basic.rs:16:1: 25:2
2020-04-14T12:09:03.3403756Z 84 
2020-04-14T12:09:03.3403897Z 85     bb2: {
2020-04-14T12:09:03.3404031Z 
2020-04-14T12:09:03.3404031Z 
2020-04-14T12:09:03.3404599Z -         _2 = &'_#2r _1[_3];              // bb2[0]: scope 1 at $DIR/region-subtyping-basic.rs:16:13: 16:18
2020-04-14T12:09:03.3405389Z -         FakeRead(ForLet, _2);            // bb2[1]: scope 1 at $DIR/region-subtyping-basic.rs:16:9: 16:10
2020-04-14T12:09:03.3406179Z -         StorageLive(_6);                 // bb2[2]: scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-14T12:09:03.3407679Z -         _6 = _2;                         // bb2[3]: scope 2 at $DIR/region-subtyping-basic.rs:17:13: 17:14
2020-04-14T12:09:03.3408561Z -         FakeRead(ForLet, _6);            // bb2[4]: scope 2 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-14T12:09:03.3409381Z -         StorageLive(_7);                 // bb2[5]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-14T12:09:03.3410221Z -         _7 = const Const(Value(Scalar(0x01)): bool); // bb2[6]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-14T12:09:03.3411069Z +         _2 = &'_#3r _1[_3];              // bb2[0]: scope 1 at $DIR/region-subtyping-basic.rs:18:13: 18:18
2020-04-14T12:09:03.3411856Z +         FakeRead(ForLet, _2);            // bb2[1]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
2020-04-14T12:09:03.3412650Z +         StorageLive(_6);                 // bb2[2]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-14T12:09:03.3413620Z +         _6 = _2;                         // bb2[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
2020-04-14T12:09:03.3414503Z +         FakeRead(ForLet, _6);            // bb2[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
2020-04-14T12:09:03.3415258Z +         StorageLive(_7);                 // bb2[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-14T12:09:03.3416084Z +         _7 = const Const(Value(Scalar(0x01)): bool); // bb2[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-14T12:09:03.3416543Z 93                                          // ty::Const
2020-04-14T12:09:03.3416948Z 94                                          // + ty: bool
2020-04-14T12:09:03.3417294Z 95                                          // + val: Value(Scalar(0x01))
2020-04-14T12:09:03.3417730Z 96                                          // mir::Constant
2020-04-14T12:09:03.3417730Z 96                                          // mir::Constant
2020-04-14T12:09:03.3418373Z -                                          // + span: $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-14T12:09:03.3419095Z +                                          // + span: $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-14T12:09:03.3419611Z 98                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
2020-04-14T12:09:03.3420400Z -         FakeRead(ForMatchedPlace, _7);   // bb2[7]: scope 3 at $DIR/region-subtyping-basic.rs:18:8: 18:12
2020-04-14T12:09:03.3421336Z -         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb3]; // bb2[8]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-14T12:09:03.3422241Z +         FakeRead(ForMatchedPlace, _7);   // bb2[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
2020-04-14T12:09:03.3423239Z +         switchInt(_7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb3]; // bb2[8]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-14T12:09:03.3423838Z 102 
2020-04-14T12:09:03.3423988Z 103     bb3: {
2020-04-14T12:09:03.3424109Z 
2020-04-14T12:09:03.3424109Z 
2020-04-14T12:09:03.3424747Z -         falseEdges -> [real: bb5, imaginary: bb4]; // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-14T12:09:03.3425639Z +         falseEdges -> [real: bb5, imaginary: bb4]; // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-14T12:09:03.3426158Z 106 
2020-04-14T12:09:03.3426317Z 107     bb4: {
2020-04-14T12:09:03.3426437Z 
2020-04-14T12:09:03.3426437Z 
2020-04-14T12:09:03.3427000Z -         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3428255Z -         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3429753Z +         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-14T12:09:03.3430945Z +         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
2020-04-14T12:09:03.3431631Z 110                                          // ty::Const
2020-04-14T12:09:03.3432175Z 111                                          // + ty: fn(usize) -> bool {use_x}
2020-04-14T12:09:03.3432578Z 112                                          // + val: Value(Scalar(<ZST>))
2020-04-14T12:09:03.3433016Z 113                                          // mir::Constant
2020-04-14T12:09:03.3433016Z 113                                          // mir::Constant
2020-04-14T12:09:03.3433717Z -                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
2020-04-14T12:09:03.3434445Z +                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
2020-04-14T12:09:03.3435266Z 115                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
2020-04-14T12:09:03.3435887Z 116                                          // ty::Const
2020-04-14T12:09:03.3436185Z 117                                          // + ty: usize
2020-04-14T12:09:03.3436371Z 
2020-04-14T12:09:03.3436637Z 118                                          // + val: Value(Scalar(0x00000016))
2020-04-14T12:09:03.3436996Z 119                                          // mir::Constant
2020-04-14T12:09:03.3437657Z -                                          // + span: $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-14T12:09:03.3438404Z +                                          // + span: $DIR/region-subtyping-basic.rs:23:15: 23:17
2020-04-14T12:09:03.3438949Z 121                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000016)) }
2020-04-14T12:09:03.3439449Z 123 
2020-04-14T12:09:03.3439547Z 
2020-04-14T12:09:03.3439684Z 124     bb5: {
2020-04-14T12:09:03.3439684Z 124     bb5: {
2020-04-14T12:09:03.3440293Z -         StorageLive(_8);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-14T12:09:03.3441103Z -         StorageLive(_9);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-14T12:09:03.3441897Z -         _9 = (*_6);                      // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:19:15: 19:17
2020-04-14T12:09:03.3442942Z -         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb6, unwind: bb1]; // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:19:9: 19:18
2020-04-14T12:09:03.3443902Z +         StorageLive(_8);                 // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3444692Z +         StorageLive(_9);                 // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-14T12:09:03.3445576Z +         _9 = (*_6);                      // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
2020-04-14T12:09:03.3446645Z +         _8 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb6, unwind: bb1]; // bb5[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
2020-04-14T12:09:03.3447251Z 129                                          // ty::Const
2020-04-14T12:09:03.3447809Z 130                                          // + ty: fn(usize) -> bool {use_x}
2020-04-14T12:09:03.3448195Z 131                                          // + val: Value(Scalar(<ZST>))
2020-04-14T12:09:03.3448648Z 132                                          // mir::Constant
2020-04-14T12:09:03.3448648Z 132                                          // mir::Constant
2020-04-14T12:09:03.3449272Z -                                          // + span: $DIR/region-subtyping-basic.rs:19:9: 19:14
2020-04-14T12:09:03.3450007Z +                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
2020-04-14T12:09:03.3450838Z 134                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
2020-04-14T12:09:03.3451366Z 136 
2020-04-14T12:09:03.3451481Z 
2020-04-14T12:09:03.3451618Z 137     bb6: {
2020-04-14T12:09:03.3451618Z 137     bb6: {
2020-04-14T12:09:03.3452215Z -         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:19:17: 19:18
2020-04-14T12:09:03.3453023Z -         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:19:18: 19:19
2020-04-14T12:09:03.3453811Z -         _0 = ();                         // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:13: 20:6
2020-04-14T12:09:03.3454592Z -         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-14T12:09:03.3455398Z +         StorageDead(_9);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
2020-04-14T12:09:03.3456205Z +         StorageDead(_8);                 // bb6[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
2020-04-14T12:09:03.3456995Z +         _0 = ();                         // bb6[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
2020-04-14T12:09:03.3457899Z +         goto -> bb8;                     // bb6[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-14T12:09:03.3458379Z 143 
2020-04-14T12:09:03.3458537Z 144     bb7: {
2020-04-14T12:09:03.3458658Z 
2020-04-14T12:09:03.3458658Z 
2020-04-14T12:09:03.3459226Z -         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
2020-04-14T12:09:03.3460028Z -         _0 = ();                         // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:20:12: 22:6
2020-04-14T12:09:03.3460799Z -         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:18:5: 22:6
2020-04-14T12:09:03.3461597Z +         StorageDead(_10);                // bb7[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
2020-04-14T12:09:03.3462408Z +         _0 = ();                         // bb7[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
2020-04-14T12:09:03.3463193Z +         goto -> bb8;                     // bb7[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
2020-04-14T12:09:03.3463679Z 149 
2020-04-14T12:09:03.3463823Z 150     bb8: {
2020-04-14T12:09:03.3463942Z 
2020-04-14T12:09:03.3463942Z 
2020-04-14T12:09:03.3464512Z -         StorageDead(_6);                 // bb8[0]: scope 2 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-14T12:09:03.3465292Z -         StorageDead(_3);                 // bb8[1]: scope 1 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-14T12:09:03.3466076Z -         StorageDead(_2);                 // bb8[2]: scope 1 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-14T12:09:03.3466866Z -         StorageDead(_1);                 // bb8[3]: scope 0 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-14T12:09:03.3467707Z -         StorageDead(_7);                 // bb8[4]: scope 0 at $DIR/region-subtyping-basic.rs:23:1: 23:2
2020-04-14T12:09:03.3468754Z -         return;                          // bb8[5]: scope 0 at $DIR/region-subtyping-basic.rs:23:2: 23:2
2020-04-14T12:09:03.3469621Z +         StorageDead(_6);                 // bb8[0]: scope 2 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-14T12:09:03.3470404Z +         StorageDead(_3);                 // bb8[1]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-14T12:09:03.3471181Z +         StorageDead(_2);                 // bb8[2]: scope 1 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-14T12:09:03.3471975Z +         StorageDead(_1);                 // bb8[3]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-14T12:09:03.3472744Z +         StorageDead(_7);                 // bb8[4]: scope 0 at $DIR/region-subtyping-basic.rs:25:1: 25:2
2020-04-14T12:09:03.3473530Z +         return;                          // bb8[5]: scope 0 at $DIR/region-subtyping-basic.rs:25:2: 25:2
2020-04-14T12:09:03.3474004Z 158 }
2020-04-14T12:09:03.3474143Z 159 
2020-04-14T12:09:03.3474245Z 
2020-04-14T12:09:03.3474245Z 
2020-04-14T12:09:03.3475100Z thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region-subtyping-basic/32bit/rustc.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-14T12:09:03.3475969Z 
2020-04-14T12:09:03.3476065Z 
2020-04-14T12:09:03.3476191Z failures:
2020-04-14T12:09:03.3476597Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-14T12:09:03.3476597Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-14T12:09:03.3476768Z 
2020-04-14T12:09:03.3477227Z test result: FAILED. 88 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T12:09:03.3477458Z 
2020-04-14T12:09:03.3477565Z 
2020-04-14T12:09:03.3477653Z 
2020-04-14T12:09:03.3481699Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-14T12:09:03.3484283Z 
2020-04-14T12:09:03.3484375Z 
2020-04-14T12:09:03.3484883Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-14T12:09:03.3484883Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-14T12:09:03.3485635Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-14T12:09:03.3486299Z == clock drift check ==
2020-04-14T12:09:03.3486299Z == clock drift check ==
2020-04-14T12:09:03.3486528Z   local time: Tue Apr 14 12:09:03 UTC 2020
2020-04-14T12:09:03.4349221Z   network time: Tue, 14 Apr 2020 12:09:03 GMT
2020-04-14T12:09:06.0265216Z 
2020-04-14T12:09:06.0265216Z 
2020-04-14T12:09:06.0368526Z ##[error]Bash exited with code '1'.
2020-04-14T12:09:06.0482384Z ##[section]Finishing: Run build
2020-04-14T12:09:06.0552098Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-14T12:09:06.0557804Z Task         : Get sources
2020-04-14T12:09:06.0558167Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T12:09:06.0558515Z Version      : 1.0.0
2020-04-14T12:09:06.0558752Z Author       : Microsoft
2020-04-14T12:09:06.0558752Z Author       : Microsoft
2020-04-14T12:09:06.0559123Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T12:09:06.0559563Z ==============================================================================
2020-04-14T12:09:06.4421115Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T12:09:06.4482333Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-14T12:09:06.4590263Z Cleaning up task key
2020-04-14T12:09:06.4591538Z Start cleaning up orphan processes.
2020-04-14T12:09:06.4807324Z Terminate orphan process: pid (3509) (python)
2020-04-14T12:09:06.5380575Z ##[section]Finishing: Finalize Job
