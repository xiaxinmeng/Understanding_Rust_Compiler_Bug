plain
2020-04-23T16:02:25.9588612Z ========================== Starting Command Output ===========================
2020-04-23T16:02:25.9593291Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3a71f0a2-5d1f-4112-b960-2a746e99c7f5.sh
2020-04-23T16:02:25.9593559Z 
2020-04-23T16:02:25.9599596Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T16:02:25.9621500Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71458/merge to s
2020-04-23T16:02:25.9625253Z Task         : Get sources
2020-04-23T16:02:25.9625604Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T16:02:25.9625887Z Version      : 1.0.0
2020-04-23T16:02:25.9626081Z Author       : Microsoft
---
2020-04-23T16:02:27.1385161Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T16:02:27.1390956Z ##[command]git config gc.auto 0
2020-04-23T16:02:27.1396589Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T16:02:27.1400527Z ##[command]git config --get-all http.proxy
2020-04-23T16:02:27.1407429Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71458/merge:refs/remotes/pull/71458/merge
---
2020-04-23T16:04:55.6836866Z  ---> 318032b5f0e2
2020-04-23T16:04:55.6838635Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T16:04:55.6840149Z  ---> Using cache
2020-04-23T16:04:55.6841067Z  ---> d44a858fd1ce
2020-04-23T16:04:55.6843130Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T16:04:55.6845781Z  ---> 58b910f50f5a
2020-04-23T16:04:55.6846184Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T16:04:55.6846690Z  ---> Using cache
2020-04-23T16:04:55.6847174Z  ---> ee7702aadba1
---
2020-04-23T16:04:55.7241214Z Looks like docker image is the same as before, not uploading
2020-04-23T16:05:02.4888099Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T16:05:02.5246139Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T16:05:02.5277828Z == clock drift check ==
2020-04-23T16:05:02.5286587Z   local time: Thu Apr 23 16:05:02 UTC 2020
2020-04-23T16:05:02.6041651Z   network time: Thu, 23 Apr 2020 16:05:02 GMT
2020-04-23T16:05:02.6066561Z Starting sccache server...
2020-04-23T16:05:02.6992416Z configure: processing command line
2020-04-23T16:05:02.6992663Z configure: 
2020-04-23T16:05:02.6993567Z configure: rust.dist-src        := False
---
2020-04-23T16:10:52.7833336Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T16:10:54.4032842Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T16:10:56.1319345Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T16:10:58.8445013Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T16:11:07.4154132Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T16:11:12.1515822Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T16:11:17.1102587Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T16:11:21.8993555Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T16:11:30.6561511Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T16:36:00.1205246Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T16:36:01.9168583Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T16:36:03.8972595Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T16:36:07.0114072Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T16:36:15.2752240Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T16:36:21.2220354Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T16:36:26.4805912Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T16:36:31.3641984Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T16:36:38.9102571Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T17:01:22.2846556Z .................................................................................................... 1700/9917
2020-04-23T17:01:26.6442441Z .................................................................................................... 1800/9917
2020-04-23T17:01:35.2985112Z .................................................................................................... 1900/9917
2020-04-23T17:01:43.7356540Z .....i.............................................................................................. 2000/9917
2020-04-23T17:01:50.2012100Z ...............................................................................................iiiii 2100/9917
2020-04-23T17:02:10.8384132Z .................................................................................................... 2300/9917
2020-04-23T17:02:13.1504604Z .................................................................................................... 2400/9917
2020-04-23T17:02:15.4875008Z .................................................................................................... 2500/9917
2020-04-23T17:02:21.3157548Z .................................................................................................... 2600/9917
---
2020-04-23T17:05:24.0912215Z .................................................................................................... 5100/9917
2020-04-23T17:05:31.3864157Z .................................................................................................... 5200/9917
2020-04-23T17:05:35.9809195Z ..................i................................................................................. 5300/9917
2020-04-23T17:05:46.1802951Z ........i........................................................................................... 5400/9917
2020-04-23T17:05:51.7839888Z ........ii.ii........i...i.......................................................................... 5500/9917
2020-04-23T17:05:59.5921555Z .......................................................i............................................ 5700/9917
2020-04-23T17:06:08.8090145Z ........................................................................................ii.......... 5800/9917
2020-04-23T17:06:15.8006326Z ...........................i........................................................................ 5900/9917
2020-04-23T17:06:21.3083212Z .................................................................................................... 6000/9917
2020-04-23T17:06:21.3083212Z .................................................................................................... 6000/9917
2020-04-23T17:06:32.4829712Z .................................................................................................... 6100/9917
2020-04-23T17:06:42.4949898Z .....................ii...i..ii...........i......................................................... 6200/9917
2020-04-23T17:06:58.8856404Z .................................................................................................... 6400/9917
2020-04-23T17:07:06.0233697Z .................................................................................................... 6500/9917
2020-04-23T17:07:06.0233697Z .................................................................................................... 6500/9917
2020-04-23T17:07:19.0097576Z ...................................................i..ii............................................ 6600/9917
2020-04-23T17:07:42.9320181Z .................................................................................................... 6800/9917
2020-04-23T17:07:45.4302095Z ....................................................i............................................... 6900/9917
2020-04-23T17:07:47.4913165Z .................................................................................................... 7000/9917
2020-04-23T17:07:49.6272394Z ............................................................................................i....... 7100/9917
---
2020-04-23T17:09:32.0108067Z .................................................................................................... 7900/9917
2020-04-23T17:09:38.0282376Z .................................................................................................... 8000/9917
2020-04-23T17:09:44.5609860Z ...........................................................i........................................ 8100/9917
2020-04-23T17:09:54.4526329Z .................................................................................................... 8200/9917
2020-04-23T17:10:00.1506714Z ........iiiiii.iiiii.i.............................................................................. 8300/9917
2020-04-23T17:10:14.1496578Z .................................................................................................... 8500/9917
2020-04-23T17:10:21.8371550Z .................................................................................................... 8600/9917
2020-04-23T17:10:35.6605706Z .................................................................................................... 8700/9917
2020-04-23T17:10:42.3016206Z .................................................................................................... 8800/9917
---
2020-04-23T17:13:03.5776990Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-23T17:13:03.5975800Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T17:13:03.8091751Z 
2020-04-23T17:13:03.8092180Z running 186 tests
2020-04-23T17:13:06.7913891Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-23T17:13:09.4622727Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-23T17:13:09.4628468Z 
2020-04-23T17:13:09.4633074Z  finished in 5.864
2020-04-23T17:13:09.4637278Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-23T17:13:09.4816378Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T17:13:11.6737591Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-23T17:13:11.6941637Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T17:13:11.8553948Z 
2020-04-23T17:13:11.8555794Z running 9 tests
2020-04-23T17:13:11.8556799Z iiiiiiiii
2020-04-23T17:13:11.8557862Z 
2020-04-23T17:13:11.8561965Z  finished in 0.162
2020-04-23T17:13:11.8569068Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-23T17:13:11.8760324Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T17:13:32.7743734Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-23T17:13:32.8002393Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T17:13:32.9985562Z 
2020-04-23T17:13:32.9985953Z running 115 tests
2020-04-23T17:13:46.9752117Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-23T17:13:48.7594401Z ...iiii.....ii.
2020-04-23T17:13:48.7595538Z 
2020-04-23T17:13:48.7600936Z  finished in 15.960
2020-04-23T17:13:48.7605964Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-23T17:13:48.7609537Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-23T17:26:28.1392985Z 
2020-04-23T17:26:28.1399353Z    Doc-tests core
2020-04-23T17:26:33.2852224Z 
2020-04-23T17:26:33.2853610Z running 2499 tests
2020-04-23T17:26:40.7671265Z ......iiiii......................................................................................... 100/2499
2020-04-23T17:26:48.1775251Z ......................................................................................ii............ 200/2499
2020-04-23T17:27:05.2430297Z ......................i............................................................................. 400/2499
2020-04-23T17:27:13.6143938Z ............................................................................i..i..................ii 500/2499
2020-04-23T17:27:19.7417079Z ii.................................................................................................. 600/2499
2020-04-23T17:27:26.4971902Z .................................................................................................... 700/2499
---
2020-04-23T17:30:55.7240896Z 
2020-04-23T17:30:55.7241222Z running 1020 tests
2020-04-23T17:31:11.2697539Z i................................................................................................... 100/1020
2020-04-23T17:31:19.8258365Z .................................................................................................... 200/1020
2020-04-23T17:31:26.1526691Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-23T17:31:30.5704918Z .................................................................................................... 400/1020
2020-04-23T17:31:36.0544562Z ....................................................i....i......................................ii.. 500/1020
2020-04-23T17:31:47.0193553Z .................................................................................................... 700/1020
2020-04-23T17:31:47.0193553Z .................................................................................................... 700/1020
2020-04-23T17:31:52.4054137Z ..............................................iiii.................................................. 800/1020
2020-04-23T17:32:05.0705577Z .................................................................................................... 900/1020
2020-04-23T17:32:10.5948548Z ....................................................................iiii............................ 1000/1020
2020-04-23T17:32:11.7647097Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-23T17:32:11.7647570Z 
2020-04-23T17:32:11.7747878Z  finished in 154.044
2020-04-23T17:32:11.7755316Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-23T17:35:32.5790811Z 
2020-04-23T17:35:32.5791222Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-23T17:35:32.5791429Z 
2020-04-23T17:35:32.5850886Z  finished in 1.099
2020-04-23T17:35:32.5856665Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-23T17:35:32.5877764Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-23T17:35:32.7990283Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T17:35:33.9705114Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-83347ed9ff950ca7
2020-04-23T17:35:33.9734178Z 
2020-04-23T17:35:33.9734651Z running 0 tests
2020-04-23T17:35:33.9734912Z 
---
2020-04-23T17:49:54.9976906Z 
2020-04-23T17:49:55.0260247Z error: Could not document `core`.
2020-04-23T17:49:55.0260475Z 
2020-04-23T17:49:55.0260609Z Caused by:
2020-04-23T17:49:55.0262896Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core src/libcore/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages -Z unstable-options --resource-suffix 1.44.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cbitcode-in-rlib=yes` (exit code: 1)
2020-04-23T17:49:55.0284890Z 
2020-04-23T17:49:55.0284890Z 
2020-04-23T17:49:55.0290778Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "-Z" "unstable-options" "--resource-suffix" "1.44.0" "--index-page" "/checkout/src/doc/index.md"
2020-04-23T17:49:55.0291926Z 
2020-04-23T17:49:55.0292015Z 
2020-04-23T17:49:55.0292836Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-23T17:49:55.0293167Z Build completed unsuccessfully in 1:43:04
2020-04-23T17:49:55.0293167Z Build completed unsuccessfully in 1:43:04
2020-04-23T17:49:55.0351540Z == clock drift check ==
2020-04-23T17:49:55.0381057Z   local time: Thu Apr 23 17:49:55 UTC 2020
2020-04-23T17:49:55.1841396Z   network time: Thu, 23 Apr 2020 17:49:55 GMT
2020-04-23T17:49:56.0752040Z 
2020-04-23T17:49:56.0752040Z 
2020-04-23T17:49:56.0853090Z ##[error]Bash exited with code '1'.
2020-04-23T17:49:56.0883771Z ##[section]Finishing: Run build
2020-04-23T17:49:56.0939205Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71458/merge to s
2020-04-23T17:49:56.0944288Z Task         : Get sources
2020-04-23T17:49:56.0944614Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T17:49:56.0944922Z Version      : 1.0.0
2020-04-23T17:49:56.0945129Z Author       : Microsoft
2020-04-23T17:49:56.0945129Z Author       : Microsoft
2020-04-23T17:49:56.0945461Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T17:49:56.0946193Z ==============================================================================
2020-04-23T17:49:56.4417190Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T17:49:56.4469749Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71458/merge to s
2020-04-23T17:49:56.4570796Z Cleaning up task key
2020-04-23T17:49:56.4572099Z Start cleaning up orphan processes.
2020-04-23T17:49:56.4788744Z Terminate orphan process: pid (5155) (python)
2020-04-23T17:49:56.5025954Z ##[section]Finishing: Finalize Job
