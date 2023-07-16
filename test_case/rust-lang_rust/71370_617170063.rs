plain
2020-04-21T11:37:00.5675502Z ========================== Starting Command Output ===========================
2020-04-21T11:37:00.5677717Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d55f162c-ab41-415f-8be0-40f19f47f03c.sh
2020-04-21T11:37:00.5677951Z 
2020-04-21T11:37:00.5680731Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T11:37:00.5698537Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T11:37:00.5701525Z Task         : Get sources
2020-04-21T11:37:00.5701862Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T11:37:00.5702129Z Version      : 1.0.0
2020-04-21T11:37:00.5702329Z Author       : Microsoft
---
2020-04-21T11:37:01.5562172Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T11:37:01.5567773Z ##[command]git config gc.auto 0
2020-04-21T11:37:01.5572267Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T11:37:01.5575524Z ##[command]git config --get-all http.proxy
2020-04-21T11:37:01.5582010Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71370/merge:refs/remotes/pull/71370/merge
---
2020-04-21T11:39:08.8363772Z  ---> 318032b5f0e2
2020-04-21T11:39:08.8366592Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-21T11:39:08.8372107Z  ---> Using cache
2020-04-21T11:39:08.8372475Z  ---> d44a858fd1ce
2020-04-21T11:39:08.8375571Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-21T11:39:08.8381450Z  ---> 58b910f50f5a
2020-04-21T11:39:08.8383694Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-21T11:39:08.8388648Z  ---> Using cache
2020-04-21T11:39:08.8388997Z  ---> ee7702aadba1
---
2020-04-21T11:39:08.8847376Z Looks like docker image is the same as before, not uploading
2020-04-21T11:39:13.6046225Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T11:39:13.6276287Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-21T11:39:13.6307094Z == clock drift check ==
2020-04-21T11:39:13.6314812Z   local time: Tue Apr 21 11:39:13 UTC 2020
2020-04-21T11:39:13.8265745Z   network time: Tue, 21 Apr 2020 11:39:13 GMT
2020-04-21T11:39:13.8299648Z Starting sccache server...
2020-04-21T11:39:13.9123135Z configure: processing command line
2020-04-21T11:39:13.9123552Z configure: 
2020-04-21T11:39:13.9124544Z configure: rust.dist-src        := False
---
2020-04-21T11:44:09.8015677Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T11:44:11.1576064Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T11:44:12.6793854Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T11:44:13.5594227Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T11:44:22.3205266Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T11:44:24.4166867Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T11:44:28.5813853Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T11:44:32.5669943Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T11:44:41.8123387Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T12:05:33.7030539Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T12:05:35.2682701Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T12:05:36.8371883Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T12:05:37.0814544Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T12:05:47.1507417Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T12:05:49.5473272Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T12:05:54.1801735Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T12:05:58.4572482Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T12:06:08.0286063Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T12:27:28.3609661Z .................................................................................................... 1600/9911
2020-04-21T12:27:34.1772979Z .................................................................................................... 1700/9911
2020-04-21T12:27:38.1087576Z .................................................................................................... 1800/9911
2020-04-21T12:27:46.0259479Z .................................................................................................... 1900/9911
2020-04-21T12:27:53.3374365Z ..i................................................................................................. 2000/9911
2020-04-21T12:27:59.4856361Z ............................................................................................iiiii... 2100/9911
2020-04-21T12:28:17.8308909Z .................................................................................................... 2300/9911
2020-04-21T12:28:19.8637147Z .................................................................................................... 2400/9911
2020-04-21T12:28:22.0350121Z .................................................................................................... 2500/9911
2020-04-21T12:28:27.4877546Z .................................................................................................... 2600/9911
---
2020-04-21T12:31:14.4718494Z .................................................................................................... 5100/9911
2020-04-21T12:31:21.1255468Z .................................................................................................... 5200/9911
2020-04-21T12:31:25.9109855Z ..............i..................................................................................... 5300/9911
2020-04-21T12:31:34.7327528Z ....i............................................................................................... 5400/9911
2020-04-21T12:31:39.5646772Z ....ii.ii........i...i.............................................................................. 5500/9911
2020-04-21T12:31:46.6983149Z ...................................................i................................................ 5700/9911
2020-04-21T12:31:55.1100809Z ...................................................................................ii............... 5800/9911
2020-04-21T12:32:01.6486085Z ......................i............................................................................. 5900/9911
2020-04-21T12:32:06.7607254Z .................................................................................................... 6000/9911
2020-04-21T12:32:06.7607254Z .................................................................................................... 6000/9911
2020-04-21T12:32:17.0082861Z .................................................................................................... 6100/9911
2020-04-21T12:32:26.5275917Z ................ii...i..ii...........i.............................................................. 6200/9911
2020-04-21T12:32:41.8483012Z .................................................................................................... 6400/9911
2020-04-21T12:32:46.8287343Z .................................................................................................... 6500/9911
2020-04-21T12:32:46.8287343Z .................................................................................................... 6500/9911
2020-04-21T12:32:56.1461559Z ..............................................i..ii................................................. 6600/9911
2020-04-21T12:33:17.2702758Z .................................................................................................... 6800/9911
2020-04-21T12:33:19.3576295Z ...............................................i.................................................... 6900/9911
2020-04-21T12:33:21.3864002Z .................................................................................................... 7000/9911
2020-04-21T12:33:23.3540755Z .......................................................................................i............ 7100/9911
---
2020-04-21T12:34:54.0475600Z .................................................................................................... 7900/9911
2020-04-21T12:35:00.1199353Z .................................................................................................... 8000/9911
2020-04-21T12:35:05.4492330Z .....................................................i.............................................. 8100/9911
2020-04-21T12:35:14.6103328Z .................................................................................................... 8200/9911
2020-04-21T12:35:19.5341573Z ..iiiiii.iiiii.i.................................................................................... 8300/9911
2020-04-21T12:35:31.8287754Z .................................................................................................... 8500/9911
2020-04-21T12:35:39.1255531Z .................................................................................................... 8600/9911
2020-04-21T12:35:51.6260580Z .................................................................................................... 8700/9911
2020-04-21T12:35:57.6862709Z .................................................................................................... 8800/9911
---
2020-04-21T12:38:01.5449017Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-21T12:38:01.5634191Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-21T12:38:01.7535520Z 
2020-04-21T12:38:01.7535892Z running 186 tests
2020-04-21T12:38:04.4295860Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-21T12:38:06.7951804Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-21T12:38:06.7956783Z 
2020-04-21T12:38:06.7957333Z  finished in 5.232
2020-04-21T12:38:06.7958092Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-21T12:38:06.8150164Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-21T12:38:08.7693388Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-21T12:38:08.7877837Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-21T12:38:08.9271884Z 
2020-04-21T12:38:08.9274026Z running 9 tests
2020-04-21T12:38:08.9275069Z iiiiiiiii
2020-04-21T12:38:08.9276112Z 
2020-04-21T12:38:08.9278823Z  finished in 0.139
2020-04-21T12:38:08.9279626Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-21T12:38:08.9453634Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-21T12:38:26.8899752Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-21T12:38:26.9112625Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-21T12:38:27.0903488Z 
2020-04-21T12:38:27.0903806Z running 115 tests
2020-04-21T12:38:39.7576974Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-21T12:38:41.3718506Z ...iiii.....ii.
2020-04-21T12:38:41.3726365Z 
2020-04-21T12:38:41.3726515Z  finished in 14.460
2020-04-21T12:38:41.3727111Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-21T12:38:41.3727756Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-21T12:49:25.3087101Z 
2020-04-21T12:49:25.3088714Z    Doc-tests core
2020-04-21T12:49:29.5766970Z 
2020-04-21T12:49:29.5777390Z running 2499 tests
2020-04-21T12:49:37.8436604Z ......iiiii......................................................................................... 100/2499
2020-04-21T12:49:46.0636211Z ......................................................................................ii............ 200/2499
2020-04-21T12:50:04.7892461Z ......................i............................................................................. 400/2499
2020-04-21T12:50:13.8795798Z ............................................................................i..i..................ii 500/2499
2020-04-21T12:50:20.8163745Z ii.................................................................................................. 600/2499
2020-04-21T12:50:28.3830118Z .................................................................................................... 700/2499
---
2020-04-21T12:53:52.8311033Z .................................................................................................... 500/764
2020-04-21T12:53:52.8313578Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-21T12:53:52.8315830Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-21T12:53:52.8317625Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-21T12:53:52.8318751Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-21T12:53:52.8319830Z ...........................................thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-21T12:53:52.8320881Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-21T12:53:52.8321840Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-21T12:53:54.7631538Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-21T12:53:54.7633079Z .thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-21T12:53:54.7635147Z ....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-21T12:53:54.7636531Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-21T12:54:03.7569498Z 
2020-04-21T12:54:03.7570044Z running 1020 tests
2020-04-21T12:54:18.6929291Z i................................................................................................... 100/1020
2020-04-21T12:54:27.8911400Z .................................................................................................... 200/1020
2020-04-21T12:54:34.6627507Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-21T12:54:39.0772771Z .................................................................................................... 400/1020
2020-04-21T12:54:45.1127521Z ....................................................i....i......................................ii.. 500/1020
2020-04-21T12:54:56.5479260Z .................................................................................................... 700/1020
2020-04-21T12:54:56.5479260Z .................................................................................................... 700/1020
2020-04-21T12:55:02.9070114Z ..............................................iiii.................................................. 800/1020
2020-04-21T12:55:15.1030434Z .................................................................................................... 900/1020
2020-04-21T12:55:20.5191196Z ....................................................................iiii............................ 1000/1020
2020-04-21T12:55:21.7043322Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-21T12:55:21.7043529Z 
2020-04-21T12:55:21.7122904Z  finished in 146.064
2020-04-21T12:55:21.7126927Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-21T12:58:05.3123981Z 
2020-04-21T12:58:05.3124222Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-21T12:58:05.3124477Z 
2020-04-21T12:58:05.3202217Z  finished in 0.910
2020-04-21T12:58:05.3203945Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-21T12:58:05.3217615Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-21T12:58:05.4956517Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T12:58:06.4205476Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-23b3e97a83a75ce2
2020-04-21T12:58:06.4236743Z 
2020-04-21T12:58:06.4237010Z running 0 tests
2020-04-21T12:58:06.4237133Z 
---
2020-04-21T13:10:49.0230905Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0231626Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0232349Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0233073Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0233812Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0235268Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0236004Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0236704Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-21T13:10:49.0237448Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-21T13:11:45.8829847Z   | ^^ expected expression
2020-04-21T13:11:45.8829969Z 
2020-04-21T13:11:45.8830307Z error: aborting due to previous error
2020-04-21T13:11:45.8830459Z 
2020-04-21T13:11:45.8830681Z Some expected error codes were not found: ["E0696"]
2020-04-21T13:11:45.8830996Z failures:
2020-04-21T13:11:45.8831584Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0696 (line 13153)
2020-04-21T13:11:45.8831869Z 
2020-04-21T13:11:45.8832109Z test result: FAILED. 868 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out
---
2020-04-21T13:11:45.8833000Z 
2020-04-21T13:11:45.8833483Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-21T13:11:45.8833819Z Build completed unsuccessfully in 1:30:59
2020-04-21T13:11:45.8834046Z == clock drift check ==
2020-04-21T13:11:45.8834290Z   local time: Tue Apr 21 13:11:45 UTC 2020
2020-04-21T13:11:45.9533568Z   network time: Tue, 21 Apr 2020 13:11:45 GMT
2020-04-21T13:11:46.3526705Z 
2020-04-21T13:11:46.3526705Z 
2020-04-21T13:11:46.3604067Z ##[error]Bash exited with code '1'.
2020-04-21T13:11:46.3618102Z ##[section]Finishing: Run build
2020-04-21T13:11:46.3664421Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T13:11:46.3669029Z Task         : Get sources
2020-04-21T13:11:46.3669328Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T13:11:46.3669614Z Version      : 1.0.0
2020-04-21T13:11:46.3669932Z Author       : Microsoft
2020-04-21T13:11:46.3669932Z Author       : Microsoft
2020-04-21T13:11:46.3670240Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T13:11:46.3670595Z ==============================================================================
2020-04-21T13:11:46.7116372Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T13:11:46.7167719Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T13:11:46.7270134Z Cleaning up task key
2020-04-21T13:11:46.7271316Z Start cleaning up orphan processes.
2020-04-21T13:11:46.7455718Z Terminate orphan process: pid (3753) (python)
2020-04-21T13:11:46.7683117Z ##[section]Finishing: Finalize Job
