plain
2020-03-31T13:01:50.2844792Z ========================== Starting Command Output ===========================
2020-03-31T13:01:50.2847794Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/92bc6101-1b6b-49df-8c40-fbd623ee9318.sh
2020-03-31T13:01:50.2848060Z 
2020-03-31T13:01:50.2852639Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T13:01:50.2872797Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70610/merge to s
2020-03-31T13:01:50.2876429Z Task         : Get sources
2020-03-31T13:01:50.2876741Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T13:01:50.2877041Z Version      : 1.0.0
2020-03-31T13:01:50.2877263Z Author       : Microsoft
---
2020-03-31T13:01:51.5185956Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T13:01:51.5193750Z ##[command]git config gc.auto 0
2020-03-31T13:01:51.5199463Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T13:01:51.5205028Z ##[command]git config --get-all http.proxy
2020-03-31T13:01:51.5215356Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70610/merge:refs/remotes/pull/70610/merge
---
2020-03-31T13:09:30.3713408Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:09:31.8358038Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:09:33.4741254Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:09:33.8366403Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:09:43.3053829Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:09:45.0474045Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:09:49.5130223Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:09:53.7840373Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:10:03.8923391Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T13:32:29.4608469Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:32:31.3278443Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:32:33.4413877Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:32:34.5706080Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:32:46.6819111Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:32:48.2631760Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:32:53.8080278Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:32:59.5112768Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:33:11.6887814Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T13:59:14.3035211Z .................................................................................................... 1700/9856
2020-03-31T13:59:18.3607150Z .................................................................................................... 1800/9856
2020-03-31T13:59:28.6085796Z ..........................................................................................i......... 1900/9856
2020-03-31T13:59:35.6618227Z .................................................................................................... 2000/9856
2020-03-31T13:59:42.2573526Z ................................................................................iiiii............... 2100/9856
2020-03-31T14:00:03.7947749Z .................................................................................................... 2300/9856
2020-03-31T14:00:06.1034358Z .................................................................................................... 2400/9856
2020-03-31T14:00:08.5903926Z .................................................................................................... 2500/9856
2020-03-31T14:00:15.2022603Z .................................................................................................... 2600/9856
---
2020-03-31T14:03:14.1793896Z ......................................................i...............i............................. 5000/9856
2020-03-31T14:03:22.4375226Z .................................................................................................... 5100/9856
2020-03-31T14:03:30.0534333Z ...................................................................................................i 5200/9856
2020-03-31T14:03:35.4197951Z .................................................................................................... 5300/9856
2020-03-31T14:03:46.6385328Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T14:03:50.4812996Z ..i................................................................................................. 5500/9856
2020-03-31T14:04:00.3320293Z ..............................i..................................................................... 5700/9856
2020-03-31T14:04:09.7102753Z ................................................ii....................................i............. 5800/9856
2020-03-31T14:04:17.2424212Z .................................................................................................... 5900/9856
2020-03-31T14:04:22.2914427Z .................................................................................................... 6000/9856
2020-03-31T14:04:22.2914427Z .................................................................................................... 6000/9856
2020-03-31T14:04:31.7099616Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T14:04:44.1956884Z .i.................................................................................................. 6200/9856
2020-03-31T14:05:00.0795996Z .................................................................................................... 6400/9856
2020-03-31T14:05:04.5870960Z .................................................................................................... 6500/9856
2020-03-31T14:05:04.5870960Z .................................................................................................... 6500/9856
2020-03-31T14:05:17.0621467Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T14:05:37.9241836Z .................................................................................................... 6800/9856
2020-03-31T14:05:40.1020537Z ..........i......................................................................................... 6900/9856
2020-03-31T14:05:42.2813276Z .................................................................................................... 7000/9856
2020-03-31T14:05:44.5177772Z ...............................................i.................................................... 7100/9856
---
2020-03-31T14:07:27.5614138Z .................................................................................................... 7800/9856
2020-03-31T14:07:32.9041133Z .................................................................................................... 7900/9856
2020-03-31T14:07:39.0157146Z .................................................................................................... 8000/9856
2020-03-31T14:07:47.2268597Z .......i............................................................................................ 8100/9856
2020-03-31T14:07:55.6732469Z ........................................................iiiiiiiiii.i................................ 8200/9856
2020-03-31T14:08:10.1385545Z i......i............................................................................................ 8400/9856
2020-03-31T14:08:15.3724731Z .................................................................................................... 8500/9856
2020-03-31T14:08:28.3722137Z .................................................................................................... 8600/9856
2020-03-31T14:08:38.1506869Z .................................................................................................... 8700/9856
---
2020-03-31T14:11:03.5988219Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-31T14:11:03.6188438Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:11:03.8270938Z 
2020-03-31T14:11:03.8272132Z running 183 tests
2020-03-31T14:11:06.5208577Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-31T14:11:09.1368307Z .......i..ii...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-31T14:11:09.1374691Z 
2020-03-31T14:11:09.1380723Z  finished in 5.519
2020-03-31T14:11:09.1388591Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-31T14:11:09.1595999Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:11:11.2877667Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-31T14:11:11.3088860Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:11:11.4750847Z 
2020-03-31T14:11:11.4752034Z running 9 tests
2020-03-31T14:11:11.4753086Z iiiiiiiii
2020-03-31T14:11:11.4754128Z 
2020-03-31T14:11:11.4754306Z  finished in 0.166
2020-03-31T14:11:11.4761621Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-31T14:11:11.4973722Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:11:31.8241233Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-31T14:11:31.8499627Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:11:32.0381762Z 
2020-03-31T14:11:32.0382109Z running 115 tests
2020-03-31T14:11:46.1164491Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-31T14:11:48.0668833Z ...iiii.....ii.
2020-03-31T14:11:48.0671196Z 
2020-03-31T14:11:48.0676826Z  finished in 16.218
2020-03-31T14:11:48.0683945Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-31T14:11:48.0689313Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:24:19.1180680Z 
2020-03-31T14:24:19.1186236Z    Doc-tests core
2020-03-31T14:24:23.7246009Z 
2020-03-31T14:24:23.7250844Z running 2489 tests
2020-03-31T14:24:32.9474123Z ......iiiii......................................................................................... 100/2489
2020-03-31T14:24:41.8519923Z .....................................................................................ii............. 200/2489
2020-03-31T14:25:02.4313195Z ....................i............................................................................... 400/2489
2020-03-31T14:25:02.4313195Z ....................i............................................................................... 400/2489
2020-03-31T14:25:12.3731200Z ..........................................................................i..i..................iiii 500/2489
2020-03-31T14:25:29.0190640Z .................................................................................................... 700/2489
2020-03-31T14:25:37.6529097Z .................................................................................................... 800/2489
2020-03-31T14:25:46.3125489Z .................................................................................................... 900/2489
2020-03-31T14:25:55.2657434Z .................................................................................................... 1000/2489
---
2020-03-31T14:29:17.6768248Z .................................................................................................... 500/760
2020-03-31T14:29:17.7093930Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-31T14:29:17.7113068Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-03-31T14:29:17.7116960Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-03-31T14:29:17.7137053Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-03-31T14:29:17.9682437Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-31T14:29:17.9717163Z .....thread '<unnamed>.' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21..
2020-03-31T14:29:17.9739078Z ..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-31T14:29:20.0515113Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-31T14:29:20.0523533Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-03-31T14:29:20.0532132Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-03-31T14:29:20.0538782Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-03-31T14:29:29.3428086Z 
2020-03-31T14:29:29.3428431Z running 1018 tests
2020-03-31T14:29:46.6738196Z i................................................................................................... 100/1018
2020-03-31T14:29:57.3412834Z .................................................................................................... 200/1018
2020-03-31T14:30:04.5453693Z ..................iii......i......i...i......i...................................................... 300/1018
2020-03-31T14:30:15.8393274Z ..................................................i....i......................................ii.... 500/1018
2020-03-31T14:30:23.3774132Z .................................................................................................... 600/1018
2020-03-31T14:30:28.2454512Z .................................................................................................... 700/1018
2020-03-31T14:30:28.2454512Z .................................................................................................... 700/1018
2020-03-31T14:30:35.3756346Z ............................................iiii.................................................... 800/1018
2020-03-31T14:30:49.0434089Z .................................................................................................... 900/1018
2020-03-31T14:30:55.3210620Z ..................................................................iiii.............................. 1000/1018
2020-03-31T14:30:56.4323309Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-31T14:30:56.4323627Z 
2020-03-31T14:30:56.4428567Z  finished in 164.409
2020-03-31T14:30:56.4431117Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-31T14:34:09.2495351Z 
2020-03-31T14:34:09.2495611Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-31T14:34:09.2496159Z 
2020-03-31T14:34:09.2556380Z  finished in 0.989
2020-03-31T14:34:09.2561382Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-31T14:34:09.2577478Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:34:09.4401909Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T14:34:10.4875234Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-f463908b68ecf375
2020-03-31T14:34:10.4907883Z 
2020-03-31T14:34:10.4908223Z running 0 tests
2020-03-31T14:34:10.4908400Z 
---
2020-03-31T14:48:58.1722808Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1723558Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1724345Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1725112Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1725964Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1727528Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1728349Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1729129Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T14:48:58.1729901Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-03-31T14:50:02.0780030Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13326) stdout ----
2020-03-31T14:50:02.0780502Z error[E0425]: cannot find value `async` in this scope
2020-03-31T14:50:02.0781498Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13330:19
2020-03-31T14:50:02.0781773Z   |
2020-03-31T14:50:02.0781996Z 5 |     let add_one = async |num: u8| {
2020-03-31T14:50:02.0782474Z 
2020-03-31T14:50:02.0782698Z error[E0425]: cannot find value `num` in this scope
2020-03-31T14:50:02.0783285Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13330:26
2020-03-31T14:50:02.0783549Z   |
2020-03-31T14:50:02.0783549Z   |
2020-03-31T14:50:02.0783772Z 5 |     let add_one = async |num: u8| {
2020-03-31T14:50:02.0784362Z   |                          |
2020-03-31T14:50:02.0784615Z   |                          not found in this scope
2020-03-31T14:50:02.0784965Z   |                          expecting a type here because of type ascription
2020-03-31T14:50:02.0785190Z 
---
2020-03-31T14:50:02.0788170Z 
2020-03-31T14:50:02.0788357Z error: aborting due to 3 previous errors
2020-03-31T14:50:02.0788525Z 
2020-03-31T14:50:02.0788983Z For more information about this error, try `rustc --explain E0425`.
2020-03-31T14:50:02.0789681Z Some expected error codes were not found: ["E0710"]
2020-03-31T14:50:02.0790426Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13339) stdout ----
2020-03-31T14:50:02.0790984Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found keyword `move`
2020-03-31T14:50:02.0791892Z   |
2020-03-31T14:50:02.0791892Z   |
2020-03-31T14:50:02.0792126Z 5 |     let add_one = async move |num: u8| {
2020-03-31T14:50:02.0792449Z   |                         ^^^^ expected one of 7 possible tokens
2020-03-31T14:50:02.0792842Z error: aborting due to previous error
2020-03-31T14:50:02.0793021Z 
2020-03-31T14:50:02.0793338Z Couldn't compile the test.
2020-03-31T14:50:02.0793481Z 
---
2020-03-31T14:50:02.0798189Z   local time: Tue Mar 31 14:50:01 UTC 2020
2020-03-31T14:50:02.0994936Z   network time: Tue, 31 Mar 2020 14:50:02 GMT
2020-03-31T14:50:02.1003338Z == end clock drift check ==
2020-03-31T14:50:02.4612528Z 
2020-03-31T14:50:02.4687424Z ##[error]Bash exited with code '1'.
2020-03-31T14:50:02.4703518Z ##[section]Finishing: Run build
2020-03-31T14:50:02.4756359Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70610/merge to s
2020-03-31T14:50:02.4761571Z Task         : Get sources
2020-03-31T14:50:02.4761921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T14:50:02.4762234Z Version      : 1.0.0
2020-03-31T14:50:02.4762476Z Author       : Microsoft
2020-03-31T14:50:02.4762476Z Author       : Microsoft
2020-03-31T14:50:02.4762851Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T14:50:02.4763256Z ==============================================================================
2020-03-31T14:50:02.8354100Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T14:50:02.8404648Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70610/merge to s
2020-03-31T14:50:02.8495103Z Cleaning up task key
2020-03-31T14:50:02.8496721Z Start cleaning up orphan processes.
2020-03-31T14:50:02.8682250Z Terminate orphan process: pid (3313) (python)
2020-03-31T14:50:02.8925185Z ##[section]Finishing: Finalize Job
