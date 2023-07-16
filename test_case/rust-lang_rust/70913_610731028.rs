plain
2020-04-08T01:29:41.7370745Z ========================== Starting Command Output ===========================
2020-04-08T01:29:41.7375584Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f0b0a68-6eb5-4ddc-b090-d933983cdd33.sh
2020-04-08T01:29:41.7376033Z 
2020-04-08T01:29:41.7380591Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T01:29:41.7409646Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70913/merge to s
2020-04-08T01:29:41.7413588Z Task         : Get sources
2020-04-08T01:29:41.7413934Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T01:29:41.7414248Z Version      : 1.0.0
2020-04-08T01:29:41.7414464Z Author       : Microsoft
---
2020-04-08T01:29:42.9163058Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T01:29:42.9175538Z ##[command]git config gc.auto 0
2020-04-08T01:29:42.9186412Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T01:29:42.9193060Z ##[command]git config --get-all http.proxy
2020-04-08T01:29:42.9206889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70913/merge:refs/remotes/pull/70913/merge
---
2020-04-08T01:32:07.2362672Z Looks like docker image is the same as before, not uploading
2020-04-08T01:32:07.9687592Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T01:32:07.9982689Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T01:32:08.0010449Z == clock drift check ==
2020-04-08T01:32:08.0021379Z   local time: Wed Apr  8 01:32:08 UTC 2020
2020-04-08T01:32:08.2934482Z   network time: Wed, 08 Apr 2020 01:32:08 GMT
2020-04-08T01:32:08.2959554Z Starting sccache server...
2020-04-08T01:32:08.3916453Z configure: processing command line
2020-04-08T01:32:08.3924597Z configure: 
2020-04-08T01:32:08.3925607Z configure: rust.dist-src        := False
---
2020-04-08T01:37:47.7590190Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T01:37:49.4193569Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T01:37:51.1505018Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T01:37:53.7154081Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T01:38:02.0336992Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T01:38:06.4736638Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T01:38:11.3807311Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T01:38:15.9732917Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T01:38:24.3607967Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T02:02:54.2884784Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T02:02:56.3036578Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T02:02:58.4418316Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T02:03:01.7141091Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T02:03:12.0499881Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T02:03:16.7161704Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T02:03:22.4285230Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T02:03:28.3400379Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T02:03:38.9165944Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T02:31:10.0458651Z .................................................................................................... 1700/9879
2020-04-08T02:31:14.3077688Z .................................................................................................... 1800/9879
2020-04-08T02:31:23.7771741Z ..................................................................................................i. 1900/9879
2020-04-08T02:31:32.0125716Z .................................................................................................... 2000/9879
2020-04-08T02:31:38.6179928Z ........................................................................................iiiii....... 2100/9879
2020-04-08T02:32:00.9430070Z .................................................................................................... 2300/9879
2020-04-08T02:32:03.2009214Z .................................................................................................... 2400/9879
2020-04-08T02:32:05.4893149Z .................................................................................................... 2500/9879
2020-04-08T02:32:11.8064781Z .................................................................................................... 2600/9879
---
2020-04-08T02:35:14.8809824Z ..............................................................i...............i..................... 5000/9879
2020-04-08T02:35:22.5666122Z .................................................................................................... 5100/9879
2020-04-08T02:35:30.4950841Z .................................................................................................... 5200/9879
2020-04-08T02:35:36.1249711Z .......i............................................................................................ 5300/9879
2020-04-08T02:35:46.6416787Z ................................................................................................ii.i 5400/9879
2020-04-08T02:35:51.6529938Z i........i...i...................................................................................... 5500/9879
2020-04-08T02:36:00.7334758Z .........................................i.......................................................... 5700/9879
2020-04-08T02:36:11.3196615Z .............................................................ii..................................... 5800/9879
2020-04-08T02:36:18.9098807Z i................................................................................................... 5900/9879
2020-04-08T02:36:24.4347005Z .................................................................................................... 6000/9879
2020-04-08T02:36:24.4347005Z .................................................................................................... 6000/9879
2020-04-08T02:36:34.6011768Z ..............................................................................................ii...i 6100/9879
2020-04-08T02:36:47.0472350Z ..ii...........i.................................................................................... 6200/9879
2020-04-08T02:37:03.0660420Z .................................................................................................... 6400/9879
2020-04-08T02:37:09.1442986Z .................................................................................................... 6500/9879
2020-04-08T02:37:09.1442986Z .................................................................................................... 6500/9879
2020-04-08T02:37:27.0168275Z ........................i..ii....................................................................... 6600/9879
2020-04-08T02:37:49.0358874Z .................................................................................................... 6800/9879
2020-04-08T02:37:51.1455897Z ........................i........................................................................... 6900/9879
2020-04-08T02:37:53.3058227Z .................................................................................................... 7000/9879
2020-04-08T02:37:55.5389922Z ...............................................................i.................................... 7100/9879
---
2020-04-08T02:39:39.0258270Z .................................................................................................... 7800/9879
2020-04-08T02:39:43.7796466Z .................................................................................................... 7900/9879
2020-04-08T02:39:49.7779491Z .................................................................................................... 8000/9879
2020-04-08T02:39:57.4646370Z ............................i....................................................................... 8100/9879
2020-04-08T02:40:06.1510598Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-08T02:40:22.6858140Z .....................i......i....................................................................... 8400/9879
2020-04-08T02:40:27.6454934Z .................................................................................................... 8500/9879
2020-04-08T02:40:38.7858436Z .................................................................................................... 8600/9879
2020-04-08T02:40:51.3814756Z .................................................................................................... 8700/9879
---
2020-04-08T02:43:19.9331114Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-08T02:43:19.9529366Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:43:20.1593053Z 
2020-04-08T02:43:20.1594228Z running 185 tests
2020-04-08T02:43:23.3103135Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-08T02:43:26.1138308Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-08T02:43:26.1152578Z 
2020-04-08T02:43:26.1152763Z  finished in 6.161
2020-04-08T02:43:26.1153586Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-08T02:43:26.1356973Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:43:28.5214131Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-08T02:43:28.5305290Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:43:28.6869525Z 
2020-04-08T02:43:28.6872194Z running 9 tests
2020-04-08T02:43:28.6875473Z iiiiiiiii
2020-04-08T02:43:28.6876748Z 
2020-04-08T02:43:28.6876902Z  finished in 0.156
2020-04-08T02:43:28.6878067Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-08T02:43:28.7071322Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:43:50.2949302Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-08T02:43:51.1983705Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T02:43:51.4049638Z 
2020-04-08T02:43:51.4051010Z running 115 tests
2020-04-08T02:44:05.8530344Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-08T02:44:07.5625650Z ...iiii.....ii.
2020-04-08T02:44:07.5628971Z 
2020-04-08T02:44:07.5630930Z  finished in 16.365
2020-04-08T02:44:07.5638502Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-08T02:44:07.5641941Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-08T02:57:44.7612946Z 
2020-04-08T02:57:44.7631332Z    Doc-tests core
2020-04-08T02:57:49.9587676Z 
2020-04-08T02:57:49.9588294Z running 2490 tests
2020-04-08T02:57:59.3395383Z ......iiiii......................................................................................... 100/2490
2020-04-08T02:58:08.4062251Z .....................................................................................ii............. 200/2490
2020-04-08T02:58:29.6425211Z ....................i............................................................................... 400/2490
2020-04-08T02:58:29.6425211Z ....................i............................................................................... 400/2490
2020-04-08T02:58:39.7683079Z ..........................................................................i..i..................iiii 500/2490
2020-04-08T02:58:56.5700897Z .................................................................................................... 700/2490
2020-04-08T02:59:05.9807513Z .................................................................................................... 800/2490
2020-04-08T02:59:15.2001823Z .................................................................................................... 900/2490
2020-04-08T02:59:24.3231009Z .................................................................................................... 1000/2490
---
2020-04-08T03:02:58.5788206Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-08T03:02:58.6107970Z .............. 600/764
2020-04-08T03:03:00.6529762Z ..........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-08T03:03:00.6531943Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-08T03:03:00.6533075Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-08T03:03:00.6533965Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
2020-04-08T03:03:00.6547182Z ...thread '...<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-08T03:03:00.6548944Z  right: `2`', src/libstd/sync/mutex.rs:657:13
2020-04-08T03:03:00.6587478Z .......thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:789:13
2020-04-08T03:03:00.6596509Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:765:13
2020-04-08T03:03:00.6603808Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:701:13
---
2020-04-08T03:03:09.7935319Z 
2020-04-08T03:03:09.7941610Z running 1019 tests
2020-04-08T03:03:28.6119637Z i................................................................................................... 100/1019
2020-04-08T03:03:39.5091464Z .................................................................................................... 200/1019
2020-04-08T03:03:47.5517261Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-08T03:03:59.8423582Z ...................................................i....i......................................ii... 500/1019
2020-04-08T03:04:07.8337422Z .................................................................................................... 600/1019
2020-04-08T03:04:13.0111470Z .................................................................................................... 700/1019
2020-04-08T03:04:13.0111470Z .................................................................................................... 700/1019
2020-04-08T03:04:20.3586392Z .............................................iiii................................................... 800/1019
2020-04-08T03:04:34.8040122Z .................................................................................................... 900/1019
2020-04-08T03:04:41.2885280Z ...................................................................iiii............................. 1000/1019
2020-04-08T03:04:42.5993820Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-08T03:04:42.5994070Z 
2020-04-08T03:04:42.6088252Z  finished in 175.614
2020-04-08T03:04:42.6094314Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-08T03:08:10.2759698Z 
2020-04-08T03:08:10.2759959Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-08T03:08:10.2760201Z 
2020-04-08T03:08:10.2822279Z  finished in 1.026
2020-04-08T03:08:10.2831018Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-08T03:08:10.2854882Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-08T03:08:10.4840562Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T03:08:11.6336153Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-997f353b1d65e825
2020-04-08T03:08:11.6364002Z 
2020-04-08T03:08:11.6364617Z running 0 tests
2020-04-08T03:08:11.6364830Z 
---
2020-04-08T03:23:48.6844350Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6846282Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6848872Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6850799Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6852518Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6856073Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6860183Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6861711Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-08T03:23:48.6863370Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-08T03:24:53.6416845Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0152 (line 2710) stdout ----
2020-04-08T03:24:53.6417334Z error[E0522]: definition of an unknown language item: `arc`
2020-04-08T03:24:53.6417902Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2713:1
2020-04-08T03:24:53.6418155Z   |
2020-04-08T03:24:53.6418326Z 5 | #[lang = "arc"]
2020-04-08T03:24:53.6418569Z   | ^^^^^^^^^^^^^^^ definition of unknown language item `arc`
2020-04-08T03:24:53.6418951Z error: aborting due to previous error
2020-04-08T03:24:53.6419108Z 
2020-04-08T03:24:53.6419515Z For more information about this error, try `rustc --explain E0522`.
2020-04-08T03:24:53.6419515Z For more information about this error, try `rustc --explain E0522`.
2020-04-08T03:24:53.6419841Z Some expected error codes were not found: ["E0152"]
2020-04-08T03:24:53.6421008Z error[E0522]: definition of an unknown language item: `arc`
2020-04-08T03:24:53.6421704Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13554:1
2020-04-08T03:24:53.6421977Z   |
2020-04-08T03:24:53.6421977Z   |
2020-04-08T03:24:53.6422122Z 5 | #[lang = "arc"]
2020-04-08T03:24:53.6422364Z   | ^^^^^^^^^^^^^^^ definition of unknown language item `arc`
2020-04-08T03:24:53.6422877Z error: aborting due to previous error
2020-04-08T03:24:53.6423039Z 
2020-04-08T03:24:53.6423463Z For more information about this error, try `rustc --explain E0522`.
2020-04-08T03:24:53.6423463Z For more information about this error, try `rustc --explain E0522`.
2020-04-08T03:24:53.6423817Z Some expected error codes were not found: ["E0718"]
2020-04-08T03:24:53.6424147Z failures:
2020-04-08T03:24:53.6424753Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0152 (line 2710)
2020-04-08T03:24:53.6425547Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0718 (line 13551)
2020-04-08T03:24:53.6425982Z 
---
2020-04-08T03:24:53.6427271Z 
2020-04-08T03:24:53.6427828Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-08T03:24:53.6428204Z Build completed unsuccessfully in 1:51:01
2020-04-08T03:24:53.6428521Z == clock drift check ==
2020-04-08T03:24:53.6428764Z   local time: Wed Apr  8 03:24:53 UTC 2020
2020-04-08T03:24:53.6429062Z   network time: Wed, 08 Apr 2020 03:24:53 GMT
2020-04-08T03:24:54.0447012Z 
2020-04-08T03:24:54.0447012Z 
2020-04-08T03:24:54.0517078Z ##[error]Bash exited with code '1'.
2020-04-08T03:24:54.0530726Z ##[section]Finishing: Run build
2020-04-08T03:24:54.0585128Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70913/merge to s
2020-04-08T03:24:54.0589764Z Task         : Get sources
2020-04-08T03:24:54.0590075Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T03:24:54.0590360Z Version      : 1.0.0
2020-04-08T03:24:54.0590576Z Author       : Microsoft
2020-04-08T03:24:54.0590576Z Author       : Microsoft
2020-04-08T03:24:54.0590889Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T03:24:54.0591255Z ==============================================================================
2020-04-08T03:24:54.6626272Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T03:24:54.6674116Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70913/merge to s
2020-04-08T03:24:54.6762189Z Cleaning up task key
2020-04-08T03:24:54.6763672Z Start cleaning up orphan processes.
2020-04-08T03:24:54.6998455Z Terminate orphan process: pid (3553) (python)
2020-04-08T03:24:54.7174328Z ##[section]Finishing: Finalize Job
