plain
2020-03-31T13:19:12.7180329Z ========================== Starting Command Output ===========================
2020-03-31T13:19:12.7182872Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d496787d-95fc-4938-8730-47e3869b3504.sh
2020-03-31T13:19:12.7183120Z 
2020-03-31T13:19:12.7186743Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T13:19:12.7207566Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T13:19:12.7210734Z Task         : Get sources
2020-03-31T13:19:12.7211057Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T13:19:12.7211350Z Version      : 1.0.0
2020-03-31T13:19:12.7211552Z Author       : Microsoft
---
2020-03-31T13:19:13.7245502Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T13:19:13.7250669Z ##[command]git config gc.auto 0
2020-03-31T13:19:13.7254471Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T13:19:13.7257750Z ##[command]git config --get-all http.proxy
2020-03-31T13:19:13.7263611Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70612/merge:refs/remotes/pull/70612/merge
---
2020-03-31T13:27:40.6970922Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:27:42.0363553Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:27:43.5182648Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:27:43.6672048Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:27:52.5517928Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:27:54.0168907Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:27:58.0764343Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:28:01.8545331Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:28:10.8617346Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T13:48:28.6446653Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T13:48:30.2544339Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T13:48:32.1201798Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T13:48:32.9584659Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T13:48:43.3120668Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T13:48:45.3506853Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T13:48:50.1833333Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T13:48:55.0603310Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T13:49:05.9805243Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T14:12:37.7771440Z .................................................................................................... 1700/9856
2020-03-31T14:12:41.4170171Z .................................................................................................... 1800/9856
2020-03-31T14:12:50.4147795Z ..........................................................................................i......... 1900/9856
2020-03-31T14:12:56.6989626Z .................................................................................................... 2000/9856
2020-03-31T14:13:02.4906645Z ................................................................................iiiii............... 2100/9856
2020-03-31T14:13:21.6085439Z .................................................................................................... 2300/9856
2020-03-31T14:13:23.5682806Z .................................................................................................... 2400/9856
2020-03-31T14:13:25.7774319Z .................................................................................................... 2500/9856
2020-03-31T14:13:31.7962839Z .................................................................................................... 2600/9856
---
2020-03-31T14:16:08.7399116Z ......................................................i...............i............................. 5000/9856
2020-03-31T14:16:16.0231730Z .................................................................................................... 5100/9856
2020-03-31T14:16:22.6791385Z ...................................................................................................i 5200/9856
2020-03-31T14:16:27.3834551Z .................................................................................................... 5300/9856
2020-03-31T14:16:37.1040253Z .....................................................................................ii.ii........i. 5400/9856
2020-03-31T14:16:40.4687696Z ..i................................................................................................. 5500/9856
2020-03-31T14:16:48.7290382Z ..............................i..................................................................... 5700/9856
2020-03-31T14:16:57.6795265Z ................................................ii....................................i............. 5800/9856
2020-03-31T14:17:04.4878261Z .................................................................................................... 5900/9856
2020-03-31T14:17:09.0076203Z .................................................................................................... 6000/9856
2020-03-31T14:17:09.0076203Z .................................................................................................... 6000/9856
2020-03-31T14:17:17.5523593Z ................................................................................ii...i..ii.......... 6100/9856
2020-03-31T14:17:29.0170143Z .i.................................................................................................. 6200/9856
2020-03-31T14:17:41.2154183Z .................................................................................................... 6400/9856
2020-03-31T14:17:44.4898548Z .................................................................................................... 6500/9856
2020-03-31T14:17:44.4898548Z .................................................................................................... 6500/9856
2020-03-31T14:17:55.5866802Z ..........i..ii..................................................................................... 6600/9856
2020-03-31T14:18:14.5667433Z .................................................................................................... 6800/9856
2020-03-31T14:18:16.5535450Z ..........i......................................................................................... 6900/9856
2020-03-31T14:18:18.5567743Z .................................................................................................... 7000/9856
2020-03-31T14:18:20.6105910Z ...............................................i.................................................... 7100/9856
---
2020-03-31T14:19:54.1465483Z .................................................................................................... 7800/9856
2020-03-31T14:19:59.0027672Z .................................................................................................... 7900/9856
2020-03-31T14:20:04.6018916Z .................................................................................................... 8000/9856
2020-03-31T14:20:12.1973926Z .......i............................................................................................ 8100/9856
2020-03-31T14:20:20.1600330Z ........................................................iiiiiiiiii.i................................ 8200/9856
2020-03-31T14:20:33.2158330Z i......i............................................................................................ 8400/9856
2020-03-31T14:20:38.0462509Z .................................................................................................... 8500/9856
2020-03-31T14:20:49.9567621Z .................................................................................................... 8600/9856
2020-03-31T14:20:59.0423085Z .................................................................................................... 8700/9856
---
2020-03-31T14:23:09.5337978Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-31T14:23:09.5516795Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:23:09.7436854Z 
2020-03-31T14:23:09.7437283Z running 183 tests
2020-03-31T14:23:12.3579051Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-31T14:23:14.7629352Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-31T14:23:14.7631982Z 
2020-03-31T14:23:14.7637288Z  finished in 5.212
2020-03-31T14:23:14.7641264Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-31T14:23:14.7822025Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:23:16.6996342Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-31T14:23:16.7180523Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:23:16.8643713Z 
2020-03-31T14:23:16.8644488Z running 9 tests
2020-03-31T14:23:16.8646116Z iiiiiiiii
2020-03-31T14:23:16.8647689Z 
2020-03-31T14:23:16.8653834Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-31T14:23:16.8654536Z  finished in 0.146
2020-03-31T14:23:16.8845005Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:23:35.7454911Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-31T14:23:35.7686255Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T14:23:35.9482284Z 
2020-03-31T14:23:35.9483215Z running 115 tests
2020-03-31T14:23:48.5487763Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-31T14:23:50.0660489Z ...iiii.....ii.
2020-03-31T14:23:50.0672296Z 
2020-03-31T14:23:50.0672456Z  finished in 14.297
2020-03-31T14:23:50.0676778Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-31T14:23:50.0681445Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T14:35:05.0944627Z 
2020-03-31T14:35:05.0947227Z    Doc-tests core
2020-03-31T14:35:09.1847627Z 
2020-03-31T14:35:09.1848271Z running 2489 tests
2020-03-31T14:35:17.7690649Z ......iiiii......................................................................................... 100/2489
2020-03-31T14:35:26.0385716Z .....................................................................................ii............. 200/2489
2020-03-31T14:35:45.0300101Z ....................i............................................................................... 400/2489
2020-03-31T14:35:45.0300101Z ....................i............................................................................... 400/2489
2020-03-31T14:35:54.1550259Z ..........................................................................i..i..................iiii 500/2489
2020-03-31T14:36:09.3280965Z .................................................................................................... 700/2489
2020-03-31T14:36:17.1723352Z .................................................................................................... 800/2489
2020-03-31T14:36:25.0036904Z .................................................................................................... 900/2489
2020-03-31T14:36:32.6578566Z .................................................................................................... 1000/2489
---
2020-03-31T14:38:36.0468106Z    Compiling std v0.0.0 (/checkout/src/libstd)
2020-03-31T14:38:37.6888787Z error[E0425]: cannot find value `cx` in this scope
2020-03-31T14:38:37.6889628Z     --> src/libstd/io/mod.rs:2890:33
2020-03-31T14:38:37.6890272Z      |
2020-03-31T14:38:37.6891117Z 2890 |             self.write_vectored(cx, &[IoSlice::new(buf)])
2020-03-31T14:38:37.6892724Z 
2020-03-31T14:38:37.6892724Z 
2020-03-31T14:38:37.6952830Z error[E0425]: cannot find value `n_bus` in this scope
2020-03-31T14:38:37.6953656Z     --> src/libstd/io/mod.rs:2968:46
2020-03-31T14:38:37.6954293Z      |
2020-03-31T14:38:37.6955087Z 2968 |                 let mut writer = test_writer(n_bus, per_call);
2020-03-31T14:38:37.6956438Z      |                                              ^^^^^ help: a local variable with a similar name exists: `n_bufs`
2020-03-31T14:38:41.0993052Z error[E0061]: this function takes 1 argument but 2 arguments were supplied
2020-03-31T14:38:41.0994888Z     --> src/libstd/io/mod.rs:2890:18
2020-03-31T14:38:41.0995736Z      |
2020-03-31T14:38:41.0995736Z      |
2020-03-31T14:38:41.0996908Z 1303 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
2020-03-31T14:38:41.0999260Z ...
2020-03-31T14:38:41.0999260Z ...
2020-03-31T14:38:41.1000244Z 2890 |             self.write_vectored(cx, &[IoSlice::new(buf)])
2020-03-31T14:38:41.1001813Z      |                  ^^^^^^^^^^^^^^ --  -------------------- supplied 2 arguments
2020-03-31T14:38:41.1004022Z      |                  expected 1 argument
2020-03-31T14:38:41.1004517Z 
2020-03-31T14:38:41.1004517Z 
2020-03-31T14:38:41.1236680Z error[E0599]: no method named `poll_write` found for struct `io::tests::TestWriter` in the current scope
2020-03-31T14:38:41.1238166Z     --> src/libstd/io/mod.rs:2914:27
2020-03-31T14:38:41.1239839Z 2882 |     struct TestWriter {
2020-03-31T14:38:41.1239839Z 2882 |     struct TestWriter {
2020-03-31T14:38:41.1241034Z      |     ----------------- method `poll_write` not found for this
2020-03-31T14:38:41.1241920Z ...
2020-03-31T14:38:41.1242863Z 2914 |         assert_eq!(writer.poll_write(&[]).unwrap(), 0);
2020-03-31T14:38:41.1244225Z      |                           ^^^^^^^^^^ method not found in `io::tests::TestWriter`
2020-03-31T14:38:41.1244960Z 
2020-03-31T14:38:41.1266735Z error[E0599]: no method named `poll_write_vectored` found for struct `io::tests::TestWriter` in the current scope
2020-03-31T14:38:41.1267938Z     --> src/libstd/io/mod.rs:2915:27
2020-03-31T14:38:41.1269539Z 2882 |     struct TestWriter {
2020-03-31T14:38:41.1269539Z 2882 |     struct TestWriter {
2020-03-31T14:38:41.1270745Z      |     ----------------- method `poll_write_vectored` not found for this
2020-03-31T14:38:41.1271663Z ...
2020-03-31T14:38:41.1272598Z 2915 |         assert_eq!(writer.poll_write_vectored(&[]).unwrap(), 0);
2020-03-31T14:38:41.1274014Z      |                           ^^^^^^^^^^^^^^^^^^^ method not found in `io::tests::TestWriter`
2020-03-31T14:38:41.1304776Z error[E0600]: cannot apply unary operator `!` to type `core::result::Result<usize, io::error::Error>`
2020-03-31T14:38:41.1306010Z     --> src/libstd/io/mod.rs:2924:9
2020-03-31T14:38:41.1306803Z      |
2020-03-31T14:38:41.1306803Z      |
2020-03-31T14:38:41.1307754Z 2924 |           assert!(writer.write_vectored(bufs), 1);
2020-03-31T14:38:41.1310002Z      |           |
2020-03-31T14:38:41.1311182Z      |           cannot apply unary operator `!`
2020-03-31T14:38:41.1313665Z      |           in this macro invocation
2020-03-31T14:38:41.1314294Z      | 
2020-03-31T14:38:41.1314294Z      | 
2020-03-31T14:38:41.1315005Z     ::: /checkout/src/libcore/macros/mod.rs:1297:5
2020-03-31T14:38:41.1315630Z      |
2020-03-31T14:38:41.1316397Z 1297 | /     macro_rules! assert {
2020-03-31T14:38:41.1317514Z 1298 | |         ($cond:expr) => {{ /* compiler built-in */ }};
2020-03-31T14:38:41.1318693Z 1299 | |         ($cond:expr,) => {{ /* compiler built-in */ }};
2020-03-31T14:38:41.1319867Z 1300 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
2020-03-31T14:38:41.1321560Z      | |_____- in this expansion of `assert!`
2020-03-31T14:38:41.1321883Z 
2020-03-31T14:38:41.1860021Z error[E0308]: mismatched types
2020-03-31T14:38:41.1860802Z     --> src/libstd/io/mod.rs:2950:14
2020-03-31T14:38:41.1860802Z     --> src/libstd/io/mod.rs:2950:14
2020-03-31T14:38:41.1861432Z      |
2020-03-31T14:38:41.1862246Z 2950 |             (&[IoSlice::new(&[1])], &[1]),
2020-03-31T14:38:41.1863469Z      |              ^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.1865151Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.1866075Z                 found reference `&[io::IoSlice<'_>; 1]`
2020-03-31T14:38:41.1866403Z 
2020-03-31T14:38:41.2298211Z error[E0308]: mismatched types
2020-03-31T14:38:41.2298211Z error[E0308]: mismatched types
2020-03-31T14:38:41.2299024Z     --> src/libstd/io/mod.rs:2950:37
2020-03-31T14:38:41.2299614Z      |
2020-03-31T14:38:41.2300396Z 2950 |             (&[IoSlice::new(&[1])], &[1]),
2020-03-31T14:38:41.2301683Z      |                                     ^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.2722640Z error[E0308]: mismatched types
2020-03-31T14:38:41.2723416Z     --> src/libstd/io/mod.rs:2951:14
2020-03-31T14:38:41.2724008Z      |
2020-03-31T14:38:41.2724008Z      |
2020-03-31T14:38:41.2724831Z 2951 |             (&[IoSlice::new(&[1, 2])], &[1, 2]),
2020-03-31T14:38:41.2726070Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.2727930Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.2728912Z                 found reference `&[io::IoSlice<'_>; 1]`
2020-03-31T14:38:41.2729261Z 
2020-03-31T14:38:41.3166403Z error[E0308]: mismatched types
2020-03-31T14:38:41.3166403Z error[E0308]: mismatched types
2020-03-31T14:38:41.3167179Z     --> src/libstd/io/mod.rs:2951:40
2020-03-31T14:38:41.3167765Z      |
2020-03-31T14:38:41.3168605Z 2951 |             (&[IoSlice::new(&[1, 2])], &[1, 2]),
2020-03-31T14:38:41.3169879Z      |                                        ^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.3602308Z error[E0308]: mismatched types
2020-03-31T14:38:41.3603078Z     --> src/libstd/io/mod.rs:2952:14
2020-03-31T14:38:41.3603725Z      |
2020-03-31T14:38:41.3603725Z      |
2020-03-31T14:38:41.3604543Z 2952 |             (&[IoSlice::new(&[1, 2, 3])], &[1, 2, 3]),
2020-03-31T14:38:41.3605795Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.3607479Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.3608394Z                 found reference `&[io::IoSlice<'_>; 1]`
2020-03-31T14:38:41.3608727Z 
2020-03-31T14:38:41.4040831Z error[E0308]: mismatched types
2020-03-31T14:38:41.4040831Z error[E0308]: mismatched types
2020-03-31T14:38:41.4041659Z     --> src/libstd/io/mod.rs:2952:43
2020-03-31T14:38:41.4042271Z      |
2020-03-31T14:38:41.4043089Z 2952 |             (&[IoSlice::new(&[1, 2, 3])], &[1, 2, 3]),
2020-03-31T14:38:41.4044405Z      |                                           ^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 3 elements
2020-03-31T14:38:41.4475350Z error[E0308]: mismatched types
2020-03-31T14:38:41.4476157Z     --> src/libstd/io/mod.rs:2953:14
2020-03-31T14:38:41.4476744Z      |
2020-03-31T14:38:41.4476744Z      |
2020-03-31T14:38:41.4477589Z 2953 |             (&[IoSlice::new(&[1, 2, 3, 4])], &[1, 2, 3, 4]),
2020-03-31T14:38:41.4478925Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.4480596Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.4481512Z                 found reference `&[io::IoSlice<'_>; 1]`
2020-03-31T14:38:41.4481837Z 
2020-03-31T14:38:41.4906395Z error[E0308]: mismatched types
2020-03-31T14:38:41.4906395Z error[E0308]: mismatched types
2020-03-31T14:38:41.4907219Z     --> src/libstd/io/mod.rs:2953:46
2020-03-31T14:38:41.4907803Z      |
2020-03-31T14:38:41.4908644Z 2953 |             (&[IoSlice::new(&[1, 2, 3, 4])], &[1, 2, 3, 4]),
2020-03-31T14:38:41.4910300Z      |                                              ^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 4 elements
2020-03-31T14:38:41.5321882Z error[E0308]: mismatched types
2020-03-31T14:38:41.5322688Z     --> src/libstd/io/mod.rs:2954:14
2020-03-31T14:38:41.5323276Z      |
2020-03-31T14:38:41.5323276Z      |
2020-03-31T14:38:41.5324134Z 2954 |             (&[IoSlice::new(&[1, 2, 3, 4, 5])], &[1, 2, 3, 4, 5]),
2020-03-31T14:38:41.5325468Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 1 element
2020-03-31T14:38:41.5327188Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.5328089Z                 found reference `&[io::IoSlice<'_>; 1]`
2020-03-31T14:38:41.5328411Z 
2020-03-31T14:38:41.5724456Z error[E0308]: mismatched types
2020-03-31T14:38:41.5724456Z error[E0308]: mismatched types
2020-03-31T14:38:41.5725215Z     --> src/libstd/io/mod.rs:2954:49
2020-03-31T14:38:41.5725804Z      |
2020-03-31T14:38:41.5726680Z 2954 |             (&[IoSlice::new(&[1, 2, 3, 4, 5])], &[1, 2, 3, 4, 5]),
2020-03-31T14:38:41.5728039Z      |                                                 ^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 5 elements
2020-03-31T14:38:41.6211479Z error[E0308]: mismatched types
2020-03-31T14:38:41.6212257Z     --> src/libstd/io/mod.rs:2955:14
2020-03-31T14:38:41.6212894Z      |
2020-03-31T14:38:41.6212894Z      |
2020-03-31T14:38:41.6214016Z 2955 |             (&[IoSlice::new(&[1]), IoSlice::new(&[2])], &[1, 2]),
2020-03-31T14:38:41.6215359Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.6217074Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.6217979Z                 found reference `&[io::IoSlice<'_>; 2]`
2020-03-31T14:38:41.6218305Z 
2020-03-31T14:38:41.6578252Z error[E0308]: mismatched types
2020-03-31T14:38:41.6578252Z error[E0308]: mismatched types
2020-03-31T14:38:41.6579018Z     --> src/libstd/io/mod.rs:2955:57
2020-03-31T14:38:41.6579639Z      |
2020-03-31T14:38:41.6580510Z 2955 |             (&[IoSlice::new(&[1]), IoSlice::new(&[2])], &[1, 2]),
2020-03-31T14:38:41.6581882Z      |                                                         ^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.6983932Z error[E0308]: mismatched types
2020-03-31T14:38:41.6984689Z     --> src/libstd/io/mod.rs:2956:14
2020-03-31T14:38:41.6985496Z      |
2020-03-31T14:38:41.6985496Z      |
2020-03-31T14:38:41.6986412Z 2956 |             (&[IoSlice::new(&[1, 1]), IoSlice::new(&[2, 2])], &[1, 1, 2, 2]),
2020-03-31T14:38:41.6987878Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.6989629Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.6990529Z                 found reference `&[io::IoSlice<'_>; 2]`
2020-03-31T14:38:41.6990875Z 
2020-03-31T14:38:41.7351375Z error[E0308]: mismatched types
2020-03-31T14:38:41.7351375Z error[E0308]: mismatched types
2020-03-31T14:38:41.7352133Z     --> src/libstd/io/mod.rs:2956:63
2020-03-31T14:38:41.7352762Z      |
2020-03-31T14:38:41.7353678Z 2956 |             (&[IoSlice::new(&[1, 1]), IoSlice::new(&[2, 2])], &[1, 1, 2, 2]),
2020-03-31T14:38:41.7355121Z      |                                                               ^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 4 elements
2020-03-31T14:38:41.7777722Z error[E0308]: mismatched types
2020-03-31T14:38:41.7779178Z     --> src/libstd/io/mod.rs:2957:14
2020-03-31T14:38:41.7780025Z      |
2020-03-31T14:38:41.7780025Z      |
2020-03-31T14:38:41.7781149Z 2957 |             (&[IoSlice::new(&[1, 1, 1]), IoSlice::new(&[2, 2, 2])], &[1, 1, 1, 2, 2, 2]),
2020-03-31T14:38:41.7782810Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.7785140Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.7786209Z                 found reference `&[io::IoSlice<'_>; 2]`
2020-03-31T14:38:41.7786697Z 
2020-03-31T14:38:41.8216086Z error[E0308]: mismatched types
2020-03-31T14:38:41.8216086Z error[E0308]: mismatched types
2020-03-31T14:38:41.8216695Z     --> src/libstd/io/mod.rs:2957:69
2020-03-31T14:38:41.8217182Z      |
2020-03-31T14:38:41.8217927Z 2957 |             (&[IoSlice::new(&[1, 1, 1]), IoSlice::new(&[2, 2, 2])], &[1, 1, 1, 2, 2, 2]),
2020-03-31T14:38:41.8219143Z      |                                                                     ^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 6 elements
2020-03-31T14:38:41.8630285Z error[E0308]: mismatched types
2020-03-31T14:38:41.8631083Z     --> src/libstd/io/mod.rs:2958:14
2020-03-31T14:38:41.8631679Z      |
2020-03-31T14:38:41.8631679Z      |
2020-03-31T14:38:41.8632690Z 2958 |             (&[IoSlice::new(&[1, 1, 1, 1]), IoSlice::new(&[2, 2, 2, 2])], &[1, 1, 1, 1, 2, 2, 2, 2]),
2020-03-31T14:38:41.8634167Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 2 elements
2020-03-31T14:38:41.8636205Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.8637128Z                 found reference `&[io::IoSlice<'_>; 2]`
2020-03-31T14:38:41.8637473Z 
2020-03-31T14:38:41.9045841Z error[E0308]: mismatched types
2020-03-31T14:38:41.9045841Z error[E0308]: mismatched types
2020-03-31T14:38:41.9046612Z     --> src/libstd/io/mod.rs:2958:75
2020-03-31T14:38:41.9047240Z      |
2020-03-31T14:38:41.9048229Z 2958 |             (&[IoSlice::new(&[1, 1, 1, 1]), IoSlice::new(&[2, 2, 2, 2])], &[1, 1, 1, 1, 2, 2, 2, 2]),
2020-03-31T14:38:41.9049804Z      |                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 8 elements
2020-03-31T14:38:41.9505793Z error[E0308]: mismatched types
2020-03-31T14:38:41.9506568Z     --> src/libstd/io/mod.rs:2959:14
2020-03-31T14:38:41.9507153Z      |
2020-03-31T14:38:41.9507153Z      |
2020-03-31T14:38:41.9508120Z 2959 |             (&[IoSlice::new(&[1]), IoSlice::new(&[2]), IoSlice::new(&[3])], &[1, 2, 3]),
2020-03-31T14:38:41.9509586Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 3 elements
2020-03-31T14:38:41.9511358Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:41.9512307Z                 found reference `&[io::IoSlice<'_>; 3]`
2020-03-31T14:38:41.9512654Z 
2020-03-31T14:38:41.9917330Z error[E0308]: mismatched types
2020-03-31T14:38:41.9917330Z error[E0308]: mismatched types
2020-03-31T14:38:41.9918111Z     --> src/libstd/io/mod.rs:2959:77
2020-03-31T14:38:41.9918738Z      |
2020-03-31T14:38:41.9919687Z 2959 |             (&[IoSlice::new(&[1]), IoSlice::new(&[2]), IoSlice::new(&[3])], &[1, 2, 3]),
2020-03-31T14:38:41.9921211Z      |                                                                             ^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 3 elements
2020-03-31T14:38:42.0350540Z error[E0308]: mismatched types
2020-03-31T14:38:42.0351345Z     --> src/libstd/io/mod.rs:2960:14
2020-03-31T14:38:42.0351965Z      |
2020-03-31T14:38:42.0351965Z      |
2020-03-31T14:38:42.0353001Z 2960 |             (&[IoSlice::new(&[1, 1]), IoSlice::new(&[2, 2]), IoSlice::new(&[3, 3])], &[1, 1, 2, 2, 3, 3]),
2020-03-31T14:38:42.0354528Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 3 elements
2020-03-31T14:38:42.0356308Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:42.0357396Z                 found reference `&[io::IoSlice<'_>; 3]`
2020-03-31T14:38:42.0357741Z 
2020-03-31T14:38:42.0747117Z error[E0308]: mismatched types
2020-03-31T14:38:42.0747117Z error[E0308]: mismatched types
2020-03-31T14:38:42.0748055Z     --> src/libstd/io/mod.rs:2960:86
2020-03-31T14:38:42.0748708Z      |
2020-03-31T14:38:42.0749719Z 2960 |             (&[IoSlice::new(&[1, 1]), IoSlice::new(&[2, 2]), IoSlice::new(&[3, 3])], &[1, 1, 2, 2, 3, 3]),
2020-03-31T14:38:42.0751334Z      |                                                                                      ^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 6 elements
2020-03-31T14:38:42.1211425Z error[E0308]: mismatched types
2020-03-31T14:38:42.1212268Z     --> src/libstd/io/mod.rs:2961:14
2020-03-31T14:38:42.1212866Z      |
2020-03-31T14:38:42.1212866Z      |
2020-03-31T14:38:42.1214376Z 2961 |             (&[IoSlice::new(&[1, 1, 1]), IoSlice::new(&[2, 2, 2]), IoSlice::new(&[3, 3, 3])], &[1, 1, 1, 2, 2, 2, 3, 3, 3]),
2020-03-31T14:38:42.1215982Z      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 3 elements
2020-03-31T14:38:42.1217785Z      = note: expected reference `&[io::IoSlice<'_>; 0]`
2020-03-31T14:38:42.1218689Z                 found reference `&[io::IoSlice<'_>; 3]`
2020-03-31T14:38:42.1219039Z 
2020-03-31T14:38:42.1620054Z error[E0308]: mismatched types
2020-03-31T14:38:42.1620054Z error[E0308]: mismatched types
2020-03-31T14:38:42.1623323Z     --> src/libstd/io/mod.rs:2961:95
2020-03-31T14:38:42.1623962Z      |
2020-03-31T14:38:42.1625043Z 2961 |             (&[IoSlice::new(&[1, 1, 1]), IoSlice::new(&[2, 2, 2]), IoSlice::new(&[3, 3, 3])], &[1, 1, 1, 2, 2, 2, 3, 3, 3]),
2020-03-31T14:38:42.1626752Z      |                                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 0 elements, found one with 9 elements
2020-03-31T14:38:42.1645785Z error[E0308]: mismatched types
2020-03-31T14:38:42.1646550Z     --> src/libstd/io/mod.rs:2968:53
2020-03-31T14:38:42.1647157Z      |
2020-03-31T14:38:42.1647157Z      |
2020-03-31T14:38:42.1647969Z 2968 |                 let mut writer = test_writer(n_bus, per_call);
2020-03-31T14:38:42.1649852Z      |                                                     |
2020-03-31T14:38:42.1649852Z      |                                                     |
2020-03-31T14:38:42.1650892Z      |                                                     expected `usize`, found `&{integer}`
2020-03-31T14:38:42.1652117Z      |                                                     help: consider dereferencing the borrow: `*per_call`
2020-03-31T14:38:42.1657965Z 
2020-03-31T14:38:42.1673630Z error[E0277]: can't compare `[u8]` with `&[{integer}; 0]`
2020-03-31T14:38:42.1674985Z      |
2020-03-31T14:38:42.1675648Z 66   | / macro_rules! assert_eq {
2020-03-31T14:38:42.1675648Z 66   | / macro_rules! assert_eq {
2020-03-31T14:38:42.1676551Z 67   | |     ($left:expr, $right:expr) => ({
2020-03-31T14:38:42.1677449Z 68   | |         match (&$left, &$right) {
2020-03-31T14:38:42.1678291Z 69   | |             (left_val, right_val) => {
2020-03-31T14:38:42.1679189Z 70   | |                 if !(*left_val == *right_val) {
2020-03-31T14:38:42.1680228Z      | |                                ^^ no implementation for `[u8] == &[{integer}; 0]`
2020-03-31T14:38:42.1681646Z 98   | |     });
2020-03-31T14:38:42.1682352Z 99   | | }
2020-03-31T14:38:42.1683125Z      | |_- in this expansion of `assert_eq!`
2020-03-31T14:38:42.1683768Z      | 
2020-03-31T14:38:42.1683768Z      | 
2020-03-31T14:38:42.1684282Z     ::: src/libstd/io/mod.rs:2970:17
2020-03-31T14:38:42.1684948Z      |
2020-03-31T14:38:42.1685616Z 2970 |                   assert_eq!(&*writer.written, &*wanted);
2020-03-31T14:38:42.1687238Z      |
2020-03-31T14:38:42.1687238Z      |
2020-03-31T14:38:42.1687971Z      = help: the trait `core::cmp::PartialEq<&[{integer}; 0]>` is not implemented for `[u8]`
2020-03-31T14:38:42.1689018Z      = note: required because of the requirements on the impl of `core::cmp::PartialEq<&&[{integer}; 0]>` for `&[u8]`
2020-03-31T14:38:48.5139529Z error: aborting due to 32 previous errors
2020-03-31T14:38:48.5140423Z 
2020-03-31T14:38:48.5141416Z Some errors have detailed explanations: E0061, E0277, E0308, E0425, E0599, E0600.
2020-03-31T14:38:48.5142444Z For more information about an error, try `rustc --explain E0061`.
---
2020-03-31T14:38:48.5428645Z   local time: Tue Mar 31 14:38:48 UTC 2020
2020-03-31T14:38:48.6496575Z   network time: Tue, 31 Mar 2020 14:38:48 GMT
2020-03-31T14:38:48.6501189Z == end clock drift check ==
2020-03-31T14:38:49.1265617Z 
2020-03-31T14:38:49.1328879Z ##[error]Bash exited with code '1'.
2020-03-31T14:38:49.1342739Z ##[section]Finishing: Run build
2020-03-31T14:38:49.1394794Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T14:38:49.1399876Z Task         : Get sources
2020-03-31T14:38:49.1400193Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T14:38:49.1400507Z Version      : 1.0.0
2020-03-31T14:38:49.1400718Z Author       : Microsoft
2020-03-31T14:38:49.1400718Z Author       : Microsoft
2020-03-31T14:38:49.1401059Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T14:38:49.1401453Z ==============================================================================
2020-03-31T14:38:49.4583700Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T14:38:49.4638530Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70612/merge to s
2020-03-31T14:38:49.4720334Z Cleaning up task key
2020-03-31T14:38:49.4721253Z Start cleaning up orphan processes.
2020-03-31T14:38:49.4888595Z Terminate orphan process: pid (3582) (python)
2020-03-31T14:38:49.5082228Z ##[section]Finishing: Finalize Job
