plain
2020-03-29T16:37:37.3532899Z ========================== Starting Command Output ===========================
2020-03-29T16:37:37.3534873Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cb5b07f3-e9ea-475f-a7ec-d1f32e3e7dcd.sh
2020-03-29T16:37:37.3535108Z 
2020-03-29T16:37:37.3538660Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-29T16:37:37.3558066Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70536/merge to s
2020-03-29T16:37:37.3561106Z Task         : Get sources
2020-03-29T16:37:37.3561390Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T16:37:37.3561679Z Version      : 1.0.0
2020-03-29T16:37:37.3561868Z Author       : Microsoft
---
2020-03-29T16:37:38.3490353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-29T16:37:38.3495654Z ##[command]git config gc.auto 0
2020-03-29T16:37:38.3499188Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-29T16:37:38.3502469Z ##[command]git config --get-all http.proxy
2020-03-29T16:37:38.3508266Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70536/merge:refs/remotes/pull/70536/merge
---
2020-03-29T16:45:59.2700036Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T16:46:00.6344000Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T16:46:02.1472202Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T16:46:02.7553377Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T16:46:10.9903633Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T16:46:12.8844079Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T16:46:16.9375819Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T16:46:20.7891248Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T16:46:49.9612870Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T17:06:46.2294977Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T17:06:47.9100711Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T17:06:49.8857649Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T17:06:50.5625053Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T17:07:01.5222939Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T17:07:03.3517669Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T17:07:08.4761977Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T17:07:13.7947189Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T17:07:51.7301572Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T17:31:11.4024016Z .................................................................................................... 1700/9855
2020-03-29T17:31:15.2896770Z .................................................................................................... 1800/9855
2020-03-29T17:31:25.0145794Z .........................................................................................i.......... 1900/9855
2020-03-29T17:31:31.3459602Z .................................................................................................... 2000/9855
2020-03-29T17:31:37.4016108Z ...............................................................................iiiii................ 2100/9855
2020-03-29T17:31:57.0675206Z .................................................................................................... 2300/9855
2020-03-29T17:31:59.0836893Z .................................................................................................... 2400/9855
2020-03-29T17:32:01.3635341Z .................................................................................................... 2500/9855
2020-03-29T17:32:09.8780483Z .................................................................................................... 2600/9855
---
2020-03-29T17:34:50.1094572Z .....................................................i...............i.............................. 5000/9855
2020-03-29T17:34:57.4189950Z .................................................................................................... 5100/9855
2020-03-29T17:35:04.3388289Z ..................................................................................................i. 5200/9855
2020-03-29T17:35:09.0999310Z .................................................................................................... 5300/9855
2020-03-29T17:35:19.1367556Z ....................................................................................ii.ii........i.. 5400/9855
2020-03-29T17:35:22.5677720Z .i.................................................................................................. 5500/9855
2020-03-29T17:35:31.1582505Z .............................i...................................................................... 5700/9855
2020-03-29T17:35:40.3306113Z ...............................................ii....................................i.............. 5800/9855
2020-03-29T17:35:47.2930681Z .................................................................................................... 5900/9855
2020-03-29T17:35:52.0638330Z .................................................................................................... 6000/9855
2020-03-29T17:35:52.0638330Z .................................................................................................... 6000/9855
2020-03-29T17:36:00.6987049Z ...............................................................................ii...i..ii........... 6100/9855
2020-03-29T17:36:12.1402755Z i................................................................................................... 6200/9855
2020-03-29T17:36:23.7986977Z .................................................................................................... 6400/9855
2020-03-29T17:36:27.1058209Z .................................................................................................... 6500/9855
2020-03-29T17:36:27.1058209Z .................................................................................................... 6500/9855
2020-03-29T17:36:38.4348686Z .........i..ii...................................................................................... 6600/9855
2020-03-29T17:36:57.2962684Z .................................................................................................... 6800/9855
2020-03-29T17:36:59.2752861Z .........i.......................................................................................... 6900/9855
2020-03-29T17:37:01.2585525Z .................................................................................................... 7000/9855
2020-03-29T17:37:03.3182653Z ..............................................i..................................................... 7100/9855
---
2020-03-29T17:38:37.2061897Z .................................................................................................... 7800/9855
2020-03-29T17:38:42.2893645Z .................................................................................................... 7900/9855
2020-03-29T17:38:48.5457780Z .................................................................................................... 8000/9855
2020-03-29T17:38:56.2902311Z ......i............................................................................................. 8100/9855
2020-03-29T17:39:04.2067190Z .......................................................iiiiiiiiii.i................................. 8200/9855
2020-03-29T17:39:12.8856731Z ...................................................................................................i 8300/9855
2020-03-29T17:39:23.5571959Z .................................................................................................... 8500/9855
2020-03-29T17:39:35.8321476Z .................................................................................................... 8600/9855
2020-03-29T17:39:45.0617886Z .................................................................................................... 8700/9855
2020-03-29T17:39:50.0738451Z .................................................................................................... 8800/9855
---
2020-03-29T17:41:57.1661890Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-29T17:41:57.1846221Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T17:41:57.3799188Z 
2020-03-29T17:41:57.3799840Z running 183 tests
2020-03-29T17:41:59.9535571Z iiii......i............ii.i...iiii...i....i...........i............i..i..................i....i..... 100/183
2020-03-29T17:42:02.3413844Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-29T17:42:02.3416666Z 
2020-03-29T17:42:02.3418203Z  finished in 5.156
2020-03-29T17:42:02.3419102Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-29T17:42:02.3607455Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T17:42:04.3038831Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-29T17:42:04.3235731Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T17:42:04.4779883Z 
2020-03-29T17:42:04.4780297Z running 9 tests
2020-03-29T17:42:04.4781468Z iiiiiiiii
2020-03-29T17:42:04.4782477Z 
2020-03-29T17:42:04.4782634Z  finished in 0.154
2020-03-29T17:42:04.4786620Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-29T17:42:04.4975352Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T17:42:23.4014216Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-29T17:42:23.4213136Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T17:42:23.5839823Z 
2020-03-29T17:42:23.5840178Z running 115 tests
2020-03-29T17:42:37.1506962Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-29T17:42:38.7811484Z ...iiii.....ii.
2020-03-29T17:42:38.7814212Z 
2020-03-29T17:42:38.7817272Z  finished in 15.360
2020-03-29T17:42:38.7822904Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-29T17:42:38.7826434Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T17:53:56.6887631Z 
2020-03-29T17:53:56.6889310Z    Doc-tests core
2020-03-29T17:54:00.8759228Z 
2020-03-29T17:54:00.8760818Z running 2487 tests
2020-03-29T17:54:09.1461969Z ......iiiii......................................................................................... 100/2487
2020-03-29T17:54:17.2243254Z .....................................................................................ii............. 200/2487
2020-03-29T17:54:35.9107351Z ....................i............................................................................... 400/2487
2020-03-29T17:54:35.9107351Z ....................i............................................................................... 400/2487
2020-03-29T17:54:44.8478372Z ..........................................................................i..i..................iiii 500/2487
2020-03-29T17:54:59.5971645Z .................................................................................................... 700/2487
2020-03-29T17:55:07.3018976Z .................................................................................................... 800/2487
2020-03-29T17:55:15.0301628Z .................................................................................................... 900/2487
2020-03-29T17:55:22.7723574Z .................................................................................................... 1000/2487
---
2020-03-29T17:58:29.5678198Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
2020-03-29T17:58:29.5682859Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:603:13
2020-03-29T17:58:29.5688445Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:616:13
2020-03-29T17:58:31.6317564Z .................................................... 700/760
2020-03-29T17:58:31.6372403Z ...............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-03-29T17:58:32.2458476Z .............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-03-29T17:58:32.2490309Z .thread '<unnamed>' panicked at 'owned string.', src/libstd/thread/mod.rs:1692:13
2020-03-29T17:58:32.2491592Z .thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
2020-03-29T17:58:37.7401086Z .............
2020-03-29T17:58:37.7401671Z test result: ok. 760 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-29T17:58:37.7402096Z 
2020-03-29T17:58:37.7421461Z      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/env-a30633ce7bfebda2
---
2020-03-29T17:58:38.4367796Z 
2020-03-29T17:58:38.4397137Z running 1018 tests
2020-03-29T17:58:54.2405175Z i................................................................................................... 100/1018
2020-03-29T17:59:03.3865732Z .................................................................................................... 200/1018
2020-03-29T17:59:10.1588537Z ..................iii......i......i...i......i...................................................... 300/1018
2020-03-29T17:59:20.4988941Z ..................................................i....i......................................ii.... 500/1018
2020-03-29T17:59:27.5599313Z .................................................................................................... 600/1018
2020-03-29T17:59:32.0020437Z .................................................................................................... 700/1018
2020-03-29T17:59:32.0020437Z .................................................................................................... 700/1018
2020-03-29T17:59:38.3812025Z ............................................iiii.................................................... 800/1018
2020-03-29T17:59:51.0937881Z .................................................................................................... 900/1018
2020-03-29T17:59:56.8949003Z ..................................................................iiii.............................. 1000/1018
2020-03-29T17:59:57.8928874Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-29T17:59:57.8929120Z 
2020-03-29T17:59:57.9035089Z  finished in 150.214
2020-03-29T17:59:57.9040694Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-29T18:02:54.8316014Z 
2020-03-29T18:02:54.8316647Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-29T18:02:54.8316890Z 
2020-03-29T18:02:54.8364207Z  finished in 0.902
2020-03-29T18:02:54.8365405Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-29T18:02:54.8377728Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T18:02:55.0164929Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T18:02:55.9216535Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-0d1665619c08d4d9
2020-03-29T18:02:55.9247551Z 
2020-03-29T18:02:55.9248169Z running 0 tests
2020-03-29T18:02:55.9248336Z 
---
2020-03-29T18:16:08.9566556Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9567616Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9568690Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9569772Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9570869Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9573032Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9574114Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9575154Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T18:16:08.9576242Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-03-29T18:17:06.4109850Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-03-29T18:17:06.4377773Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T18:17:06.6375114Z 
2020-03-29T18:17:06.6376062Z running 209 tests
2020-03-29T18:17:34.4644554Z ......................i...ii.................................F.....................................i 100/209
2020-03-29T18:18:06.4606308Z .......................................iiiiii......i..............iii............................... 200/209
2020-03-29T18:18:10.6560582Z failures:
2020-03-29T18:18:10.6565089Z 
2020-03-29T18:18:10.6569108Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-29T18:18:10.6569359Z 
2020-03-29T18:18:10.6569359Z 
2020-03-29T18:18:10.6569507Z error: make failed
2020-03-29T18:18:10.6569717Z status: exit code: 2
2020-03-29T18:18:10.6569896Z command: "make"
2020-03-29T18:18:10.6570043Z stdout:
2020-03-29T18:18:10.6570423Z ------------------------------------------
2020-03-29T18:18:10.6570699Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-29T18:18:10.6570884Z 
2020-03-29T18:18:10.6573149Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-29T18:18:10.6575058Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-29T18:18:10.6575627Z Makefile:6: recipe for target 'all' failed
2020-03-29T18:18:10.6576121Z ------------------------------------------
2020-03-29T18:18:10.6576323Z stderr:
2020-03-29T18:18:10.6576661Z ------------------------------------------
2020-03-29T18:18:10.6577066Z error[E0463]: can't find crate for `rustc`
---
2020-03-29T18:18:10.6581854Z 
2020-03-29T18:18:10.6582026Z error: aborting due to previous error
2020-03-29T18:18:10.6582354Z 
2020-03-29T18:18:10.6584699Z For more information about this error, try `rustc --explain E0463`.
2020-03-29T18:18:10.6585100Z make: *** [all] Error 1
2020-03-29T18:18:10.6585570Z ------------------------------------------
2020-03-29T18:18:10.6585745Z 
2020-03-29T18:18:10.6585836Z 
2020-03-29T18:18:10.6585923Z 
2020-03-29T18:18:10.6585923Z 
2020-03-29T18:18:10.6586082Z failures:
2020-03-29T18:18:10.6586479Z     [run-make] run-make-fulldeps/hotplug_codegen_backend
2020-03-29T18:18:10.6586663Z 
2020-03-29T18:18:10.6587126Z test result: FAILED. 193 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
2020-03-29T18:18:10.6587566Z 
2020-03-29T18:18:10.6587791Z 
2020-03-29T18:18:10.6587883Z 
2020-03-29T18:18:10.6597058Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-29T18:18:10.6603097Z 
2020-03-29T18:18:10.6603191Z 
2020-03-29T18:18:10.6603675Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-29T18:18:10.6604075Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-29T18:18:10.6604075Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-29T18:18:10.6604659Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-29T18:18:10.6605078Z Build completed unsuccessfully in 1:35:25
2020-03-29T18:18:13.2042228Z == clock drift check ==
2020-03-29T18:18:13.2097506Z   local time: Sun Mar 29 18:18:13 UTC 2020
2020-03-29T18:18:13.3058642Z   network time: Sun, 29 Mar 2020 18:18:13 GMT
2020-03-29T18:18:13.3066779Z == end clock drift check ==
2020-03-29T18:18:15.2535629Z 
2020-03-29T18:18:15.2621963Z ##[error]Bash exited with code '1'.
2020-03-29T18:18:15.2632943Z ##[section]Finishing: Run build
2020-03-29T18:18:15.2681757Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70536/merge to s
2020-03-29T18:18:15.2687024Z Task         : Get sources
2020-03-29T18:18:15.2687371Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T18:18:15.2687687Z Version      : 1.0.0
2020-03-29T18:18:15.2687925Z Author       : Microsoft
2020-03-29T18:18:15.2687925Z Author       : Microsoft
2020-03-29T18:18:15.2688277Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-29T18:18:15.2688694Z ==============================================================================
2020-03-29T18:18:15.5770341Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-29T18:18:15.5820450Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70536/merge to s
2020-03-29T18:18:15.5898527Z Cleaning up task key
2020-03-29T18:18:15.5899692Z Start cleaning up orphan processes.
2020-03-29T18:18:15.6060083Z Terminate orphan process: pid (4034) (python)
2020-03-29T18:18:15.6326969Z ##[section]Finishing: Finalize Job
