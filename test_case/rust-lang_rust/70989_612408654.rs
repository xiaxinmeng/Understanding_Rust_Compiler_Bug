plain
2020-04-11T10:44:15.9319722Z ========================== Starting Command Output ===========================
2020-04-11T10:44:15.9335917Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/31e349e0-5c8e-4a86-a56c-cb4972ec7a13.sh
2020-04-11T10:44:15.9533913Z 
2020-04-11T10:44:15.9584524Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T10:44:15.9603941Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T10:44:15.9607894Z Task         : Get sources
2020-04-11T10:44:15.9608180Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T10:44:15.9608456Z Version      : 1.0.0
2020-04-11T10:44:15.9608662Z Author       : Microsoft
---
2020-04-11T10:44:17.5826908Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T10:44:17.6176694Z ##[command]git config gc.auto 0
2020-04-11T10:44:17.6180645Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T10:44:17.6183766Z ##[command]git config --get-all http.proxy
2020-04-11T10:44:17.6190939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-11T10:48:39.2487406Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-11T10:48:39.2957656Z  ---> Running in f7960bf5057e
2020-04-11T10:48:40.0471826Z Removing intermediate container f7960bf5057e
2020-04-11T10:48:40.0472835Z  ---> 0d8770ce0d10
2020-04-11T10:48:40.0474059Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-11T10:48:41.0400054Z Removing intermediate container 0aac24434045
2020-04-11T10:48:41.0401067Z  ---> 4096a3da9b69
2020-04-11T10:48:41.0401389Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-11T10:48:41.0806247Z  ---> Running in 154db14d5377
---
2020-04-11T10:48:43.0594886Z Successfully built a31f8380b06e
2020-04-11T10:48:43.0685338Z Successfully tagged rust-ci:latest
2020-04-11T10:48:43.1162669Z Built container sha256:a31f8380b06e6065d7ef3f9b616a1ecc46a7e8b86c5320645aae8e8fd8fea9a9
2020-04-11T10:48:43.1180912Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/cf6945e9097f0f4cb1887fc32b481a554ad8b9fbbe9ab7d5f97da51aa02586b6695575bcd35291cb514a2db1c5469b6790f35dbd05af37dfc0a40a0be5375cd6
2020-04-11T10:49:33.6603735Z upload failed: - to s3://rust-lang-ci-sccache2/docker/cf6945e9097f0f4cb1887fc32b481a554ad8b9fbbe9ab7d5f97da51aa02586b6695575bcd35291cb514a2db1c5469b6790f35dbd05af37dfc0a40a0be5375cd6 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-11T10:49:34.1399439Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-11T10:49:34.1426018Z == clock drift check ==
2020-04-11T10:49:34.1435170Z   local time: Sat Apr 11 10:49:34 UTC 2020
2020-04-11T10:49:34.1435170Z   local time: Sat Apr 11 10:49:34 UTC 2020
2020-04-11T10:49:34.4314640Z   network time: Sat, 11 Apr 2020 10:49:34 GMT
2020-04-11T10:49:34.4338804Z Starting sccache server...
2020-04-11T10:49:34.5136037Z configure: processing command line
2020-04-11T10:49:34.5137691Z configure: 
2020-04-11T10:49:34.5139688Z configure: rust.dist-src        := False
---
2020-04-11T10:54:04.4038617Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T10:54:05.6900631Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T10:54:07.1509737Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T10:54:07.9815514Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T10:54:16.0458717Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T10:54:17.9183405Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T10:54:21.8353273Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T10:54:25.5051877Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T10:54:34.1229489Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T11:14:23.7732286Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T11:14:25.2746990Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T11:14:27.0487453Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T11:14:28.3680278Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T11:14:38.0674328Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T11:14:40.2494521Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T11:14:44.9739228Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T11:14:49.7671642Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T11:14:59.8248124Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T11:37:47.0038503Z .................................................................................................... 1700/9886
2020-04-11T11:37:50.9017195Z .................................................................................................... 1800/9886
2020-04-11T11:37:58.5792947Z .................................................................................................... 1900/9886
2020-04-11T11:38:05.8876882Z ....i............................................................................................... 2000/9886
2020-04-11T11:38:11.6189724Z ..............................................................................................iiiii. 2100/9886
2020-04-11T11:38:30.3451312Z .................................................................................................... 2300/9886
2020-04-11T11:38:32.2788548Z .................................................................................................... 2400/9886
2020-04-11T11:38:34.2876655Z .................................................................................................... 2500/9886
2020-04-11T11:38:39.5206153Z .................................................................................................... 2600/9886
---
2020-04-11T11:41:13.9450767Z ....................................................................i...............i............... 5000/9886
2020-04-11T11:41:20.8141172Z .................................................................................................... 5100/9886
2020-04-11T11:41:27.5345041Z .................................................................................................... 5200/9886
2020-04-11T11:41:32.1695218Z .............i...................................................................................... 5300/9886
2020-04-11T11:41:40.8017402Z ..i................................................................................................. 5400/9886
2020-04-11T11:41:45.4986920Z ..ii.ii........i...i................................................................................ 5500/9886
2020-04-11T11:41:52.4267399Z ...............................................i.................................................... 5700/9886
2020-04-11T11:42:01.7289583Z ...................................................................ii............................... 5800/9886
2020-04-11T11:42:07.2946122Z ......i............................................................................................. 5900/9886
2020-04-11T11:42:12.1895643Z .................................................................................................... 6000/9886
2020-04-11T11:42:12.1895643Z .................................................................................................... 6000/9886
2020-04-11T11:42:20.3332397Z .................................................................................................... 6100/9886
2020-04-11T11:42:29.7935078Z ii...i..ii...........i.............................................................................. 6200/9886
2020-04-11T11:42:41.5955694Z .................................................................................................... 6400/9886
2020-04-11T11:42:44.2155612Z .................................................................................................... 6500/9886
2020-04-11T11:42:44.2155612Z .................................................................................................... 6500/9886
2020-04-11T11:42:53.8142136Z ..............................i..ii................................................................. 6600/9886
2020-04-11T11:43:10.7066176Z .................................................................................................... 6800/9886
2020-04-11T11:43:12.3360484Z ..............................i..................................................................... 6900/9886
2020-04-11T11:43:13.9696796Z .................................................................................................... 7000/9886
2020-04-11T11:43:15.7718559Z .....................................................................i.............................. 7100/9886
---
2020-04-11T11:44:33.1640259Z .................................................................................................... 7800/9886
2020-04-11T11:44:36.6329543Z .................................................................................................... 7900/9886
2020-04-11T11:44:42.0941941Z .................................................................................................... 8000/9886
2020-04-11T11:44:47.7040365Z ..................................i................................................................. 8100/9886
2020-04-11T11:44:54.6953941Z ..................................................................................iiiiii.iiiii.i.... 8200/9886
2020-04-11T11:45:07.0444659Z ............................i......i................................................................ 8400/9886
2020-04-11T11:45:09.6856808Z .................................................................................................... 8500/9886
2020-04-11T11:45:18.4286201Z .................................................................................................... 8600/9886
2020-04-11T11:45:28.4691213Z .................................................................................................... 8700/9886
---
2020-04-11T11:47:25.1614586Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-11T11:47:25.1776990Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T11:47:26.0683287Z 
2020-04-11T11:47:26.0694913Z running 185 tests
2020-04-11T11:47:27.4578869Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-11T11:47:29.5647671Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-11T11:47:29.5650827Z 
2020-04-11T11:47:29.5651021Z  finished in 4.376
2020-04-11T11:47:29.5651651Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-11T11:47:29.5721981Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T11:47:31.2794656Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-11T11:47:31.2956260Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T11:47:31.4137092Z 
2020-04-11T11:47:31.4137382Z running 9 tests
2020-04-11T11:47:31.4138152Z iiiiiiiii
2020-04-11T11:47:31.4138841Z 
2020-04-11T11:47:31.4138947Z  finished in 0.117
2020-04-11T11:47:31.4139350Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-11T11:47:31.4308486Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T11:47:46.9986328Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-11T11:47:47.0182007Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T11:47:48.0681292Z 
2020-04-11T11:47:48.0681903Z running 115 tests
2020-04-11T11:47:57.3738058Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i..i...ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-11T11:47:58.7474685Z ...iiii.....ii.
2020-04-11T11:47:58.7493883Z 
2020-04-11T11:47:58.7494021Z  finished in 11.729
2020-04-11T11:47:58.7496297Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T11:47:58.7497288Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T11:57:34.6805260Z 
2020-04-11T11:57:34.6806236Z    Doc-tests core
2020-04-11T11:57:38.1174259Z 
2020-04-11T11:57:38.1180026Z running 2490 tests
2020-04-11T11:57:45.1925758Z ......iiiii......................................................................................... 100/2490
2020-04-11T11:57:51.9969358Z .....................................................................................ii............. 200/2490
2020-04-11T11:58:07.8941312Z ....................i............................................................................... 400/2490
2020-04-11T11:58:07.8941312Z ....................i............................................................................... 400/2490
2020-04-11T11:58:15.2751587Z ..........................................................................i..i..................iiii 500/2490
2020-04-11T11:58:27.5954117Z .................................................................................................... 700/2490
2020-04-11T11:58:33.9357301Z .................................................................................................... 800/2490
2020-04-11T11:58:40.2907185Z .................................................................................................... 900/2490
2020-04-11T11:58:46.6081509Z .................................................................................................... 1000/2490
---
2020-04-11T12:01:26.1069314Z ..........thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
2020-04-11T12:01:26.1070121Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-04-11T12:01:26.1070494Z   left: `1`,
2020-04-11T12:01:26.1071022Z  right: `2`', src/libstd/sync/mutex.rs:657:13
2020-04-11T12:01:26.1223091Z ........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:789:13
2020-04-11T12:01:26.1232956Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:765:13
2020-04-11T12:01:26.1245441Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:701:13
2020-04-11T12:01:26.1250704Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:629:13
2020-04-11T12:01:26.1255948Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
2020-04-11T12:01:26.1261024Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:603:13
2020-04-11T12:01:26.1272314Z .thread '<unnamed>' panicked at 'explicit panic.', src/libstd/sync/rwlock.rs:616:13
2020-04-11T12:01:28.1770327Z ....................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-04-11T12:01:28.7847931Z ............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-04-11T12:01:28.7854754Z .thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1692:13
2020-04-11T12:01:28.7862286Z .thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
---
2020-04-11T12:01:35.0743863Z 
2020-04-11T12:01:35.0744188Z running 1019 tests
2020-04-11T12:01:49.3248234Z i................................................................................................... 100/1019
2020-04-11T12:01:57.6288324Z .................................................................................................... 200/1019
2020-04-11T12:02:03.7893290Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-11T12:02:14.2076672Z ...................................................i....i......................................ii... 500/1019
2020-04-11T12:02:21.4374439Z .................................................................................................... 600/1019
2020-04-11T12:02:26.1061557Z .................................................................................................... 700/1019
2020-04-11T12:02:26.1061557Z .................................................................................................... 700/1019
2020-04-11T12:02:32.3963649Z .............................................iiii................................................... 800/1019
2020-04-11T12:02:44.5578939Z .................................................................................................... 900/1019
2020-04-11T12:02:49.7708248Z ...................................................................iiii............................. 1000/1019
2020-04-11T12:02:50.8756790Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-11T12:02:50.8757257Z 
2020-04-11T12:02:50.8858680Z  finished in 141.516
2020-04-11T12:02:50.8864466Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-11T12:05:32.0280184Z 
2020-04-11T12:05:32.0280392Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-11T12:05:32.0280580Z 
2020-04-11T12:05:32.0346022Z  finished in 0.821
2020-04-11T12:05:32.0346884Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T12:05:32.0347559Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-11T12:05:32.2017092Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T12:05:33.0590394Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-fa888ad82aaa654f
2020-04-11T12:05:33.0617502Z 
2020-04-11T12:05:33.0617767Z running 0 tests
2020-04-11T12:05:33.0617896Z 
---
2020-04-11T12:17:43.9330403Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9331305Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9332092Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9332877Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9333905Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9334597Z Set({"/checkout/src/librustc_ skipping - compiler/librustdoc docs disabled
2020-04-11T12:17:43.9336510Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9337302Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9338057Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T12:17:43.9338829Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-11T12:18:34.8667473Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T12:18:34.8961608Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T12:18:35.0692029Z 
2020-04-11T12:18:35.0692623Z running 210 tests
2020-04-11T12:19:00.7084981Z ......................i...ii.......................................................................i 100/210
2020-04-11T12:19:31.2010642Z ........................................iiiiii......i..............iii.............................. 200/210
2020-04-11T12:19:35.8121003Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-04-11T12:19:35.8121410Z 
2020-04-11T12:19:35.8124677Z  finished in 60.916
2020-04-11T12:19:35.8127965Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-04-11T12:20:48.0524992Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-11T12:20:48.0704025Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T12:20:48.1928782Z 
2020-04-11T12:20:48.1930035Z running 13 tests
2020-04-11T12:20:48.5354343Z .iiiiiii.iii.
2020-04-11T12:20:48.5356387Z 
2020-04-11T12:20:48.5356554Z  finished in 0.465
2020-04-11T12:20:48.5412346Z Build completed successfully in 1:29:52
2020-04-11T12:20:48.5412346Z Build completed successfully in 1:29:52
2020-04-11T12:20:48.5423261Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-11T12:20:49.6095491Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T12:20:49.7953216Z     Finished release [optimized] target(s) in 0.18s
2020-04-11T12:20:49.7991185Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-11T12:20:49.8083184Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T12:21:59.9749761Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-11T12:21:59.9749920Z 23     }
2020-04-11T12:21:59.9750017Z 24 }
2020-04-11T12:21:59.9750122Z 25 
2020-04-11T12:21:59.9750468Z - // HACK(eddyb) this is for testing if PR CI catches a 32-bit mir-opt failure.
2020-04-11T12:21:59.9750745Z - 
2020-04-11T12:21:59.9750920Z 28 alloc0 (static: FOO, size: 8, align: 4) {
2020-04-11T12:21:59.9751299Z 29     ╾alloc24+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-11T12:21:59.9751552Z 
2020-04-11T12:21:59.9751552Z 
2020-04-11T12:21:59.9752166Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/32bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3159:25
2020-04-11T12:21:59.9753298Z 
2020-04-11T12:21:59.9753365Z 
2020-04-11T12:21:59.9753455Z failures:
2020-04-11T12:21:59.9753739Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-11T12:21:59.9753739Z     [mir-opt] mir-opt/const_allocation2.rs
2020-04-11T12:21:59.9753873Z 
2020-04-11T12:21:59.9754216Z test result: FAILED. 88 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-11T12:21:59.9754396Z 
2020-04-11T12:21:59.9755178Z main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-11T12:21:59.9756249Z 
2020-04-11T12:21:59.9756324Z 
2020-04-11T12:21:59.9759174Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-11T12:21:59.9761098Z 
2020-04-11T12:21:59.9761166Z 
2020-04-11T12:21:59.9761166Z 
2020-04-11T12:21:59.9761611Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-11T12:21:59.9799295Z == clock drift check ==
2020-04-11T12:21:59.9810796Z   local time: Sat Apr 11 12:21:59 UTC 2020
2020-04-11T12:21:59.9810796Z   local time: Sat Apr 11 12:21:59 UTC 2020
2020-04-11T12:22:00.0780587Z   network time: Sat, 11 Apr 2020 12:22:00 GMT
2020-04-11T12:22:02.6417360Z 
2020-04-11T12:22:02.6417360Z 
2020-04-11T12:22:02.6484561Z ##[error]Bash exited with code '1'.
2020-04-11T12:22:02.6495065Z ##[section]Finishing: Run build
2020-04-11T12:22:02.6548015Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T12:22:02.6553148Z Task         : Get sources
2020-04-11T12:22:02.6553501Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T12:22:02.6553813Z Version      : 1.0.0
2020-04-11T12:22:02.6554034Z Author       : Microsoft
2020-04-11T12:22:02.6554034Z Author       : Microsoft
2020-04-11T12:22:02.6554511Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T12:22:02.6555086Z ==============================================================================
2020-04-11T12:22:03.0061680Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T12:22:03.0112762Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T12:22:03.0184509Z Cleaning up task key
2020-04-11T12:22:03.0185430Z Start cleaning up orphan processes.
2020-04-11T12:22:03.0325602Z Terminate orphan process: pid (4461) (python)
2020-04-11T12:22:03.1766319Z ##[section]Finishing: Finalize Job
