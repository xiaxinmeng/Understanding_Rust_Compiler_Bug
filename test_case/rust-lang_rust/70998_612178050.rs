plain
2020-04-10T17:25:21.9046456Z ========================== Starting Command Output ===========================
2020-04-10T17:25:21.9051539Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/206fc0db-28ad-4423-9222-6d2bd0b6bcf6.sh
2020-04-10T17:25:21.9051958Z 
2020-04-10T17:25:21.9057522Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T17:25:21.9076530Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-10T17:25:21.9079709Z Task         : Get sources
2020-04-10T17:25:21.9079967Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T17:25:21.9080315Z Version      : 1.0.0
2020-04-10T17:25:21.9080485Z Author       : Microsoft
---
2020-04-10T17:25:22.7325809Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T17:25:22.7330548Z ##[command]git config gc.auto 0
2020-04-10T17:25:22.7359021Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T17:25:22.7373971Z ##[command]git config --get-all http.proxy
2020-04-10T17:25:22.7467173Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70998/merge:refs/remotes/pull/70998/merge
---
2020-04-10T17:27:33.8954555Z Looks like docker image is the same as before, not uploading
2020-04-10T17:27:37.2837873Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T17:27:37.3179063Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T17:27:37.3206758Z == clock drift check ==
2020-04-10T17:27:37.3235152Z   local time: Fri Apr 10 17:27:37 UTC 2020
2020-04-10T17:27:37.6171591Z   network time: Fri, 10 Apr 2020 17:27:37 GMT
2020-04-10T17:27:37.6220555Z Starting sccache server...
2020-04-10T17:27:37.7084699Z configure: processing command line
2020-04-10T17:27:37.7088972Z configure: 
2020-04-10T17:27:37.7090679Z configure: rust.dist-src        := False
---
2020-04-10T17:33:05.9229012Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T17:33:07.4722476Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T17:33:09.0923346Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T17:33:10.7502792Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T17:33:19.5177701Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T17:33:22.8167218Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T17:33:27.5741257Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T17:33:32.0165585Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T17:33:41.4407137Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T17:57:22.8792197Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T17:57:24.8120646Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T17:57:26.9313221Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T17:57:29.8048699Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T17:57:39.9704714Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T17:57:44.1361894Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T17:57:49.8425147Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T17:57:55.7070011Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T17:58:06.1954687Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T18:24:07.8772201Z .................................................................................................... 1600/9883
2020-04-10T18:24:13.8515618Z .................................................................................................... 1700/9883
2020-04-10T18:24:17.9258322Z .................................................................................................... 1800/9883
2020-04-10T18:24:26.3011598Z .................................................................................................... 1900/9883
2020-04-10T18:24:34.0165455Z ..i................................................................................................. 2000/9883
2020-04-10T18:24:40.1295609Z ............................................................................................iiiii... 2100/9883
2020-04-10T18:25:00.5981063Z .................................................................................................... 2300/9883
2020-04-10T18:25:02.7100767Z .................................................................................................... 2400/9883
2020-04-10T18:25:04.8542774Z .................................................................................................... 2500/9883
2020-04-10T18:25:10.4118683Z .................................................................................................... 2600/9883
---
2020-04-10T18:28:09.7874802Z .................................................................................................... 5100/9883
2020-04-10T18:28:17.1722497Z .................................................................................................... 5200/9883
2020-04-10T18:28:22.0851472Z ...........i........................................................................................ 5300/9883
2020-04-10T18:28:32.2143921Z .................................................................................................... 5400/9883
2020-04-10T18:28:37.1261731Z ii.ii........i...i.................................................................................. 5500/9883
2020-04-10T18:28:45.2122676Z .............................................i...................................................... 5700/9883
2020-04-10T18:28:55.5861230Z .................................................................ii................................. 5800/9883
2020-04-10T18:29:02.5241136Z ....i............................................................................................... 5900/9883
2020-04-10T18:29:08.0248786Z .................................................................................................... 6000/9883
2020-04-10T18:29:08.0248786Z .................................................................................................... 6000/9883
2020-04-10T18:29:17.5378306Z ..................................................................................................ii 6100/9883
2020-04-10T18:29:29.0689354Z ...i..ii...........i................................................................................ 6200/9883
2020-04-10T18:29:44.4915085Z .................................................................................................... 6400/9883
2020-04-10T18:29:50.0230644Z .................................................................................................... 6500/9883
2020-04-10T18:29:50.0230644Z .................................................................................................... 6500/9883
2020-04-10T18:30:08.7308173Z ............................i..ii................................................................... 6600/9883
2020-04-10T18:30:29.9638879Z .................................................................................................... 6800/9883
2020-04-10T18:30:31.9486674Z ............................i....................................................................... 6900/9883
2020-04-10T18:30:33.9305037Z .................................................................................................... 7000/9883
2020-04-10T18:30:36.0143812Z ...................................................................i................................ 7100/9883
---
2020-04-10T18:32:15.5225256Z .................................................................................................... 7800/9883
2020-04-10T18:32:19.7055207Z .................................................................................................... 7900/9883
2020-04-10T18:32:26.4983528Z .................................................................................................... 8000/9883
2020-04-10T18:32:33.4860940Z ................................i................................................................... 8100/9883
2020-04-10T18:32:41.9502509Z ................................................................................iiiiii.iiii.i....... 8200/9883
2020-04-10T18:32:57.4771140Z .........................i......i................................................................... 8400/9883
2020-04-10T18:33:01.3229172Z .................................................................................................... 8500/9883
2020-04-10T18:33:12.2244990Z .................................................................................................... 8600/9883
2020-04-10T18:33:24.6967787Z .................................................................................................... 8700/9883
---
2020-04-10T18:35:51.4101350Z  finished in 7.899
2020-04-10T18:35:51.4367214Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T18:35:51.6432636Z 
2020-04-10T18:35:51.6434128Z running 185 tests
2020-04-10T18:35:54.3423895Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-10T18:35:57.0196787Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-10T18:35:57.0205140Z 
2020-04-10T18:35:57.0207105Z  finished in 5.582
2020-04-10T18:35:57.0208524Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-10T18:35:57.0404283Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T18:35:59.1329366Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-10T18:35:59.1500418Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T18:35:59.3097091Z 
2020-04-10T18:35:59.3097504Z running 9 tests
2020-04-10T18:35:59.3098574Z iiiiiiiii
2020-04-10T18:35:59.3099537Z 
2020-04-10T18:35:59.3100191Z  finished in 0.159
2020-04-10T18:35:59.3105591Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-10T18:35:59.3315348Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T18:36:19.2078282Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-10T18:36:20.3728353Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T18:36:20.5621309Z 
2020-04-10T18:36:20.5621636Z running 115 tests
2020-04-10T18:36:34.2904020Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-10T18:36:36.1615820Z ...iiii.....ii.
2020-04-10T18:36:36.1618159Z 
2020-04-10T18:36:36.1620137Z  finished in 15.789
2020-04-10T18:36:36.1629402Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-10T18:36:36.1630591Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T18:49:32.8024916Z 
2020-04-10T18:49:32.8025624Z    Doc-tests core
2020-04-10T18:49:37.4792883Z 
2020-04-10T18:49:37.4801084Z running 2490 tests
2020-04-10T18:49:46.3256466Z ......iiiii......................................................................................... 100/2490
2020-04-10T18:49:54.8726185Z .....................................................................................ii............. 200/2490
2020-04-10T18:50:14.9239660Z ....................i............................................................................... 400/2490
2020-04-10T18:50:14.9239660Z ....................i............................................................................... 400/2490
2020-04-10T18:50:24.7778846Z ..........................................................................i..i..................iiii 500/2490
2020-04-10T18:50:41.2209228Z .................................................................................................... 700/2490
2020-04-10T18:50:49.6966591Z .................................................................................................... 800/2490
2020-04-10T18:50:58.1149862Z .................................................................................................... 900/2490
2020-04-10T18:51:06.4333968Z .................................................................................................... 1000/2490
---
2020-04-10T18:54:23.0574200Z .................................................................................................... 500/764
2020-04-10T18:54:23.0935835Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-10T18:54:23.0952777Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-10T18:54:23.0976566Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-10T18:54:23.1023488Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-10T18:54:23.3154787Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-10T18:54:23.3173024Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-04-10T18:54:23.3184185Z thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-10T18:54:23.3209960Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-10T18:54:25.3927987Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-10T18:54:25.3937255Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-10T18:54:25.3961772Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-10T18:54:25.3964003Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-10T18:54:34.4108149Z 
2020-04-10T18:54:34.4109037Z running 1019 tests
2020-04-10T18:54:52.3078557Z i................................................................................................... 100/1019
2020-04-10T18:55:02.4214909Z .................................................................................................... 200/1019
2020-04-10T18:55:09.8029036Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-10T18:55:21.3813774Z ...................................................i....i......................................ii... 500/1019
2020-04-10T18:55:29.3862606Z .................................................................................................... 600/1019
2020-04-10T18:55:34.4191940Z .................................................................................................... 700/1019
2020-04-10T18:55:34.4191940Z .................................................................................................... 700/1019
2020-04-10T18:55:41.3587103Z .............................................iiii................................................... 800/1019
2020-04-10T18:55:55.0520808Z .................................................................................................... 900/1019
2020-04-10T18:56:01.1302700Z ...................................................................iiii............................. 1000/1019
2020-04-10T18:56:02.3890601Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-10T18:56:02.3890810Z 
2020-04-10T18:56:02.3990600Z  finished in 164.390
2020-04-10T18:56:02.4000629Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-10T18:59:16.7755972Z 
2020-04-10T18:59:16.7756183Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-10T18:59:16.7756377Z 
2020-04-10T18:59:16.7806608Z  finished in 0.954
2020-04-10T18:59:16.7812941Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-10T18:59:16.7829748Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T18:59:16.9680173Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T18:59:17.9482374Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-df0a59a2997e82ef
2020-04-10T18:59:17.9508878Z 
2020-04-10T18:59:17.9509344Z running 0 tests
2020-04-10T18:59:17.9509466Z 
---
2020-04-10T19:14:06.1218225Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1218887Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1219548Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1220207Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1220886Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1222311Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1222997Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1223642Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T19:14:06.1224326Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-10T19:15:07.8634238Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0746 (line 14136) stdout ----
2020-04-10T19:15:07.8634686Z error[E0746]: return type cannot have an unboxed trait object
2020-04-10T19:15:07.8635273Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14148:13
2020-04-10T19:15:07.8635523Z    |
2020-04-10T19:15:07.8635811Z 14 | fn foo() -> dyn T {
2020-04-10T19:15:07.8636464Z    |
2020-04-10T19:15:07.8636797Z    = note: currently nothing is being returned, depending on the final implementation you could change the return type in different ways
2020-04-10T19:15:07.8636797Z    = note: currently nothing is being returned, depending on the final implementation you could change the return type in different ways
2020-04-10T19:15:07.8637616Z    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
2020-04-10T19:15:07.8638516Z    = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
2020-04-10T19:15:07.8639229Z help: you could use some type `T` that is `T: Sized` as the return type if all return paths will have the same type
2020-04-10T19:15:07.8639863Z 14 | fn foo() -> T {
2020-04-10T19:15:07.8640034Z    |             ^
2020-04-10T19:15:07.8640034Z    |             ^
2020-04-10T19:15:07.8640425Z help: you could use `impl T` as the return type if all return paths will have the same type but you want to expose only the trait in the signature
2020-04-10T19:15:07.8641061Z 14 | fn foo() -> impl T {
2020-04-10T19:15:07.8641262Z    |             ^^^^^^
2020-04-10T19:15:07.8641530Z help: you could use a boxed trait object if all return paths `impl` trait `T`
2020-04-10T19:15:07.8641764Z    |
2020-04-10T19:15:07.8641764Z    |
2020-04-10T19:15:07.8642085Z 14 | fn foo() -> Box<dyn T> {
2020-04-10T19:15:07.8642282Z    |             ^^^^^^^^^^
2020-04-10T19:15:07.8642415Z 
2020-04-10T19:15:07.8642599Z error: aborting due to previous error
2020-04-10T19:15:07.8642750Z 
2020-04-10T19:15:07.8643145Z For more information about this error, try `rustc --explain E0746`.
2020-04-10T19:15:07.8643458Z Some expected error codes were not found: ["E0277"]
2020-04-10T19:15:07.8643780Z failures:
2020-04-10T19:15:07.8644322Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0746 (line 14136)
2020-04-10T19:15:07.8644686Z 
2020-04-10T19:15:07.8644955Z test result: FAILED. 861 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out
---
2020-04-10T19:15:07.8645836Z 
2020-04-10T19:15:07.8646343Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-10T19:15:07.8646699Z Build completed unsuccessfully in 1:45:50
2020-04-10T19:15:07.8646923Z == clock drift check ==
2020-04-10T19:15:07.8647147Z   local time: Fri Apr 10 19:15:07 UTC 2020
2020-04-10T19:15:07.8974392Z   network time: Fri, 10 Apr 2020 19:15:07 GMT
2020-04-10T19:15:08.2882157Z 
2020-04-10T19:15:08.2882157Z 
2020-04-10T19:15:08.2963454Z ##[error]Bash exited with code '1'.
2020-04-10T19:15:08.2978807Z ##[section]Finishing: Run build
2020-04-10T19:15:08.3030966Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-10T19:15:08.3036464Z Task         : Get sources
2020-04-10T19:15:08.3036801Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T19:15:08.3037119Z Version      : 1.0.0
2020-04-10T19:15:08.3037329Z Author       : Microsoft
2020-04-10T19:15:08.3037329Z Author       : Microsoft
2020-04-10T19:15:08.3037668Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T19:15:08.3038076Z ==============================================================================
2020-04-10T19:15:08.6444648Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T19:15:08.6488255Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-10T19:15:08.6573645Z Cleaning up task key
2020-04-10T19:15:08.6574934Z Start cleaning up orphan processes.
2020-04-10T19:15:08.6757444Z Terminate orphan process: pid (3519) (python)
2020-04-10T19:15:08.6992553Z ##[section]Finishing: Finalize Job
