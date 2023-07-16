plain
2020-03-27T11:55:27.3320703Z ========================== Starting Command Output ===========================
2020-03-27T11:55:27.3324980Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/776d805b-68b2-456d-895d-144d7f897efa.sh
2020-03-27T11:55:27.3325440Z 
2020-03-27T11:55:27.3330185Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T11:55:27.3348946Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70467/merge to s
2020-03-27T11:55:27.3352136Z Task         : Get sources
2020-03-27T11:55:27.3352450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T11:55:27.3352734Z Version      : 1.0.0
2020-03-27T11:55:27.3352927Z Author       : Microsoft
---
2020-03-27T11:55:28.3314485Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T11:55:28.3320136Z ##[command]git config gc.auto 0
2020-03-27T11:55:28.3323780Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T11:55:28.3327156Z ##[command]git config --get-all http.proxy
2020-03-27T11:55:28.3333182Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70467/merge:refs/remotes/pull/70467/merge
---
2020-03-27T12:03:20.5365009Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T12:03:22.2669345Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T12:03:23.2904100Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T12:03:33.2196832Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T12:03:35.4235911Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T12:03:36.9918338Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T12:03:46.4953667Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T12:04:20.8742235Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T12:04:57.4265080Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T12:06:56.9203407Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T12:27:55.8427362Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T12:27:57.9566041Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T12:27:59.3845159Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T12:28:11.4227058Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T12:28:14.6009296Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T12:28:16.3324720Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T12:28:28.3100471Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T12:29:11.2950527Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T12:29:58.8093249Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T12:32:19.7811216Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T12:55:50.3677660Z .................................................................................................... 1700/9849
2020-03-27T12:55:54.4076088Z .................................................................................................... 1800/9849
2020-03-27T12:56:04.8968948Z .........................................................................................i.......... 1900/9849
2020-03-27T12:56:12.1098260Z .................................................................................................... 2000/9849
2020-03-27T12:56:19.0469812Z ................................................................................iiiii............... 2100/9849
2020-03-27T12:56:41.5772699Z .................................................................................................... 2300/9849
2020-03-27T12:56:43.9077119Z .................................................................................................... 2400/9849
2020-03-27T12:56:46.4842144Z .................................................................................................... 2500/9849
2020-03-27T12:56:56.1077271Z .................................................................................................... 2600/9849
---
2020-03-27T12:59:50.5183755Z .....................................................i...............i.............................. 5000/9849
2020-03-27T12:59:58.4714353Z .................................................................................................... 5100/9849
2020-03-27T13:00:06.3034460Z ..................................................................................................i. 5200/9849
2020-03-27T13:00:11.6523245Z .................................................................................................... 5300/9849
2020-03-27T13:00:22.9434600Z ...................................................................................ii.ii........i... 5400/9849
2020-03-27T13:00:26.6573244Z i................................................................................................... 5500/9849
2020-03-27T13:00:36.6745942Z ............................i....................................................................... 5700/9849
2020-03-27T13:00:46.5547259Z .............................................ii....................................i................ 5800/9849
2020-03-27T13:00:54.4310816Z .................................................................................................... 5900/9849
2020-03-27T13:00:59.9480631Z .................................................................................................... 6000/9849
2020-03-27T13:00:59.9480631Z .................................................................................................... 6000/9849
2020-03-27T13:01:09.4804897Z .............................................................................ii...i..ii...........i. 6100/9849
2020-03-27T13:01:30.6133494Z .................................................................................................... 6300/9849
2020-03-27T13:01:37.7553127Z .................................................................................................... 6400/9849
2020-03-27T13:01:44.9086082Z .................................................................................................... 6500/9849
2020-03-27T13:01:44.9086082Z .................................................................................................... 6500/9849
2020-03-27T13:01:59.5957326Z .......i..ii........................................................................................ 6600/9849
2020-03-27T13:02:20.2366010Z .................................................................................................... 6800/9849
2020-03-27T13:02:22.3825069Z .......i............................................................................................ 6900/9849
2020-03-27T13:02:24.4619847Z .................................................................................................... 7000/9849
2020-03-27T13:02:26.8541265Z ...........................................i........................................................ 7100/9849
---
2020-03-27T13:04:05.5346554Z .................................................................................................... 7700/9849
2020-03-27T13:04:10.6992575Z .................................................................................................... 7800/9849
2020-03-27T13:04:15.6980589Z .................................................................................................... 7900/9849
2020-03-27T13:04:23.0489446Z .................................................................................................... 8000/9849
2020-03-27T13:04:30.7623545Z i................................................................................................... 8100/9849
2020-03-27T13:04:38.9442401Z .................................................iiiiiiiiii.i....................................... 8200/9849
2020-03-27T13:04:53.4032922Z i................................................................................................... 8400/9849
2020-03-27T13:04:58.6581067Z .................................................................................................... 8500/9849
2020-03-27T13:05:12.5476479Z .................................................................................................... 8600/9849
2020-03-27T13:05:22.7189857Z .................................................................................................... 8700/9849
---
2020-03-27T13:07:52.8223402Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-27T13:07:52.8470850Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T13:07:53.0695678Z 
2020-03-27T13:07:53.0696041Z running 184 tests
2020-03-27T13:07:55.9128355Z iiii......i.............ii.i..iiii....i....i...........i............i..i..................i....i.... 100/184
2020-03-27T13:07:58.7176688Z ........i.i.i...iii..iiiiiiiiiiiiii.ii......................iii.............ii......
2020-03-27T13:07:58.7180958Z 
2020-03-27T13:07:58.7183094Z  finished in 5.870
2020-03-27T13:07:58.7184348Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-27T13:07:58.7375431Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-27T13:08:00.8865024Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-27T13:08:00.9111203Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T13:08:01.0726350Z 
2020-03-27T13:08:01.0726668Z running 9 tests
2020-03-27T13:08:01.0728090Z iiiiiiiii
2020-03-27T13:08:01.0729288Z 
2020-03-27T13:08:01.0729460Z  finished in 0.161
2020-03-27T13:08:01.0736524Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-27T13:08:01.0941137Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-27T13:08:22.5555604Z o") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-27T13:08:22.5780542Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T13:08:22.7709942Z 
2020-03-27T13:08:22.7710207Z running 115 tests
2020-03-27T13:08:36.6985489Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-27T13:08:38.5106261Z ...iiii.....ii.
2020-03-27T13:08:38.5107750Z 
2020-03-27T13:08:38.5111767Z  finished in 15.933
2020-03-27T13:08:38.5118366Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-27T13:08:38.5123389Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-27T13:22:09.7531313Z 
2020-03-27T13:22:09.7532435Z    Doc-tests core
2020-03-27T13:22:14.6420312Z 
2020-03-27T13:22:14.6421152Z running 2486 tests
2020-03-27T13:22:24.0447446Z ......iiiii......................................................................................... 100/2486
2020-03-27T13:22:33.1600407Z .....................................................................................ii............. 200/2486
2020-03-27T13:22:54.3654842Z ....................i............................................................................... 400/2486
2020-03-27T13:22:54.3654842Z ....................i............................................................................... 400/2486
2020-03-27T13:23:04.8035105Z .........................................................................i..i..................iiii. 500/2486
2020-03-27T13:23:22.1070208Z .................................................................................................... 700/2486
2020-03-27T13:23:31.3353620Z .................................................................................................... 800/2486
2020-03-27T13:23:40.1540696Z .................................................................................................... 900/2486
2020-03-27T13:23:48.8663851Z .................................................................................................... 1000/2486
---
2020-03-27T13:27:29.9537707Z 
2020-03-27T13:27:29.9538808Z running 1012 tests
2020-03-27T13:27:48.2113177Z i................................................................................................... 100/1012
2020-03-27T13:27:58.5446372Z .................................................................................................... 200/1012
2020-03-27T13:28:05.5844001Z ..................iii......i......i...i......i...................................................... 300/1012
2020-03-27T13:28:17.5482633Z ............................................i....i......................................ii.......... 500/1012
2020-03-27T13:28:24.8895486Z .................................................................................................... 600/1012
2020-03-27T13:28:30.2021024Z .................................................................................................... 700/1012
2020-03-27T13:28:30.2021024Z .................................................................................................... 700/1012
2020-03-27T13:28:36.9872549Z ......................................iiii.......................................................... 800/1012
2020-03-27T13:28:51.1837554Z .................................................................................................... 900/1012
2020-03-27T13:28:58.0578842Z ............................................................iiii.................................... 1000/1012
2020-03-27T13:28:58.5236908Z test result: ok. 992 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-27T13:28:58.5237469Z 
2020-03-27T13:28:58.5355640Z  finished in 169.839
2020-03-27T13:28:58.5361022Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-27T13:32:18.4824226Z 
2020-03-27T13:32:18.4824516Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-27T13:32:18.4824787Z 
2020-03-27T13:32:18.4885306Z  finished in 1.019
2020-03-27T13:32:18.4892600Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-27T13:32:18.4913496Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T13:32:18.6816586Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T13:32:19.4905776Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-66859c913373332c
2020-03-27T13:32:19.4935149Z 
2020-03-27T13:32:19.4935464Z running 0 tests
2020-03-27T13:32:19.4935613Z 
---
2020-03-27T13:47:53.5494185Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5495023Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5495833Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5496618Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5497424Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5499025Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5499834Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5500605Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-27T13:47:53.5501400Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-03-27T13:48:57.7637057Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-03-27T13:48:57.7882129Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-27T13:48:58.0226207Z 
2020-03-27T13:48:58.0227824Z running 210 tests
2020-03-27T13:49:30.7525000Z ......................i...ii...............................F.......................................i 100/210
2020-03-27T13:50:04.8945638Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-27T13:50:09.9851016Z failures:
2020-03-27T13:50:09.9856558Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T13:50:09.9857027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T13:50:09.9862269Z 
2020-03-27T13:50:09.9862269Z 
2020-03-27T13:50:09.9862893Z ---- [run-make] run-make-fulldeps/foreign-exceptions stdout ----
2020-03-27T13:50:09.9863139Z 
2020-03-27T13:50:09.9863313Z error: make failed
2020-03-27T13:50:09.9863513Z status: exit code: 2
2020-03-27T13:50:09.9863725Z command: "make"
2020-03-27T13:50:09.9863885Z stdout:
2020-03-27T13:50:09.9864290Z ------------------------------------------
2020-03-27T13:50:09.9865132Z c++ -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o foo.cpp
2020-03-27T13:50:09.9866597Z ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o
2020-03-27T13:50:09.9870225Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions  foo.rs -lfoo -lstdc++
2020-03-27T13:50:09.9874379Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/foo
2020-03-27T13:50:09.9875678Z caught C++ exception
2020-03-27T13:50:09.9876200Z Makefile:4: recipe for target 'all' failed
2020-03-27T13:50:09.9876389Z 
2020-03-27T13:50:09.9876764Z ------------------------------------------
2020-03-27T13:50:09.9876764Z ------------------------------------------
2020-03-27T13:50:09.9876991Z stderr:
2020-03-27T13:50:09.9877372Z ------------------------------------------
2020-03-27T13:50:09.9877865Z ar: `u' modifier ignored since `D' is the default (see `U')
2020-03-27T13:50:09.9878474Z foo: foo.cpp:38: void throw_cxx_exception(): Assertion `rust_ok' failed.
2020-03-27T13:50:09.9878792Z Aborted (core dumped)
2020-03-27T13:50:09.9878998Z make: *** [all] Error 134
2020-03-27T13:50:09.9879541Z ------------------------------------------
2020-03-27T13:50:09.9879715Z 
2020-03-27T13:50:09.9879813Z 
2020-03-27T13:50:09.9879926Z 
2020-03-27T13:50:09.9879926Z 
2020-03-27T13:50:09.9880059Z failures:
2020-03-27T13:50:09.9880469Z     [run-make] run-make-fulldeps/foreign-exceptions
2020-03-27T13:50:09.9880660Z 
2020-03-27T13:50:09.9881191Z test result: FAILED. 194 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
2020-03-27T13:50:09.9881466Z 
2020-03-27T13:50:09.9881600Z 
2020-03-27T13:50:09.9881699Z 
2020-03-27T13:50:09.9891629Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T13:50:09.9898538Z 
2020-03-27T13:50:09.9898638Z 
2020-03-27T13:50:09.9899198Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T13:50:09.9899580Z Build completed unsuccessfully in 1:50:38
2020-03-27T13:50:09.9899580Z Build completed unsuccessfully in 1:50:38
2020-03-27T13:50:11.6278905Z == clock drift check ==
2020-03-27T13:50:11.6313274Z   local time: Fri Mar 27 13:50:11 UTC 2020
2020-03-27T13:50:11.7990241Z   network time: Fri, 27 Mar 2020 13:50:11 GMT
2020-03-27T13:50:11.7992680Z == end clock drift check ==
2020-03-27T13:50:13.5833939Z 
2020-03-27T13:50:13.5908764Z ##[error]Bash exited with code '1'.
2020-03-27T13:50:13.5924067Z ##[section]Finishing: Run build
2020-03-27T13:50:13.5980411Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70467/merge to s
2020-03-27T13:50:13.5986071Z Task         : Get sources
2020-03-27T13:50:13.5986464Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T13:50:13.5986845Z Version      : 1.0.0
2020-03-27T13:50:13.5987100Z Author       : Microsoft
2020-03-27T13:50:13.5987100Z Author       : Microsoft
2020-03-27T13:50:13.5987503Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T13:50:13.5987990Z ==============================================================================
2020-03-27T13:50:13.9575652Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T13:50:13.9625707Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70467/merge to s
2020-03-27T13:50:13.9723769Z Cleaning up task key
2020-03-27T13:50:13.9725051Z Start cleaning up orphan processes.
2020-03-27T13:50:13.9933654Z Terminate orphan process: pid (3720) (python)
2020-03-27T13:50:14.0480020Z ##[section]Finishing: Finalize Job
