plain
2020-03-13T23:57:49.3397178Z ========================== Starting Command Output ===========================
2020-03-13T23:57:49.3400076Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3a937bc7-f248-435f-910c-3685f6ba543c.sh
2020-03-13T23:57:49.3400415Z 
2020-03-13T23:57:49.3403952Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T23:57:49.3424296Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-13T23:57:49.3427002Z Task         : Get sources
2020-03-13T23:57:49.3427254Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T23:57:49.3427515Z Version      : 1.0.0
2020-03-13T23:57:49.3427678Z Author       : Microsoft
---
2020-03-13T23:57:50.3428264Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T23:57:50.3434720Z ##[command]git config gc.auto 0
2020-03-13T23:57:50.3439225Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T23:57:50.3443453Z ##[command]git config --get-all http.proxy
2020-03-13T23:57:50.3450569Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69965/merge:refs/remotes/pull/69965/merge
---
2020-03-14T00:50:21.5927064Z .................................................................................................... 1700/9771
2020-03-14T00:50:26.2490169Z .................................................................................................... 1800/9771
2020-03-14T00:50:37.9941281Z ...................................................................i................................ 1900/9771
2020-03-14T00:50:44.9354227Z .................................................................................................... 2000/9771
2020-03-14T00:51:00.0404786Z .........................................................iiiii...................................... 2100/9771
2020-03-14T00:51:10.6341125Z .................................................................................................... 2300/9771
2020-03-14T00:51:12.9031732Z .................................................................................................... 2400/9771
2020-03-14T00:51:16.0330947Z .................................................................................................... 2500/9771
2020-03-14T00:51:38.3799366Z .................................................................................................... 2600/9771
---
2020-03-14T00:54:19.0935864Z ............................i...............i....................................................... 5000/9771
2020-03-14T00:54:28.8283835Z .................................................................................................... 5100/9771
2020-03-14T00:54:34.9562852Z .......................................................................i............................ 5200/9771
2020-03-14T00:54:40.4608520Z .................................................................................................... 5300/9771
2020-03-14T00:54:49.7597434Z ....................................................ii.ii........i...i.............................. 5400/9771
2020-03-14T00:54:56.9415800Z .................................................................................................... 5600/9771
2020-03-14T00:55:05.4592092Z .................................................................................................... 5700/9771
2020-03-14T00:55:10.9854677Z ............................................i....................................................... 5800/9771
2020-03-14T00:55:16.6091960Z .................................................................................................... 5900/9771
2020-03-14T00:55:16.6091960Z .................................................................................................... 5900/9771
2020-03-14T00:55:25.6310526Z .................................................................................................... 6000/9771
2020-03-14T00:55:31.2789712Z ......................................ii...i..ii...........i........................................ 6100/9771
2020-03-14T00:55:49.6064044Z .................................................................................................... 6300/9771
2020-03-14T00:55:53.0788995Z .................................................................................................... 6400/9771
2020-03-14T00:55:53.0788995Z .................................................................................................... 6400/9771
2020-03-14T00:55:57.5215838Z .....................................................................i..ii.......................... 6500/9771
2020-03-14T00:56:17.3098227Z .................................................................................................... 6700/9771
2020-03-14T00:56:24.7794834Z ...................................................................i................................ 6800/9771
2020-03-14T00:56:26.5343626Z .................................................................................................... 6900/9771
2020-03-14T00:56:28.4253903Z .................................................................................................... 7000/9771
---
2020-03-14T00:57:59.8483542Z .................................................................................................... 7800/9771
2020-03-14T00:58:05.1887018Z .................................................................................................... 7900/9771
2020-03-14T00:58:10.3619801Z ...................................................i................................................ 8000/9771
2020-03-14T00:58:19.8256738Z .................................................................................................... 8100/9771
2020-03-14T00:58:24.7318660Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-14T00:58:37.1802940Z .................................................................................................... 8400/9771
2020-03-14T00:58:46.6867390Z .................................................................................................... 8500/9771
2020-03-14T00:58:58.2865237Z .................................................................................................... 8600/9771
2020-03-14T00:59:03.6434337Z .................................................................................................... 8700/9771
---
2020-03-14T01:01:07.7983415Z  finished in 6.597
2020-03-14T01:01:07.8161695Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:07.9595670Z 
2020-03-14T01:01:07.9596252Z running 179 tests
2020-03-14T01:01:10.6125033Z iiii......i...........ii..iiii....i....i...........i............i..i..................i....i........ 100/179
2020-03-14T01:01:12.6881733Z ....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-03-14T01:01:12.6884879Z 
2020-03-14T01:01:12.6890477Z  finished in 4.873
2020-03-14T01:01:12.7072768Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:12.8532817Z 
---
2020-03-14T01:01:14.5597278Z  finished in 1.852
2020-03-14T01:01:14.5778091Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:14.7208812Z 
2020-03-14T01:01:14.7209150Z running 9 tests
2020-03-14T01:01:14.7210241Z iiiiiiiii
2020-03-14T01:01:14.7211484Z 
2020-03-14T01:01:14.7213595Z  finished in 0.143
2020-03-14T01:01:14.7393327Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:14.8874985Z 
---
2020-03-14T01:01:32.8147429Z  finished in 18.075
2020-03-14T01:01:32.8339618Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:32.9871244Z 
2020-03-14T01:01:32.9872115Z running 115 tests
2020-03-14T01:01:45.2262505Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-14T01:01:46.6850209Z ...iiii.....ii.
2020-03-14T01:01:46.6851806Z 
2020-03-14T01:01:46.6857810Z  finished in 13.851
2020-03-14T01:01:46.6863701Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:01:46.6864437Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-14T01:13:37.2181508Z 
2020-03-14T01:13:37.2182700Z    Doc-tests core
2020-03-14T01:13:41.2685554Z 
2020-03-14T01:13:41.2686545Z running 2480 tests
2020-03-14T01:13:49.2539482Z ......iiiii......................................................................................... 100/2480
2020-03-14T01:13:57.3820565Z ....................................................................................ii.............. 200/2480
2020-03-14T01:14:15.5920979Z ...................i................................................................................ 400/2480
2020-03-14T01:14:15.5920979Z ...................i................................................................................ 400/2480
2020-03-14T01:14:23.9269649Z ........................................................................i..i..................iiii.. 500/2480
2020-03-14T01:14:38.4326861Z .................................................................................................... 700/2480
2020-03-14T01:14:46.0356894Z .................................................................................................... 800/2480
2020-03-14T01:14:53.4552037Z .................................................................................................... 900/2480
2020-03-14T01:15:00.8978815Z .................................................................................................... 1000/2480
---
2020-03-14T01:18:13.0754221Z 
2020-03-14T01:18:13.0754667Z running 1010 tests
2020-03-14T01:18:29.0399608Z i................................................................................................... 100/1010
2020-03-14T01:18:38.2698317Z .................................................................................................... 200/1010
2020-03-14T01:18:44.7471529Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-14T01:18:49.3698451Z .................................................................................................... 400/1010
2020-03-14T01:18:55.5790617Z ............................................i..i......................................ii............ 500/1010
2020-03-14T01:19:06.9505746Z .................................................................................................... 700/1010
2020-03-14T01:19:06.9505746Z .................................................................................................... 700/1010
2020-03-14T01:19:13.1419749Z ....................................iiii............................................................ 800/1010
2020-03-14T01:19:25.8642278Z .................................................................................................... 900/1010
2020-03-14T01:19:32.0286751Z ..........................................................iiii...................................... 1000/1010
2020-03-14T01:19:32.4244886Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-14T01:19:32.4245186Z 
2020-03-14T01:19:32.4334466Z  finished in 150.466
2020-03-14T01:19:32.4348291Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-14T01:36:22.9693067Z  finished in 41.138
2020-03-14T01:36:23.9136972Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T01:36:23.9174674Z 
2020-03-14T01:36:23.9174989Z running 210 tests
2020-03-14T01:36:55.1590042Z ......................i...ii.................................F.....................................i 100/210
2020-03-14T01:37:35.7269874Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-14T01:37:40.8851439Z failures:
2020-03-14T01:37:40.8861328Z 
2020-03-14T01:37:40.8862305Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-14T01:37:40.8862536Z 
2020-03-14T01:37:40.8862536Z 
2020-03-14T01:37:40.8862718Z error: make failed
2020-03-14T01:37:40.8862922Z status: exit code: 2
2020-03-14T01:37:40.8863117Z command: "make"
2020-03-14T01:37:40.8863281Z stdout:
2020-03-14T01:37:40.8863713Z ------------------------------------------
2020-03-14T01:37:40.8864014Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-14T01:37:40.8864245Z 
2020-03-14T01:37:40.8866478Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-14T01:37:40.8868425Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-14T01:37:40.8869056Z Makefile:4: recipe for target 'all' failed
2020-03-14T01:37:40.8869632Z ------------------------------------------
2020-03-14T01:37:40.8869842Z stderr:
2020-03-14T01:37:40.8870217Z ------------------------------------------
2020-03-14T01:37:40.8870595Z error[E0433]: failed to resolve: could not find `symbol_names` in `rustc_codegen_utils`
2020-03-14T01:37:40.8870595Z error[E0433]: failed to resolve: could not find `symbol_names` in `rustc_codegen_utils`
2020-03-14T01:37:40.8871100Z   --> the_backend.rs:51:30
2020-03-14T01:37:40.8871288Z    |
2020-03-14T01:37:40.8871560Z 51 |         rustc_codegen_utils::symbol_names::provide(providers);
2020-03-14T01:37:40.8872018Z    |                              ^^^^^^^^^^^^ could not find `symbol_names` in `rustc_codegen_utils`
2020-03-14T01:37:40.8872295Z 
2020-03-14T01:37:40.8872688Z warning: ignoring --out-dir flag due to -o flag
2020-03-14T01:37:40.8873084Z error: aborting due to previous error
2020-03-14T01:37:40.8873250Z 
2020-03-14T01:37:40.8873707Z For more information about this error, try `rustc --explain E0433`.
2020-03-14T01:37:40.8873707Z For more information about this error, try `rustc --explain E0433`.
2020-03-14T01:37:40.8874014Z make: *** [all] Error 1
2020-03-14T01:37:40.8874897Z ------------------------------------------
2020-03-14T01:37:40.8875066Z 
2020-03-14T01:37:40.8875180Z 
2020-03-14T01:37:40.8875397Z 
---
2020-03-14T01:37:40.8877569Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T01:37:40.8878023Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T01:37:40.8882161Z 
2020-03-14T01:37:40.8882351Z 
2020-03-14T01:37:40.8893445Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T01:37:40.8902390Z 
2020-03-14T01:37:40.8902496Z 
2020-03-14T01:37:40.8902740Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T01:37:40.8903075Z Build completed unsuccessfully in 1:34:25
2020-03-14T01:37:40.8903075Z Build completed unsuccessfully in 1:34:25
2020-03-14T01:37:40.9012743Z == clock drift check ==
2020-03-14T01:37:40.9036233Z   local time: Sat Mar 14 01:37:40 UTC 2020
2020-03-14T01:37:41.2128819Z   network time: Sat, 14 Mar 2020 01:37:41 GMT
2020-03-14T01:37:41.2129319Z == end clock drift check ==
2020-03-14T01:37:43.3888204Z 
2020-03-14T01:37:43.3970093Z ##[error]Bash exited with code '1'.
2020-03-14T01:37:43.3986673Z ##[section]Finishing: Run build
2020-03-14T01:37:43.4056386Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-14T01:37:43.4062341Z Task         : Get sources
2020-03-14T01:37:43.4062687Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T01:37:43.4063036Z Version      : 1.0.0
2020-03-14T01:37:43.4063263Z Author       : Microsoft
2020-03-14T01:37:43.4063263Z Author       : Microsoft
2020-03-14T01:37:43.4063618Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T01:37:43.4064038Z ==============================================================================
2020-03-14T01:37:43.7778218Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T01:37:43.7826141Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-14T01:37:43.7919056Z Cleaning up task key
2020-03-14T01:37:43.7920906Z Start cleaning up orphan processes.
2020-03-14T01:37:43.8101745Z Terminate orphan process: pid (4162) (python)
2020-03-14T01:37:43.8180466Z ##[section]Finishing: Finalize Job
