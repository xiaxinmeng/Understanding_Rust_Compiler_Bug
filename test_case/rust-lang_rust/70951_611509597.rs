plain
2020-04-09T11:14:16.2515150Z ========================== Starting Command Output ===========================
2020-04-09T11:14:16.2521043Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e0ed475d-ae9c-4baa-baa5-0a1ab9787d1f.sh
2020-04-09T11:14:16.2521572Z 
2020-04-09T11:14:16.2525738Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T11:14:16.2541788Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70951/merge to s
2020-04-09T11:14:16.2545038Z Task         : Get sources
2020-04-09T11:14:16.2545277Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T11:14:16.2545561Z Version      : 1.0.0
2020-04-09T11:14:16.2545718Z Author       : Microsoft
---
2020-04-09T11:14:17.6943329Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T11:14:17.6951184Z ##[command]git config gc.auto 0
2020-04-09T11:14:17.6956111Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T11:14:17.6960134Z ##[command]git config --get-all http.proxy
2020-04-09T11:14:17.6968908Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70951/merge:refs/remotes/pull/70951/merge
---
2020-04-09T11:17:30.5619206Z Looks like docker image is the same as before, not uploading
2020-04-09T11:17:37.9402204Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T11:17:37.9730092Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T11:17:37.9754643Z == clock drift check ==
2020-04-09T11:17:37.9763274Z   local time: Thu Apr  9 11:17:37 UTC 2020
2020-04-09T11:17:38.0071927Z   network time: Thu, 09 Apr 2020 11:17:38 GMT
2020-04-09T11:17:38.0093855Z Starting sccache server...
2020-04-09T11:17:38.0924925Z configure: processing command line
2020-04-09T11:17:38.0925677Z configure: 
2020-04-09T11:17:38.0926644Z configure: rust.dist-src        := False
---
2020-04-09T11:22:25.0351593Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T11:22:26.5119845Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T11:22:28.0565236Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T11:22:29.9412656Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T11:22:37.4414157Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T11:22:40.8331679Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T11:22:44.9935010Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T11:22:48.8444668Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T11:22:56.7221321Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T11:40:43.8760206Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T11:40:45.4758654Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T11:40:47.2887338Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T11:40:49.5512841Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T11:40:59.0803127Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T11:41:02.7107331Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T11:41:07.6359279Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T11:41:12.7119532Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T11:41:21.9777454Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T12:01:37.2155944Z .................................................................................................... 1700/9879
2020-04-09T12:01:40.8878646Z .................................................................................................... 1800/9879
2020-04-09T12:01:49.0196755Z ..................................................................................................i. 1900/9879
2020-04-09T12:01:56.5353403Z .................................................................................................... 2000/9879
2020-04-09T12:02:02.5455562Z ........................................................................................iiiii....... 2100/9879
2020-04-09T12:02:23.2501260Z .................................................................................................... 2300/9879
2020-04-09T12:02:25.1657947Z .................................................................................................... 2400/9879
2020-04-09T12:02:27.3480825Z .................................................................................................... 2500/9879
2020-04-09T12:02:33.0898795Z .................................................................................................... 2600/9879
---
2020-04-09T12:05:21.2523207Z ..............................................................i...............i..................... 5000/9879
2020-04-09T12:05:27.6536288Z .................................................................................................... 5100/9879
2020-04-09T12:05:34.5714912Z .................................................................................................... 5200/9879
2020-04-09T12:05:39.4171953Z .......i............................................................................................ 5300/9879
2020-04-09T12:05:48.9979146Z ................................................................................................ii.i 5400/9879
2020-04-09T12:05:53.6475678Z i........i...i...................................................................................... 5500/9879
2020-04-09T12:06:01.6933917Z .........................................i.......................................................... 5700/9879
2020-04-09T12:06:10.8922800Z .............................................................ii..................................... 5800/9879
2020-04-09T12:06:17.8782743Z i................................................................................................... 5900/9879
2020-04-09T12:06:22.8891063Z .................................................................................................... 6000/9879
2020-04-09T12:06:22.8891063Z .................................................................................................... 6000/9879
2020-04-09T12:06:31.9688848Z ..............................................................................................ii...i 6100/9879
2020-04-09T12:06:43.2086658Z ..ii...........i.................................................................................... 6200/9879
2020-04-09T12:06:58.0479432Z .................................................................................................... 6400/9879
2020-04-09T12:07:03.6576587Z .................................................................................................... 6500/9879
2020-04-09T12:07:03.6576587Z .................................................................................................... 6500/9879
2020-04-09T12:07:18.5401703Z ........................i..ii....................................................................... 6600/9879
2020-04-09T12:07:39.2231619Z .................................................................................................... 6800/9879
2020-04-09T12:07:41.2827494Z ........................i........................................................................... 6900/9879
2020-04-09T12:07:43.3995489Z .................................................................................................... 7000/9879
2020-04-09T12:07:45.5770244Z ...............................................................i.................................... 7100/9879
---
2020-04-09T12:09:17.9300517Z .................................................................................................... 7800/9879
2020-04-09T12:09:22.1056339Z .................................................................................................... 7900/9879
2020-04-09T12:09:27.7963662Z .................................................................................................... 8000/9879
2020-04-09T12:09:35.1806134Z ............................i....................................................................... 8100/9879
2020-04-09T12:09:42.9379453Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-09T12:09:57.5965878Z .....................i......i....................................................................... 8400/9879
2020-04-09T12:10:01.9206072Z .................................................................................................... 8500/9879
2020-04-09T12:10:11.9025104Z .................................................................................................... 8600/9879
2020-04-09T12:10:23.0433625Z .................................................................................................... 8700/9879
---
2020-04-09T12:12:33.9407490Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-09T12:12:33.9616787Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-09T12:12:34.1784194Z 
2020-04-09T12:12:34.1784967Z running 185 tests
2020-04-09T12:12:36.5564918Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-09T12:12:39.0197685Z .......i.i.i...iii..iiiiiiiiiiiiiiii........................iii..............ii......
2020-04-09T12:12:39.0201795Z 
2020-04-09T12:12:39.0208826Z  finished in 5.059
2020-04-09T12:12:39.0227218Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-09T12:12:39.0416613Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-09T12:12:41.0701777Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-09T12:12:41.0897117Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-09T12:12:41.2300312Z 
2020-04-09T12:12:41.2300621Z running 9 tests
2020-04-09T12:12:41.2302584Z iiiiiiiii
2020-04-09T12:12:41.2303382Z 
2020-04-09T12:12:41.2303522Z  finished in 0.140
2020-04-09T12:12:41.2304226Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-09T12:12:41.2467108Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-09T12:12:59.9681395Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-09T12:12:59.9955369Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-09T12:13:00.1801502Z 
2020-04-09T12:13:00.1802215Z running 115 tests
2020-04-09T12:13:13.9471962Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-09T12:13:15.8477365Z ...iiii.....ii.
2020-04-09T12:13:15.8479015Z 
2020-04-09T12:13:15.8479199Z  finished in 15.852
2020-04-09T12:13:15.8535181Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-09T12:13:15.8535898Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-09T12:24:47.2109950Z 
2020-04-09T12:24:47.2110953Z    Doc-tests core
2020-04-09T12:24:51.5282569Z 
2020-04-09T12:24:51.5283572Z running 2490 tests
2020-04-09T12:24:59.8942490Z ......iiiii......................................................................................... 100/2490
2020-04-09T12:25:07.9490412Z .....................................................................................ii............. 200/2490
2020-04-09T12:25:26.7288412Z ....................i............................................................................... 400/2490
2020-04-09T12:25:26.7288412Z ....................i............................................................................... 400/2490
2020-04-09T12:25:35.7247108Z ..........................................................................i..i..................iiii 500/2490
2020-04-09T12:25:50.3299882Z .................................................................................................... 700/2490
2020-04-09T12:25:57.7453999Z .................................................................................................... 800/2490
2020-04-09T12:26:05.1467046Z .................................................................................................... 900/2490
2020-04-09T12:26:12.6711187Z .................................................................................................... 1000/2490
---
2020-04-09T12:29:29.4456347Z 
2020-04-09T12:29:29.4457409Z running 1019 tests
2020-04-09T12:29:45.8720146Z i................................................................................................... 100/1019
2020-04-09T12:29:54.8851758Z .................................................................................................... 200/1019
2020-04-09T12:30:01.6575162Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-09T12:30:12.1614615Z ...................................................i....i......................................ii... 500/1019
2020-04-09T12:30:18.9622458Z .................................................................................................... 600/1019
2020-04-09T12:30:23.3503669Z .................................................................................................... 700/1019
2020-04-09T12:30:23.3503669Z .................................................................................................... 700/1019
2020-04-09T12:30:29.5502263Z .............................................iiii................................................... 800/1019
2020-04-09T12:30:43.5403577Z .................................................................................................... 900/1019
2020-04-09T12:30:48.1439834Z ...................................................................iiii............................. 1000/1019
2020-04-09T12:30:49.3021398Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-09T12:30:49.3021592Z 
2020-04-09T12:30:49.3112058Z  finished in 151.423
2020-04-09T12:30:49.3120077Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-09T12:33:48.8265037Z 
2020-04-09T12:33:48.8265259Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-09T12:33:48.8265463Z 
2020-04-09T12:33:48.8315887Z  finished in 0.909
2020-04-09T12:33:48.8321986Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-09T12:33:48.8350221Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-09T12:33:49.0170226Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T12:33:49.9834045Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-dc8c35248385a45c
2020-04-09T12:33:49.9858840Z 
2020-04-09T12:33:49.9859042Z running 0 tests
2020-04-09T12:33:49.9859151Z 
---
2020-04-09T12:47:24.2198286Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2204511Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2205460Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2206737Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2207699Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2209048Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2209690Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2210343Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-09T12:47:24.2211151Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-09T12:48:22.2785292Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-09T12:48:22.3090884Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-09T12:48:22.5067561Z 
2020-04-09T12:48:22.5068084Z running 210 tests
2020-04-09T12:48:50.3094755Z ......................i...ii.................................F.....................................i 100/210
2020-04-09T12:49:26.3486631Z ........................................iiiiii......i..............iii.............................. 200/210
2020-04-09T12:49:27.4149990Z failures:
2020-04-09T12:49:27.4150095Z 
2020-04-09T12:49:27.4150529Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-04-09T12:49:27.4150719Z 
2020-04-09T12:49:27.4150719Z 
2020-04-09T12:49:27.4150847Z error: make failed
2020-04-09T12:49:27.4151013Z status: exit code: 2
2020-04-09T12:49:27.4151189Z command: "make"
2020-04-09T12:49:27.4151324Z stdout:
2020-04-09T12:49:27.4151652Z ------------------------------------------
2020-04-09T12:49:27.4151920Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-04-09T12:49:27.4152115Z 
2020-04-09T12:49:27.4153951Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-04-09T12:49:27.4155569Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-04-09T12:49:27.4156103Z Makefile:6: recipe for target 'all' failed
2020-04-09T12:49:27.4156571Z ------------------------------------------
2020-04-09T12:49:27.4156743Z stderr:
2020-04-09T12:49:27.4157074Z ------------------------------------------
2020-04-09T12:49:27.4157074Z ------------------------------------------
2020-04-09T12:49:27.4157457Z warning: ignoring --out-dir flag due to -o flag
2020-04-09T12:49:27.4158239Z error[E0053]: method `codegen_crate` has an incompatible type for trait
2020-04-09T12:49:27.4158703Z   --> the_backend.rs:67:5
2020-04-09T12:49:27.4158864Z    |
2020-04-09T12:49:27.4159196Z 67 | /     fn codegen_crate<'a, 'tcx>(
2020-04-09T12:49:27.4159196Z 67 | /     fn codegen_crate<'a, 'tcx>(
2020-04-09T12:49:27.4159391Z 68 | |         &self,
2020-04-09T12:49:27.4159732Z 69 | |         tcx: TyCtxt<'tcx>,
2020-04-09T12:49:27.4159971Z 70 | |         _metadata: EncodedMetadata,
2020-04-09T12:49:27.4160171Z ...  |
2020-04-09T12:49:27.4160386Z 75 | |         Box::new(tcx.crate_name(LOCAL_CRATE) as Symbol)
2020-04-09T12:49:27.4160608Z 76 | |     }
2020-04-09T12:49:27.4160974Z    | |_____^ expected struct `rustc_incremental::query::plumbing::QueryCtxt`, found struct `rustc_middle::ty::TyCtxt`
2020-04-09T12:49:27.4161302Z    |
2020-04-09T12:49:27.4162194Z    = note: expected fn pointer `fn(&TheBackend, rustc_incremental::query::plumbing::QueryCtxt<'tcx>, rustc_middle::middle::cstore::EncodedMetadata, _) -> std::boxed::Box<_>`
2020-04-09T12:49:27.4163339Z               found fn pointer `fn(&TheBackend, rustc_middle::ty::TyCtxt<'_>, rustc_middle::middle::cstore::EncodedMetadata, _) -> std::boxed::Box<_>`
2020-04-09T12:49:27.4164098Z error: aborting due to previous error
2020-04-09T12:49:27.4164261Z 
2020-04-09T12:49:27.4164679Z For more information about this error, try `rustc --explain E0053`.
2020-04-09T12:49:27.4164679Z For more information about this error, try `rustc --explain E0053`.
2020-04-09T12:49:27.4164923Z make: *** [all] Error 1
2020-04-09T12:49:27.4165541Z ------------------------------------------
2020-04-09T12:49:27.4165687Z 
2020-04-09T12:49:27.4165769Z 
2020-04-09T12:49:27.4165849Z 
---
2020-04-09T12:49:27.4167718Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-09T12:49:27.4168075Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-09T12:49:27.4168286Z 
2020-04-09T12:49:27.4168366Z 
2020-04-09T12:49:27.4176391Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-09T12:49:27.4181989Z 
2020-04-09T12:49:27.4182070Z 
2020-04-09T12:49:27.4182527Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T12:49:27.4182853Z Build completed unsuccessfully in 1:30:25
2020-04-09T12:49:27.4182853Z Build completed unsuccessfully in 1:30:25
2020-04-09T12:49:27.4183056Z == clock drift check ==
2020-04-09T12:49:27.4183275Z   local time: Thu Apr  9 12:49:27 UTC 2020
2020-04-09T12:49:27.4904269Z   network time: Thu, 09 Apr 2020 12:49:27 GMT
2020-04-09T12:49:28.2311682Z 
2020-04-09T12:49:28.2311682Z 
2020-04-09T12:49:28.2376797Z ##[error]Bash exited with code '1'.
2020-04-09T12:49:28.2392065Z ##[section]Finishing: Run build
2020-04-09T12:49:28.2439231Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70951/merge to s
2020-04-09T12:49:28.2443832Z Task         : Get sources
2020-04-09T12:49:28.2444103Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T12:49:28.2444364Z Version      : 1.0.0
2020-04-09T12:49:28.2444536Z Author       : Microsoft
2020-04-09T12:49:28.2444536Z Author       : Microsoft
2020-04-09T12:49:28.2444810Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T12:49:28.2445144Z ==============================================================================
2020-04-09T12:49:28.5520017Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T12:49:28.5559272Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70951/merge to s
2020-04-09T12:49:28.5646322Z Cleaning up task key
2020-04-09T12:49:28.5647566Z Start cleaning up orphan processes.
2020-04-09T12:49:28.5819729Z Terminate orphan process: pid (3777) (python)
2020-04-09T12:49:28.6058935Z ##[section]Finishing: Finalize Job
