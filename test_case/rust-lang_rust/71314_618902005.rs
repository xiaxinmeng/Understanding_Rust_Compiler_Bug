plain
2020-04-24T07:39:20.1067083Z ========================== Starting Command Output ===========================
2020-04-24T07:39:20.1069609Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1a62edbf-1ba9-42b6-894d-f39519681082.sh
2020-04-24T07:39:20.1069862Z 
2020-04-24T07:39:20.1073555Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T07:39:20.1091491Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-24T07:39:20.1094585Z Task         : Get sources
2020-04-24T07:39:20.1094860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T07:39:20.1095129Z Version      : 1.0.0
2020-04-24T07:39:20.1095330Z Author       : Microsoft
---
2020-04-24T07:39:21.1005345Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T07:39:21.1010733Z ##[command]git config gc.auto 0
2020-04-24T07:39:21.1014821Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T07:39:21.1018291Z ##[command]git config --get-all http.proxy
2020-04-24T07:39:21.1024480Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71314/merge:refs/remotes/pull/71314/merge
---
2020-04-24T07:41:30.4138565Z  ---> cb2676f08729
2020-04-24T07:41:30.4139497Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-24T07:41:30.4161371Z  ---> Using cache
2020-04-24T07:41:30.4163038Z  ---> df25ce111862
2020-04-24T07:41:30.4234127Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-24T07:41:30.4236199Z  ---> 599b9ac96b27
2020-04-24T07:41:30.4236473Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-24T07:41:30.4236873Z  ---> Using cache
2020-04-24T07:41:30.4237191Z  ---> 091087e35a36
---
2020-04-24T07:41:30.4629632Z Looks like docker image is the same as before, not uploading
2020-04-24T07:41:37.0004620Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T07:41:37.0286739Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T07:41:37.0313751Z == clock drift check ==
2020-04-24T07:41:37.0330272Z   local time: Fri Apr 24 07:41:37 UTC 2020
2020-04-24T07:41:37.1983126Z   network time: Fri, 24 Apr 2020 07:41:37 GMT
2020-04-24T07:41:37.2018251Z Starting sccache server...
2020-04-24T07:41:37.2835493Z configure: processing command line
2020-04-24T07:41:37.2835775Z configure: 
2020-04-24T07:41:37.2836737Z configure: rust.dist-src        := False
---
2020-04-24T07:46:42.7461320Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T07:46:44.1439520Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T07:46:45.6904684Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T07:46:47.2189433Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T07:46:55.1416551Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T07:46:58.3942257Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T07:47:02.7654763Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T07:47:06.8417173Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T07:47:14.7139821Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T08:08:58.7107375Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T08:09:00.2254524Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T08:09:01.9795046Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T08:09:02.8607917Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T08:09:12.9659932Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T08:09:15.4185477Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T08:09:19.9762833Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T08:09:24.2281738Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T08:09:34.1429644Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T08:31:38.6453265Z .................................................................................................... 1700/9921
2020-04-24T08:31:42.8951555Z .................................................................................................... 1800/9921
2020-04-24T08:31:51.0524605Z .................................................................................................... 1900/9921
2020-04-24T08:31:58.9109317Z ......i............................................................................................. 2000/9921
2020-04-24T08:32:05.0146469Z ................................................................................................iiii 2100/9921
2020-04-24T08:32:18.1626656Z i................................................................................................... 2200/9921
2020-04-24T08:32:26.2328529Z .................................................................................................... 2400/9921
2020-04-24T08:32:28.4834315Z .................................................................................................... 2500/9921
2020-04-24T08:32:34.0471115Z .................................................................................................... 2600/9921
2020-04-24T08:32:51.4595793Z .................................................................................................... 2700/9921
---
2020-04-24T08:35:26.4221942Z .................................................................................................... 5100/9921
2020-04-24T08:35:33.3167676Z .................................................................................................... 5200/9921
2020-04-24T08:35:38.0433796Z ....................i............................................................................... 5300/9921
2020-04-24T08:35:47.5046644Z ..........i......................................................................................... 5400/9921
2020-04-24T08:35:52.9459033Z ..........ii.ii........i...i........................................................................ 5500/9921
2020-04-24T08:36:00.5609423Z .........................................................i.......................................... 5700/9921
2020-04-24T08:36:09.1152949Z ..........................................................................................ii........ 5800/9921
2020-04-24T08:36:15.7004089Z .............................i...................................................................... 5900/9921
2020-04-24T08:36:21.0594043Z .................................................................................................... 6000/9921
2020-04-24T08:36:21.0594043Z .................................................................................................... 6000/9921
2020-04-24T08:36:31.2025566Z .................................................................................................... 6100/9921
2020-04-24T08:36:41.2005231Z .......................ii...i..ii...........i....................................................... 6200/9921
2020-04-24T08:36:56.8298527Z .................................................................................................... 6400/9921
2020-04-24T08:37:00.1408680Z .................................................................................................... 6500/9921
2020-04-24T08:37:00.1408680Z .................................................................................................... 6500/9921
2020-04-24T08:37:06.3946194Z .....................................................i..ii.......................................... 6600/9921
2020-04-24T08:37:30.3207316Z .................................................................................................... 6800/9921
2020-04-24T08:37:33.0742295Z ......................................................i............................................. 6900/9921
2020-04-24T08:37:35.0751247Z .................................................................................................... 7000/9921
2020-04-24T08:37:37.1041201Z ................................................................................................i... 7100/9921
---
2020-04-24T08:39:11.8972729Z .................................................................................................... 7900/9921
2020-04-24T08:39:17.2716949Z .................................................................................................... 8000/9921
2020-04-24T08:39:23.5758257Z ...............................................................i.................................... 8100/9921
2020-04-24T08:39:33.4108222Z .................................................................................................... 8200/9921
2020-04-24T08:39:38.6587364Z ............iiiiii.iiiii.i.......................................................................... 8300/9921
2020-04-24T08:39:52.0281638Z .................................................................................................... 8500/9921
2020-04-24T08:39:58.0566320Z .................................................................................................... 8600/9921
2020-04-24T08:40:12.0448069Z .................................................................................................... 8700/9921
2020-04-24T08:40:18.6753081Z .................................................................................................... 8800/9921
---
2020-04-24T08:42:31.9488408Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-24T08:42:31.9676818Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T08:42:32.1617764Z 
2020-04-24T08:42:32.1618083Z running 186 tests
2020-04-24T08:42:35.0297411Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-24T08:42:37.5759970Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-24T08:42:37.5770396Z 
2020-04-24T08:42:37.5770551Z  finished in 5.608
2020-04-24T08:42:37.5774168Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-24T08:42:37.5964436Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T08:42:39.7577000Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-24T08:42:39.7793841Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T08:42:39.9307580Z 
2020-04-24T08:42:39.9309024Z running 9 tests
2020-04-24T08:42:39.9317431Z iiiiiiiii
2020-04-24T08:42:39.9318825Z 
2020-04-24T08:42:39.9333160Z  finished in 0.153
2020-04-24T08:42:39.9337350Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-24T08:42:39.9534256Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T08:42:59.5718814Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-24T08:42:59.5944817Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T08:42:59.7846789Z 
2020-04-24T08:42:59.7847084Z running 115 tests
2020-04-24T08:43:13.2858092Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-24T08:43:15.0047820Z ...iiii.....ii.
2020-04-24T08:43:15.0049578Z 
2020-04-24T08:43:15.0054740Z  finished in 15.411
2020-04-24T08:43:15.0061923Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-24T08:43:15.0062645Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T08:54:27.7744264Z warning: 2 warnings emitted
2020-04-24T08:54:27.7744534Z 
2020-04-24T08:54:27.7792121Z 
2020-04-24T08:54:27.7792362Z running 2499 tests
2020-04-24T08:54:36.2759951Z ......iiiii......................................................................................... 100/2499
2020-04-24T08:54:44.6614256Z ......................................................................................ii............ 200/2499
2020-04-24T08:55:03.6485131Z ......................i............................................................................. 400/2499
2020-04-24T08:55:12.9053431Z ............................................................................i..i..................ii 500/2499
2020-04-24T08:55:19.9531269Z ii.................................................................................................. 600/2499
2020-04-24T08:55:28.0813565Z .................................................................................................... 700/2499
---
2020-04-24T08:59:09.1431374Z 
2020-04-24T08:59:09.1432533Z running 1020 tests
2020-04-24T08:59:24.8307996Z i................................................................................................... 100/1020
2020-04-24T08:59:34.4159751Z .................................................................................................... 200/1020
2020-04-24T08:59:41.5886327Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-24T08:59:46.2278846Z .................................................................................................... 400/1020
2020-04-24T08:59:52.4570509Z ....................................................i....i......................................ii.. 500/1020
2020-04-24T09:00:04.4466268Z .................................................................................................... 700/1020
2020-04-24T09:00:04.4466268Z .................................................................................................... 700/1020
2020-04-24T09:00:10.9966742Z ..............................................iiii.................................................. 800/1020
2020-04-24T09:00:23.6150734Z .................................................................................................... 900/1020
2020-04-24T09:00:29.2573273Z ....................................................................iiii............................ 1000/1020
2020-04-24T09:00:30.4650434Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-24T09:00:30.4650653Z 
2020-04-24T09:00:30.4779690Z  finished in 150.815
2020-04-24T09:00:30.4781829Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-24T09:03:23.9147292Z 
2020-04-24T09:03:23.9147748Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-24T09:03:23.9148130Z 
2020-04-24T09:03:23.9205321Z  finished in 0.964
2020-04-24T09:03:23.9212020Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-24T09:03:23.9231677Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T09:03:24.1244744Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T09:03:25.0931102Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-2ddea488951f4db1
2020-04-24T09:03:25.0959050Z 
2020-04-24T09:03:25.0959276Z running 0 tests
2020-04-24T09:03:25.0959395Z 
---
2020-04-24T09:16:39.6952950Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6953665Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6954468Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6955198Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6955932Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6957395Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6958118Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6958819Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T09:16:39.6959564Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-24T09:17:40.4455134Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-24T09:17:40.4786688Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T09:17:40.6943570Z 
2020-04-24T09:17:40.6945062Z running 211 tests
2020-04-24T09:18:09.6615646Z ......................i...ii.......................................................................i 100/211
2020-04-24T09:18:42.3071051Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-24T09:18:49.1968738Z .......ii..
2020-04-24T09:18:49.1970480Z 
2020-04-24T09:18:49.1975400Z  finished in 68.719
2020-04-24T09:18:49.1982987Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-24T09:18:49.1989403Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-24T09:18:52.9194565Z ---- /checkout/src/doc/unstable-book/src/language-features/cfg-version.md - The_tracking_issue_for_this_feature_is__::Examples (line 14) stdout ----
2020-04-24T09:18:52.9194968Z error: invalid version string
2020-04-24T09:18:52.9198183Z  --> /checkout/src/doc/unstable-book/src/language-features/cfg-version.md:17:15
2020-04-24T09:18:52.9198442Z   |
2020-04-24T09:18:52.9198601Z 5 | #[cfg(version("1.42"))]
2020-04-24T09:18:52.9198952Z 
2020-04-24T09:18:52.9199110Z error: invalid version string
2020-04-24T09:18:52.9199608Z   --> /checkout/src/doc/unstable-book/src/language-features/cfg-version.md:22:19
2020-04-24T09:18:52.9199871Z    |
2020-04-24T09:18:52.9199871Z    |
2020-04-24T09:18:52.9200040Z 10 | #[cfg(not(version("1.42")))]
2020-04-24T09:18:52.9200391Z 
2020-04-24T09:18:52.9200552Z error: invalid version string
2020-04-24T09:18:52.9201048Z   --> /checkout/src/doc/unstable-book/src/language-features/cfg-version.md:28:21
2020-04-24T09:18:52.9201972Z    |
2020-04-24T09:18:52.9201972Z    |
2020-04-24T09:18:52.9202146Z 16 |     if cfg!(version("1.42")) {
2020-04-24T09:18:52.9202491Z 
2020-04-24T09:18:52.9202677Z error: aborting due to 3 previous errors
2020-04-24T09:18:52.9202831Z 
2020-04-24T09:18:52.9203173Z Couldn't compile the test.
---
2020-04-24T09:18:52.9205824Z 
2020-04-24T09:18:52.9206377Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-24T09:18:52.9206713Z Build completed unsuccessfully in 1:35:43
2020-04-24T09:18:52.9254459Z == clock drift check ==
2020-04-24T09:18:52.9272500Z   local time: Fri Apr 24 09:18:52 UTC 2020
2020-04-24T09:18:52.9807731Z   network time: Fri, 24 Apr 2020 09:18:52 GMT
2020-04-24T09:18:54.7556103Z 
2020-04-24T09:18:54.7556103Z 
2020-04-24T09:18:54.7634178Z ##[error]Bash exited with code '1'.
2020-04-24T09:18:54.7650274Z ##[section]Finishing: Run build
2020-04-24T09:18:54.7696530Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-24T09:18:54.7702195Z Task         : Get sources
2020-04-24T09:18:54.7702514Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T09:18:54.7702816Z Version      : 1.0.0
2020-04-24T09:18:54.7703021Z Author       : Microsoft
2020-04-24T09:18:54.7703021Z Author       : Microsoft
2020-04-24T09:18:54.7703344Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T09:18:54.7703728Z ==============================================================================
2020-04-24T09:18:55.1093755Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T09:18:55.1142091Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-24T09:18:55.1237573Z Cleaning up task key
2020-04-24T09:18:55.1238892Z Start cleaning up orphan processes.
2020-04-24T09:18:55.1426107Z Terminate orphan process: pid (3888) (python)
2020-04-24T09:18:55.1671890Z ##[section]Finishing: Finalize Job
