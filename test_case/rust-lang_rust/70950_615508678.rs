plain
2020-04-17T22:11:28.5884568Z ========================== Starting Command Output ===========================
2020-04-17T22:11:28.5887387Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/770d3d2c-799b-4c7e-b135-911d4d87c0cf.sh
2020-04-17T22:11:28.5887682Z 
2020-04-17T22:11:28.5892003Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T22:11:28.5912334Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-17T22:11:28.5915896Z Task         : Get sources
2020-04-17T22:11:28.5916246Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T22:11:28.5916584Z Version      : 1.0.0
2020-04-17T22:11:28.5916804Z Author       : Microsoft
---
2020-04-17T22:11:29.5993870Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T22:11:29.5998080Z ##[command]git config gc.auto 0
2020-04-17T22:11:29.6000650Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T22:11:29.6002842Z ##[command]git config --get-all http.proxy
2020-04-17T22:11:29.6007015Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70950/merge:refs/remotes/pull/70950/merge
---
2020-04-17T22:13:27.3100028Z  ---> f58a2bb1e753
2020-04-17T22:13:27.3100701Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-17T22:13:27.3101252Z  ---> Using cache
2020-04-17T22:13:27.3101642Z  ---> d079cc6b6db8
2020-04-17T22:13:27.3102361Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-17T22:13:27.3103160Z  ---> 4183ca46ee56
2020-04-17T22:13:27.3103321Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-17T22:13:27.3103591Z  ---> Using cache
2020-04-17T22:13:27.3103852Z  ---> 69e7f8a2a2fb
---
2020-04-17T22:13:27.3366953Z Looks like docker image is the same as before, not uploading
2020-04-17T22:13:34.9089930Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T22:13:34.9295167Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T22:13:34.9319268Z == clock drift check ==
2020-04-17T22:13:34.9330992Z   local time: Fri Apr 17 22:13:34 UTC 2020
2020-04-17T22:13:35.1188854Z   network time: Fri, 17 Apr 2020 22:13:35 GMT
2020-04-17T22:13:35.1214980Z Starting sccache server...
2020-04-17T22:13:35.1912153Z configure: processing command line
2020-04-17T22:13:35.1912388Z configure: 
2020-04-17T22:13:35.1913214Z configure: rust.dist-src        := False
---
2020-04-17T22:17:34.1056419Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T22:17:35.1269891Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T22:17:36.2511822Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T22:17:36.5783288Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T22:17:43.3132850Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T22:17:44.5141630Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T22:17:47.6776096Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T22:17:50.6812908Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T22:17:57.9341653Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T22:34:15.7566228Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T22:34:17.0764829Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T22:34:18.5529474Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T22:34:19.6175430Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T22:34:27.5487805Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T22:34:29.4466083Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T22:34:33.3006893Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T22:34:37.2852193Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T22:34:45.6955584Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T22:53:37.3592232Z .................................................................................................... 1700/9906
2020-04-17T22:53:40.3986659Z .................................................................................................... 1800/9906
2020-04-17T22:53:47.0333965Z .................................................................................................i.. 1900/9906
2020-04-17T22:53:52.9546089Z .................................................................................................... 2000/9906
2020-04-17T22:53:57.7532425Z .......................................................................................iiiii........ 2100/9906
2020-04-17T22:54:13.6694538Z .................................................................................................... 2300/9906
2020-04-17T22:54:15.2522536Z .................................................................................................... 2400/9906
2020-04-17T22:54:16.9420372Z .................................................................................................... 2500/9906
2020-04-17T22:54:21.4721685Z .................................................................................................... 2600/9906
---
2020-04-17T22:56:27.4359131Z .................................................................................................... 4900/9906
2020-04-17T22:56:31.1057156Z ...............................................................i...............i.................... 5000/9906
2020-04-17T22:56:36.6057964Z .................................................................................................... 5100/9906
2020-04-17T22:56:42.2819916Z .................................................................................................... 5200/9906
2020-04-17T22:56:46.7898215Z .........i.........................................................................................i 5300/9906
2020-04-17T22:56:53.6155696Z ...................................................................................................i 5400/9906
2020-04-17T22:56:57.2316385Z i.ii........i...i................................................................................... 5500/9906
2020-04-17T22:57:03.1468347Z .............................................i...................................................... 5700/9906
2020-04-17T22:57:10.1220313Z .............................................................................ii..................... 5800/9906
2020-04-17T22:57:15.3039158Z ................i................................................................................... 5900/9906
2020-04-17T22:57:19.4378223Z .................................................................................................... 6000/9906
2020-04-17T22:57:19.4378223Z .................................................................................................... 6000/9906
2020-04-17T22:57:27.5019986Z .................................................................................................... 6100/9906
2020-04-17T22:57:35.5332898Z ..........ii...i..ii...........i.................................................................... 6200/9906
2020-04-17T22:57:46.9801761Z .................................................................................................... 6400/9906
2020-04-17T22:57:49.6177817Z .................................................................................................... 6500/9906
2020-04-17T22:57:49.6177817Z .................................................................................................... 6500/9906
2020-04-17T22:57:58.2363532Z ..........................................i..ii..................................................... 6600/9906
2020-04-17T22:58:14.5750075Z .................................................................................................... 6800/9906
2020-04-17T22:58:16.1728330Z ...........................................i........................................................ 6900/9906
2020-04-17T22:58:17.7182133Z .................................................................................................... 7000/9906
2020-04-17T22:58:19.2442056Z ...................................................................................i................ 7100/9906
---
2020-04-17T22:59:31.1325897Z .................................................................................................... 7800/9906
2020-04-17T22:59:34.5641158Z .................................................................................................... 7900/9906
2020-04-17T22:59:39.4336425Z .................................................................................................... 8000/9906
2020-04-17T22:59:43.7441935Z .................................................i.................................................. 8100/9906
2020-04-17T22:59:51.5596432Z ..................................................................................................ii 8200/9906
2020-04-17T22:59:55.6147948Z iiii.iiiii.i........................................................................................ 8300/9906
2020-04-17T23:00:05.6165361Z .................................................................................................... 8500/9906
2020-04-17T23:00:11.7893583Z .................................................................................................... 8600/9906
2020-04-17T23:00:22.3321515Z .................................................................................................... 8700/9906
2020-04-17T23:00:27.2714350Z .................................................................................................... 8800/9906
---
2020-04-17T23:02:10.4506333Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-17T23:02:10.4657211Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:02:10.6230770Z 
2020-04-17T23:02:10.6231130Z running 185 tests
2020-04-17T23:02:12.6304807Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-17T23:02:14.5470466Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-17T23:02:14.5474536Z 
2020-04-17T23:02:14.5474950Z  finished in 4.081
2020-04-17T23:02:14.5480520Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-17T23:02:14.5627118Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T23:02:16.1222698Z test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-17T23:02:16.1362168Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:02:16.2495064Z 
2020-04-17T23:02:16.2496432Z running 9 tests
2020-04-17T23:02:16.2497242Z iiiiiiiii
2020-04-17T23:02:16.2498424Z 
2020-04-17T23:02:16.2498567Z  finished in 0.112
2020-04-17T23:02:16.2499227Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-17T23:02:16.2651136Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T23:02:31.5138029Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-17T23:02:31.5338972Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:02:31.6836519Z 
2020-04-17T23:02:31.6836848Z running 115 tests
2020-04-17T23:02:41.6160255Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-17T23:02:42.8843348Z ...iiii.....ii.
2020-04-17T23:02:42.8844526Z 
2020-04-17T23:02:42.8850372Z  finished in 11.351
2020-04-17T23:02:42.8864877Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T23:02:42.8868201Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T23:11:43.6226395Z 
2020-04-17T23:11:43.6227585Z    Doc-tests core
2020-04-17T23:11:47.0463628Z 
2020-04-17T23:11:47.0464023Z running 2496 tests
2020-04-17T23:11:53.8504758Z ......iiiii......................................................................................... 100/2496
2020-04-17T23:12:00.3512415Z .....................................................................................ii............. 200/2496
2020-04-17T23:12:15.3801512Z ....................i............................................................................... 400/2496
2020-04-17T23:12:15.3801512Z ....................i............................................................................... 400/2496
2020-04-17T23:12:22.7993605Z ..........................................................................i..i..................iiii 500/2496
2020-04-17T23:12:34.6407031Z .................................................................................................... 700/2496
2020-04-17T23:12:40.8190260Z .................................................................................................... 800/2496
2020-04-17T23:12:46.9868791Z .................................................................................................... 900/2496
2020-04-17T23:12:53.1753377Z .................................................................................................... 1000/2496
---
2020-04-17T23:15:32.1294530Z 
2020-04-17T23:15:32.1294825Z running 1020 tests
2020-04-17T23:15:44.9549069Z i................................................................................................... 100/1020
2020-04-17T23:15:52.4329976Z .................................................................................................... 200/1020
2020-04-17T23:15:58.0226860Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-17T23:16:01.5178740Z .................................................................................................... 400/1020
2020-04-17T23:16:06.4071257Z ....................................................i....i......................................ii.. 500/1020
2020-04-17T23:16:15.6248919Z .................................................................................................... 700/1020
2020-04-17T23:16:15.6248919Z .................................................................................................... 700/1020
2020-04-17T23:16:20.7822638Z ..............................................iiii.................................................. 800/1020
2020-04-17T23:16:31.2143597Z .................................................................................................... 900/1020
2020-04-17T23:16:35.6543834Z ....................................................................iiii............................ 1000/1020
2020-04-17T23:16:36.6639417Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-17T23:16:36.6639614Z 
2020-04-17T23:16:36.6741021Z  finished in 123.212
2020-04-17T23:16:36.6745413Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-17T23:18:53.0726812Z 
2020-04-17T23:18:53.0726996Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-17T23:18:53.0727186Z 
2020-04-17T23:18:53.0786901Z  finished in 0.718
2020-04-17T23:18:53.0792208Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-17T23:18:53.0806163Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:18:53.2223934Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T23:18:53.9825190Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-734b126b44c5c47c
2020-04-17T23:18:53.9855041Z 
2020-04-17T23:18:53.9855261Z running 0 tests
2020-04-17T23:18:53.9855371Z 
---
2020-04-17T23:29:18.5636452Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5637456Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5638496Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5639516Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5640641Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5642746Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5643793Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5644793Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T23:29:18.5645824Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-17T23:30:05.0902623Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T23:30:05.1191101Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:30:05.2940523Z 
2020-04-17T23:30:05.2940921Z running 211 tests
2020-04-17T23:30:29.3119328Z ......................i...ii.......................................................................i 100/211
2020-04-17T23:30:55.7123462Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-17T23:31:02.6482762Z .......ii..
2020-04-17T23:31:02.6483605Z 
2020-04-17T23:31:02.6490419Z  finished in 57.530
2020-04-17T23:31:02.6495663Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-17T23:31:02.6499716Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-17T23:31:13.2830835Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-17T23:31:13.2964895Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-17T23:31:13.4114448Z  finished in 3.221
2020-04-17T23:31:13.4123066Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-17T23:31:14.1520263Z Checking "alias-1" ... OK
2020-04-17T23:31:14.2024027Z Checking "alias-2" ... OK
2020-04-17T23:31:14.2460269Z Checking "alias-3" ... OK
2020-04-17T23:31:14.2977051Z Checking "alias" ... OK
2020-04-17T23:31:14.3542835Z Checking "basic" ... OK
2020-04-17T23:31:14.4071389Z Checking "deduplication" ... OK
2020-04-17T23:31:14.4423108Z Checking "enum-option" ... OK
2020-04-17T23:31:14.4962943Z Checking "filter-crate" ... OK
2020-04-17T23:31:14.5428379Z Checking "fn-forget" ... OK
2020-04-17T23:31:14.6117098Z Checking "from_u" ... OK
2020-04-17T23:31:14.6832288Z Checking "keyword" ... OK
2020-04-17T23:31:14.7167726Z Checking "macro-check" ... OK
2020-04-17T23:31:14.7405935Z Checking "macro-print" ... OK
2020-04-17T23:31:14.7937465Z Checking "multi-query" ... OK
2020-04-17T23:31:14.8156985Z Checking "never" ... OK
2020-04-17T23:31:14.8272466Z Checking "quoted" ... OK
2020-04-17T23:31:14.8440390Z Checking "return-specific-literal" ... OK
2020-04-17T23:31:14.8944308Z Checking "return-specific" ... OK
2020-04-17T23:31:14.9199879Z Checking "should-fail" ... OK
2020-04-17T23:31:14.9682371Z Checking "string-from_ut" ... OK
2020-04-17T23:31:15.0120254Z Checking "struct-vec" ... OK
2020-04-17T23:31:15.0986025Z Checking "vec-new" ... OK
2020-04-17T23:31:15.1202266Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:31:15.2480946Z 
2020-04-17T23:31:15.2481337Z running 6 tests
2020-04-17T23:31:18.9135520Z ......
---
2020-04-17T23:32:06.6593535Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-17T23:32:06.6755636Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:32:06.8010229Z 
2020-04-17T23:32:06.8010569Z running 13 tests
2020-04-17T23:32:07.1152862Z .iiiiiii.iii.
2020-04-17T23:32:07.1153724Z 
2020-04-17T23:32:07.1158940Z  finished in 0.440
2020-04-17T23:32:07.1207562Z Build completed successfully in 1:17:12
2020-04-17T23:32:07.1207562Z Build completed successfully in 1:17:12
2020-04-17T23:32:07.1220845Z + python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-17T23:32:08.1005355Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T23:32:08.2715703Z     Finished release [optimized] target(s) in 0.16s
2020-04-17T23:32:08.2776307Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-17T23:32:08.2865643Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T23:33:09.1295981Z 
2020-04-17T23:33:09.1296745Z ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
2020-04-17T23:33:09.1297118Z 23 |
2020-04-17T23:33:09.1297549Z 24 fn main() -> () {
2020-04-17T23:33:09.1298220Z 25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:15:11: 15:11
2020-04-17T23:33:09.1299117Z -     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x0000000000000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:16:9: 16:14
2020-04-17T23:33:09.1300018Z +     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:16:9: 16:14
2020-04-17T23:33:09.1300838Z 27     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:17:16: 17:17
2020-04-17T23:33:09.1301847Z 28     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:17:14: 17:18
2020-04-17T23:33:09.1302661Z 29     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:17:14: 17:18
2020-04-17T23:33:09.1303284Z 45 
2020-04-17T23:33:09.1303554Z 46     bb0: {
2020-04-17T23:33:09.1303554Z 46     bb0: {
2020-04-17T23:33:09.1304199Z 47         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:16:9: 16:14
2020-04-17T23:33:09.1305687Z -         _1 = [const Const(Value(Scalar(0x0000000000000001)): usize), const Const(Value(Scalar(0x0000000000000002)): usize), const Const(Value(Scalar(0x0000000000000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:16:17: 16:26
2020-04-17T23:33:09.1307855Z +         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:16:17: 16:26
2020-04-17T23:33:09.1309714Z 49                                          // ty::Const
2020-04-17T23:33:09.1310912Z 50                                          // + ty: usize
2020-04-17T23:33:09.1311671Z -                                          // + val: Value(Scalar(0x0000000000000001))
2020-04-17T23:33:09.1312191Z +                                          // + val: Value(Scalar(0x00000001))
2020-04-17T23:33:09.1312612Z 52                                          // mir::Constant
2020-04-17T23:33:09.1313344Z 53                                          // + span: $DIR/region-subtyping-basic.rs:16:18: 16:19
2020-04-17T23:33:09.1314202Z -                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
2020-04-17T23:33:09.1314834Z +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000001)) }
2020-04-17T23:33:09.1315338Z 55                                          // ty::Const
2020-04-17T23:33:09.1315716Z 56                                          // + ty: usize
2020-04-17T23:33:09.1316373Z -                                          // + val: Value(Scalar(0x0000000000000002))
2020-04-17T23:33:09.1316863Z +                                          // + val: Value(Scalar(0x00000002))
2020-04-17T23:33:09.1317285Z 58                                          // mir::Constant
2020-04-17T23:33:09.1318108Z 59                                          // + span: $DIR/region-subtyping-basic.rs:16:21: 16:22
2020-04-17T23:33:09.1318846Z -                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000002)) }
2020-04-17T23:33:09.1319412Z +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000002)) }
2020-04-17T23:33:09.1319842Z 61                                          // ty::Const
2020-04-17T23:33:09.1320254Z 62                                          // + ty: usize
2020-04-17T23:33:09.1320743Z -                                          // + val: Value(Scalar(0x0000000000000003))
2020-04-17T23:33:09.1321116Z +                                          // + val: Value(Scalar(0x00000003))
2020-04-17T23:33:09.1321451Z 64                                          // mir::Constant
2020-04-17T23:33:09.1321992Z 65                                          // + span: $DIR/region-subtyping-basic.rs:16:24: 16:25
2020-04-17T23:33:09.1322622Z -                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000003)) }
2020-04-17T23:33:09.1323108Z +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000003)) }
2020-04-17T23:33:09.1323736Z 67         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:16:9: 16:14
2020-04-17T23:33:09.1324390Z 68         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:17:9: 17:10
2020-04-17T23:33:09.1325020Z 69         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:17:16: 17:17
2020-04-17T23:33:09.1325438Z 
2020-04-17T23:33:09.1326075Z -         _3 = const Const(Value(Scalar(0x0000000000000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:17:16: 17:17
2020-04-17T23:33:09.1326834Z +         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:17:16: 17:17
2020-04-17T23:33:09.1327258Z 71                                          // ty::Const
2020-04-17T23:33:09.1327581Z 72                                          // + ty: usize
2020-04-17T23:33:09.1328061Z -                                          // + val: Value(Scalar(0x0000000000000000))
2020-04-17T23:33:09.1328438Z +                                          // + val: Value(Scalar(0x00000000))
2020-04-17T23:33:09.1328754Z 74                                          // mir::Constant
2020-04-17T23:33:09.1329423Z 75                                          // + span: $DIR/region-subtyping-basic.rs:17:16: 17:17
2020-04-17T23:33:09.1330194Z -                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000000)) }
2020-04-17T23:33:09.1330663Z +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000000)) }
2020-04-17T23:33:09.1331310Z 77         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:17:14: 17:18
2020-04-17T23:33:09.1331952Z 78         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:17:14: 17:18
2020-04-17T23:33:09.1332968Z 79         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> [success: bb2, unwind: bb1]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:17:14: 17:18
2020-04-17T23:33:09.1333593Z 107 
2020-04-17T23:33:09.1333778Z 108     bb4: {
2020-04-17T23:33:09.1333778Z 108     bb4: {
2020-04-17T23:33:09.1334305Z 109         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:22:9: 22:18
2020-04-17T23:33:09.1335232Z -         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x0000000000000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:9: 22:18
2020-04-17T23:33:09.1336279Z +         _10 = const Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb7, unwind: bb1]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:9: 22:18
2020-04-17T23:33:09.1336822Z 111                                          // ty::Const
2020-04-17T23:33:09.1337320Z 112                                          // + ty: fn(usize) -> bool {use_x}
2020-04-17T23:33:09.1337675Z 113                                          // + val: Value(Scalar(<ZST>))
2020-04-17T23:33:09.1337919Z 
2020-04-17T23:33:09.1338457Z 116                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
2020-04-17T23:33:09.1338864Z 117                                          // ty::Const
2020-04-17T23:33:09.1339145Z 118                                          // + ty: usize
2020-04-17T23:33:09.1339643Z -                                          // + val: Value(Scalar(0x0000000000000016))
2020-04-17T23:33:09.1340013Z +                                          // + val: Value(Scalar(0x00000016))
2020-04-17T23:33:09.1340611Z 120                                          // mir::Constant
2020-04-17T23:33:09.1341175Z 121                                          // + span: $DIR/region-subtyping-basic.rs:22:15: 22:17
2020-04-17T23:33:09.1342187Z -                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000016)) }
2020-04-17T23:33:09.1342583Z +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x00000016)) }
2020-04-17T23:33:09.1342894Z 124 
2020-04-17T23:33:09.1342990Z 125     bb5: {
2020-04-17T23:33:09.1343168Z 
2020-04-17T23:33:09.1343168Z 
2020-04-17T23:33:09.1343800Z thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region-subtyping-basic/rustc.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-17T23:33:09.1344382Z 
2020-04-17T23:33:09.1344447Z 
2020-04-17T23:33:09.1344536Z failures:
2020-04-17T23:33:09.1344828Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-17T23:33:09.1344828Z     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
2020-04-17T23:33:09.1344945Z 
2020-04-17T23:33:09.1345269Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-17T23:33:09.1345431Z 
2020-04-17T23:33:09.1345509Z 
2020-04-17T23:33:09.1345571Z 
2020-04-17T23:33:09.1348028Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--pass" "build" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-17T23:33:09.1350059Z 
2020-04-17T23:33:09.1350133Z 
2020-04-17T23:33:09.1350133Z 
2020-04-17T23:33:09.1350603Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
2020-04-17T23:33:09.1420651Z == clock drift check ==
2020-04-17T23:33:09.1420651Z == clock drift check ==
2020-04-17T23:33:09.8350715Z   local time: Fri Apr 17 23:33:09 UTC 2020
2020-04-17T23:33:09.8353304Z   network time: Fri, 17 Apr 2020 23:33:09 GMT
2020-04-17T23:33:11.7859304Z 
2020-04-17T23:33:11.7859304Z 
2020-04-17T23:33:11.7929500Z ##[error]Bash exited with code '1'.
2020-04-17T23:33:11.7939781Z ##[section]Finishing: Run build
2020-04-17T23:33:11.7998373Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-17T23:33:11.8002589Z Task         : Get sources
2020-04-17T23:33:11.8002839Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T23:33:11.8003068Z Version      : 1.0.0
2020-04-17T23:33:11.8003244Z Author       : Microsoft
2020-04-17T23:33:11.8003244Z Author       : Microsoft
2020-04-17T23:33:11.8003501Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T23:33:11.8003791Z ==============================================================================
2020-04-17T23:33:12.0669611Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T23:33:12.0713345Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70950/merge to s
2020-04-17T23:33:12.0776582Z Cleaning up task key
2020-04-17T23:33:12.0777468Z Start cleaning up orphan processes.
2020-04-17T23:33:12.0916688Z Terminate orphan process: pid (4096) (python)
2020-04-17T23:33:12.1163194Z ##[section]Finishing: Finalize Job
