plain
2020-04-25T19:19:50.0981065Z ========================== Starting Command Output ===========================
2020-04-25T19:19:50.0986586Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1a8be272-941e-4b3e-964d-32bdfab3c3a1.sh
2020-04-25T19:19:50.0987074Z 
2020-04-25T19:19:50.0992451Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-25T19:19:50.1012103Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T19:19:50.1016228Z Task         : Get sources
2020-04-25T19:19:50.1016526Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T19:19:50.1016840Z Version      : 1.0.0
2020-04-25T19:19:50.1017035Z Author       : Microsoft
---
2020-04-25T19:19:51.1098692Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-25T19:19:51.1104876Z ##[command]git config gc.auto 0
2020-04-25T19:19:51.1108703Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-25T19:19:51.1113482Z ##[command]git config --get-all http.proxy
2020-04-25T19:19:51.1128537Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71555/merge:refs/remotes/pull/71555/merge
---
2020-04-25T19:21:44.0533005Z  ---> cb2676f08729
2020-04-25T19:21:44.0533835Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-25T19:21:44.0534488Z  ---> Using cache
2020-04-25T19:21:44.0534838Z  ---> df25ce111862
2020-04-25T19:21:44.0535847Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-25T19:21:44.0536985Z  ---> 599b9ac96b27
2020-04-25T19:21:44.0537209Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-25T19:21:44.0537604Z  ---> Using cache
2020-04-25T19:21:44.0537955Z  ---> 091087e35a36
---
2020-04-25T19:21:44.1040122Z Looks like docker image is the same as before, not uploading
2020-04-25T19:21:51.0198492Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-25T19:21:51.0507574Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-25T19:21:51.0534422Z == clock drift check ==
2020-04-25T19:21:51.0544369Z   local time: Sat Apr 25 19:21:51 UTC 2020
2020-04-25T19:21:51.2186990Z   network time: Sat, 25 Apr 2020 19:21:51 GMT
2020-04-25T19:21:51.2212382Z Starting sccache server...
2020-04-25T19:21:51.2999153Z configure: processing command line
2020-04-25T19:21:51.2999992Z configure: 
2020-04-25T19:21:51.3001986Z configure: rust.dist-src        := False
---
2020-04-25T19:24:24.4178130Z    Compiling unicode-width v0.1.6
2020-04-25T19:24:24.5077908Z    Compiling getopts v0.2.21
2020-04-25T19:24:35.8514693Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-25T19:24:45.1886799Z     Finished release [optimized] target(s) in 1m 07s
2020-04-25T19:24:45.1893190Z {"reason":"build-finished","success":true}
2020-04-25T19:24:45.2136443Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T19:24:45.7813906Z    Compiling cfg-if v0.1.10
2020-04-25T19:24:45.7814493Z    Compiling libc v0.2.69
2020-04-25T19:24:45.8278397Z    Compiling semver-parser v0.7.0
---
2020-04-25T19:27:34.5944432Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T19:27:36.1988233Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T19:27:37.9120234Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-25T19:27:39.5056490Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T19:27:48.9850044Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T19:27:51.9741856Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-25T19:27:56.8235396Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T19:28:01.4387972Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-25T19:28:11.7767859Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-25T19:41:53.5112153Z    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-25T19:43:59.5824378Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-25T19:44:07.9176748Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-25T19:48:18.3288973Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-25T19:48:19.0215042Z {"reason":"build-finished","success":true}
2020-04-25T19:48:19.0711364Z Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-25T19:48:19.0746945Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-25T19:48:19.0772284Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T19:48:19.4064777Z    Compiling cc v1.0.50
---
2020-04-25T19:49:13.4403163Z    Compiling unicode-width v0.1.6
2020-04-25T19:49:13.5475068Z    Compiling getopts v0.2.21
2020-04-25T19:49:26.6319893Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-25T19:49:37.0148067Z     Finished release [optimized] target(s) in 1m 17s
2020-04-25T19:49:37.0152924Z {"reason":"build-finished","success":true}
2020-04-25T19:49:37.0295897Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T19:49:37.6606109Z    Compiling cfg-if v0.1.10
2020-04-25T19:49:37.6614063Z    Compiling libc v0.2.69
2020-04-25T19:49:37.7079114Z    Compiling semver-parser v0.7.0
---
2020-04-25T19:52:41.7347088Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T19:52:43.5538345Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T19:52:45.5306883Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-25T19:52:46.9870619Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T19:52:57.2388086Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T19:53:01.0441851Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-25T19:53:06.1689743Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T19:53:11.0915077Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-25T19:53:20.8242479Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-25T20:10:01.8744155Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-25T20:10:10.5743766Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-25T20:14:41.9433419Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-25T20:14:42.6063641Z     Finished release [optimized] target(s) in 25m 05s
2020-04-25T20:14:42.6069156Z {"reason":"build-finished","success":true}
2020-04-25T20:14:42.6623683Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-25T20:14:42.6636472Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T20:14:42.6637851Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-25T20:14:42.6647229Z Building test helpers
---
2020-04-25T20:15:27.7931429Z    Compiling serde_derive v1.0.81
2020-04-25T20:15:57.2367861Z    Compiling serde_json v1.0.40
2020-04-25T20:15:58.8182148Z    Compiling rustfix v0.5.0
2020-04-25T20:16:02.3070295Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-25T20:16:17.2203777Z {"reason":"build-finished","success":true}
2020-04-25T20:16:17.2544038Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T20:16:20.7611197Z 
2020-04-25T20:16:20.7612995Z running 9922 tests
2020-04-25T20:16:35.2405290Z .................................................................................................... 100/9922
---
2020-04-25T20:18:31.2610619Z .................................................................................................... 1700/9922
2020-04-25T20:18:35.7828782Z .................................................................................................... 1800/9922
2020-04-25T20:18:44.4982055Z .................................................................................................... 1900/9922
2020-04-25T20:18:53.0665923Z ......i............................................................................................. 2000/9922
2020-04-25T20:18:59.7189632Z ................................................................................................iiii 2100/9922
2020-04-25T20:19:14.3121415Z i................................................................................................... 2200/9922
2020-04-25T20:19:22.9666832Z .................................................................................................... 2400/9922
2020-04-25T20:19:25.3557365Z .................................................................................................... 2500/9922
2020-04-25T20:19:31.1888279Z .................................................................................................... 2600/9922
2020-04-25T20:19:50.3215488Z .................................................................................................... 2700/9922
---
2020-04-25T20:22:36.8181934Z .................................................................................................... 5100/9922
2020-04-25T20:22:44.1778968Z .................................................................................................... 5200/9922
2020-04-25T20:22:48.9018616Z ...................i................................................................................ 5300/9922
2020-04-25T20:22:58.9754384Z ..........i......................................................................................... 5400/9922
2020-04-25T20:23:04.6905635Z ..........ii.ii........i...i........................................................................ 5500/9922
2020-04-25T20:23:12.6657387Z .........................................................i.......................................... 5700/9922
2020-04-25T20:23:21.6771311Z ..........................................................................................ii........ 5800/9922
2020-04-25T20:23:28.5063084Z .............................i...................................................................... 5900/9922
2020-04-25T20:23:34.0503536Z .................................................................................................... 6000/9922
2020-04-25T20:23:34.0503536Z .................................................................................................... 6000/9922
2020-04-25T20:23:44.8330489Z .................................................................................................... 6100/9922
2020-04-25T20:23:55.3627472Z .......................ii....i.ii...........i....................................................... 6200/9922
2020-04-25T20:24:11.6781068Z .................................................................................................... 6400/9922
2020-04-25T20:24:18.6060422Z .................................................................................................... 6500/9922
2020-04-25T20:24:18.6060422Z .................................................................................................... 6500/9922
2020-04-25T20:24:33.6030175Z .....................................................i..ii.......................................... 6600/9922
2020-04-25T20:25:03.8575440Z .................................................................................................... 6800/9922
2020-04-25T20:25:06.9007907Z ......................................................i............................................. 6900/9922
2020-04-25T20:25:09.0280244Z .................................................................................................... 7000/9922
2020-04-25T20:25:11.1948202Z ................................................................................................i... 7100/9922
---
2020-04-25T20:26:53.2184586Z .................................................................................................... 7900/9922
2020-04-25T20:26:58.8503537Z .................................................................................................... 8000/9922
2020-04-25T20:27:05.4913102Z ...............................................................i.................................... 8100/9922
2020-04-25T20:27:15.8375821Z .................................................................................................... 8200/9922
2020-04-25T20:27:21.3114316Z ............iiiiii.iiiii.i.......................................................................... 8300/9922
2020-04-25T20:27:35.3364723Z .................................................................................................... 8500/9922
2020-04-25T20:27:41.5910532Z .................................................................................................... 8600/9922
2020-04-25T20:27:56.5722116Z .................................................................................................... 8700/9922
2020-04-25T20:28:03.5725402Z .................................................................................................... 8800/9922
---
2020-04-25T20:30:25.3070661Z s/tidy"]
2020-04-25T20:30:25.3267607Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T20:30:25.5702584Z 
2020-04-25T20:30:25.5703534Z running 186 tests
2020-04-25T20:30:28.8647387Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-25T20:30:31.6446924Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-25T20:30:31.6452554Z 
2020-04-25T20:30:31.6452767Z  finished in 6.318
2020-04-25T20:30:31.6454054Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-25T20:30:31.6652258Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T20:30:33.9607945Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-25T20:30:33.9808131Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T20:30:34.1401509Z 
2020-04-25T20:30:34.1401846Z running 9 tests
2020-04-25T20:30:34.1403234Z iiiiiiiii
2020-04-25T20:30:34.1404200Z 
2020-04-25T20:30:34.1406928Z  finished in 0.159
2020-04-25T20:30:34.1412744Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-25T20:30:34.1617969Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T20:30:56.4382934Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-25T20:30:56.4625412Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T20:30:56.6485351Z 
2020-04-25T20:30:56.6485682Z running 115 tests
2020-04-25T20:31:10.9398515Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-25T20:31:13.1817731Z ...iiii.....ii.
2020-04-25T20:31:13.1818863Z 
2020-04-25T20:31:13.1819000Z  finished in 16.339
2020-04-25T20:31:13.1819527Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-25T20:31:13.1820124Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T20:44:08.4295796Z 
2020-04-25T20:44:08.4296893Z    Doc-tests core
2020-04-25T20:44:13.9205624Z 
2020-04-25T20:44:13.9206468Z running 2499 tests
2020-04-25T20:44:23.1957113Z ......iiiii......................................................................................... 100/2499
2020-04-25T20:44:32.7701556Z ......................................................................................ii............ 200/2499
2020-04-25T20:44:55.6967939Z ......................i............................................................................. 400/2499
2020-04-25T20:45:06.7949904Z ............................................................................i..i..................ii 500/2499
2020-04-25T20:45:15.0433560Z ii.................................................................................................. 600/2499
2020-04-25T20:45:24.3767935Z .................................................................................................... 700/2499
---
2020-04-25T20:49:33.2098577Z 
2020-04-25T20:49:33.2098942Z running 1020 tests
2020-04-25T20:49:50.9008566Z i................................................................................................... 100/1020
2020-04-25T20:50:01.3818648Z .................................................................................................... 200/1020
2020-04-25T20:50:08.8991945Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-25T20:50:13.8454396Z .................................................................................................... 400/1020
2020-04-25T20:50:20.6190281Z ....................................................i....i......................................ii.. 500/1020
2020-04-25T20:50:33.4153365Z .................................................................................................... 700/1020
2020-04-25T20:50:33.4153365Z .................................................................................................... 700/1020
2020-04-25T20:50:40.4983416Z ..............................................iiii.................................................. 800/1020
2020-04-25T20:50:54.4043259Z .................................................................................................... 900/1020
2020-04-25T20:51:00.6083775Z ....................................................................iiii............................ 1000/1020
2020-04-25T20:51:01.8709905Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-25T20:51:01.8710211Z 
2020-04-25T20:51:01.8863046Z  finished in 166.084
2020-04-25T20:51:01.8868291Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-25T20:51:48.3007800Z 
2020-04-25T20:51:48.3008257Z error[E0603]: struct `Ident` is private
2020-04-25T20:51:48.3008847Z   --> src/librustc_ast_pretty/pprust/tests.rs:29:31
2020-04-25T20:51:48.3009435Z    |
2020-04-25T20:51:48.3010178Z 29 |         let abba_ident = ast::Ident::from_str("abba");
2020-04-25T20:51:48.3011443Z    |
2020-04-25T20:51:48.3011904Z note: the struct `Ident` is defined here
2020-04-25T20:51:48.3012433Z   --> /checkout/src/librustc_ast/ast.rs:35:35
2020-04-25T20:51:48.3012853Z    |
2020-04-25T20:51:48.3012853Z    |
2020-04-25T20:51:48.3013662Z 35 | use rustc_span::symbol::{kw, sym, Ident, Symbol};
2020-04-25T20:51:48.3014317Z    |                                   ^^^^^
2020-04-25T20:51:48.3014549Z 
2020-04-25T20:51:48.3015091Z error[E0603]: struct `Ident` is private
2020-04-25T20:51:48.3015663Z   --> src/librustc_ast_pretty/pprust/tests.rs:44:26
2020-04-25T20:51:48.3016106Z    |
2020-04-25T20:51:48.3016743Z 44 |         let ident = ast::Ident::from_str("principal_skinner");
2020-04-25T20:51:48.3018152Z    |
2020-04-25T20:51:48.3018620Z note: the struct `Ident` is defined here
2020-04-25T20:51:48.3019147Z   --> /checkout/src/librustc_ast/ast.rs:35:35
2020-04-25T20:51:48.3019564Z    |
---
2020-04-25T20:51:48.7214556Z 
2020-04-25T20:51:48.7215199Z To learn more, run the command again with --verbose.
2020-04-25T20:51:48.7233087Z 
2020-04-25T20:51:48.7233515Z 
2020-04-25T20:51:48.7235218Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_ast_pretty" "--" "--quiet"
2020-04-25T20:51:48.7236312Z 
2020-04-25T20:51:48.7236424Z 
2020-04-25T20:51:48.7246638Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-25T20:51:48.7247225Z Build completed unsuccessfully in 1:28:12
2020-04-25T20:51:48.7247225Z Build completed unsuccessfully in 1:28:12
2020-04-25T20:51:48.7307567Z == clock drift check ==
2020-04-25T20:51:48.7327240Z   local time: Sat Apr 25 20:51:48 UTC 2020
2020-04-25T20:51:48.8996185Z   network time: Sat, 25 Apr 2020 20:51:48 GMT
2020-04-25T20:51:49.3108420Z 
2020-04-25T20:51:49.3108420Z 
2020-04-25T20:51:49.3148954Z ##[error]Bash exited with code '1'.
2020-04-25T20:51:49.3179902Z ##[section]Finishing: Run build
2020-04-25T20:51:49.3266070Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T20:51:49.3271930Z Task         : Get sources
2020-04-25T20:51:49.3272290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T20:51:49.3272643Z Version      : 1.0.0
2020-04-25T20:51:49.3272892Z Author       : Microsoft
2020-04-25T20:51:49.3272892Z Author       : Microsoft
2020-04-25T20:51:49.3273260Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-25T20:51:49.3273678Z ==============================================================================
2020-04-25T20:51:49.7253333Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-25T20:51:49.7301564Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T20:51:49.7408107Z Cleaning up task key
2020-04-25T20:51:49.7409562Z Start cleaning up orphan processes.
2020-04-25T20:51:49.7634417Z Terminate orphan process: pid (3586) (python)
2020-04-25T20:51:49.7901751Z ##[section]Finishing: Finalize Job
