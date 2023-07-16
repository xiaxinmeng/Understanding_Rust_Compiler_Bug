plain
2020-04-03T08:25:00.4289031Z ========================== Starting Command Output ===========================
2020-04-03T08:25:00.4291689Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/25e66082-5477-43a9-8af3-2986931c9d5b.sh
2020-04-03T08:25:00.4291968Z 
2020-04-03T08:25:00.4295647Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T08:25:00.4314156Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-03T08:25:00.4317315Z Task         : Get sources
2020-04-03T08:25:00.4317621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T08:25:00.4317937Z Version      : 1.0.0
2020-04-03T08:25:00.4318136Z Author       : Microsoft
---
2020-04-03T08:25:01.4295120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T08:25:01.4300436Z ##[command]git config gc.auto 0
2020-04-03T08:25:01.4303735Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T08:25:01.4306777Z ##[command]git config --get-all http.proxy
2020-04-03T08:25:01.4312241Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70674/merge:refs/remotes/pull/70674/merge
---
2020-04-03T08:27:31.0430402Z Looks like docker image is the same as before, not uploading
2020-04-03T08:27:38.6224256Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T08:27:38.6461291Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-03T08:27:38.6484574Z == clock drift check ==
2020-04-03T08:27:38.6492444Z   local time: Fri Apr  3 08:27:38 UTC 2020
2020-04-03T08:27:38.9846729Z   network time: Fri, 03 Apr 2020 08:27:38 GMT
2020-04-03T08:27:38.9870650Z Starting sccache server...
2020-04-03T08:27:39.0605033Z configure: processing command line
2020-04-03T08:27:39.0606063Z configure: 
2020-04-03T08:27:39.0607603Z configure: rust.dist-src        := False
---
2020-04-03T08:32:13.5052022Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T08:32:14.7880262Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T08:32:16.2326087Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T08:32:16.8472642Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T08:32:25.0510139Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T08:32:26.5294454Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T08:32:30.5018739Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T08:32:34.0045743Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T08:32:43.0804503Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T08:53:13.9429435Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T08:53:15.5220799Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T08:53:17.3375686Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T08:53:19.5099044Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T08:53:28.6647917Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T08:53:32.4242151Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T08:53:37.2913304Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T08:53:42.1436022Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T08:53:50.7644274Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T09:18:05.1986412Z .................................................................................................... 1700/9870
2020-04-03T09:18:08.8919745Z .................................................................................................... 1800/9870
2020-04-03T09:18:17.0647539Z ...............................................................................................i.... 1900/9870
2020-04-03T09:18:24.0745019Z .................................................................................................... 2000/9870
2020-04-03T09:18:30.1928328Z .....................................................................................iiiii.......... 2100/9870
2020-04-03T09:18:49.4834952Z .................................................................................................... 2300/9870
2020-04-03T09:18:51.5391098Z .................................................................................................... 2400/9870
2020-04-03T09:18:53.6745677Z .................................................................................................... 2500/9870
2020-04-03T09:18:59.4533015Z .................................................................................................... 2600/9870
---
2020-04-03T09:21:43.6145888Z ...........................................................i...............i........................ 5000/9870
2020-04-03T09:21:50.3905586Z .................................................................................................... 5100/9870
2020-04-03T09:21:57.4655403Z .................................................................................................... 5200/9870
2020-04-03T09:22:02.0812591Z ....i............................................................................................... 5300/9870
2020-04-03T09:22:12.1993965Z ..........................................................................................ii.ii..... 5400/9870
2020-04-03T09:22:16.4193591Z ...i...i............................................................................................ 5500/9870
2020-04-03T09:22:25.2321373Z ...................................i................................................................ 5700/9870
2020-04-03T09:22:34.6939096Z .......................................................ii....................................i...... 5800/9870
2020-04-03T09:22:41.6723618Z .................................................................................................... 5900/9870
2020-04-03T09:22:46.3085707Z .................................................................................................... 6000/9870
2020-04-03T09:22:46.3085707Z .................................................................................................... 6000/9870
2020-04-03T09:22:55.0246393Z .......................................................................................ii...i..ii... 6100/9870
2020-04-03T09:23:14.8551787Z .................................................................................................... 6300/9870
2020-04-03T09:23:20.6623997Z .................................................................................................... 6400/9870
2020-04-03T09:23:23.3702150Z .................................................................................................... 6500/9870
2020-04-03T09:23:23.3702150Z .................................................................................................... 6500/9870
2020-04-03T09:23:34.9702504Z .................i..ii.............................................................................. 6600/9870
2020-04-03T09:23:53.6037734Z .................................................................................................... 6800/9870
2020-04-03T09:23:55.3778323Z .................i.................................................................................. 6900/9870
2020-04-03T09:23:57.3040030Z .................................................................................................... 7000/9870
2020-04-03T09:23:59.3202656Z ........................................................i........................................... 7100/9870
---
2020-04-03T09:25:30.9456919Z .................................................................................................... 7800/9870
2020-04-03T09:25:35.2664250Z .................................................................................................... 7900/9870
2020-04-03T09:25:40.9015649Z .................................................................................................... 8000/9870
2020-04-03T09:25:48.5926722Z ....................i............................................................................... 8100/9870
2020-04-03T09:25:56.2245552Z .....................................................................iiiiiiiiii.i................... 8200/9870
2020-04-03T09:26:11.1445200Z .............i......i............................................................................... 8400/9870
2020-04-03T09:26:15.7343545Z .................................................................................................... 8500/9870
2020-04-03T09:26:26.0252576Z .................................................................................................... 8600/9870
2020-04-03T09:26:36.7199501Z .................................................................................................... 8700/9870
---
2020-04-03T09:28:50.3565295Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-03T09:28:50.3756029Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T09:28:50.5713164Z 
2020-04-03T09:28:50.5714355Z running 183 tests
2020-04-03T09:28:53.1276806Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-04-03T09:28:55.5338074Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-04-03T09:28:55.5342110Z 
2020-04-03T09:28:55.5344786Z  finished in 5.158
2020-04-03T09:28:55.5355777Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-03T09:28:55.5535956Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T09:28:57.4615005Z  not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-03T09:28:57.4803533Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T09:28:57.6289525Z 
2020-04-03T09:28:57.6290217Z running 9 tests
2020-04-03T09:28:57.6294025Z iiiiiiiii
2020-04-03T09:28:57.6295320Z 
2020-04-03T09:28:57.6300497Z  finished in 0.149
2020-04-03T09:28:57.6305303Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-03T09:28:57.6494846Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T09:29:16.4474739Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-03T09:29:16.4690309Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T09:29:16.6403650Z 
2020-04-03T09:29:16.6404003Z running 115 tests
2020-04-03T09:29:29.4224584Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-03T09:29:31.0324308Z ...iiii.....ii.
2020-04-03T09:29:31.0325713Z 
2020-04-03T09:29:31.0330466Z  finished in 14.564
2020-04-03T09:29:31.0336258Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-03T09:29:31.0339081Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-03T09:41:01.8699304Z 
2020-04-03T09:41:01.8704324Z    Doc-tests core
2020-04-03T09:41:05.7686926Z 
2020-04-03T09:41:05.7687818Z running 2489 tests
2020-04-03T09:41:13.7710731Z ......iiiii......................................................................................... 100/2489
2020-04-03T09:41:21.5769776Z .....................................................................................ii............. 200/2489
2020-04-03T09:41:39.6180837Z ....................i............................................................................... 400/2489
2020-04-03T09:41:39.6180837Z ....................i............................................................................... 400/2489
2020-04-03T09:41:48.0961094Z ..........................................................................i..i..................iiii 500/2489
2020-04-03T09:42:01.8738349Z .................................................................................................... 700/2489
2020-04-03T09:42:09.1795315Z .................................................................................................... 800/2489
2020-04-03T09:42:16.8893188Z .................................................................................................... 900/2489
2020-04-03T09:42:24.8830385Z .................................................................................................... 1000/2489
---
2020-04-03T09:45:38.9544140Z 
2020-04-03T09:45:38.9544724Z running 1018 tests
2020-04-03T09:45:55.4010782Z i................................................................................................... 100/1018
2020-04-03T09:46:05.4685340Z .................................................................................................... 200/1018
2020-04-03T09:46:12.7333725Z ..................iii......i......i...i......i...................................................... 300/1018
2020-04-03T09:46:23.0495339Z ..................................................i....i......................................ii.... 500/1018
2020-04-03T09:46:30.0823272Z .................................................................................................... 600/1018
2020-04-03T09:46:34.5454932Z .................................................................................................... 700/1018
2020-04-03T09:46:34.5454932Z .................................................................................................... 700/1018
2020-04-03T09:46:41.0178788Z ............................................iiii.................................................... 800/1018
2020-04-03T09:46:53.8698618Z .................................................................................................... 900/1018
2020-04-03T09:46:59.7325657Z ..................................................................iiii.............................. 1000/1018
2020-04-03T09:47:00.8570769Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-03T09:47:00.8571034Z 
2020-04-03T09:47:00.8673334Z  finished in 155.478
2020-04-03T09:47:00.8677294Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-03T09:49:54.9138400Z 
2020-04-03T09:49:54.9138865Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-03T09:49:54.9139384Z 
2020-04-03T09:49:54.9198799Z  finished in 0.950
2020-04-03T09:49:54.9200500Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-03T09:49:54.9221710Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T09:49:55.1002743Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T09:49:56.1419643Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-6f3dda2037ca248c
2020-04-03T09:49:56.1450422Z 
2020-04-03T09:49:56.1453039Z running 0 tests
2020-04-03T09:49:56.1453174Z 
---
2020-04-03T10:02:56.9303784Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9304400Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9305041Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9305666Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9306295Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9307563Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9308201Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9308999Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-03T10:02:56.9309723Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-03T10:03:52.1594590Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-03T10:03:52.1920048Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-03T10:03:52.3802196Z 
2020-04-03T10:03:52.3802645Z running 209 tests
2020-04-03T10:04:20.2531699Z ......................i...ii...................................F...................................i 100/209
2020-04-03T10:04:57.5265919Z .......................................iiiiii......i..............iii............................... 200/209
2020-04-03T10:04:58.8731842Z failures:
2020-04-03T10:04:58.8734913Z 
2020-04-03T10:04:58.8737087Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-04-03T10:04:58.8737599Z 
2020-04-03T10:04:58.8737599Z 
2020-04-03T10:04:58.8738237Z error: make failed
2020-04-03T10:04:58.8738758Z status: exit code: 2
2020-04-03T10:04:58.8739325Z command: "make"
2020-04-03T10:04:58.8739774Z stdout:
2020-04-03T10:04:58.8741247Z ------------------------------------------
2020-04-03T10:04:58.8741966Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-04-03T10:04:58.8743245Z 
2020-04-03T10:04:58.8748847Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-04-03T10:04:58.8750907Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-04-03T10:04:58.8751477Z Makefile:6: recipe for target 'all' failed
2020-04-03T10:04:58.8751997Z ------------------------------------------
2020-04-03T10:04:58.8752185Z stderr:
2020-04-03T10:04:58.8752524Z ------------------------------------------
2020-04-03T10:04:58.8752524Z ------------------------------------------
2020-04-03T10:04:58.8755885Z warning: ignoring --out-dir flag due to -o flag
2020-04-03T10:04:58.8756232Z error[E0308]: mismatched types
2020-04-03T10:04:58.8756867Z   --> the_backend.rs:57:13
2020-04-03T10:04:58.8757040Z    |
2020-04-03T10:04:58.8757040Z    |
2020-04-03T10:04:58.8757530Z 57 |             tcx.arena.alloc(Default::default()) // Just a dummy
2020-04-03T10:04:58.8758221Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::collections::HashMap`, found `&mut _`
2020-04-03T10:04:58.8759512Z    = note:         expected struct `std::collections::HashMap<std::string::String, std::option::Option<rustc_span::Symbol>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
2020-04-03T10:04:58.8760718Z            found mutable reference `&mut _`
2020-04-03T10:04:58.8760902Z 
2020-04-03T10:04:58.8762009Z error: aborting due to previous error
2020-04-03T10:04:58.8762009Z error: aborting due to previous error
2020-04-03T10:04:58.8762302Z 
2020-04-03T10:04:58.8762833Z For more information about this error, try `rustc --explain E0308`.
2020-04-03T10:04:58.8763223Z make: *** [all] Error 1
2020-04-03T10:04:58.8763748Z ------------------------------------------
2020-04-03T10:04:58.8763920Z 
2020-04-03T10:04:58.8764017Z 
2020-04-03T10:04:58.8764148Z 
---
2020-04-03T10:04:58.8766172Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-03T10:04:58.8766590Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-03T10:04:58.8766825Z 
2020-04-03T10:04:58.8766939Z 
2020-04-03T10:04:58.8780877Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-03T10:04:58.8787895Z 
2020-04-03T10:04:58.8787998Z 
2020-04-03T10:04:58.8788636Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-03T10:04:58.8789025Z Build completed unsuccessfully in 1:35:58
2020-04-03T10:04:58.8789025Z Build completed unsuccessfully in 1:35:58
2020-04-03T10:04:58.8849064Z == clock drift check ==
2020-04-03T10:04:58.8867694Z   local time: Fri Apr  3 10:04:58 UTC 2020
2020-04-03T10:04:59.1713135Z   network time: Fri, 03 Apr 2020 10:04:59 GMT
2020-04-03T10:05:01.0599752Z 
2020-04-03T10:05:01.0599752Z 
2020-04-03T10:05:01.0694118Z ##[error]Bash exited with code '1'.
2020-04-03T10:05:01.0707476Z ##[section]Finishing: Run build
2020-04-03T10:05:01.0755737Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-03T10:05:01.0760936Z Task         : Get sources
2020-04-03T10:05:01.0761287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T10:05:01.0761620Z Version      : 1.0.0
2020-04-03T10:05:01.0761844Z Author       : Microsoft
2020-04-03T10:05:01.0761844Z Author       : Microsoft
2020-04-03T10:05:01.0762204Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T10:05:01.0762626Z ==============================================================================
2020-04-03T10:05:01.4284754Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T10:05:01.4338302Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70674/merge to s
2020-04-03T10:05:01.4423279Z Cleaning up task key
2020-04-03T10:05:01.4424543Z Start cleaning up orphan processes.
2020-04-03T10:05:01.4658905Z Terminate orphan process: pid (4548) (python)
2020-04-03T10:05:01.4904386Z ##[section]Finishing: Finalize Job
