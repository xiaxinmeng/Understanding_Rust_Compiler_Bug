plain
2020-04-03T18:32:10.3124468Z ========================== Starting Command Output ===========================
2020-04-03T18:32:10.3127417Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bad288ad-5b0e-4bbd-ab13-ad768f079a8e.sh
2020-04-03T18:32:10.3127743Z 
2020-04-03T18:32:10.3132337Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T18:32:10.3155069Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T18:32:10.3158540Z Task         : Get sources
2020-04-03T18:32:10.3158830Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T18:32:10.3159112Z Version      : 1.0.0
2020-04-03T18:32:10.3159303Z Author       : Microsoft
---
2020-04-03T18:32:11.3207752Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T18:32:11.3212602Z ##[command]git config gc.auto 0
2020-04-03T18:32:11.3215869Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T18:32:11.3219210Z ##[command]git config --get-all http.proxy
2020-04-03T18:32:11.3224714Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70733/merge:refs/remotes/pull/70733/merge
---
2020-04-03T18:34:12.2404788Z Looks like docker image is the same as before, not uploading
2020-04-03T18:34:19.0110712Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T18:34:19.0334521Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T18:34:19.0361159Z == clock drift check ==
2020-04-03T18:34:19.0368945Z   local time: Fri Apr  3 18:34:19 UTC 2020
2020-04-03T18:34:19.1067119Z   network time: Fri, 03 Apr 2020 18:34:19 GMT
2020-04-03T18:34:19.1091783Z Starting sccache server...
2020-04-03T18:34:19.1937015Z configure: processing command line
2020-04-03T18:34:19.1938008Z configure: 
2020-04-03T18:34:19.1939727Z configure: rust.dist-src        := False
---
2020-04-03T18:38:58.8737356Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T18:39:00.3164940Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T18:39:01.8863099Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T18:39:02.9234372Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T18:39:11.9461837Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T18:39:14.2468584Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T18:39:18.4020942Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T18:39:22.2593591Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T18:39:31.0156188Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T18:59:09.8191103Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T18:59:11.4771867Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T18:59:13.3455251Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T18:59:14.6739378Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T18:59:25.4163260Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T18:59:27.6588772Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T18:59:32.8797255Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T18:59:38.1175381Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T18:59:49.1561509Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T19:22:05.1803328Z .................................................................................................... 1700/9872
2020-04-03T19:22:08.8216159Z .................................................................................................... 1800/9872
2020-04-03T19:22:16.6974021Z ...............................................................................................i.... 1900/9872
2020-04-03T19:22:23.6043067Z .................................................................................................... 2000/9872
2020-04-03T19:22:29.2303316Z .....................................................................................iiiii.......... 2100/9872
2020-04-03T19:22:46.7815486Z .................................................................................................... 2300/9872
2020-04-03T19:22:48.5917416Z .................................................................................................... 2400/9872
2020-04-03T19:22:50.5475431Z .................................................................................................... 2500/9872
2020-04-03T19:22:55.8514613Z .................................................................................................... 2600/9872
---
2020-04-03T19:25:25.8247131Z ...........................................................i...............i........................ 5000/9872
2020-04-03T19:25:32.0747299Z .................................................................................................... 5100/9872
2020-04-03T19:25:38.7845545Z .................................................................................................... 5200/9872
2020-04-03T19:25:43.5006385Z ....i............................................................................................... 5300/9872
2020-04-03T19:25:52.7296834Z ............................................................................................ii.ii... 5400/9872
2020-04-03T19:25:56.8257865Z .....i...i.......................................................................................... 5500/9872
2020-04-03T19:26:04.7263004Z .....................................i.............................................................. 5700/9872
2020-04-03T19:26:13.9249290Z .........................................................ii....................................i.... 5800/9872
2020-04-03T19:26:21.2766043Z .................................................................................................... 5900/9872
2020-04-03T19:26:25.6143102Z .................................................................................................... 6000/9872
2020-04-03T19:26:25.6143102Z .................................................................................................... 6000/9872
2020-04-03T19:26:33.7240690Z .........................................................................................ii...i..ii. 6100/9872
2020-04-03T19:26:50.3628110Z .................................................................................................... 6300/9872
2020-04-03T19:26:55.7303953Z .................................................................................................... 6400/9872
2020-04-03T19:26:58.0273359Z .................................................................................................... 6500/9872
2020-04-03T19:26:58.0273359Z .................................................................................................... 6500/9872
2020-04-03T19:27:07.8783036Z ...................i..ii............................................................................ 6600/9872
2020-04-03T19:27:24.2953589Z .................................................................................................... 6800/9872
2020-04-03T19:27:25.9884250Z ...................i................................................................................ 6900/9872
2020-04-03T19:27:28.0031372Z .................................................................................................... 7000/9872
2020-04-03T19:27:30.1403487Z ..........................................................i......................................... 7100/9872
---
2020-04-03T19:28:49.1462001Z .................................................................................................... 7800/9872
2020-04-03T19:28:52.8771555Z .................................................................................................... 7900/9872
2020-04-03T19:28:57.7111412Z .................................................................................................... 8000/9872
2020-04-03T19:29:04.6685434Z ......................i............................................................................. 8100/9872
2020-04-03T19:29:12.0610770Z .......................................................................iiiiiiiiii.i................. 8200/9872
2020-04-03T19:29:26.7795138Z ...............i......i............................................................................. 8400/9872
2020-04-03T19:29:31.4054790Z .................................................................................................... 8500/9872
2020-04-03T19:29:42.3389102Z .................................................................................................... 8600/9872
2020-04-03T19:29:54.2988194Z .................................................................................................... 8700/9872
---
2020-04-03T19:31:49.7832904Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-03T19:31:49.7999132Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T19:31:49.9598167Z 
2020-04-03T19:31:49.9598742Z running 183 tests
2020-04-03T19:31:52.4901049Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-04-03T19:31:54.6348160Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-04-03T19:31:54.6354361Z 
2020-04-03T19:31:54.6359686Z  finished in 4.836
2020-04-03T19:31:54.6364316Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-03T19:31:54.6532219Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T19:31:56.5877365Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-03T19:31:56.6065366Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T19:31:56.7404705Z 
2020-04-03T19:31:56.7405631Z running 9 tests
2020-04-03T19:31:56.7407122Z iiiiiiiii
2020-04-03T19:31:56.7408177Z 
2020-04-03T19:31:56.7408418Z  finished in 0.134
2020-04-03T19:31:56.7412954Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-03T19:31:56.7600069Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T19:32:15.3883954Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-03T19:32:15.4077640Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T19:32:15.5617441Z 
2020-04-03T19:32:15.5617716Z running 115 tests
2020-04-03T19:32:26.9746529Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-03T19:32:28.5065946Z ...iiii.....ii.
2020-04-03T19:32:28.5067837Z 
2020-04-03T19:32:28.5068529Z  finished in 13.099
2020-04-03T19:32:28.5074719Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-03T19:32:28.5077717Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T19:41:46.9774310Z error: aborting due to previous error
2020-04-03T19:41:46.9774505Z 
2020-04-03T19:41:46.9774918Z For more information about this error, try `rustc --explain E0658`.
2020-04-03T19:41:46.9775333Z Couldn't compile the test.
2020-04-03T19:41:46.9775797Z ---- sync.rs - sync::Arc<T>::inc_strong_count (line 767) stdout ----
2020-04-03T19:41:46.9776236Z error[E0599]: no method named `into_raw` found for struct `std::sync::Arc<{integer}>` in the current scope
2020-04-03T19:41:46.9776707Z   --> sync.rs:775:19
2020-04-03T19:41:46.9776922Z    |
2020-04-03T19:41:46.9777163Z 11 |    let ptr = five.into_raw() as *const ();
2020-04-03T19:41:46.9777786Z    |              |    |
2020-04-03T19:41:46.9778262Z    |              |    this is an associated function, not a method
2020-04-03T19:41:46.9778262Z    |              |    this is an associated function, not a method
2020-04-03T19:41:46.9778766Z    |              help: use associated function syntax instead: `std::sync::Arc::<{integer}>::into_raw`
2020-04-03T19:41:46.9779407Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-03T19:41:46.9779840Z    = note: the candidate is defined in an impl for the type `std::sync::Arc<_>`
2020-04-03T19:41:46.9780122Z 
2020-04-03T19:41:46.9780335Z error: aborting due to previous error
---
2020-04-03T19:41:46.9809628Z 
2020-04-03T19:41:46.9816421Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-03T19:41:46.9816700Z Build completed unsuccessfully in 1:06:02
2020-04-03T19:41:46.9862591Z == clock drift check ==
2020-04-03T19:41:46.9881755Z   local time: Fri Apr  3 19:41:46 UTC 2020
2020-04-03T19:41:47.0369164Z   network time: Fri, 03 Apr 2020 19:41:47 GMT
2020-04-03T19:41:47.3852569Z 
2020-04-03T19:41:47.3852569Z 
2020-04-03T19:41:47.3881569Z ##[error]Bash exited with code '1'.
2020-04-03T19:41:47.3902398Z ##[section]Finishing: Run build
2020-04-03T19:41:47.3949661Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T19:41:47.3953473Z Task         : Get sources
2020-04-03T19:41:47.3953718Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T19:41:47.3953945Z Version      : 1.0.0
2020-04-03T19:41:47.3954126Z Author       : Microsoft
2020-04-03T19:41:47.3954126Z Author       : Microsoft
2020-04-03T19:41:47.3954377Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T19:41:47.3954666Z ==============================================================================
2020-04-03T19:41:47.6398395Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T19:41:47.6435825Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T19:41:47.6523119Z Cleaning up task key
2020-04-03T19:41:47.6524023Z Start cleaning up orphan processes.
2020-04-03T19:41:47.6666955Z Terminate orphan process: pid (3419) (python)
2020-04-03T19:41:47.6793174Z ##[section]Finishing: Finalize Job
