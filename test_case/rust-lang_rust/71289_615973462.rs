plain
2020-04-18T21:17:59.5439264Z ========================== Starting Command Output ===========================
2020-04-18T21:17:59.5445226Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e052b6ff-6ef6-4778-9626-8c2e068d2596.sh
2020-04-18T21:17:59.5445844Z 
2020-04-18T21:17:59.5452592Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T21:17:59.5474189Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T21:17:59.5478141Z Task         : Get sources
2020-04-18T21:17:59.5478445Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T21:17:59.5478741Z Version      : 1.0.0
2020-04-18T21:17:59.5478977Z Author       : Microsoft
---
2020-04-18T21:18:00.5390259Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T21:18:00.5400439Z ##[command]git config gc.auto 0
2020-04-18T21:18:00.5410024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T21:18:00.5420072Z ##[command]git config --get-all http.proxy
2020-04-18T21:18:00.5432214Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71289/merge:refs/remotes/pull/71289/merge
---
2020-04-18T21:20:20.5450349Z  ---> 318032b5f0e2
2020-04-18T21:20:20.5451240Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T21:20:20.5457711Z  ---> Using cache
2020-04-18T21:20:20.5458158Z  ---> d44a858fd1ce
2020-04-18T21:20:20.5459251Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T21:20:20.5532699Z  ---> 58b910f50f5a
2020-04-18T21:20:20.5543874Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T21:20:20.5544750Z  ---> Using cache
2020-04-18T21:20:20.5545210Z  ---> ee7702aadba1
---
2020-04-18T21:20:20.6043424Z Looks like docker image is the same as before, not uploading
2020-04-18T21:20:27.9495750Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T21:20:27.9779656Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T21:20:27.9817237Z == clock drift check ==
2020-04-18T21:20:27.9822420Z   local time: Sat Apr 18 21:20:27 UTC 2020
2020-04-18T21:20:28.3004995Z   network time: Sat, 18 Apr 2020 21:20:28 GMT
2020-04-18T21:20:28.3038673Z Starting sccache server...
2020-04-18T21:20:28.4001335Z configure: processing command line
2020-04-18T21:20:28.4002080Z configure: 
2020-04-18T21:20:28.4003265Z configure: rust.dist-src        := False
---
2020-04-18T21:25:56.3100078Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T21:25:57.8903810Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T21:25:59.5745511Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T21:26:01.3181260Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T21:26:09.7821639Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T21:26:13.9390553Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T21:26:18.6376964Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T21:26:23.1301948Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T21:26:31.6183438Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T21:49:59.2816406Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T21:50:00.9025793Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T21:50:02.8500679Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T21:50:04.7816952Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T21:50:13.9255969Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T21:50:18.4671550Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T21:50:23.2789946Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T21:50:27.8589166Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T21:50:36.4800866Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T22:14:12.9642073Z .................................................................................................... 1700/9906
2020-04-18T22:14:16.8892657Z .................................................................................................... 1800/9906
2020-04-18T22:14:25.4261898Z .................................................................................................i.. 1900/9906
2020-04-18T22:14:32.9496027Z .................................................................................................... 2000/9906
2020-04-18T22:14:39.2058460Z .......................................................................................iiiii........ 2100/9906
2020-04-18T22:14:58.6562248Z .................................................................................................... 2300/9906
2020-04-18T22:15:00.7328989Z .................................................................................................... 2400/9906
2020-04-18T22:15:02.9895041Z .................................................................................................... 2500/9906
2020-04-18T22:15:08.9964695Z .................................................................................................... 2600/9906
---
2020-04-18T22:17:54.4321782Z .................................................................................................... 4900/9906
2020-04-18T22:17:59.2918727Z ...............................................................i...............i.................... 5000/9906
2020-04-18T22:18:06.6757870Z .................................................................................................... 5100/9906
2020-04-18T22:18:13.5694457Z .................................................................................................... 5200/9906
2020-04-18T22:18:18.5365998Z .........i.........................................................................................i 5300/9906
2020-04-18T22:18:28.3356131Z ...................................................................................................i 5400/9906
2020-04-18T22:18:33.0812409Z i.ii........i...i................................................................................... 5500/9906
2020-04-18T22:18:40.6605717Z ..............................................i..................................................... 5700/9906
2020-04-18T22:18:49.7959265Z ..............................................................................ii.................... 5800/9906
2020-04-18T22:18:56.9950596Z .................i.................................................................................. 5900/9906
2020-04-18T22:19:02.4091904Z .................................................................................................... 6000/9906
2020-04-18T22:19:02.4091904Z .................................................................................................... 6000/9906
2020-04-18T22:19:13.0591024Z .................................................................................................... 6100/9906
2020-04-18T22:19:23.4511619Z ...........ii...i..ii...........i................................................................... 6200/9906
2020-04-18T22:19:39.0583902Z .................................................................................................... 6400/9906
2020-04-18T22:19:45.4705076Z .................................................................................................... 6500/9906
2020-04-18T22:19:45.4705076Z .................................................................................................... 6500/9906
2020-04-18T22:19:58.1705461Z .........................................i..ii...................................................... 6600/9906
2020-04-18T22:20:19.6804007Z .................................................................................................... 6800/9906
2020-04-18T22:20:21.8050411Z ..........................................i......................................................... 6900/9906
2020-04-18T22:20:23.8702335Z .................................................................................................... 7000/9906
2020-04-18T22:20:26.0156433Z ..................................................................................i................. 7100/9906
---
2020-04-18T22:21:57.8559823Z .................................................................................................... 7800/9906
2020-04-18T22:22:02.1910531Z .................................................................................................... 7900/9906
2020-04-18T22:22:08.4581406Z .................................................................................................... 8000/9906
2020-04-18T22:22:14.1862679Z ................................................i................................................... 8100/9906
2020-04-18T22:22:23.7673071Z .................................................................................................iii 8200/9906
2020-04-18T22:22:29.0771174Z iii.iiiii.i......................................................................................... 8300/9906
2020-04-18T22:22:42.2157885Z .................................................................................................... 8500/9906
2020-04-18T22:22:50.4037720Z .................................................................................................... 8600/9906
2020-04-18T22:23:04.4846397Z .................................................................................................... 8700/9906
2020-04-18T22:23:11.2682124Z .................................................................................................... 8800/9906
---
2020-04-18T22:25:27.5176761Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T22:25:27.5321031Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T22:25:27.7565894Z 
2020-04-18T22:25:27.7566268Z running 186 tests
2020-04-18T22:25:30.8102536Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-18T22:25:33.5879482Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-18T22:25:33.5885209Z 
2020-04-18T22:25:33.5889742Z  finished in 6.057
2020-04-18T22:25:33.5895832Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-18T22:25:33.6066405Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T22:25:35.8781606Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-18T22:25:35.8973067Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T22:25:36.0378896Z 
2020-04-18T22:25:36.0380699Z running 9 tests
2020-04-18T22:25:36.0382528Z iiiiiiiii
2020-04-18T22:25:36.0383920Z 
2020-04-18T22:25:36.0384286Z  finished in 0.141
2020-04-18T22:25:36.0385014Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-18T22:25:36.0574077Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T22:25:57.0902439Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-18T22:25:57.1106704Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T22:25:57.2802170Z 
2020-04-18T22:25:57.2802981Z running 115 tests
2020-04-18T22:26:11.1679474Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-18T22:26:12.7475140Z ...iiii.....ii.
2020-04-18T22:26:12.7477524Z 
2020-04-18T22:26:12.7481085Z  finished in 15.637
2020-04-18T22:26:12.7485890Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-18T22:26:12.7489455Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T22:39:47.2023288Z 
2020-04-18T22:39:47.2025221Z    Doc-tests core
2020-04-18T22:39:52.0850529Z 
2020-04-18T22:39:52.0851342Z running 2496 tests
2020-04-18T22:40:00.8524903Z ......iiiii......................................................................................... 100/2496
2020-04-18T22:40:09.8052655Z .....................................................................................ii............. 200/2496
2020-04-18T22:40:30.5534207Z ....................i............................................................................... 400/2496
2020-04-18T22:40:30.5534207Z ....................i............................................................................... 400/2496
2020-04-18T22:40:40.4732541Z ..........................................................................i..i..................iiii 500/2496
2020-04-18T22:40:56.8269896Z .................................................................................................... 700/2496
2020-04-18T22:41:05.3083256Z .................................................................................................... 800/2496
2020-04-18T22:41:13.7887887Z .................................................................................................... 900/2496
2020-04-18T22:41:22.5695820Z .................................................................................................... 1000/2496
---
2020-04-18T22:44:40.5373952Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-18T22:44:40.5379603Z . 300/764
2020-04-18T22:44:40.6362642Z .................................................................................................... 400/764
2020-04-18T22:44:42.7096345Z .................................................................................................... 500/764
2020-04-18T22:44:42.7578075Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-18T22:44:42.7595844Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-18T22:44:42.7604562Z .<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-18T22:44:42.7625785Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-18T22:44:43.0706415Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-18T22:44:43.0727052Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-04-18T22:44:43.0745513Z ..thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-18T22:44:43.0765015Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-18T22:44:45.1154175Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-18T22:44:45.1158394Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-18T22:44:45.1162655Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-18T22:44:45.1163906Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-18T22:44:54.1989288Z 
2020-04-18T22:44:54.1990269Z running 1020 tests
2020-04-18T22:45:11.2727911Z i................................................................................................... 100/1020
2020-04-18T22:45:21.2210713Z .................................................................................................... 200/1020
2020-04-18T22:45:28.7107234Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-18T22:45:33.6067185Z .................................................................................................... 400/1020
2020-04-18T22:45:40.2463450Z ....................................................i....i......................................ii.. 500/1020
2020-04-18T22:45:52.8699983Z .................................................................................................... 700/1020
2020-04-18T22:45:52.8699983Z .................................................................................................... 700/1020
2020-04-18T22:45:59.8303624Z ..............................................iiii.................................................. 800/1020
2020-04-18T22:46:13.2735151Z .................................................................................................... 900/1020
2020-04-18T22:46:19.3273204Z ....................................................................iiii............................ 1000/1020
2020-04-18T22:46:20.5389563Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-18T22:46:20.5389823Z 
2020-04-18T22:46:20.5501812Z  finished in 162.223
2020-04-18T22:46:20.5513084Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-18T22:49:29.6041179Z 
2020-04-18T22:49:29.6041762Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-18T22:49:29.6042126Z 
2020-04-18T22:49:29.6096689Z  finished in 0.928
2020-04-18T22:49:29.6103976Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-18T22:49:29.6118323Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T22:49:29.8117864Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T22:49:30.8524930Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-317fb1a974c1061d
2020-04-18T22:49:30.8555147Z 
2020-04-18T22:49:30.8555369Z running 0 tests
2020-04-18T22:49:30.8555709Z 
---
2020-04-18T23:04:34.7480876Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-18T23:04:34.7481824Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-18T23:04:34.7482810Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-18T23:04:34.7483767Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-18T23:04:34.7484736Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-18T23:04:34.7486884Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc"Documenting stage2 rustdoc (x86_64-unknown-linux-gnu)
2020-04-18T23:04:34.7487720Z  skipping - compiler/librustdoc docs disabled
2020-04-18T23:04:34.7488385Z Documenting error index (x86_64-unknown-linux-gnu)
2020-04-18T23:04:34.7489006Z  -- not in ["src/tools/tidy"]
---
2020-04-18T23:05:37.9630779Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-18T23:05:37.9984750Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T23:05:38.2351281Z 
2020-04-18T23:05:38.2351853Z running 211 tests
2020-04-18T23:06:09.5377307Z ......................i...ii.......................................................................i 100/211
2020-04-18T23:06:48.7483669Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-18T23:06:53.8564495Z .......ii..
2020-04-18T23:06:53.8567317Z 
2020-04-18T23:06:53.8568043Z  finished in 75.858
2020-04-18T23:06:53.8575534Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-18T23:06:53.8580794Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-18T23:07:07.8028307Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-18T23:07:07.8222438Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-18T23:07:07.9761889Z  finished in 4.326
2020-04-18T23:07:07.9781591Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-18T23:07:09.1342187Z Checking "alias-1" ... OK
2020-04-18T23:07:09.2662979Z Checking "alias-2" ... OK
2020-04-18T23:07:09.3397989Z Checking "alias-3" ... OK
2020-04-18T23:07:09.4092364Z Checking "alias" ... OK
2020-04-18T23:07:09.6177063Z Checking "basic" ... OK
2020-04-18T23:07:09.7208902Z Checking "deduplication" ... OK
2020-04-18T23:07:09.7948553Z Checking "enum-option" ... OK
2020-04-18T23:07:09.9199872Z Checking "filter-crate" ... OK
2020-04-18T23:07:10.0316724Z Checking "fn-forget" ... OK
2020-04-18T23:07:10.1735022Z Checking "from_u" ... OK
2020-04-18T23:07:10.2586155Z Checking "keyword" ... OK
2020-04-18T23:07:10.3273251Z Checking "macro-check" ... OK
2020-04-18T23:07:10.3737521Z Checking "macro-print" ... OK
2020-04-18T23:07:10.4881584Z Checking "multi-query" ... OK
2020-04-18T23:07:10.5429782Z Checking "never" ... OK
2020-04-18T23:07:10.5670805Z Checking "quoted" ... OK
2020-04-18T23:07:10.6055187Z Checking "return-specific-literal" ... OK
2020-04-18T23:07:10.7383939Z Checking "return-specific" ... OK
2020-04-18T23:07:10.8689867Z Checking "should-fail" ... OK
2020-04-18T23:07:11.0657729Z Checking "string-from_ut" ... OK
2020-04-18T23:07:11.1813102Z Checking "struct-vec" ... OK
2020-04-18T23:07:11.3644439Z Checking "vec-new" ... OK
2020-04-18T23:07:11.6336307Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T23:07:12.0113784Z 
2020-04-18T23:07:12.0114126Z running 6 tests
2020-04-18T23:07:18.9111147Z ......
---
2020-04-18T23:07:34.5743136Z 
2020-04-18T23:07:34.5743646Z ---- [ui] rustdoc-ui/coverage/basic.rs stdout ----
2020-04-18T23:07:34.5743872Z diff of stdout:
2020-04-18T23:07:34.5744001Z 
2020-04-18T23:07:34.5744475Z 1 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5744934Z 2 | File                                | Documented |      Total | Percentage |
2020-04-18T23:07:34.5745617Z 3 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5746153Z - | ...est/rustdoc-ui/coverage/basic.rs |          7 |         14 |      50.0% |
2020-04-18T23:07:34.5746667Z + | ...est/rustdoc-ui/coverage/basic.rs |          8 |         16 |      50.0% |
2020-04-18T23:07:34.5747426Z 5 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5748005Z - | Total                               |          7 |         14 |      50.0% |
2020-04-18T23:07:34.5748358Z + | Total                               |          8 |         16 |      50.0% |
2020-04-18T23:07:34.5748930Z 7 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5749262Z 
2020-04-18T23:07:34.5749352Z 
2020-04-18T23:07:34.5749565Z The actual stdout differed from the expected stdout.
2020-04-18T23:07:34.5750151Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/basic.stdout
2020-04-18T23:07:34.5750151Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/basic.stdout
2020-04-18T23:07:34.5751018Z To update references, rerun the tests and pass the `--bless` flag
2020-04-18T23:07:34.5751493Z To only update this specific test, also pass `--test-args coverage/basic.rs`
2020-04-18T23:07:34.5751830Z error: 1 errors occurred comparing output.
2020-04-18T23:07:34.5752019Z status: exit code: 0
2020-04-18T23:07:34.5752019Z status: exit code: 0
2020-04-18T23:07:34.5753732Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/auxiliary"
2020-04-18T23:07:34.5755292Z ------------------------------------------
2020-04-18T23:07:34.5755835Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5755835Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5756162Z | File                                | Documented |      Total | Percentage |
2020-04-18T23:07:34.5756637Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5757112Z | ...est/rustdoc-ui/coverage/basic.rs |          8 |         16 |      50.0% |
2020-04-18T23:07:34.5757725Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5758048Z | Total                               |          8 |         16 |      50.0% |
2020-04-18T23:07:34.5758553Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5759059Z ------------------------------------------
2020-04-18T23:07:34.5759224Z stderr:
2020-04-18T23:07:34.5759547Z ------------------------------------------
2020-04-18T23:07:34.5759685Z 
2020-04-18T23:07:34.5759685Z 
2020-04-18T23:07:34.5759981Z ------------------------------------------
2020-04-18T23:07:34.5760116Z 
2020-04-18T23:07:34.5760195Z 
2020-04-18T23:07:34.5760543Z ---- [ui] rustdoc-ui/coverage/statics-consts.rs stdout ----
2020-04-18T23:07:34.5760746Z diff of stdout:
2020-04-18T23:07:34.5760851Z 
2020-04-18T23:07:34.5761246Z 1 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5761560Z 2 | File                                | Documented |      Total | Percentage |
2020-04-18T23:07:34.5762041Z 3 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5762549Z - | ...oc-ui/coverage/statics-consts.rs |          6 |          7 |      85.7% |
2020-04-18T23:07:34.5763146Z + | ...oc-ui/coverage/statics-consts.rs |          7 |          8 |      87.5% |
2020-04-18T23:07:34.5763649Z 5 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5764181Z - | Total                               |          6 |          7 |      85.7% |
2020-04-18T23:07:34.5764510Z + | Total                               |          7 |          8 |      87.5% |
2020-04-18T23:07:34.5765016Z 7 +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5765341Z 
2020-04-18T23:07:34.5765425Z 
2020-04-18T23:07:34.5765602Z The actual stdout differed from the expected stdout.
2020-04-18T23:07:34.5766318Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts/statics-consts.stdout
2020-04-18T23:07:34.5766318Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts/statics-consts.stdout
2020-04-18T23:07:34.5766842Z To update references, rerun the tests and pass the `--bless` flag
2020-04-18T23:07:34.5767335Z To only update this specific test, also pass `--test-args coverage/statics-consts.rs`
2020-04-18T23:07:34.5767685Z error: 1 errors occurred comparing output.
2020-04-18T23:07:34.5767873Z status: exit code: 0
2020-04-18T23:07:34.5767873Z status: exit code: 0
2020-04-18T23:07:34.5769427Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/statics-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts/auxiliary"
2020-04-18T23:07:34.5770799Z ------------------------------------------
2020-04-18T23:07:34.5771227Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5771227Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5771549Z | File                                | Documented |      Total | Percentage |
2020-04-18T23:07:34.5772023Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5772498Z | ...oc-ui/coverage/statics-consts.rs |          7 |          8 |      87.5% |
2020-04-18T23:07:34.5772989Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5773292Z | Total                               |          7 |          8 |      87.5% |
2020-04-18T23:07:34.5774181Z +-------------------------------------+------------+------------+------------+
2020-04-18T23:07:34.5774842Z ------------------------------------------
2020-04-18T23:07:34.5775032Z stderr:
2020-04-18T23:07:34.5775426Z ------------------------------------------
2020-04-18T23:07:34.5775574Z 
---
2020-04-18T23:07:34.5778321Z Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T23:07:34.5778633Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T23:07:34.5778830Z 
2020-04-18T23:07:34.5778913Z 
2020-04-18T23:07:34.5796491Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T23:07:34.5799726Z 
2020-04-18T23:07:34.5799834Z 
2020-04-18T23:07:34.5800543Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T23:07:34.5800963Z Build completed unsuccessfully in 1:45:26
2020-04-18T23:07:34.5800963Z Build completed unsuccessfully in 1:45:26
2020-04-18T23:07:34.5804038Z == clock drift check ==
2020-04-18T23:07:34.5842246Z   local time: Sat Apr 18 23:07:34 UTC 2020
2020-04-18T23:07:34.9106760Z   network time: Sat, 18 Apr 2020 23:07:34 GMT
2020-04-18T23:07:35.8464641Z 
2020-04-18T23:07:35.8464641Z 
2020-04-18T23:07:35.8559399Z ##[error]Bash exited with code '1'.
2020-04-18T23:07:35.8576993Z ##[section]Finishing: Run build
2020-04-18T23:07:35.8634152Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T23:07:35.8640231Z Task         : Get sources
2020-04-18T23:07:35.8640587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T23:07:35.8640937Z Version      : 1.0.0
2020-04-18T23:07:35.8641173Z Author       : Microsoft
2020-04-18T23:07:35.8641173Z Author       : Microsoft
2020-04-18T23:07:35.8641550Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T23:07:35.8641988Z ==============================================================================
2020-04-18T23:07:36.2166803Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T23:07:36.2210766Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T23:07:36.2310645Z Cleaning up task key
2020-04-18T23:07:36.2312070Z Start cleaning up orphan processes.
2020-04-18T23:07:36.2540819Z Terminate orphan process: pid (3592) (python)
2020-04-18T23:07:36.2842271Z ##[section]Finishing: Finalize Job
