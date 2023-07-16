plain
2020-04-22T12:22:49.3980756Z ========================== Starting Command Output ===========================
2020-04-22T12:22:49.3982873Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1d03633c-2ae6-4e1d-b571-9500591c22ba.sh
2020-04-22T12:22:49.3983053Z 
2020-04-22T12:22:49.3986801Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T12:22:49.4033041Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71426/merge to s
2020-04-22T12:22:49.4036490Z Task         : Get sources
2020-04-22T12:22:49.4036745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T12:22:49.4036942Z Version      : 1.0.0
2020-04-22T12:22:49.4037076Z Author       : Microsoft
---
2020-04-22T12:22:50.4067376Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T12:22:50.4071512Z ##[command]git config gc.auto 0
2020-04-22T12:22:50.4074140Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T12:22:50.4076526Z ##[command]git config --get-all http.proxy
2020-04-22T12:22:50.4081580Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71426/merge:refs/remotes/pull/71426/merge
---
2020-04-22T12:26:13.3931321Z  ---> 318032b5f0e2
2020-04-22T12:26:13.3933943Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T12:26:13.3934622Z  ---> Using cache
2020-04-22T12:26:13.3934896Z  ---> d44a858fd1ce
2020-04-22T12:26:13.3937458Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T12:26:13.3938504Z  ---> 58b910f50f5a
2020-04-22T12:26:13.3938674Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T12:26:13.3938954Z  ---> Using cache
2020-04-22T12:26:13.3939227Z  ---> ee7702aadba1
---
2020-04-22T12:26:13.4831864Z Looks like docker image is the same as before, not uploading
2020-04-22T12:26:21.0311035Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T12:26:21.0611507Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T12:26:21.0634429Z == clock drift check ==
2020-04-22T12:26:21.0642872Z   local time: Wed Apr 22 12:26:21 UTC 2020
2020-04-22T12:26:21.3545310Z   network time: Wed, 22 Apr 2020 12:26:21 GMT
2020-04-22T12:26:21.3572522Z Starting sccache server...
2020-04-22T12:26:21.4383733Z configure: processing command line
2020-04-22T12:26:21.4384066Z configure: 
2020-04-22T12:26:21.4385699Z configure: rust.dist-src        := False
---
2020-04-22T12:31:32.5756416Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T12:31:34.0427303Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T12:31:35.6425374Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T12:31:37.5632680Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T12:31:45.6737901Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T12:31:49.0260218Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T12:31:53.4949833Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T12:31:57.7510892Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T12:32:06.3328643Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T12:53:51.7342856Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T12:53:53.3595160Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T12:53:55.1176619Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T12:53:55.9395405Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T12:54:05.5768024Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T12:54:08.3320474Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T12:54:13.0363346Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T12:54:17.3208122Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T12:54:26.7673079Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T13:16:20.4278491Z .................................................................................................... 1600/9913
2020-04-22T13:16:26.2556010Z .................................................................................................... 1700/9913
2020-04-22T13:16:30.1213272Z .................................................................................................... 1800/9913
2020-04-22T13:16:38.0191655Z .................................................................................................... 1900/9913
2020-04-22T13:16:45.1976171Z ..i................................................................................................. 2000/9913
2020-04-22T13:16:51.2122666Z ............................................................................................iiiii... 2100/9913
2020-04-22T13:17:10.1903553Z .................................................................................................... 2300/9913
2020-04-22T13:17:12.2098616Z .................................................................................................... 2400/9913
2020-04-22T13:17:14.3763614Z .................................................................................................... 2500/9913
2020-04-22T13:17:19.7217019Z .................................................................................................... 2600/9913
---
2020-04-22T13:20:03.9332495Z .................................................................................................... 5100/9913
2020-04-22T13:20:10.3959905Z .................................................................................................... 5200/9913
2020-04-22T13:20:14.9804997Z ...............i.................................................................................... 5300/9913
2020-04-22T13:20:23.8558461Z .....i.............................................................................................. 5400/9913
2020-04-22T13:20:28.5800618Z .....ii.ii........i...i............................................................................. 5500/9913
2020-04-22T13:20:35.6179663Z ....................................................i............................................... 5700/9913
2020-04-22T13:20:43.7577123Z .....................................................................................ii............. 5800/9913
2020-04-22T13:20:50.0813608Z ........................i........................................................................... 5900/9913
2020-04-22T13:20:55.1419655Z .................................................................................................... 6000/9913
2020-04-22T13:20:55.1419655Z .................................................................................................... 6000/9913
2020-04-22T13:21:04.9897841Z .................................................................................................... 6100/9913
2020-04-22T13:21:14.3572539Z ..................ii...i..ii...........i............................................................ 6200/9913
2020-04-22T13:21:29.2697241Z .................................................................................................... 6400/9913
2020-04-22T13:21:35.6328526Z .................................................................................................... 6500/9913
2020-04-22T13:21:35.6328526Z .................................................................................................... 6500/9913
2020-04-22T13:21:49.4210897Z ................................................i..ii............................................... 6600/9913
2020-04-22T13:22:10.9032452Z .................................................................................................... 6800/9913
2020-04-22T13:22:13.1897364Z .................................................i.................................................. 6900/9913
2020-04-22T13:22:15.1669722Z .................................................................................................... 7000/9913
2020-04-22T13:22:17.1059365Z .........................................................................................i.......... 7100/9913
---
2020-04-22T13:23:49.6479858Z .................................................................................................... 7900/9913
2020-04-22T13:23:55.8248410Z .................................................................................................... 8000/9913
2020-04-22T13:24:01.2271541Z .......................................................i............................................ 8100/9913
2020-04-22T13:24:10.6286777Z .................................................................................................... 8200/9913
2020-04-22T13:24:15.7365726Z ....iiiiii.iiiii.i.................................................................................. 8300/9913
2020-04-22T13:24:28.2394942Z .................................................................................................... 8500/9913
2020-04-22T13:24:35.4813106Z .................................................................................................... 8600/9913
2020-04-22T13:24:48.2726445Z .................................................................................................... 8700/9913
2020-04-22T13:24:54.3087302Z .................................................................................................... 8800/9913
---
2020-04-22T13:27:00.2633680Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-22T13:27:00.2826362Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T13:27:00.4644708Z 
2020-04-22T13:27:00.4645493Z running 186 tests
2020-04-22T13:27:03.2590285Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-22T13:27:05.7280645Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-22T13:27:05.7288677Z 
2020-04-22T13:27:05.7292570Z  finished in 5.446
2020-04-22T13:27:05.7299653Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-22T13:27:05.7457457Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-22T13:27:07.7622394Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-22T13:27:07.7808614Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T13:27:07.9439283Z 
2020-04-22T13:27:07.9440881Z running 9 tests
2020-04-22T13:27:07.9442015Z iiiiiiiii
2020-04-22T13:27:07.9442836Z 
2020-04-22T13:27:07.9443146Z  finished in 0.163
2020-04-22T13:27:07.9451196Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-22T13:27:07.9646256Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-22T13:27:27.4271871Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-22T13:27:27.4473222Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T13:27:27.6257907Z 
2020-04-22T13:27:27.6259194Z running 115 tests
2020-04-22T13:27:40.6646758Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-22T13:27:42.3690286Z ...iiii.....ii.
2020-04-22T13:27:42.3691163Z 
2020-04-22T13:27:42.3694642Z  finished in 14.921
2020-04-22T13:27:42.3698882Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T13:27:42.3699439Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-22T13:39:08.8978673Z 
2020-04-22T13:39:08.8979633Z    Doc-tests core
2020-04-22T13:39:13.5021264Z 
2020-04-22T13:39:13.5021757Z running 2499 tests
2020-04-22T13:39:22.0643094Z ......iiiii......................................................................................... 100/2499
2020-04-22T13:39:30.5195213Z ......................................................................................ii............ 200/2499
2020-04-22T13:39:49.6544191Z ......................i............................................................................. 400/2499
2020-04-22T13:39:59.0201039Z ............................................................................i..i..................ii 500/2499
2020-04-22T13:40:06.0459840Z ii.................................................................................................. 600/2499
2020-04-22T13:40:14.1509901Z .................................................................................................... 700/2499
---
2020-04-22T13:43:42.2924987Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-22T13:43:42.2934337Z . 300/764
2020-04-22T13:43:42.4104934Z .................................................................................................... 400/764
2020-04-22T13:43:44.4955472Z .................................................................................................... 500/764
2020-04-22T13:43:44.5196653Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-22T13:43:44.5213882Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-22T13:43:44.5219510Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-22T13:43:44.5246588Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-22T13:43:44.7632958Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:.1997.:22
2020-04-22T13:43:44.7646506Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-04-22T13:43:44.7665846Z ......thread '<unnamed>' panicked at 'thread '<unnamed>called `Result::unwrap()` on an `Err` value: RecvError' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-22T13:43:44.7773438Z ............. 600/764
2020-04-22T13:43:46.8099889Z ..........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-22T13:43:46.8116301Z ............thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-22T13:43:46.8117163Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
---
2020-04-22T13:43:55.8100980Z 
2020-04-22T13:43:55.8101296Z running 1020 tests
2020-04-22T13:44:11.8332599Z i................................................................................................... 100/1020
2020-04-22T13:44:21.3153037Z .................................................................................................... 200/1020
2020-04-22T13:44:28.3782930Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-22T13:44:32.8988342Z .................................................................................................... 400/1020
2020-04-22T13:44:39.0650572Z ....................................................i....i......................................ii.. 500/1020
2020-04-22T13:44:51.0049159Z .................................................................................................... 700/1020
2020-04-22T13:44:51.0049159Z .................................................................................................... 700/1020
2020-04-22T13:44:58.1410675Z ..............................................iiii.................................................. 800/1020
2020-04-22T13:45:11.0497879Z .................................................................................................... 900/1020
2020-04-22T13:45:16.9992539Z ....................................................................iiii............................ 1000/1020
2020-04-22T13:45:18.2580302Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-22T13:45:18.2580860Z 
2020-04-22T13:45:18.2681987Z  finished in 152.281
2020-04-22T13:45:18.2689998Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-22T13:48:13.0904521Z 
2020-04-22T13:48:13.0904929Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-22T13:48:13.0905292Z 
2020-04-22T13:48:13.0973247Z  finished in 0.910
2020-04-22T13:48:13.0990334Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-22T13:48:13.1005745Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-22T13:48:13.2718040Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T13:48:14.2143667Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-d73c43fe7d053db1
2020-04-22T13:48:14.2172594Z 
2020-04-22T13:48:14.2173100Z running 0 tests
2020-04-22T13:48:14.2173244Z 
---
2020-04-22T14:01:31.0293572Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0294130Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0294675Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0295236Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0295789Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0296908Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0297465Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0297997Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-22T14:01:31.0298750Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-22T14:02:31.2093800Z 
2020-04-22T14:02:31.2093901Z failures:
2020-04-22T14:02:31.2093999Z 
2020-04-22T14:02:31.2094430Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0751 (line 14420) stdout ----
2020-04-22T14:02:31.2094783Z error[E0658]: negative trait bounds are not yet fully implemented; use marker types for now
2020-04-22T14:02:31.2095605Z   |
2020-04-22T14:02:31.2095605Z   |
2020-04-22T14:02:31.2095732Z 5 | impl !MyTrait for i32 { }
2020-04-22T14:02:31.2096001Z   |
2020-04-22T14:02:31.2096001Z   |
2020-04-22T14:02:31.2096488Z   = note: see issue #68318 <***/issues/68318> for more information
2020-04-22T14:02:31.2096785Z   = help: add `#![feature(negative_impls)]` to the crate attributes to enable
2020-04-22T14:02:31.2097201Z error[E0748]: found both positive and negative implementation of trait `main::MyTrait` for type `i32`:
2020-04-22T14:02:31.2097739Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14423:1
2020-04-22T14:02:31.2097932Z   |
2020-04-22T14:02:31.2098056Z 4 | impl MyTrait for i32 { }
2020-04-22T14:02:31.2098056Z 4 | impl MyTrait for i32 { }
2020-04-22T14:02:31.2098380Z   | -------------------- positive implementation here
2020-04-22T14:02:31.2098584Z 5 | impl !MyTrait for i32 { }
2020-04-22T14:02:31.2098770Z   | ^^^^^^^^^^^^^^^^^^^^^ negative implementation here
2020-04-22T14:02:31.2099060Z error: aborting due to 2 previous errors
2020-04-22T14:02:31.2099182Z 
2020-04-22T14:02:31.2099348Z Some errors have detailed explanations: E0658, E0748.
2020-04-22T14:02:31.2099743Z For more information about an error, try `rustc --explain E0658`.
2020-04-22T14:02:31.2099743Z For more information about an error, try `rustc --explain E0658`.
2020-04-22T14:02:31.2100084Z Some expected error codes were not found: ["E0751"]
2020-04-22T14:02:31.2100332Z failures:
2020-04-22T14:02:31.2100815Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0751 (line 14420)
2020-04-22T14:02:31.2101097Z 
2020-04-22T14:02:31.2101292Z test result: FAILED. 866 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out
---
2020-04-22T14:02:31.2102007Z 
2020-04-22T14:02:31.2102392Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T14:02:31.2102658Z Build completed unsuccessfully in 1:34:35
2020-04-22T14:02:31.2102837Z == clock drift check ==
2020-04-22T14:02:31.2103033Z   local time: Wed Apr 22 14:02:31 UTC 2020
2020-04-22T14:02:31.2630702Z   network time: Wed, 22 Apr 2020 14:02:31 GMT
2020-04-22T14:02:31.7944302Z 
2020-04-22T14:02:31.7944302Z 
2020-04-22T14:02:31.8028582Z ##[error]Bash exited with code '1'.
2020-04-22T14:02:31.8057285Z ##[section]Finishing: Run build
2020-04-22T14:02:31.8104406Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71426/merge to s
2020-04-22T14:02:31.8108922Z Task         : Get sources
2020-04-22T14:02:31.8109779Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T14:02:31.8110041Z Version      : 1.0.0
2020-04-22T14:02:31.8110212Z Author       : Microsoft
2020-04-22T14:02:31.8110212Z Author       : Microsoft
2020-04-22T14:02:31.8110641Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T14:02:31.8110958Z ==============================================================================
2020-04-22T14:02:32.1453673Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T14:02:32.1496036Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71426/merge to s
2020-04-22T14:02:32.1583331Z Cleaning up task key
2020-04-22T14:02:32.1584323Z Start cleaning up orphan processes.
2020-04-22T14:02:32.1765550Z Terminate orphan process: pid (4389) (python)
2020-04-22T14:02:32.1992992Z ##[section]Finishing: Finalize Job
