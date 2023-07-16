plain
2020-04-06T08:28:27.9878509Z ========================== Starting Command Output ===========================
2020-04-06T08:28:27.9895722Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/de194ed8-bc7b-40a8-bd1d-cc2b85c47700.sh
2020-04-06T08:28:28.0089801Z 
2020-04-06T08:28:28.0146537Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T08:28:28.0165305Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T08:28:28.0168713Z Task         : Get sources
2020-04-06T08:28:28.0169023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T08:28:28.0169318Z Version      : 1.0.0
2020-04-06T08:28:28.0169517Z Author       : Microsoft
---
2020-04-06T08:28:28.7355304Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T08:28:28.7364177Z ##[command]git config gc.auto 0
2020-04-06T08:28:28.7479223Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T08:28:28.7502120Z ##[command]git config --get-all http.proxy
2020-04-06T08:28:28.7508287Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70834/merge:refs/remotes/pull/70834/merge
---
2020-04-06T08:31:54.0977214Z Looks like docker image is the same as before, not uploading
2020-04-06T08:32:02.4041396Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T08:32:02.4326033Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T08:32:02.4383936Z == clock drift check ==
2020-04-06T08:32:02.4384251Z   local time: Mon Apr  6 08:32:02 UTC 2020
2020-04-06T08:32:02.7059021Z   network time: Mon, 06 Apr 2020 08:32:02 GMT
2020-04-06T08:32:02.7081415Z Starting sccache server...
2020-04-06T08:32:02.7915996Z configure: processing command line
2020-04-06T08:32:02.7916279Z configure: 
2020-04-06T08:32:02.7918463Z configure: rust.dist-src        := False
---
2020-04-06T08:37:02.0102797Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T08:37:03.4132832Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T08:37:05.0020566Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T08:37:05.1025555Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T08:37:14.6804122Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T08:37:16.2850353Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T08:37:20.5475757Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T08:37:24.5542944Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T08:37:34.3949117Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T08:59:29.5636116Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T08:59:31.3488433Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T08:59:33.3637466Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T08:59:34.3733378Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T08:59:45.9147246Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T08:59:47.9616559Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T08:59:53.1736788Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T08:59:58.7027357Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T09:00:10.5735349Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T09:25:41.1138393Z .................................................................................................... 1700/9878
2020-04-06T09:25:45.1564754Z .................................................................................................... 1800/9878
2020-04-06T09:25:53.9568918Z .................................................................................................i.. 1900/9878
2020-04-06T09:26:01.7767120Z .................................................................................................... 2000/9878
2020-04-06T09:26:08.3117123Z .......................................................................................iiiii........ 2100/9878
2020-04-06T09:26:29.2761394Z .................................................................................................... 2300/9878
2020-04-06T09:26:31.3990287Z .................................................................................................... 2400/9878
2020-04-06T09:26:33.6277792Z .................................................................................................... 2500/9878
2020-04-06T09:26:39.7122812Z .................................................................................................... 2600/9878
---
2020-04-06T09:29:31.9953631Z .............................................................i...............i...................... 5000/9878
2020-04-06T09:29:39.4109966Z .................................................................................................... 5100/9878
2020-04-06T09:29:47.2460535Z .................................................................................................... 5200/9878
2020-04-06T09:29:52.5011541Z ......i............................................................................................. 5300/9878
2020-04-06T09:30:02.7607069Z ...............................................................................................ii.ii 5400/9878
2020-04-06T09:30:07.9746827Z ........i...i....................................................................................... 5500/9878
2020-04-06T09:30:17.2882255Z ........................................i........................................................... 5700/9878
2020-04-06T09:30:17.2882255Z ........................................i........................................................... 5700/9878
2020-04-06T09:30:27.2633459Z ............................................................ii.....................................i 5800/9878
2020-04-06T09:30:40.3167484Z .................................................................................................... 6000/9878
2020-04-06T09:30:40.3167484Z .................................................................................................... 6000/9878
2020-04-06T09:30:50.3711090Z .............................................................................................ii...i. 6100/9878
2020-04-06T09:31:02.3970965Z .ii...........i..................................................................................... 6200/9878
2020-04-06T09:31:18.2740025Z .................................................................................................... 6400/9878
2020-04-06T09:31:21.0226298Z .................................................................................................... 6500/9878
2020-04-06T09:31:21.0226298Z .................................................................................................... 6500/9878
2020-04-06T09:31:33.6197372Z .......................i..ii........................................................................ 6600/9878
2020-04-06T09:31:54.5609778Z .................................................................................................... 6800/9878
2020-04-06T09:31:56.7000128Z .......................i............................................................................ 6900/9878
2020-04-06T09:31:58.7931371Z .................................................................................................... 7000/9878
2020-04-06T09:32:01.0603766Z ..............................................................i..................................... 7100/9878
---
2020-04-06T09:33:39.3330153Z .................................................................................................... 7800/9878
2020-04-06T09:33:43.6175121Z .................................................................................................... 7900/9878
2020-04-06T09:33:49.6291622Z .................................................................................................... 8000/9878
2020-04-06T09:33:57.2580349Z ...........................i........................................................................ 8100/9878
2020-04-06T09:34:05.4089236Z ............................................................................iiiiiiiiii.i............ 8200/9878
2020-04-06T09:34:21.0538232Z ....................i......i........................................................................ 8400/9878
2020-04-06T09:34:25.7266789Z .................................................................................................... 8500/9878
2020-04-06T09:34:36.4282459Z .................................................................................................... 8600/9878
2020-04-06T09:34:48.5348622Z .................................................................................................... 8700/9878
---
2020-04-06T09:37:14.4666732Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-06T09:37:14.4922037Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T09:37:14.7095218Z 
2020-04-06T09:37:14.7095612Z running 185 tests
2020-04-06T09:37:17.6126030Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-06T09:37:20.3117072Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-06T09:37:20.3120144Z 
2020-04-06T09:37:20.3127405Z  finished in 5.820
2020-04-06T09:37:20.3134348Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-06T09:37:20.3328529Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T09:37:22.5632149Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-06T09:37:22.5834745Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T09:37:22.7420690Z 
2020-04-06T09:37:22.7421387Z running 9 tests
2020-04-06T09:37:22.7422894Z iiiiiiiii
2020-04-06T09:37:22.7424289Z 
2020-04-06T09:37:22.7428443Z  finished in 0.159
2020-04-06T09:37:22.7434774Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-06T09:37:22.7631073Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T09:37:44.0431766Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-06T09:37:44.0693986Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T09:37:44.2739321Z 
2020-04-06T09:37:44.2739794Z running 115 tests
2020-04-06T09:37:59.4910632Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-06T09:38:01.1690468Z ...iiii.....ii.
2020-04-06T09:38:01.1691806Z 
2020-04-06T09:38:01.1698857Z  finished in 17.100
2020-04-06T09:38:01.1709081Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-06T09:38:01.1713717Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T09:50:15.9878895Z 
2020-04-06T09:50:15.9883005Z    Doc-tests core
2020-04-06T09:50:20.4902738Z 
2020-04-06T09:50:20.4903719Z running 2492 tests
2020-04-06T09:50:29.5258721Z ......iiiii......................................................................................... 100/2492
2020-04-06T09:50:38.0964522Z .....................................................................................ii............. 200/2492
2020-04-06T09:50:58.1634178Z ......................i............................................................................. 400/2492
2020-04-06T09:51:08.0596292Z ............................................................................i..i..................ii 500/2492
2020-04-06T09:51:15.5386822Z ii.................................................................................................. 600/2492
2020-04-06T09:51:23.8983237Z .................................................................................................... 700/2492
---
2020-04-06T09:55:01.8322977Z ...............................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-06T09:55:01.8331003Z .. 300/760
2020-04-06T09:55:01.9258254Z .................................................................................................... 400/760
2020-04-06T09:55:04.0132029Z .................................................................................................... 500/760
2020-04-06T09:55:04.0567672Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-06T09:55:04.0584831Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-06T09:55:04.0596743Z thread '.<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-06T09:55:04.0612134Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-06T09:55:04.4210985Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-06T09:55:04.4254706Z ....thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', .src/libstd/sync/mpsc/mod.rs.:.2034:...21thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-06T09:55:04.4674630Z ................. 600/760
2020-04-06T09:55:06.4936631Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-06T09:55:06.4950734Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-06T09:55:06.4962613Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
---
2020-04-06T09:55:15.5060095Z 
2020-04-06T09:55:15.5060592Z running 1018 tests
2020-04-06T09:55:32.3307198Z i................................................................................................... 100/1018
2020-04-06T09:55:42.0987477Z .................................................................................................... 200/1018
2020-04-06T09:55:49.3047919Z ..................iii......i......i...i......i...................................................... 300/1018
2020-04-06T09:56:00.2320391Z ..................................................i....i......................................ii.... 500/1018
2020-04-06T09:56:07.5900824Z .................................................................................................... 600/1018
2020-04-06T09:56:12.3895481Z .................................................................................................... 700/1018
2020-04-06T09:56:12.3895481Z .................................................................................................... 700/1018
2020-04-06T09:56:19.8173001Z ............................................iiii.................................................... 800/1018
2020-04-06T09:56:32.4956446Z .................................................................................................... 900/1018
2020-04-06T09:56:38.4969558Z ..................................................................iiii.............................. 1000/1018
2020-04-06T09:56:39.5064516Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-06T09:56:39.5065021Z 
2020-04-06T09:56:39.5175076Z  finished in 158.207
2020-04-06T09:56:39.5180925Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-06T09:59:45.2754952Z 
2020-04-06T09:59:45.2755417Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-06T09:59:45.2755823Z 
2020-04-06T09:59:45.2846950Z  finished in 0.984
2020-04-06T09:59:45.2857173Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-06T09:59:45.2881096Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T09:59:45.4724160Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T09:59:46.4745194Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-6282e4abc4e5c2cd
2020-04-06T09:59:46.4778716Z 
2020-04-06T09:59:46.4779206Z running 0 tests
2020-04-06T09:59:46.4779903Z 
---
2020-04-06T10:13:59.1489936Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1490729Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1491532Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1492331Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1493153Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1495123Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1495957Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1496738Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T10:13:59.1497694Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-06T10:14:11.5741940Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-04-06T10:14:11.9803124Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-04-06T10:14:12.1501479Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-04-06T10:14:13.5753860Z     Finished release [optimized] target(s) in 1.59s
2020-04-06T10:14:13.8639374Z core/future/fn.pending.html:3: broken link - core/future/struct.Poll.html
2020-04-06T10:14:16.0839568Z std/future/fn.pending.html:3: broken link - std/future/struct.Poll.html
2020-04-06T10:14:21.0155129Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-04-06T10:14:21.0187779Z 
2020-04-06T10:14:21.0188150Z 
2020-04-06T10:14:21.0189338Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-04-06T10:14:21.0189876Z expected success, got: exit code: 101
2020-04-06T10:14:21.0189876Z expected success, got: exit code: 101
2020-04-06T10:14:21.0190070Z 
2020-04-06T10:14:21.0190187Z 
2020-04-06T10:14:21.0197983Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-06T10:14:21.0198394Z Build completed unsuccessfully in 1:40:49
2020-04-06T10:14:21.0256790Z == clock drift check ==
2020-04-06T10:14:21.0274539Z   local time: Mon Apr  6 10:14:21 UTC 2020
2020-04-06T10:14:21.0579190Z   network time: Mon, 06 Apr 2020 10:14:21 GMT
2020-04-06T10:14:22.5886521Z 
2020-04-06T10:14:22.5886521Z 
2020-04-06T10:14:22.5957746Z ##[error]Bash exited with code '1'.
2020-04-06T10:14:22.5970866Z ##[section]Finishing: Run build
2020-04-06T10:14:22.6023038Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T10:14:22.6028697Z Task         : Get sources
2020-04-06T10:14:22.6029064Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T10:14:22.6029412Z Version      : 1.0.0
2020-04-06T10:14:22.6029649Z Author       : Microsoft
2020-04-06T10:14:22.6029649Z Author       : Microsoft
2020-04-06T10:14:22.6030031Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T10:14:22.6030474Z ==============================================================================
2020-04-06T10:14:22.9413538Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T10:14:22.9454835Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T10:14:22.9536724Z Cleaning up task key
2020-04-06T10:14:22.9538224Z Start cleaning up orphan processes.
2020-04-06T10:14:22.9712766Z Terminate orphan process: pid (3853) (python)
2020-04-06T10:14:22.9978009Z ##[section]Finishing: Finalize Job
