plain
2020-03-31T21:03:46.3834022Z ========================== Starting Command Output ===========================
2020-03-31T21:03:46.3836898Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0c1fa194-215f-4287-bedb-bbfa8d836732.sh
2020-03-31T21:03:46.3837195Z 
2020-03-31T21:03:46.3840961Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T21:03:46.3859603Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70634/merge to s
2020-03-31T21:03:46.3862802Z Task         : Get sources
2020-03-31T21:03:46.3863113Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T21:03:46.3863427Z Version      : 1.0.0
2020-03-31T21:03:46.3863631Z Author       : Microsoft
---
2020-03-31T21:03:47.6747151Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T21:03:47.6754808Z ##[command]git config gc.auto 0
2020-03-31T21:03:47.6760123Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T21:03:47.6764949Z ##[command]git config --get-all http.proxy
2020-03-31T21:03:47.6774002Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70634/merge:refs/remotes/pull/70634/merge
---
2020-03-31T21:11:14.5638764Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:11:15.8913769Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:11:17.3916596Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:11:17.4084373Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:11:26.4368333Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:11:27.9130266Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:11:31.9370501Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:11:35.7575807Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:11:44.8076595Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T21:32:11.9697580Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:32:13.6101883Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:32:15.5649222Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:32:16.6430333Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:32:27.0730086Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:32:29.4002849Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:32:34.3534269Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:32:39.4308887Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:32:50.1680674Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T21:56:34.9276187Z .................................................................................................... 1700/9861
2020-03-31T21:56:38.6246140Z .................................................................................................... 1800/9861
2020-03-31T21:56:46.8162879Z ..............................................................................................i..... 1900/9861
2020-03-31T21:56:54.0914325Z .................................................................................................... 2000/9861
2020-03-31T21:57:00.0306863Z ....................................................................................iiiii........... 2100/9861
2020-03-31T21:57:19.6130971Z .................................................................................................... 2300/9861
2020-03-31T21:57:21.6181225Z .................................................................................................... 2400/9861
2020-03-31T21:57:23.7266444Z .................................................................................................... 2500/9861
2020-03-31T21:57:29.3376015Z .................................................................................................... 2600/9861
---
2020-03-31T22:00:09.2179401Z ..........................................................i...............i......................... 5000/9861
2020-03-31T22:00:16.2951953Z .................................................................................................... 5100/9861
2020-03-31T22:00:23.3886078Z .................................................................................................... 5200/9861
2020-03-31T22:00:27.9724166Z ...i................................................................................................ 5300/9861
2020-03-31T22:00:37.5574012Z .........................................................................................ii.ii...... 5400/9861
2020-03-31T22:00:41.3885298Z ..i...i............................................................................................. 5500/9861
2020-03-31T22:00:49.6045698Z ..................................i................................................................. 5700/9861
2020-03-31T22:00:58.7934081Z ....................................................ii....................................i......... 5800/9861
2020-03-31T22:01:05.8283142Z .................................................................................................... 5900/9861
2020-03-31T22:01:10.3349725Z .................................................................................................... 6000/9861
2020-03-31T22:01:10.3349725Z .................................................................................................... 6000/9861
2020-03-31T22:01:19.0829683Z ....................................................................................ii...i..ii...... 6100/9861
2020-03-31T22:01:38.5286121Z .................................................................................................... 6300/9861
2020-03-31T22:01:43.2048072Z .................................................................................................... 6400/9861
2020-03-31T22:01:46.0621467Z .................................................................................................... 6500/9861
2020-03-31T22:01:46.0621467Z .................................................................................................... 6500/9861
2020-03-31T22:01:57.8118960Z ..............i..ii................................................................................. 6600/9861
2020-03-31T22:02:16.7692100Z .................................................................................................... 6800/9861
2020-03-31T22:02:18.6580999Z ..............i..................................................................................... 6900/9861
2020-03-31T22:02:20.5810683Z .................................................................................................... 7000/9861
2020-03-31T22:02:22.5877425Z ....................................................i............................................... 7100/9861
---
2020-03-31T22:03:55.2526704Z .................................................................................................... 7800/9861
2020-03-31T22:03:59.9474749Z .................................................................................................... 7900/9861
2020-03-31T22:04:05.0707072Z .................................................................................................... 8000/9861
2020-03-31T22:04:12.7948689Z ............i....................................................................................... 8100/9861
2020-03-31T22:04:20.3390962Z .............................................................iiiiiiiiii.i........................... 8200/9861
2020-03-31T22:04:33.9157884Z .....i......i....................................................................................... 8400/9861
2020-03-31T22:04:38.4638989Z .................................................................................................... 8500/9861
2020-03-31T22:04:49.4827149Z .................................................................................................... 8600/9861
2020-03-31T22:04:59.4467759Z .................................................................................................... 8700/9861
---
2020-03-31T22:07:11.8161486Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-31T22:07:11.8345397Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T22:07:12.0356577Z 
2020-03-31T22:07:12.0356904Z running 183 tests
2020-03-31T22:07:14.7924833Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-31T22:07:17.2492327Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-31T22:07:17.2495981Z 
2020-03-31T22:07:17.2500381Z  finished in 5.415
2020-03-31T22:07:17.2506369Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-31T22:07:17.2689351Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T22:07:19.4148311Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-31T22:07:19.4336313Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T22:07:19.5828254Z 
2020-03-31T22:07:19.5828758Z running 9 tests
2020-03-31T22:07:19.5830387Z iiiiiiiii
2020-03-31T22:07:19.5832446Z 
2020-03-31T22:07:19.5832719Z  finished in 0.148
2020-03-31T22:07:19.5833908Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-31T22:07:19.6044236Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T22:07:39.0637968Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-31T22:07:39.0851322Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T22:07:39.2639575Z 
2020-03-31T22:07:39.2640673Z running 115 tests
2020-03-31T22:07:52.3879923Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-31T22:07:54.0981009Z ...iiii.....ii.
2020-03-31T22:07:54.0982423Z 
2020-03-31T22:07:54.0984416Z  finished in 15.013
2020-03-31T22:07:54.0993137Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-31T22:07:54.0997936Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-31T22:19:25.2264265Z 
2020-03-31T22:19:25.2265600Z    Doc-tests core
2020-03-31T22:19:29.4898771Z 
2020-03-31T22:19:29.4899280Z running 2489 tests
2020-03-31T22:19:37.9209507Z ......iiiii......................................................................................... 100/2489
2020-03-31T22:19:46.0946875Z .....................................................................................ii............. 200/2489
2020-03-31T22:20:04.9578996Z ....................i............................................................................... 400/2489
2020-03-31T22:20:04.9578996Z ....................i............................................................................... 400/2489
2020-03-31T22:20:13.9918134Z ..........................................................................i..i..................iiii 500/2489
2020-03-31T22:20:29.0479391Z .................................................................................................... 700/2489
2020-03-31T22:20:36.9330604Z .................................................................................................... 800/2489
2020-03-31T22:20:44.7787324Z .................................................................................................... 900/2489
2020-03-31T22:20:52.5846821Z .................................................................................................... 1000/2489
---
2020-03-31T22:24:00.3188413Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
2020-03-31T22:24:00.3189059Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:603:13
2020-03-31T22:24:00.3189673Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:616:13
2020-03-31T22:24:02.3603608Z ................................................ 700/760
2020-03-31T22:24:02.3690132Z .................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-03-31T22:24:02.9767548Z ...........thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-03-31T22:24:02.9777721Z thread '.<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1692:13
2020-03-31T22:24:02.9786908Z .thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
2020-03-31T22:24:02.9799275Z ..thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-03-31T22:24:08.4874631Z test result: ok. 760 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-31T22:24:08.4875252Z 
2020-03-31T22:24:08.4883456Z      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/env-74d10a54eae8cc86
2020-03-31T22:24:08.4897358Z 
---
2020-03-31T22:24:09.2168607Z 
2020-03-31T22:24:09.2170072Z running 1018 tests
2020-03-31T22:24:25.4547822Z i................................................................................................... 100/1018
2020-03-31T22:24:35.1505633Z .................................................................................................... 200/1018
2020-03-31T22:24:42.3979948Z ..................iii......i......i...i......i...................................................... 300/1018
2020-03-31T22:24:53.3414669Z ..................................................i....i......................................ii.... 500/1018
2020-03-31T22:25:00.7012369Z .................................................................................................... 600/1018
2020-03-31T22:25:05.3536920Z .................................................................................................... 700/1018
2020-03-31T22:25:05.3536920Z .................................................................................................... 700/1018
2020-03-31T22:25:12.0110021Z ............................................iiii.................................................... 800/1018
2020-03-31T22:25:24.9109961Z .................................................................................................... 900/1018
2020-03-31T22:25:30.8698292Z ..................................................................iiii.............................. 1000/1018
2020-03-31T22:25:31.8333475Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-31T22:25:31.8333755Z 
2020-03-31T22:25:31.8429443Z  finished in 154.477
2020-03-31T22:25:31.8434007Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-31T22:28:26.5922447Z 
2020-03-31T22:28:26.5922914Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-31T22:28:26.5923306Z 
2020-03-31T22:28:26.5978166Z  finished in 0.961
2020-03-31T22:28:26.5979279Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-31T22:28:26.5993400Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T22:28:26.7732099Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T22:28:27.6963966Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-55cc5925cd6aa0b6
2020-03-31T22:28:27.6995744Z 
2020-03-31T22:28:27.6995982Z running 0 tests
2020-03-31T22:28:27.6996112Z 
---
2020-03-31T22:42:00.5986120Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5986874Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5987662Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5988434Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5989218Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5990790Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5991581Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5992336Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-31T22:42:00.5993116Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-03-31T22:42:59.2832583Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-03-31T22:42:59.3109295Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-31T22:42:59.5142286Z 
2020-03-31T22:42:59.5143087Z running 209 tests
2020-03-31T22:43:27.8879271Z ......................i...ii...................................F...................................i 100/209
2020-03-31T22:43:59.6581744Z .......................................iiiiii......i..............iii............................... 200/209
2020-03-31T22:44:06.2061163Z failures:
2020-03-31T22:44:06.2063651Z 
2020-03-31T22:44:06.2067255Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-31T22:44:06.2068425Z 
2020-03-31T22:44:06.2068425Z 
2020-03-31T22:44:06.2069059Z error: make failed
2020-03-31T22:44:06.2069743Z status: exit code: 2
2020-03-31T22:44:06.2070133Z command: "make"
2020-03-31T22:44:06.2070497Z stdout:
2020-03-31T22:44:06.2073924Z ------------------------------------------
2020-03-31T22:44:06.2075596Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-31T22:44:06.2076512Z 
2020-03-31T22:44:06.2079871Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-31T22:44:06.2088616Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-31T22:44:06.2089889Z Makefile:6: recipe for target 'all' failed
2020-03-31T22:44:06.2090727Z ------------------------------------------
2020-03-31T22:44:06.2091076Z stderr:
2020-03-31T22:44:06.2091580Z ------------------------------------------
2020-03-31T22:44:06.2092070Z error[E0432]: unresolved import `rustc_middle::util::common::ErrorReported`
2020-03-31T22:44:06.2092070Z error[E0432]: unresolved import `rustc_middle::util::common::ErrorReported`
2020-03-31T22:44:06.2092709Z   --> the_backend.rs:21:5
2020-03-31T22:44:06.2093019Z    |
2020-03-31T22:44:06.2093364Z 21 | use rustc_middle::util::common::ErrorReported;
2020-03-31T22:44:06.2093905Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `ErrorReported` in `util::common`
2020-03-31T22:44:06.2094306Z 
2020-03-31T22:44:06.2094820Z warning: ignoring --out-dir flag due to -o flag
2020-03-31T22:44:06.2116301Z error: aborting due to previous error
2020-03-31T22:44:06.2116703Z 
2020-03-31T22:44:06.2117569Z For more information about this error, try `rustc --explain E0432`.
2020-03-31T22:44:06.2117569Z For more information about this error, try `rustc --explain E0432`.
2020-03-31T22:44:06.2118723Z make: *** [all] Error 1
2020-03-31T22:44:06.2119584Z ------------------------------------------
2020-03-31T22:44:06.2119905Z 
2020-03-31T22:44:06.2120112Z 
2020-03-31T22:44:06.2120398Z 
---
2020-03-31T22:44:06.2123210Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-31T22:44:06.2123927Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T22:44:06.2124274Z 
2020-03-31T22:44:06.2124481Z 
2020-03-31T22:44:06.2134318Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-31T22:44:06.2141694Z 
2020-03-31T22:44:06.2141793Z 
2020-03-31T22:44:06.2142373Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-31T22:44:06.2142762Z Build completed unsuccessfully in 1:36:07
2020-03-31T22:44:06.2142762Z Build completed unsuccessfully in 1:36:07
2020-03-31T22:44:06.2187676Z == clock drift check ==
2020-03-31T22:44:06.2206004Z   local time: Tue Mar 31 22:44:06 UTC 2020
2020-03-31T22:44:06.5172779Z   network time: Tue, 31 Mar 2020 22:44:06 GMT
2020-03-31T22:44:06.5180520Z == end clock drift check ==
2020-03-31T22:44:08.9489255Z 
2020-03-31T22:44:08.9569903Z ##[error]Bash exited with code '1'.
2020-03-31T22:44:08.9583669Z ##[section]Finishing: Run build
2020-03-31T22:44:08.9631407Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70634/merge to s
2020-03-31T22:44:08.9636744Z Task         : Get sources
2020-03-31T22:44:08.9637097Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T22:44:08.9637415Z Version      : 1.0.0
2020-03-31T22:44:08.9637655Z Author       : Microsoft
2020-03-31T22:44:08.9637655Z Author       : Microsoft
2020-03-31T22:44:08.9638008Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T22:44:08.9638418Z ==============================================================================
2020-03-31T22:44:09.2872935Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T22:44:09.2935307Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70634/merge to s
2020-03-31T22:44:09.3018209Z Cleaning up task key
2020-03-31T22:44:09.3019490Z Start cleaning up orphan processes.
2020-03-31T22:44:09.3201628Z Terminate orphan process: pid (3471) (python)
2020-03-31T22:44:09.3464274Z ##[section]Finishing: Finalize Job
