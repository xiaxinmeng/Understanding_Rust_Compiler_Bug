plain
2020-04-25T16:18:23.9415592Z ========================== Starting Command Output ===========================
2020-04-25T16:18:23.9420703Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aa756419-ff71-4d4a-8c4b-d33d0275dbad.sh
2020-04-25T16:18:23.9421176Z 
2020-04-25T16:18:23.9426246Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-25T16:18:23.9446079Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T16:18:23.9449658Z Task         : Get sources
2020-04-25T16:18:23.9450070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T16:18:23.9450349Z Version      : 1.0.0
2020-04-25T16:18:23.9450541Z Author       : Microsoft
---
2020-04-25T16:18:25.4978328Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-25T16:18:25.4986492Z ##[command]git config gc.auto 0
2020-04-25T16:18:25.4995088Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-25T16:18:25.5000985Z ##[command]git config --get-all http.proxy
2020-04-25T16:18:25.5012811Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71555/merge:refs/remotes/pull/71555/merge
---
2020-04-25T16:20:44.2738353Z  ---> cb2676f08729
2020-04-25T16:20:44.2739156Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-25T16:20:44.2742589Z  ---> Using cache
2020-04-25T16:20:44.2742951Z  ---> df25ce111862
2020-04-25T16:20:44.2743926Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-25T16:20:44.2769580Z  ---> 599b9ac96b27
2020-04-25T16:20:44.2769839Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-25T16:20:44.2770571Z  ---> Using cache
2020-04-25T16:20:44.2770907Z  ---> 091087e35a36
---
2020-04-25T16:20:44.3147021Z Looks like docker image is the same as before, not uploading
2020-04-25T16:20:52.2253251Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-25T16:20:52.2543453Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-25T16:20:52.2594053Z == clock drift check ==
2020-04-25T16:20:52.2594661Z   local time: Sat Apr 25 16:20:52 UTC 2020
2020-04-25T16:20:52.2867190Z   network time: Sat, 25 Apr 2020 16:20:52 GMT
2020-04-25T16:20:52.2894165Z Starting sccache server...
2020-04-25T16:20:52.3747743Z configure: processing command line
2020-04-25T16:20:52.3748468Z configure: 
2020-04-25T16:20:52.3749459Z configure: rust.dist-src        := False
---
2020-04-25T16:26:51.7237307Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T16:26:53.4771799Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T16:26:55.3613500Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-25T16:26:58.6419425Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T16:27:07.4315089Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T16:27:12.2145753Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-25T16:27:17.7005483Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T16:27:22.5562588Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-25T16:27:31.9514381Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-25T16:53:31.5698751Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-25T16:53:33.4910985Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-25T16:53:35.5638658Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-25T16:53:36.6064992Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-25T16:53:47.8413771Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-25T16:53:51.6007787Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-25T16:53:56.9404013Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-25T16:54:02.1297297Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-25T16:54:12.0445139Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-25T17:20:22.9749388Z .................................................................................................... 1700/9922
2020-04-25T17:20:27.5777119Z .................................................................................................... 1800/9922
2020-04-25T17:20:36.8642593Z .................................................................................................... 1900/9922
2020-04-25T17:20:45.6501467Z ......i............................................................................................. 2000/9922
2020-04-25T17:20:52.4060765Z ................................................................................................iiii 2100/9922
2020-04-25T17:21:07.3513010Z i................................................................................................... 2200/9922
2020-04-25T17:21:16.4523769Z .................................................................................................... 2400/9922
2020-04-25T17:21:18.8962494Z .................................................................................................... 2500/9922
2020-04-25T17:21:25.1317966Z .................................................................................................... 2600/9922
2020-04-25T17:21:46.3795287Z .................................................................................................... 2700/9922
---
2020-04-25T17:24:39.3819454Z .................................................................................................... 5100/9922
2020-04-25T17:24:47.3057817Z .................................................................................................... 5200/9922
2020-04-25T17:24:52.3985548Z ...................i................................................................................ 5300/9922
2020-04-25T17:25:02.7843039Z ..........i......................................................................................... 5400/9922
2020-04-25T17:25:08.8445486Z ..........ii.ii........i...i........................................................................ 5500/9922
2020-04-25T17:25:17.2453647Z .........................................................i.......................................... 5700/9922
2020-04-25T17:25:26.6209843Z ..........................................................................................ii........ 5800/9922
2020-04-25T17:25:33.7702167Z .............................i...................................................................... 5900/9922
2020-04-25T17:25:39.9807985Z .................................................................................................... 6000/9922
2020-04-25T17:25:39.9807985Z .................................................................................................... 6000/9922
2020-04-25T17:25:51.4897014Z .................................................................................................... 6100/9922
2020-04-25T17:26:02.0498100Z .......................ii...i..ii...........i....................................................... 6200/9922
2020-04-25T17:26:19.6017898Z .................................................................................................... 6400/9922
2020-04-25T17:26:27.3267151Z .................................................................................................... 6500/9922
2020-04-25T17:26:27.3267151Z .................................................................................................... 6500/9922
2020-04-25T17:26:39.4074725Z .....................................................i..ii.......................................... 6600/9922
2020-04-25T17:27:07.7208279Z .................................................................................................... 6800/9922
2020-04-25T17:27:11.0135753Z ......................................................i............................................. 6900/9922
2020-04-25T17:27:13.2541667Z .................................................................................................... 7000/9922
2020-04-25T17:27:15.6269828Z ................................................................................................i... 7100/9922
---
2020-04-25T17:29:05.4098809Z .................................................................................................... 7900/9922
2020-04-25T17:29:11.2675329Z .................................................................................................... 8000/9922
2020-04-25T17:29:18.4038259Z ...............................................................i.................................... 8100/9922
2020-04-25T17:29:29.3873770Z .................................................................................................... 8200/9922
2020-04-25T17:29:35.1884349Z ............iiiiii.iiiii.i.......................................................................... 8300/9922
2020-04-25T17:29:50.4232405Z .................................................................................................... 8500/9922
2020-04-25T17:29:57.2324556Z .................................................................................................... 8600/9922
2020-04-25T17:30:13.1220789Z .................................................................................................... 8700/9922
2020-04-25T17:30:20.5501105Z .................................................................................................... 8800/9922
---
2020-04-25T17:32:49.9496199Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-25T17:32:49.9705477Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T17:32:50.1887756Z 
2020-04-25T17:32:50.1888802Z running 186 tests
2020-04-25T17:32:53.4870735Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-25T17:32:56.3687770Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-25T17:32:56.3692166Z 
2020-04-25T17:32:56.3696622Z  finished in 6.399
2020-04-25T17:32:56.3703751Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-25T17:32:56.3991932Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T17:32:58.6454162Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-25T17:32:58.6705759Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T17:32:58.8390249Z 
2020-04-25T17:32:58.8393370Z running 9 tests
2020-04-25T17:32:58.8394815Z iiiiiiiii
2020-04-25T17:32:58.8398983Z 
2020-04-25T17:32:58.8399159Z  finished in 0.169
2020-04-25T17:32:58.8400192Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-25T17:32:58.8605978Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T17:33:20.6811715Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-25T17:33:20.7082450Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-25T17:33:20.9268796Z 
2020-04-25T17:33:20.9269187Z running 115 tests
2020-04-25T17:33:35.2660309Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-25T17:33:37.2107351Z ...iiii.....ii.
2020-04-25T17:33:37.2112759Z 
2020-04-25T17:33:37.2117950Z  finished in 16.504
2020-04-25T17:33:37.2133518Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-25T17:33:37.2134795Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-25T17:34:06.4978430Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2020-04-25T17:34:06.4978617Z 
2020-04-25T17:34:06.4979014Z error: test compilation failed although it shouldn't!
2020-04-25T17:34:06.4979270Z status: exit code: 1
2020-04-25T17:34:06.4981015Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2020-04-25T17:34:06.4982697Z ------------------------------------------
2020-04-25T17:34:06.4982857Z 
2020-04-25T17:34:06.4983228Z ------------------------------------------
2020-04-25T17:34:06.4983419Z stderr:
2020-04-25T17:34:06.4983419Z stderr:
2020-04-25T17:34:06.4983782Z ------------------------------------------
2020-04-25T17:34:06.4984102Z error[E0433]: failed to resolve: use of undeclared type or module `Ident`
2020-04-25T17:34:06.4984664Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:62:39
2020-04-25T17:34:06.4984909Z    |
2020-04-25T17:34:06.4985166Z LL |     let seg = PathSegment::from_ident(Ident::from_str("x"));
2020-04-25T17:34:06.4985759Z 
2020-04-25T17:34:06.4986104Z error[E0433]: failed to resolve: use of undeclared type or module `Ident`
2020-04-25T17:34:06.4986727Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:83:51
2020-04-25T17:34:06.4986963Z    |
2020-04-25T17:34:06.4986963Z    |
2020-04-25T17:34:06.4987227Z LL |                 let seg = PathSegment::from_ident(Ident::from_str("x"));
2020-04-25T17:34:06.4988828Z 
2020-04-25T17:34:06.4991170Z error[E0433]: failed to resolve: use of undeclared type or module `Ident`
2020-04-25T17:34:06.4991876Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:135:69
2020-04-25T17:34:06.4992114Z    |
2020-04-25T17:34:06.4992114Z    |
2020-04-25T17:34:06.4992671Z LL |                 iter_exprs(depth - 1, &mut |e| g(ExprKind::Field(e, Ident::from_str("f"))));
2020-04-25T17:34:06.4995590Z 
2020-04-25T17:34:06.4995830Z error[E0433]: failed to resolve: use of undeclared type or module `Ident`
2020-04-25T17:34:06.4996538Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:154:45
2020-04-25T17:34:06.4996776Z    |
2020-04-25T17:34:06.4996776Z    |
2020-04-25T17:34:06.4997030Z LL |                 let path = Path::from_ident(Ident::from_str("S"));
2020-04-25T17:34:06.4997669Z 
2020-04-25T17:34:06.4997844Z error: aborting due to 4 previous errors
2020-04-25T17:34:06.4997999Z 
2020-04-25T17:34:06.4998436Z For more information about this error, try `rustc --explain E0433`.
---
2020-04-25T17:34:06.5000512Z test result: FAILED. 63 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-25T17:34:06.5000748Z 
2020-04-25T17:34:06.5000883Z 
2020-04-25T17:34:06.5000975Z 
2020-04-25T17:34:06.5005495Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-25T17:34:06.5008159Z 
2020-04-25T17:34:06.5008259Z 
2020-04-25T17:34:06.5009592Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-25T17:34:06.5009985Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-25T17:34:06.5009985Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-25T17:34:06.5010605Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-25T17:34:06.5011084Z Build completed unsuccessfully in 1:11:21
2020-04-25T17:34:06.5059464Z == clock drift check ==
2020-04-25T17:34:06.5077295Z   local time: Sat Apr 25 17:34:06 UTC 2020
2020-04-25T17:34:06.5307331Z   network time: Sat, 25 Apr 2020 17:34:06 GMT
2020-04-25T17:34:07.3041662Z 
2020-04-25T17:34:07.3041662Z 
2020-04-25T17:34:07.3120114Z ##[error]Bash exited with code '1'.
2020-04-25T17:34:07.3134441Z ##[section]Finishing: Run build
2020-04-25T17:34:07.3196959Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T17:34:07.3202516Z Task         : Get sources
2020-04-25T17:34:07.3202844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T17:34:07.3203130Z Version      : 1.0.0
2020-04-25T17:34:07.3203767Z Author       : Microsoft
2020-04-25T17:34:07.3203767Z Author       : Microsoft
2020-04-25T17:34:07.3204089Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-25T17:34:07.3205136Z ==============================================================================
2020-04-25T17:34:07.6934199Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-25T17:34:07.6981472Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71555/merge to s
2020-04-25T17:34:07.7086587Z Cleaning up task key
2020-04-25T17:34:07.7088043Z Start cleaning up orphan processes.
2020-04-25T17:34:07.7411245Z Terminate orphan process: pid (5114) (python)
2020-04-25T17:34:07.7461168Z ##[section]Finishing: Finalize Job
