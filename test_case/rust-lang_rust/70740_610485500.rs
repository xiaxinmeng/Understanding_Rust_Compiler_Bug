plain
2020-04-07T14:33:02.5955946Z ========================== Starting Command Output ===========================
2020-04-07T14:33:02.5959153Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/27bfcd20-e360-4cd5-92a9-4240c1a74929.sh
2020-04-07T14:33:02.5959460Z 
2020-04-07T14:33:02.5963463Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T14:33:02.5981463Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70740/merge to s
2020-04-07T14:33:02.5984625Z Task         : Get sources
2020-04-07T14:33:02.5986183Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T14:33:02.5986484Z Version      : 1.0.0
2020-04-07T14:33:02.5986680Z Author       : Microsoft
---
2020-04-07T14:33:03.5900715Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T14:33:03.5912254Z ##[command]git config gc.auto 0
2020-04-07T14:33:03.5919350Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T14:33:03.5927233Z ##[command]git config --get-all http.proxy
2020-04-07T14:33:03.5936609Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70740/merge:refs/remotes/pull/70740/merge
---
2020-04-07T14:35:10.1805697Z Looks like docker image is the same as before, not uploading
2020-04-07T14:35:14.6439200Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T14:35:14.6727657Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T14:35:14.6757960Z == clock drift check ==
2020-04-07T14:35:14.6766148Z   local time: Tue Apr  7 14:35:14 UTC 2020
2020-04-07T14:35:14.9704524Z   network time: Tue, 07 Apr 2020 14:35:14 GMT
2020-04-07T14:35:14.9736864Z Starting sccache server...
2020-04-07T14:35:15.0545660Z configure: processing command line
2020-04-07T14:35:15.0546259Z configure: 
2020-04-07T14:35:15.0547681Z configure: rust.dist-src        := False
---
2020-04-07T14:40:37.4367991Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T14:40:39.0357361Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T14:40:40.6872991Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T14:40:41.7917734Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T14:40:51.1194686Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T14:40:53.7628583Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T14:40:58.3693865Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T14:41:02.6972836Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T14:41:12.6112730Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T15:04:39.9159903Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T15:04:41.7334553Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T15:04:43.7458380Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T15:04:44.9468785Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T15:04:56.8023644Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T15:04:59.4247351Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T15:05:04.9546187Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T15:05:10.5484769Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T15:05:22.6440840Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T15:30:41.0398941Z .................................................................................................... 1700/9879
2020-04-07T15:30:45.3448842Z .................................................................................................... 1800/9879
2020-04-07T15:30:54.4732820Z ..................................................................................................i. 1900/9879
2020-04-07T15:31:02.5408712Z .................................................................................................... 2000/9879
2020-04-07T15:31:08.9428292Z ........................................................................................iiiii....... 2100/9879
2020-04-07T15:31:30.3617238Z .................................................................................................... 2300/9879
2020-04-07T15:31:32.4858947Z .................................................................................................... 2400/9879
2020-04-07T15:31:34.6959053Z .................................................................................................... 2500/9879
2020-04-07T15:31:40.6585911Z .................................................................................................... 2600/9879
---
2020-04-07T15:34:35.5526133Z ..............................................................i...............i..................... 5000/9879
2020-04-07T15:34:42.9143659Z .................................................................................................... 5100/9879
2020-04-07T15:34:50.4599399Z .................................................................................................... 5200/9879
2020-04-07T15:34:55.7960792Z .......i............................................................................................ 5300/9879
2020-04-07T15:35:05.9934214Z ................................................................................................ii.i 5400/9879
2020-04-07T15:35:10.8135424Z i........i...i...................................................................................... 5500/9879
2020-04-07T15:35:19.1056981Z .........................................i.......................................................... 5700/9879
2020-04-07T15:35:28.9129964Z .............................................................ii..................................... 5800/9879
2020-04-07T15:35:35.8684342Z i................................................................................................... 5900/9879
2020-04-07T15:35:40.9488997Z .................................................................................................... 6000/9879
2020-04-07T15:35:40.9488997Z .................................................................................................... 6000/9879
2020-04-07T15:35:50.4813503Z ..............................................................................................ii...i 6100/9879
2020-04-07T15:36:02.2007103Z ..ii...........i.................................................................................... 6200/9879
2020-04-07T15:36:17.6423851Z .................................................................................................... 6400/9879
2020-04-07T15:36:23.2758900Z .................................................................................................... 6500/9879
2020-04-07T15:36:23.2758900Z .................................................................................................... 6500/9879
2020-04-07T15:36:35.8104153Z ........................i..ii....................................................................... 6600/9879
2020-04-07T15:36:56.7974133Z .................................................................................................... 6800/9879
2020-04-07T15:36:58.7942749Z ........................i........................................................................... 6900/9879
2020-04-07T15:37:00.7682814Z .................................................................................................... 7000/9879
2020-04-07T15:37:02.8681919Z ...............................................................i.................................... 7100/9879
---
2020-04-07T15:38:40.9459729Z .................................................................................................... 7800/9879
2020-04-07T15:38:45.4158224Z .................................................................................................... 7900/9879
2020-04-07T15:38:51.3862149Z .................................................................................................... 8000/9879
2020-04-07T15:38:59.0846556Z ............................i....................................................................... 8100/9879
2020-04-07T15:39:07.9387162Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-07T15:39:23.5379221Z .....................i......i....................................................................... 8400/9879
2020-04-07T15:39:28.1973784Z .................................................................................................... 8500/9879
2020-04-07T15:39:38.8008181Z .................................................................................................... 8600/9879
2020-04-07T15:39:50.8641204Z .................................................................................................... 8700/9879
---
2020-04-07T15:42:12.9203715Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-07T15:42:12.9382281Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T15:42:13.1457414Z 
2020-04-07T15:42:13.1458635Z running 185 tests
2020-04-07T15:42:15.8055639Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-07T15:42:18.4128245Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-07T15:42:18.4131012Z 
2020-04-07T15:42:18.4137180Z  finished in 5.475
2020-04-07T15:42:18.4144513Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-07T15:42:18.4320273Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-07T15:42:20.4328928Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-07T15:42:20.4520791Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T15:42:20.6073883Z 
2020-04-07T15:42:20.6075180Z running 9 tests
2020-04-07T15:42:20.6083501Z iiiiiiiii
2020-04-07T15:42:20.6087647Z 
2020-04-07T15:42:20.6087848Z  finished in 0.154
2020-04-07T15:42:20.6105233Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-07T15:42:20.6278575Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-07T15:42:40.1222336Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-07T15:42:40.1429720Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T15:42:40.3365512Z 
2020-04-07T15:42:40.3365835Z running 115 tests
2020-04-07T15:42:54.3377070Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-07T15:42:56.1221230Z ...iiii.....ii.
2020-04-07T15:42:56.1227764Z 
2020-04-07T15:42:56.1231329Z  finished in 15.980
2020-04-07T15:42:56.1236002Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-07T15:42:56.1239654Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-07T15:55:43.9064368Z 
2020-04-07T15:55:43.9065859Z    Doc-tests core
2020-04-07T15:55:48.6234615Z 
2020-04-07T15:55:48.6235818Z running 2490 tests
2020-04-07T15:55:57.3719924Z ......iiiii......................................................................................... 100/2490
2020-04-07T15:56:05.9792679Z .....................................................................................ii............. 200/2490
2020-04-07T15:56:25.5595200Z ....................i............................................................................... 400/2490
2020-04-07T15:56:25.5595200Z ....................i............................................................................... 400/2490
2020-04-07T15:56:35.0334097Z ..........................................................................i..i..................iiii 500/2490
2020-04-07T15:56:50.5347959Z .................................................................................................... 700/2490
2020-04-07T15:56:58.5517610Z .................................................................................................... 800/2490
2020-04-07T15:57:06.7388707Z .................................................................................................... 900/2490
2020-04-07T15:57:14.6602502Z .................................................................................................... 1000/2490
---
2020-04-07T16:00:30.3412395Z .................................................................................................... 500/763
2020-04-07T16:00:30.3772929Z .....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-07T16:00:30.3781148Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-07T16:00:30.3784838Z <unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-07T16:00:30.3801629Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-07T16:00:30.5761552Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-07T16:00:30.5803526Z ..........thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-07T16:00:30.5806992Z .src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-07T16:00:32.6417833Z .......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-07T16:00:32.6420787Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-07T16:00:32.6428997Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-07T16:00:32.6430958Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-07T16:00:41.6767639Z 
2020-04-07T16:00:41.6768004Z running 1019 tests
2020-04-07T16:00:59.3971196Z i................................................................................................... 100/1019
2020-04-07T16:01:09.7218961Z .................................................................................................... 200/1019
2020-04-07T16:01:17.1747474Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-07T16:01:28.8853294Z ...................................................i....i......................................ii... 500/1019
2020-04-07T16:01:36.6255760Z .................................................................................................... 600/1019
2020-04-07T16:01:41.4455770Z .................................................................................................... 700/1019
2020-04-07T16:01:41.4455770Z .................................................................................................... 700/1019
2020-04-07T16:01:48.3506670Z .............................................iiii................................................... 800/1019
2020-04-07T16:02:01.9467388Z .................................................................................................... 900/1019
2020-04-07T16:02:07.8899345Z ...................................................................iiii............................. 1000/1019
2020-04-07T16:02:09.0855836Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-07T16:02:09.0856334Z 
2020-04-07T16:02:09.0977350Z  finished in 165.106
2020-04-07T16:02:09.0981886Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-07T16:05:22.8893248Z 
2020-04-07T16:05:22.8893511Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T16:05:22.8893727Z 
2020-04-07T16:05:22.8893969Z  finished in 0.969
2020-04-07T16:05:22.8894864Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-07T16:05:22.8895660Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T16:05:22.8896408Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T16:05:23.8723496Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-e794ad4ca63c576f
2020-04-07T16:05:23.8748084Z 
2020-04-07T16:05:23.8748332Z running 0 tests
2020-04-07T16:05:23.8748454Z 
---
2020-04-07T16:20:19.9730755Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9731474Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9732184Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9733012Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9733940Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9735761Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9736882Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9737750Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-07T16:20:19.9738492Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-07T16:21:21.2047729Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-07T16:21:21.2361752Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T16:21:21.4630689Z 
2020-04-07T16:21:21.4634130Z running 210 tests
2020-04-07T16:21:52.9903616Z ......................i...ii.......................................................................i 100/210
2020-04-07T16:22:28.7358365Z ........................................iiiiii......i..F...........iii.............................. 200/210
2020-04-07T16:22:32.6706822Z failures:
2020-04-07T16:22:32.6715326Z 
2020-04-07T16:22:32.6716030Z ---- [run-make] run-make-fulldeps/relocation-model stdout ----
2020-04-07T16:22:32.6716281Z 
2020-04-07T16:22:32.6716281Z 
2020-04-07T16:22:32.6716438Z error: make failed
2020-04-07T16:22:32.6716650Z status: exit code: 2
2020-04-07T16:22:32.6716858Z command: "make"
2020-04-07T16:22:32.6717013Z stdout:
2020-04-07T16:22:32.6717404Z ------------------------------------------
2020-04-07T16:22:32.6719491Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model  -C relocation-model=static foo.rs
2020-04-07T16:22:32.6721020Z Makefile:17: recipe for target 'others' failed
2020-04-07T16:22:32.6721563Z ------------------------------------------
2020-04-07T16:22:32.6721789Z stderr:
2020-04-07T16:22:32.6722150Z ------------------------------------------
2020-04-07T16:22:32.6722439Z error: linking with `cc` failed: exit code: 1
2020-04-07T16:22:32.6722439Z error: linking with `cc` failed: exit code: 1
2020-04-07T16:22:32.6722672Z   |
2020-04-07T16:22:32.6732568Z   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.4.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.belfx9afw9cmv8.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4c48c2d1f04744c2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-ec5b7f8a670eea42.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-ce7a87dba1f08c55.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-3d56f20c37180f70.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-ddf2431199be19cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-8e7006a0e1bdfe17.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-88b01f153b0d07f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-50bf4b84e7d3fac7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-9e706b3c3b2224bb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9407c0e2b0c1954d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-985ed700904cb888.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-d5e52133a0263c12.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-664a74b296ae1e0d.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-42bfcbfddce464c7.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
2020-04-07T16:22:32.6740518Z   = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/relocation-model/relocation-model/foo.foo.7rcbfp3g-cgu.0.rcgu.o: relocation R_X86_64_32 against symbol `rust_eh_personality' can not be used when making a PIE object; recompile with -fPIC
2020-04-07T16:22:32.6741331Z           /usr/bin/ld: final link failed: Nonrepresentable section on output
2020-04-07T16:22:32.6741696Z           collect2: error: ld returned 1 exit status
2020-04-07T16:22:32.6742074Z 
2020-04-07T16:22:32.6742254Z error: aborting due to previous error
2020-04-07T16:22:32.6742584Z 
2020-04-07T16:22:32.6742584Z 
2020-04-07T16:22:32.6742772Z make: *** [others] Error 1
2020-04-07T16:22:32.6743522Z ------------------------------------------
2020-04-07T16:22:32.6743869Z 
2020-04-07T16:22:32.6743978Z 
2020-04-07T16:22:32.6744069Z 
---
2020-04-07T16:22:32.6745967Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-07T16:22:32.6746385Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T16:22:32.6759812Z 
2020-04-07T16:22:32.6759986Z 
2020-04-07T16:22:32.6769606Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T16:22:32.6776107Z 
2020-04-07T16:22:32.6776220Z 
2020-04-07T16:22:32.6776769Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T16:22:32.6777129Z Build completed unsuccessfully in 1:45:44
2020-04-07T16:22:32.6777129Z Build completed unsuccessfully in 1:45:44
2020-04-07T16:22:32.6804194Z == clock drift check ==
2020-04-07T16:22:32.6826315Z   local time: Tue Apr  7 16:22:32 UTC 2020
2020-04-07T16:22:32.9776089Z   network time: Tue, 07 Apr 2020 16:22:32 GMT
2020-04-07T16:22:35.2125770Z 
2020-04-07T16:22:35.2125770Z 
2020-04-07T16:22:35.2201859Z ##[error]Bash exited with code '1'.
2020-04-07T16:22:35.2237472Z ##[section]Finishing: Run build
2020-04-07T16:22:35.2289459Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70740/merge to s
2020-04-07T16:22:35.2296595Z Task         : Get sources
2020-04-07T16:22:35.2296926Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T16:22:35.2297231Z Version      : 1.0.0
2020-04-07T16:22:35.2297480Z Author       : Microsoft
2020-04-07T16:22:35.2297480Z Author       : Microsoft
2020-04-07T16:22:35.2297822Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T16:22:35.2298216Z ==============================================================================
2020-04-07T16:22:35.5836558Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T16:22:35.5884298Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70740/merge to s
2020-04-07T16:22:35.5969896Z Cleaning up task key
2020-04-07T16:22:35.5971314Z Start cleaning up orphan processes.
2020-04-07T16:22:35.6148701Z Terminate orphan process: pid (3397) (python)
2020-04-07T16:22:35.6386016Z ##[section]Finishing: Finalize Job
