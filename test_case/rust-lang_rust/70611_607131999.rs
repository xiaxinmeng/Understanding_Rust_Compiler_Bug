plain
2020-04-01T07:34:49.4917560Z ========================== Starting Command Output ===========================
2020-04-01T07:34:49.4919970Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/36d6a385-1cbc-4371-9ed9-00e341db993e.sh
2020-04-01T07:34:49.4920229Z 
2020-04-01T07:34:49.4923612Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T07:34:49.4941560Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T07:34:49.4944915Z Task         : Get sources
2020-04-01T07:34:49.4945224Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T07:34:49.4945526Z Version      : 1.0.0
2020-04-01T07:34:49.4945726Z Author       : Microsoft
---
2020-04-01T07:34:50.5044555Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T07:34:50.5050094Z ##[command]git config gc.auto 0
2020-04-01T07:34:50.5053879Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T07:34:50.5058480Z ##[command]git config --get-all http.proxy
2020-04-01T07:34:50.5066097Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70611/merge:refs/remotes/pull/70611/merge
---
2020-04-01T07:37:01.4573384Z Looks like docker image is the same as before, not uploading
2020-04-01T07:37:06.1022498Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T07:37:06.1353424Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T07:37:06.1378750Z == clock drift check ==
2020-04-01T07:37:06.1387156Z   local time: Wed Apr  1 07:37:06 UTC 2020
2020-04-01T07:37:06.2010924Z   network time: Wed, 01 Apr 2020 07:37:06 GMT
2020-04-01T07:37:06.2044465Z Starting sccache server...
2020-04-01T07:37:06.2805414Z configure: processing command line
2020-04-01T07:37:06.2805990Z configure: 
2020-04-01T07:37:06.2806801Z configure: rust.dist-src        := False
---
2020-04-01T07:41:36.1156101Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T07:41:37.3714271Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T07:41:38.7520284Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T07:41:39.2075164Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T07:41:47.3930703Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T07:41:48.9101359Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T07:41:52.8271660Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T07:41:56.5059334Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T07:42:05.1461236Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T08:01:44.8855290Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T08:01:46.5053233Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T08:01:48.3082475Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T08:01:48.9896799Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T08:01:59.4013735Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T08:02:01.3507129Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T08:02:06.1474677Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T08:02:11.0143807Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T08:02:21.3013633Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T08:25:20.8257232Z .................................................................................................... 1700/9865
2020-04-01T08:25:24.6349690Z .................................................................................................... 1800/9865
2020-04-01T08:25:32.8960008Z ...............................................................................................i.... 1900/9865
2020-04-01T08:25:40.1580675Z .................................................................................................... 2000/9865
2020-04-01T08:25:46.1278611Z .....................................................................................iiiii.......... 2100/9865
2020-04-01T08:26:05.6028019Z .................................................................................................... 2300/9865
2020-04-01T08:26:07.6093809Z .................................................................................................... 2400/9865
2020-04-01T08:26:09.7507884Z .................................................................................................... 2500/9865
2020-04-01T08:26:15.3646185Z .................................................................................................... 2600/9865
---
2020-04-01T08:28:54.5926567Z ...........................................................i...............i........................ 5000/9865
2020-04-01T08:29:02.0323430Z .................................................................................................... 5100/9865
2020-04-01T08:29:09.3462944Z .................................................................................................... 5200/9865
2020-04-01T08:29:14.1708610Z ....i............................................................................................... 5300/9865
2020-04-01T08:29:24.1793138Z ..........................................................................................ii.ii..... 5400/9865
2020-04-01T08:29:28.2639258Z ...i...i............................................................................................ 5500/9865
2020-04-01T08:29:36.7611810Z ...................................i................................................................ 5700/9865
2020-04-01T08:29:45.9869995Z .......................................................ii....................................i...... 5800/9865
2020-04-01T08:29:53.1039471Z .................................................................................................... 5900/9865
2020-04-01T08:29:57.7059020Z .................................................................................................... 6000/9865
2020-04-01T08:29:57.7059020Z .................................................................................................... 6000/9865
2020-04-01T08:30:06.2643151Z .......................................................................................ii...i..ii... 6100/9865
2020-04-01T08:30:25.7479610Z .................................................................................................... 6300/9865
2020-04-01T08:30:30.0591262Z .................................................................................................... 6400/9865
2020-04-01T08:30:32.8440312Z .................................................................................................... 6500/9865
2020-04-01T08:30:32.8440312Z .................................................................................................... 6500/9865
2020-04-01T08:30:44.3655619Z .................i..ii.............................................................................. 6600/9865
2020-04-01T08:31:03.4533272Z .................................................................................................... 6800/9865
2020-04-01T08:31:05.4168583Z .................i.................................................................................. 6900/9865
2020-04-01T08:31:07.3155661Z .................................................................................................... 7000/9865
2020-04-01T08:31:09.3604667Z ........................................................i........................................... 7100/9865
---
2020-04-01T08:32:39.4006406Z .................................................................................................... 7800/9865
2020-04-01T08:32:44.0903101Z .................................................................................................... 7900/9865
2020-04-01T08:32:48.9638778Z .................................................................................................... 8000/9865
2020-04-01T08:32:56.4624312Z ................i................................................................................... 8100/9865
2020-04-01T08:33:04.0616426Z .................................................................iiiiiiiiii.i....................... 8200/9865
2020-04-01T08:33:17.7815460Z .........i......i................................................................................... 8400/9865
2020-04-01T08:33:22.1703830Z .................................................................................................... 8500/9865
2020-04-01T08:33:32.6671815Z .................................................................................................... 8600/9865
2020-04-01T08:33:43.0220269Z .................................................................................................... 8700/9865
---
2020-04-01T08:35:52.8698683Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-01T08:35:52.8893775Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-01T08:35:53.0907613Z 
2020-04-01T08:35:53.0907975Z running 183 tests
2020-04-01T08:35:55.6579317Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-04-01T08:35:58.0598443Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-04-01T08:35:58.0602047Z 
2020-04-01T08:35:58.0608497Z  finished in 5.171
2020-04-01T08:35:58.0612502Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-01T08:35:58.0791828Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-01T08:35:59.9662350Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-01T08:35:59.9852405Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-01T08:36:00.1353939Z 
2020-04-01T08:36:00.1354292Z running 9 tests
2020-04-01T08:36:00.1355495Z iiiiiiiii
2020-04-01T08:36:00.1359404Z 
2020-04-01T08:36:00.1359586Z  finished in 0.150
2020-04-01T08:36:00.1361241Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-01T08:36:00.1545723Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-01T08:36:18.1368826Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-01T08:36:18.1582291Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-01T08:36:18.3420030Z 
2020-04-01T08:36:18.3420983Z running 115 tests
2020-04-01T08:36:30.7398824Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-01T08:36:32.3261776Z ...iiii.....ii.
2020-04-01T08:36:32.3263515Z 
2020-04-01T08:36:32.3314258Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-01T08:36:32.3315048Z  finished in 14.169
2020-04-01T08:36:32.3315800Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-01T08:47:39.8550335Z 
2020-04-01T08:47:39.8551157Z    Doc-tests core
2020-04-01T08:47:43.9485720Z 
2020-04-01T08:47:43.9493912Z running 2489 tests
2020-04-01T08:47:52.0177574Z ......iiiii......................................................................................... 100/2489
2020-04-01T08:48:00.0366594Z .....................................................................................ii............. 200/2489
2020-04-01T08:48:18.1171442Z ....................i............................................................................... 400/2489
2020-04-01T08:48:18.1171442Z ....................i............................................................................... 400/2489
2020-04-01T08:48:26.8008294Z ..........................................................................i..i..................iiii 500/2489
2020-04-01T08:48:41.1072609Z .................................................................................................... 700/2489
2020-04-01T08:48:48.5190888Z .................................................................................................... 800/2489
2020-04-01T08:48:56.0826219Z .................................................................................................... 900/2489
2020-04-01T08:49:03.4952170Z .................................................................................................... 1000/2489
---
2020-04-01T08:52:17.0841930Z 
2020-04-01T08:52:17.0842238Z running 1018 tests
2020-04-01T08:52:32.4731889Z i................................................................................................... 100/1018
2020-04-01T08:52:41.4272859Z .................................................................................................... 200/1018
2020-04-01T08:52:48.0789833Z ..................iii......i......i...i......i...................................................... 300/1018
2020-04-01T08:52:58.3181458Z ..................................................i....i......................................ii.... 500/1018
2020-04-01T08:53:04.9961286Z .................................................................................................... 600/1018
2020-04-01T08:53:09.3400264Z .................................................................................................... 700/1018
2020-04-01T08:53:09.3400264Z .................................................................................................... 700/1018
2020-04-01T08:53:15.6020455Z ............................................iiii.................................................... 800/1018
2020-04-01T08:53:28.0263881Z .................................................................................................... 900/1018
2020-04-01T08:53:33.5415346Z ..................................................................iiii.............................. 1000/1018
2020-04-01T08:53:34.4582759Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-01T08:53:34.4583065Z 
2020-04-01T08:53:34.4689364Z  finished in 145.881
2020-04-01T08:53:34.4694158Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-01T08:56:25.6180616Z 
2020-04-01T08:56:25.6180925Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-01T08:56:25.6181220Z 
2020-04-01T08:56:25.6234295Z  finished in 0.884
2020-04-01T08:56:25.6238131Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-01T08:56:25.6250627Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-01T08:56:26.2683418Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T08:56:26.6841095Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-aa05ddd74004cf7f
2020-04-01T08:56:26.6867660Z 
2020-04-01T08:56:26.6868057Z running 0 tests
2020-04-01T08:56:26.6868281Z 
---
2020-04-01T09:09:33.6309148Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6309843Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6310570Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6311287Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6312000Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6313443Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6314152Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6314865Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-01T09:09:33.6315580Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-01T09:10:30.5429894Z 
2020-04-01T09:10:30.5430002Z failures:
2020-04-01T09:10:30.5430083Z 
2020-04-01T09:10:30.5430506Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13362) stdout ----
2020-04-01T09:10:30.5430876Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found keyword `move`
2020-04-01T09:10:30.5431523Z   |
2020-04-01T09:10:30.5431523Z   |
2020-04-01T09:10:30.5431684Z 5 |     let add_one = async move |num: u8| { // ok!
2020-04-01T09:10:30.5431937Z   |                         ^^^^ expected one of 7 possible tokens
2020-04-01T09:10:30.5432214Z error: aborting due to previous error
2020-04-01T09:10:30.5432328Z 
2020-04-01T09:10:30.5432561Z Couldn't compile the test.
2020-04-01T09:10:30.5433016Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13349) stdout ----
2020-04-01T09:10:30.5433016Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13349) stdout ----
2020-04-01T09:10:30.5433332Z error[E0425]: cannot find value `async` in this scope
2020-04-01T09:10:30.5433927Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13353:19
2020-04-01T09:10:30.5434142Z   |
2020-04-01T09:10:30.5434324Z 5 |     let add_one = async |num: u8| { // error!
2020-04-01T09:10:30.5434734Z 
2020-04-01T09:10:30.5434904Z error[E0425]: cannot find value `num` in this scope
2020-04-01T09:10:30.5435368Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13353:26
2020-04-01T09:10:30.5435642Z   |
2020-04-01T09:10:30.5435642Z   |
2020-04-01T09:10:30.5435993Z 5 |     let add_one = async |num: u8| { // error!
2020-04-01T09:10:30.5436509Z   |                          |
2020-04-01T09:10:30.5436747Z   |                          not found in this scope
2020-04-01T09:10:30.5437072Z   |                          expecting a type here because of type ascription
2020-04-01T09:10:30.5437283Z 
---
2020-04-01T09:10:30.5438814Z 
2020-04-01T09:10:30.5438989Z error: aborting due to 3 previous errors
2020-04-01T09:10:30.5439148Z 
2020-04-01T09:10:30.5439547Z For more information about this error, try `rustc --explain E0425`.
2020-04-01T09:10:30.5439884Z Some expected error codes were not found: ["E0708"]
2020-04-01T09:10:30.5440194Z failures:
2020-04-01T09:10:30.5440894Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13349)
2020-04-01T09:10:30.5441883Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0708 (line 13362)
2020-04-01T09:10:30.5442204Z 
---
2020-04-01T09:10:30.5443433Z 
2020-04-01T09:10:30.5451609Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T09:10:30.5452427Z Build completed unsuccessfully in 1:32:01
2020-04-01T09:10:30.5452861Z == clock drift check ==
2020-04-01T09:10:30.5453191Z   local time: Wed Apr  1 09:10:30 UTC 2020
2020-04-01T09:10:30.6519160Z   network time: Wed, 01 Apr 2020 09:10:30 GMT
2020-04-01T09:10:31.1211891Z 
2020-04-01T09:10:31.1211891Z 
2020-04-01T09:10:31.1267082Z ##[error]Bash exited with code '1'.
2020-04-01T09:10:31.1281121Z ##[section]Finishing: Run build
2020-04-01T09:10:31.1330609Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T09:10:31.1335788Z Task         : Get sources
2020-04-01T09:10:31.1336026Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T09:10:31.1336260Z Version      : 1.0.0
2020-04-01T09:10:31.1336414Z Author       : Microsoft
2020-04-01T09:10:31.1336414Z Author       : Microsoft
2020-04-01T09:10:31.1336659Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T09:10:31.1336964Z ==============================================================================
2020-04-01T09:10:31.4436977Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T09:10:31.4500852Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70611/merge to s
2020-04-01T09:10:31.4576707Z Cleaning up task key
2020-04-01T09:10:31.4577844Z Start cleaning up orphan processes.
2020-04-01T09:10:31.4752712Z Terminate orphan process: pid (3487) (python)
2020-04-01T09:10:31.4953571Z ##[section]Finishing: Finalize Job
