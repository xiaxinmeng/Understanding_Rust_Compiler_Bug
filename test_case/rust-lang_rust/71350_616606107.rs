plain
2020-04-20T13:03:01.5556394Z ========================== Starting Command Output ===========================
2020-04-20T13:03:01.5560542Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/72d230ea-d196-4545-ae10-95754dcfcf13.sh
2020-04-20T13:03:01.5561020Z 
2020-04-20T13:03:01.5578181Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T13:03:01.5598784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71350/merge to s
2020-04-20T13:03:01.5602551Z Task         : Get sources
2020-04-20T13:03:01.5602892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T13:03:01.5603160Z Version      : 1.0.0
2020-04-20T13:03:01.5603346Z Author       : Microsoft
---
2020-04-20T13:03:04.3128573Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T13:03:04.3141116Z ##[command]git config gc.auto 0
2020-04-20T13:03:04.3167860Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T13:03:04.3172618Z ##[command]git config --get-all http.proxy
2020-04-20T13:03:04.3185705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71350/merge:refs/remotes/pull/71350/merge
---
2020-04-20T13:05:29.7095996Z  ---> 318032b5f0e2
2020-04-20T13:05:29.7096741Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-20T13:05:29.7097335Z  ---> Using cache
2020-04-20T13:05:29.7097657Z  ---> d44a858fd1ce
2020-04-20T13:05:29.7098573Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-20T13:05:29.7099614Z  ---> 58b910f50f5a
2020-04-20T13:05:29.7099819Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-20T13:05:29.7100178Z  ---> Using cache
2020-04-20T13:05:29.7100497Z  ---> ee7702aadba1
---
2020-04-20T13:05:29.7486220Z Looks like docker image is the same as before, not uploading
2020-04-20T13:05:37.6863355Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T13:05:37.7144291Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T13:05:37.7175019Z == clock drift check ==
2020-04-20T13:05:37.7185449Z   local time: Mon Apr 20 13:05:37 UTC 2020
2020-04-20T13:05:37.7958005Z   network time: Mon, 20 Apr 2020 13:05:37 GMT
2020-04-20T13:05:37.7985775Z Starting sccache server...
2020-04-20T13:05:37.8881342Z configure: processing command line
2020-04-20T13:05:37.8882103Z configure: 
2020-04-20T13:05:37.8883164Z configure: rust.dist-src        := False
---
2020-04-20T13:11:25.4627436Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T13:11:27.1476776Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T13:11:28.8871559Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T13:11:31.1673543Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T13:11:40.0586947Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T13:11:43.9719010Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T13:11:48.8999692Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T13:11:53.5905090Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T13:12:02.9212979Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T13:36:33.6475862Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T13:36:35.4143477Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T13:36:37.4170745Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T13:36:38.2401613Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T13:36:48.9380148Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T13:36:52.3882554Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T13:36:57.5296269Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T13:37:02.3458252Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T13:37:13.1085479Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T14:01:59.9050858Z .................................................................................................... 1700/9908
2020-04-20T14:02:04.1434612Z .................................................................................................... 1800/9908
2020-04-20T14:02:13.0826660Z ...................................................................................................i 1900/9908
2020-04-20T14:02:21.2962527Z .................................................................................................... 2000/9908
2020-04-20T14:02:27.7103567Z .........................................................................................iiiii...... 2100/9908
2020-04-20T14:02:48.6158944Z .................................................................................................... 2300/9908
2020-04-20T14:02:50.9356935Z .................................................................................................... 2400/9908
2020-04-20T14:02:53.3106566Z .................................................................................................... 2500/9908
2020-04-20T14:02:59.8201094Z .................................................................................................... 2600/9908
---
2020-04-20T14:05:56.9867019Z .................................................................i...............i.................. 5000/9908
2020-04-20T14:06:04.6774213Z .................................................................................................... 5100/9908
2020-04-20T14:06:12.3442032Z .................................................................................................... 5200/9908
2020-04-20T14:06:17.7590215Z ...........i........................................................................................ 5300/9908
2020-04-20T14:06:28.2055999Z .i.................................................................................................. 5400/9908
2020-04-20T14:06:33.6850874Z .ii.ii........i...i................................................................................. 5500/9908
2020-04-20T14:06:41.9799472Z ................................................i................................................... 5700/9908
2020-04-20T14:06:51.6422315Z ................................................................................ii.................. 5800/9908
2020-04-20T14:06:59.2561409Z ...................i................................................................................ 5900/9908
2020-04-20T14:07:05.1281736Z .................................................................................................... 6000/9908
2020-04-20T14:07:05.1281736Z .................................................................................................... 6000/9908
2020-04-20T14:07:16.5260736Z .................................................................................................... 6100/9908
2020-04-20T14:07:27.3337147Z .............ii...i..ii............i................................................................ 6200/9908
2020-04-20T14:07:43.6003783Z .................................................................................................... 6400/9908
2020-04-20T14:07:50.7871799Z .................................................................................................... 6500/9908
2020-04-20T14:07:50.7871799Z .................................................................................................... 6500/9908
2020-04-20T14:08:05.5786686Z ...........................................i..ii.................................................... 6600/9908
2020-04-20T14:08:28.3255556Z .................................................................................................... 6800/9908
2020-04-20T14:08:30.6559942Z ............................................i....................................................... 6900/9908
2020-04-20T14:08:32.7717442Z .................................................................................................... 7000/9908
2020-04-20T14:08:35.8122348Z ....................................................................................i............... 7100/9908
---
2020-04-20T14:10:12.8981630Z .................................................................................................... 7800/9908
2020-04-20T14:10:17.6632881Z .................................................................................................... 7900/9908
2020-04-20T14:10:24.4107987Z .................................................................................................... 8000/9908
2020-04-20T14:10:30.3963043Z ..................................................i................................................. 8100/9908
2020-04-20T14:10:40.8459829Z ...................................................................................................i 8200/9908
2020-04-20T14:10:46.3783221Z iiiii.iiiii.i....................................................................................... 8300/9908
2020-04-20T14:11:00.4844532Z .................................................................................................... 8500/9908
2020-04-20T14:11:08.9322234Z .................................................................................................... 8600/9908
2020-04-20T14:11:23.3971737Z .................................................................................................... 8700/9908
2020-04-20T14:11:30.1669327Z .................................................................................................... 8800/9908
---
2020-04-20T14:13:52.0139701Z  finished in 8.183
2020-04-20T14:13:52.0344128Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-20T14:13:52.2513136Z 
2020-04-20T14:13:52.2513518Z running 186 tests
2020-04-20T14:13:55.3111069Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-20T14:13:58.0137371Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-20T14:13:58.0139940Z 
2020-04-20T14:13:58.0145268Z  finished in 5.980
2020-04-20T14:13:58.0160418Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-20T14:13:58.0364008Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-20T14:14:00.2362040Z idy"]
2020-04-20T14:14:00.2580137Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-20T14:14:00.4376219Z 
2020-04-20T14:14:00.4376895Z running 9 tests
2020-04-20T14:14:00.4378412Z iiiiiiiii
2020-04-20T14:14:00.4379606Z 
2020-04-20T14:14:00.4379910Z  finished in 0.180
2020-04-20T14:14:00.4386355Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-20T14:14:00.4663334Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-20T14:14:21.9373492Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-20T14:14:21.9619399Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-20T14:14:22.1689063Z 
2020-04-20T14:14:22.1689924Z running 115 tests
2020-04-20T14:14:36.2122170Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-20T14:14:38.1151208Z ...iiii.....ii.
2020-04-20T14:14:38.1161703Z 
2020-04-20T14:14:38.1161854Z  finished in 16.153
2020-04-20T14:14:38.1181779Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-20T14:14:38.1182472Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-20T14:27:29.9687122Z 
2020-04-20T14:27:29.9688522Z    Doc-tests core
2020-04-20T14:27:35.1798108Z 
2020-04-20T14:27:35.1799191Z running 2497 tests
2020-04-20T14:27:44.7122067Z ......iiiii......................................................................................... 100/2497
2020-04-20T14:27:53.8703622Z .....................................................................................ii............. 200/2497
2020-04-20T14:28:15.1439263Z .....................i.............................................................................. 400/2497
2020-04-20T14:28:15.1439263Z .....................i.............................................................................. 400/2497
2020-04-20T14:28:25.5803898Z ...........................................................................i..i..................iii 500/2497
2020-04-20T14:28:33.5415152Z i................................................................................................... 600/2497
2020-04-20T14:28:51.3687665Z .................................................................................................... 800/2497
2020-04-20T14:29:00.1513665Z .................................................................................................... 900/2497
2020-04-20T14:29:09.3594693Z .................................................................................................... 1000/2497
2020-04-20T14:29:18.1749589Z .................................................................................................... 1100/2497
---
2020-04-20T14:32:49.9403333Z 
2020-04-20T14:32:49.9404134Z running 1020 tests
2020-04-20T14:33:08.4312358Z i................................................................................................... 100/1020
2020-04-20T14:33:19.0818108Z .................................................................................................... 200/1020
2020-04-20T14:33:27.0145973Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-20T14:33:32.0626185Z .................................................................................................... 400/1020
2020-04-20T14:33:38.9169658Z ....................................................i....i......................................ii.. 500/1020
2020-04-20T14:33:52.1141315Z .................................................................................................... 700/1020
2020-04-20T14:33:52.1141315Z .................................................................................................... 700/1020
2020-04-20T14:33:59.3704706Z ..............................................iiii.................................................. 800/1020
2020-04-20T14:34:13.4483259Z .................................................................................................... 900/1020
2020-04-20T14:34:19.8059206Z ....................................................................iiii............................ 1000/1020
2020-04-20T14:34:21.1069868Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-20T14:34:21.1071318Z 
2020-04-20T14:34:21.1197072Z  finished in 168.732
2020-04-20T14:34:21.1202915Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-20T14:37:41.1984185Z 
2020-04-20T14:37:41.1984456Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-20T14:37:41.1984674Z 
2020-04-20T14:37:41.2048602Z  finished in 1.060
2020-04-20T14:37:41.2054050Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-20T14:37:41.2073382Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-20T14:37:41.4143983Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T14:37:42.5160338Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-7ea1b105aa9adf52
2020-04-20T14:37:42.5188259Z 
2020-04-20T14:37:42.5188605Z running 0 tests
2020-04-20T14:37:42.5188731Z 
---
2020-04-20T14:52:40.0412726Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0413773Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0416845Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0417882Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0421980Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0427817Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0429123Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0429894Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-20T14:52:40.0430657Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-20T14:53:45.6450836Z 
2020-04-20T14:53:45.6450986Z failures:
2020-04-20T14:53:45.6451097Z 
2020-04-20T14:53:45.6451691Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0230 (line 4126) stdout ----
2020-04-20T14:53:45.6452140Z error[E0277]: the trait bound `bool: main::Index<u8>` is not satisfied
2020-04-20T14:53:45.6453097Z    |
2020-04-20T14:53:45.6453097Z    |
2020-04-20T14:53:45.6453277Z 5  | fn foo<T: Index<u8>>(x: T){}
2020-04-20T14:53:45.6453989Z ...
2020-04-20T14:53:45.6453989Z ...
2020-04-20T14:53:45.6454178Z 10 | foo(true); // `bool` does not implement `Index<u8>`
2020-04-20T14:53:45.6454460Z    |     ^^^^ the type `bool` cannot be indexed by `u8`
2020-04-20T14:53:45.6454881Z    = help: the trait `main::Index<u8>` is not implemented for `bool`
2020-04-20T14:53:45.6455100Z 
2020-04-20T14:53:45.6455263Z error: aborting due to previous error
2020-04-20T14:53:45.6455407Z 
2020-04-20T14:53:45.6455407Z 
2020-04-20T14:53:45.6455793Z For more information about this error, try `rustc --explain E0277`.
2020-04-20T14:53:45.6456106Z Some expected error codes were not found: ["E0230"]
2020-04-20T14:53:45.6456746Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0231 (line 4157) stdout ----
2020-04-20T14:53:45.6457175Z error[E0277]: the trait bound `bool: main::Index<u8>` is not satisfied
2020-04-20T14:53:45.6457972Z    |
2020-04-20T14:53:45.6457972Z    |
2020-04-20T14:53:45.6458143Z 5  | fn foo<T: Index<u8>>(x: T){}
2020-04-20T14:53:45.6458842Z ...
2020-04-20T14:53:45.6458842Z ...
2020-04-20T14:53:45.6459033Z 10 | foo(true); // `bool` does not implement `Index<u8>`
2020-04-20T14:53:45.6459318Z    |     ^^^^ the type `bool` cannot be indexed by `u8`
2020-04-20T14:53:45.6459731Z    = help: the trait `main::Index<u8>` is not implemented for `bool`
2020-04-20T14:53:45.6459928Z 
2020-04-20T14:53:45.6460107Z error: aborting due to previous error
2020-04-20T14:53:45.6460250Z 
2020-04-20T14:53:45.6460250Z 
2020-04-20T14:53:45.6460640Z For more information about this error, try `rustc --explain E0277`.
2020-04-20T14:53:45.6460952Z Some expected error codes were not found: ["E0231"]
2020-04-20T14:53:45.6461593Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0232 (line 4186) stdout ----
2020-04-20T14:53:45.6462139Z error[E0277]: the trait bound `bool: main::Index<u8>` is not satisfied
2020-04-20T14:53:45.6462957Z    |
2020-04-20T14:53:45.6462957Z    |
2020-04-20T14:53:45.6463128Z 5  | fn foo<T: Index<u8>>(x: T){}
2020-04-20T14:53:45.6463838Z ...
2020-04-20T14:53:45.6463838Z ...
2020-04-20T14:53:45.6464028Z 10 | foo(true); // `bool` does not implement `Index<u8>`
2020-04-20T14:53:45.6464308Z    |     ^^^^ the type `bool` cannot be indexed by `u8`
2020-04-20T14:53:45.6464717Z    = help: the trait `main::Index<u8>` is not implemented for `bool`
2020-04-20T14:53:45.6464916Z 
2020-04-20T14:53:45.6465158Z error: aborting due to previous error
2020-04-20T14:53:45.6465304Z 
2020-04-20T14:53:45.6465304Z 
2020-04-20T14:53:45.6465689Z For more information about this error, try `rustc --explain E0277`.
2020-04-20T14:53:45.6466006Z Some expected error codes were not found: ["E0232"]
2020-04-20T14:53:45.6467015Z error[E0603]: constant `X` is private
2020-04-20T14:53:45.6467506Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5730:14
2020-04-20T14:53:45.6467821Z   |
2020-04-20T14:53:45.6467974Z 6 | pub use foo::X;
---
2020-04-20T14:53:45.6469728Z 
2020-04-20T14:53:45.6469893Z error: aborting due to previous error
2020-04-20T14:53:45.6470041Z 
2020-04-20T14:53:45.6470425Z For more information about this error, try `rustc --explain E0603`.
2020-04-20T14:53:45.6470736Z Some expected error codes were not found: ["E0364"]
2020-04-20T14:53:45.6471945Z Test compiled successfully, but it's marked `compile_fail`.
2020-04-20T14:53:45.6472597Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0727 (line 13833) stdout ----
2020-04-20T14:53:45.6472997Z error: expected identifier, found reserved keyword `yield`
2020-04-20T14:53:45.6473515Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13838:9
---
2020-04-20T14:53:45.6477262Z 
2020-04-20T14:53:45.6477422Z error: aborting due to 2 previous errors
2020-04-20T14:53:45.6477563Z 
2020-04-20T14:53:45.6477935Z For more information about this error, try `rustc --explain E0422`.
2020-04-20T14:53:45.6478236Z Some expected error codes were not found: ["E0727"]
2020-04-20T14:53:45.6478607Z failures:
2020-04-20T14:53:45.6479166Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0230 (line 4126)
2020-04-20T14:53:45.6479849Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0231 (line 4157)
2020-04-20T14:53:45.6480527Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0232 (line 4186)
---
2020-04-20T14:53:45.6484502Z 
2020-04-20T14:53:45.6485111Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-20T14:53:45.6485460Z Build completed unsuccessfully in 1:46:20
2020-04-20T14:53:45.6485769Z == clock drift check ==
2020-04-20T14:53:45.6486025Z   local time: Mon Apr 20 14:53:45 UTC 2020
2020-04-20T14:53:45.6795889Z   network time: Mon, 20 Apr 2020 14:53:45 GMT
2020-04-20T14:53:46.0897755Z 
2020-04-20T14:53:46.0897755Z 
2020-04-20T14:53:46.0991793Z ##[error]Bash exited with code '1'.
2020-04-20T14:53:46.1020243Z ##[section]Finishing: Run build
2020-04-20T14:53:46.1078893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71350/merge to s
2020-04-20T14:53:46.1084442Z Task         : Get sources
2020-04-20T14:53:46.1084959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T14:53:46.1085428Z Version      : 1.0.0
2020-04-20T14:53:46.1085654Z Author       : Microsoft
2020-04-20T14:53:46.1085654Z Author       : Microsoft
2020-04-20T14:53:46.1085984Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T14:53:46.1086356Z ==============================================================================
2020-04-20T14:53:46.4762325Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T14:53:46.4814265Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71350/merge to s
2020-04-20T14:53:46.4918372Z Cleaning up task key
2020-04-20T14:53:46.4919828Z Start cleaning up orphan processes.
2020-04-20T14:53:46.5130805Z Terminate orphan process: pid (3509) (python)
2020-04-20T14:53:46.5316725Z ##[section]Finishing: Finalize Job
