plain
2020-04-05T00:10:11.4164352Z ========================== Starting Command Output ===========================
2020-04-05T00:10:11.4168543Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9add5743-b602-4f53-9c73-08cca991e1a1.sh
2020-04-05T00:10:11.4168984Z 
2020-04-05T00:10:11.4173762Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T00:10:11.4192789Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-05T00:10:11.4196216Z Task         : Get sources
2020-04-05T00:10:11.4196514Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T00:10:11.4196816Z Version      : 1.0.0
2020-04-05T00:10:11.4197223Z Author       : Microsoft
---
2020-04-05T00:10:12.9182031Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T00:10:12.9194427Z ##[command]git config gc.auto 0
2020-04-05T00:10:12.9200050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T00:10:13.9050835Z ##[command]git config --get-all http.proxy
2020-04-05T00:10:13.9064542Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70772/merge:refs/remotes/pull/70772/merge
---
2020-04-05T00:13:52.3500013Z Looks like docker image is the same as before, not uploading
2020-04-05T00:14:01.0723626Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T00:14:01.1056954Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T00:14:01.1073478Z == clock drift check ==
2020-04-05T00:14:01.1083965Z   local time: Sun Apr  5 00:14:01 UTC 2020
2020-04-05T00:14:01.2701905Z   network time: Sun, 05 Apr 2020 00:14:01 GMT
2020-04-05T00:14:01.2725956Z Starting sccache server...
2020-04-05T00:14:01.3646920Z configure: processing command line
2020-04-05T00:14:01.3647293Z configure: 
2020-04-05T00:14:01.3652395Z configure: rust.dist-src        := False
---
2020-04-05T00:19:44.2622839Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T00:19:45.8855500Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T00:19:47.5771674Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T00:19:49.7676263Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T00:19:58.4747621Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T00:20:02.3298855Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T00:20:07.2497613Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T00:20:11.7537547Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T00:20:21.2811953Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T00:45:09.9867748Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T00:45:11.9682631Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T00:45:14.2332119Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T00:45:16.8497770Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T00:45:28.0615852Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T00:45:32.0060909Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T00:45:37.9568627Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T00:45:43.8889545Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T00:45:55.3141596Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T01:14:02.4276218Z .................................................................................................... 1700/9875
2020-04-05T01:14:06.4656575Z .................................................................................................... 1800/9875
2020-04-05T01:14:15.8922228Z ................................................................................................i... 1900/9875
2020-04-05T01:14:24.1514297Z .................................................................................................... 2000/9875
2020-04-05T01:14:30.9079314Z ......................................................................................iiiii......... 2100/9875
2020-04-05T01:14:53.3049037Z .................................................................................................... 2300/9875
2020-04-05T01:14:55.5405490Z .................................................................................................... 2400/9875
2020-04-05T01:14:57.8225817Z .................................................................................................... 2500/9875
2020-04-05T01:15:04.1083687Z .................................................................................................... 2600/9875
---
2020-04-05T01:18:06.9258447Z ............................................................i...............i....................... 5000/9875
2020-04-05T01:18:14.4678305Z .................................................................................................... 5100/9875
2020-04-05T01:18:22.8138830Z .................................................................................................... 5200/9875
2020-04-05T01:18:28.1214882Z .....i.............................................................................................. 5300/9875
2020-04-05T01:18:38.6807388Z ..............................................................................................ii.ii. 5400/9875
2020-04-05T01:18:43.5427025Z .......i...i........................................................................................ 5500/9875
2020-04-05T01:18:52.5803080Z .......................................i............................................................ 5700/9875
2020-04-05T01:18:52.5803080Z .......................................i............................................................ 5700/9875
2020-04-05T01:19:02.6658277Z ...........................................................ii....................................i.. 5800/9875
2020-04-05T01:19:15.7023204Z .................................................................................................... 6000/9875
2020-04-05T01:19:15.7023204Z .................................................................................................... 6000/9875
2020-04-05T01:19:25.6012583Z ...........................................................................................ii...i..i 6100/9875
2020-04-05T01:19:38.1023329Z i...........i....................................................................................... 6200/9875
2020-04-05T01:19:54.6090032Z .................................................................................................... 6400/9875
2020-04-05T01:20:00.5928457Z .................................................................................................... 6500/9875
2020-04-05T01:20:00.5928457Z .................................................................................................... 6500/9875
2020-04-05T01:20:25.8944933Z .....................i..ii.......................................................................... 6600/9875
2020-04-05T01:20:46.8974379Z .................................................................................................... 6800/9875
2020-04-05T01:20:49.0257346Z .....................i.............................................................................. 6900/9875
2020-04-05T01:20:51.1411487Z .................................................................................................... 7000/9875
2020-04-05T01:20:53.4559044Z ............................................................i....................................... 7100/9875
---
2020-04-05T01:22:39.0081399Z .................................................................................................... 7800/9875
2020-04-05T01:22:43.6947763Z .................................................................................................... 7900/9875
2020-04-05T01:22:49.7157663Z .................................................................................................... 8000/9875
2020-04-05T01:22:58.0027647Z ........................i........................................................................... 8100/9875
2020-04-05T01:23:06.4020914Z .........................................................................iiiiiiiiii.i............... 8200/9875
2020-04-05T01:23:22.9477579Z ..................i.....i........................................................................... 8400/9875
2020-04-05T01:23:27.9353155Z .................................................................................................... 8500/9875
2020-04-05T01:23:39.7484100Z .................................................................................................... 8600/9875
2020-04-05T01:23:52.4506754Z .................................................................................................... 8700/9875
---
2020-04-05T01:26:25.7872248Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-05T01:26:25.8063307Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T01:26:26.0277376Z 
2020-04-05T01:26:26.0280578Z running 185 tests
2020-04-05T01:26:28.8163708Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-05T01:26:31.8348033Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-05T01:26:31.8352758Z 
2020-04-05T01:26:31.8360955Z  finished in 5.837
2020-04-05T01:26:31.8362182Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-05T01:26:31.8363348Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T01:26:33.8171313Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-05T01:26:33.8358432Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T01:26:33.9949084Z 
2020-04-05T01:26:33.9949528Z running 9 tests
2020-04-05T01:26:33.9950679Z iiiiiiiii
2020-04-05T01:26:33.9951866Z 
2020-04-05T01:26:33.9952308Z  finished in 0.158
2020-04-05T01:26:33.9953086Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-05T01:26:34.0139878Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T01:26:54.7775986Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-05T01:26:54.8000197Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-05T01:26:54.9989273Z 
2020-04-05T01:26:54.9989605Z running 115 tests
2020-04-05T01:27:09.5923282Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-05T01:27:11.2845509Z ...iiii.....ii.
2020-04-05T01:27:11.2850833Z 
2020-04-05T01:27:11.2851746Z  finished in 16.484
2020-04-05T01:27:11.2884562Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-05T01:27:11.2885399Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-05T01:41:03.0367687Z 
2020-04-05T01:41:03.0369158Z    Doc-tests core
2020-04-05T01:41:08.1853793Z 
2020-04-05T01:41:08.1854249Z running 2489 tests
2020-04-05T01:41:17.6590622Z ......iiiii......................................................................................... 100/2489
2020-04-05T01:41:26.9250696Z .....................................................................................ii............. 200/2489
2020-04-05T01:41:48.5288887Z ....................i............................................................................... 400/2489
2020-04-05T01:41:48.5288887Z ....................i............................................................................... 400/2489
2020-04-05T01:41:58.8296863Z ..........................................................................i..i..................iiii 500/2489
2020-04-05T01:42:15.7560596Z .................................................................................................... 700/2489
2020-04-05T01:42:24.5868977Z .................................................................................................... 800/2489
2020-04-05T01:42:33.2098095Z .................................................................................................... 900/2489
2020-04-05T01:42:41.9414087Z .................................................................................................... 1000/2489
---
2020-04-05T01:46:10.9559236Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-05T01:46:10.9566326Z .. 300/761
2020-04-05T01:46:11.0603878Z .................................................................................................... 400/761
2020-04-05T01:46:13.1515714Z .................................................................................................... 500/761
2020-04-05T01:46:13.1793021Z ...................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-05T01:46:13.1813704Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-05T01:46:13.1829127Z .thread '<unnamed>.' panicked at '..called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-05T01:46:13.1840919Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-05T01:46:13.5097179Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-05T01:46:13.5126088Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-05T01:46:13.5147664Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-05T01:46:13.5241496Z ................ 600/761
2020-04-05T01:46:15.5458577Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-05T01:46:15.5467597Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-04-05T01:46:24.6531423Z 
2020-04-05T01:46:24.6550469Z running 1019 tests
2020-04-05T01:46:43.2306800Z i................................................................................................... 100/1019
2020-04-05T01:46:53.6676062Z .................................................................................................... 200/1019
2020-04-05T01:47:01.4006433Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-05T01:47:13.1130126Z ...................................................i....i......................................ii... 500/1019
2020-04-05T01:47:20.8693017Z .................................................................................................... 600/1019
2020-04-05T01:47:25.9459833Z .................................................................................................... 700/1019
2020-04-05T01:47:25.9459833Z .................................................................................................... 700/1019
2020-04-05T01:47:33.2199489Z .............................................iiii................................................... 800/1019
2020-04-05T01:47:47.7224179Z .................................................................................................... 900/1019
2020-04-05T01:47:54.0350846Z ...................................................................iiii............................. 1000/1019
2020-04-05T01:47:55.3532516Z failures:
2020-04-05T01:47:55.3532651Z 
2020-04-05T01:47:55.3533664Z ---- io/mod.rs - io::BufRead::read_while (line 1899) stdout ----
2020-04-05T01:47:55.3534569Z error[E0658]: use of unstable library feature 'buf_read_read_while'
2020-04-05T01:47:55.3534569Z error[E0658]: use of unstable library feature 'buf_read_read_while'
2020-04-05T01:47:55.3537062Z   --> io/mod.rs:1907:8
2020-04-05T01:47:55.3537270Z    |
2020-04-05T01:47:55.3537768Z 11 | cursor.read_while(&mut buf, |b| b != b'-')
2020-04-05T01:47:55.3538229Z    |
2020-04-05T01:47:55.3538229Z    |
2020-04-05T01:47:55.3538570Z    = help: add `#![feature(buf_read_read_while)]` to the crate attributes to enable
2020-04-05T01:47:55.3539067Z error: aborting due to previous error
2020-04-05T01:47:55.3539257Z 
2020-04-05T01:47:55.3539793Z For more information about this error, try `rustc --explain E0658`.
2020-04-05T01:47:55.3540299Z Couldn't compile the test.
---
2020-04-05T01:47:55.3660582Z 
2020-04-05T01:47:55.3661299Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-05T01:47:55.3661748Z Build completed unsuccessfully in 1:32:08
2020-04-05T01:47:55.3717768Z == clock drift check ==
2020-04-05T01:47:55.3732872Z   local time: Sun Apr  5 01:47:55 UTC 2020
2020-04-05T01:47:55.4747816Z   network time: Sun, 05 Apr 2020 01:47:55 GMT
2020-04-05T01:47:56.1365975Z 
2020-04-05T01:47:56.1365975Z 
2020-04-05T01:47:56.1454497Z ##[error]Bash exited with code '1'.
2020-04-05T01:47:56.1469332Z ##[section]Finishing: Run build
2020-04-05T01:47:56.1546250Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-05T01:47:56.1551874Z Task         : Get sources
2020-04-05T01:47:56.1552225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T01:47:56.1552546Z Version      : 1.0.0
2020-04-05T01:47:56.1552788Z Author       : Microsoft
2020-04-05T01:47:56.1552788Z Author       : Microsoft
2020-04-05T01:47:56.1553145Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T01:47:56.1553757Z ==============================================================================
2020-04-05T01:47:56.5355887Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T01:47:56.5405577Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-05T01:47:56.5504586Z Cleaning up task key
2020-04-05T01:47:56.5505854Z Start cleaning up orphan processes.
2020-04-05T01:47:56.5703217Z Terminate orphan process: pid (3818) (python)
2020-04-05T01:47:56.9187360Z ##[section]Finishing: Finalize Job
