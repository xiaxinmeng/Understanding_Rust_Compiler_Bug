plain
2020-03-19T18:14:50.0667768Z ========================== Starting Command Output ===========================
2020-03-19T18:14:50.0670381Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/04ba3c4d-6039-4ccd-a531-b3f995927b88.sh
2020-03-19T18:14:50.0670643Z 
2020-03-19T18:14:50.0675619Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T18:14:50.0696165Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70161/merge to s
2020-03-19T18:14:50.0699825Z Task         : Get sources
2020-03-19T18:14:50.0700336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T18:14:50.0700712Z Version      : 1.0.0
2020-03-19T18:14:50.0700902Z Author       : Microsoft
---
2020-03-19T18:14:51.0697491Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T18:14:51.0705007Z ##[command]git config gc.auto 0
2020-03-19T18:14:51.0709202Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T18:14:51.0713415Z ##[command]git config --get-all http.proxy
2020-03-19T18:14:51.0719563Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70161/merge:refs/remotes/pull/70161/merge
---
2020-03-19T19:15:28.7454041Z .................................................................................................... 1700/9803
2020-03-19T19:15:33.1047082Z .................................................................................................... 1800/9803
2020-03-19T19:15:44.7737353Z ..........................................................................i......................... 1900/9803
2020-03-19T19:15:51.4140213Z .................................................................................................... 2000/9803
2020-03-19T19:16:00.3273109Z ................................................................iiiii............................... 2100/9803
2020-03-19T19:16:18.4305787Z .................................................................................................... 2300/9803
2020-03-19T19:16:20.7214526Z .................................................................................................... 2400/9803
2020-03-19T19:16:23.8202115Z .................................................................................................... 2500/9803
2020-03-19T19:16:45.1535392Z .................................................................................................... 2600/9803
---
2020-03-19T19:19:33.4934500Z .....................................i...............i.............................................. 5000/9803
2020-03-19T19:19:42.7963615Z .................................................................................................... 5100/9803
2020-03-19T19:19:49.4891133Z ................................................................................i................... 5200/9803
2020-03-19T19:19:55.2858091Z .................................................................................................... 5300/9803
2020-03-19T19:20:06.2056264Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-19T19:20:15.1158136Z i................................................................................................... 5600/9803
2020-03-19T19:20:25.2260679Z .....i.............................................................................................. 5700/9803
2020-03-19T19:20:32.0317911Z ........................................................i........................................... 5800/9803
2020-03-19T19:20:39.1212761Z .................................................................................................... 5900/9803
2020-03-19T19:20:39.1212761Z .................................................................................................... 5900/9803
2020-03-19T19:20:47.8742446Z .................................................................................................... 6000/9803
2020-03-19T19:20:56.2409390Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-19T19:21:17.3860317Z .................................................................................................... 6300/9803
2020-03-19T19:21:24.8990223Z .................................................................................................... 6400/9803
2020-03-19T19:21:24.8990223Z .................................................................................................... 6400/9803
2020-03-19T19:21:33.5111224Z ................................................................................i..ii............... 6500/9803
2020-03-19T19:22:00.2320859Z .................................................................................................... 6700/9803
2020-03-19T19:22:10.2636823Z ...............................................................................i.................... 6800/9803
2020-03-19T19:22:12.4686082Z .................................................................................................... 6900/9803
2020-03-19T19:22:14.6732066Z .................................................................................................... 7000/9803
---
2020-03-19T19:24:03.3911503Z .................................................................................................... 7800/9803
2020-03-19T19:24:09.0748215Z .................................................................................................... 7900/9803
2020-03-19T19:24:15.3404038Z ..................................................................i................................. 8000/9803
2020-03-19T19:24:25.8541939Z .................................................................................................... 8100/9803
2020-03-19T19:24:31.8722004Z ...............iiiiiiiiii.i......................................................................... 8200/9803
2020-03-19T19:24:46.9748971Z .................................................................................................... 8400/9803
2020-03-19T19:24:53.7673033Z .................................................................................................... 8500/9803
2020-03-19T19:25:10.0681266Z .................................................................................................... 8600/9803
2020-03-19T19:25:17.2147289Z .................................................................................................... 8700/9803
---
2020-03-19T19:27:46.2834158Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-19T19:27:46.3049598Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T19:27:46.5203371Z 
2020-03-19T19:27:46.5203720Z running 183 tests
2020-03-19T19:27:49.4900455Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-19T19:27:52.2528473Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-19T19:27:52.2536400Z 
2020-03-19T19:27:52.2540551Z  finished in 5.949
2020-03-19T19:27:52.2548411Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-19T19:27:52.2739064Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T19:27:54.6258288Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-19T19:27:54.6479731Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T19:27:54.8119453Z 
2020-03-19T19:27:54.8120202Z running 9 tests
2020-03-19T19:27:54.8121425Z iiiiiiiii
2020-03-19T19:27:54.8124260Z 
2020-03-19T19:27:54.8129695Z  finished in 0.165
2020-03-19T19:27:54.8152040Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-19T19:27:54.8347280Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T19:28:17.0955741Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-19T19:28:17.1195744Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T19:28:17.3140613Z 
2020-03-19T19:28:17.3141309Z running 115 tests
2020-03-19T19:28:31.4448757Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-19T19:28:33.2766974Z ...iiii.....ii.
2020-03-19T19:28:33.2770829Z 
2020-03-19T19:28:33.2773741Z  finished in 16.157
2020-03-19T19:28:33.2778098Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-19T19:28:33.2781486Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T19:41:57.2411852Z 
2020-03-19T19:41:57.2414463Z    Doc-tests core
2020-03-19T19:42:02.0266361Z 
2020-03-19T19:42:02.0268168Z running 2481 tests
2020-03-19T19:42:11.6615140Z ......iiiii......................................................................................... 100/2481
2020-03-19T19:42:20.9492333Z .....................................................................................ii............. 200/2481
2020-03-19T19:42:42.3464259Z ....................i............................................................................... 400/2481
2020-03-19T19:42:42.3464259Z ....................i............................................................................... 400/2481
2020-03-19T19:42:52.6771152Z .........................................................................i..i..................iiii. 500/2481
2020-03-19T19:43:09.5613445Z .................................................................................................... 700/2481
2020-03-19T19:43:18.5213586Z .................................................................................................... 800/2481
2020-03-19T19:43:27.3779547Z .................................................................................................... 900/2481
2020-03-19T19:43:36.2517949Z .................................................................................................... 1000/2481
---
2020-03-19T19:47:22.0726096Z 
2020-03-19T19:47:22.0727817Z running 1012 tests
2020-03-19T19:47:40.6096897Z i................................................................................................... 100/1012
2020-03-19T19:47:51.3815776Z .................................................................................................... 200/1012
2020-03-19T19:47:58.7463069Z ..................iii......i......i...i......i...................................................... 300/1012
2020-03-19T19:48:11.1569515Z ............................................i....i......................................ii.......... 500/1012
2020-03-19T19:48:18.9904745Z .................................................................................................... 600/1012
2020-03-19T19:48:24.6463220Z .................................................................................................... 700/1012
2020-03-19T19:48:24.6463220Z .................................................................................................... 700/1012
2020-03-19T19:48:31.8192356Z ......................................iiii.......................................................... 800/1012
2020-03-19T19:48:46.4203069Z .................................................................................................... 900/1012
2020-03-19T19:48:53.6232293Z ............................................................iiii.................................... 1000/1012
2020-03-19T19:48:54.1241868Z test result: ok. 992 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-19T19:48:54.1243525Z 
2020-03-19T19:48:54.1354273Z  finished in 173.602
2020-03-19T19:48:54.1360509Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-19T20:09:00.9590963Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-03-19T20:09:00.9909246Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T20:09:01.2262102Z 
2020-03-19T20:09:01.2262381Z running 210 tests
2020-03-19T20:09:35.0078898Z ......................i...ii..................................F....................................i 100/210
2020-03-19T20:10:11.5692236Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-19T20:10:16.2471678Z failures:
2020-03-19T20:10:16.2480598Z 
2020-03-19T20:10:16.2481674Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-19T20:10:16.2482209Z 
2020-03-19T20:10:16.2482209Z 
2020-03-19T20:10:16.2482742Z error: make failed
2020-03-19T20:10:16.2483070Z status: exit code: 2
2020-03-19T20:10:16.2483303Z command: "make"
2020-03-19T20:10:16.2486715Z stdout:
2020-03-19T20:10:16.2487552Z ------------------------------------------
2020-03-19T20:10:16.2496560Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-19T20:10:16.2497154Z 
2020-03-19T20:10:16.2500283Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-19T20:10:16.2503219Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-19T20:10:16.2504387Z Makefile:4: recipe for target 'all' failed
2020-03-19T20:10:16.2505629Z ------------------------------------------
2020-03-19T20:10:16.2506190Z stderr:
2020-03-19T20:10:16.2507449Z ------------------------------------------
2020-03-19T20:10:16.2507449Z ------------------------------------------
2020-03-19T20:10:16.2508264Z warning: ignoring --out-dir flag due to -o flag
2020-03-19T20:10:16.2509433Z error[E0308]: mismatched types
2020-03-19T20:10:16.2510173Z   --> the_backend.rs:57:53
2020-03-19T20:10:16.2510657Z    |
2020-03-19T20:10:16.2510657Z    |
2020-03-19T20:10:16.2511185Z 57 |         providers.exported_symbols = |_tcx, _crate| Arc::new(Vec::new());
2020-03-19T20:10:16.2512428Z    |                                                     |
2020-03-19T20:10:16.2513107Z    |                                                     expected reference, found struct `std::sync::Arc`
2020-03-19T20:10:16.2513107Z    |                                                     expected reference, found struct `std::sync::Arc`
2020-03-19T20:10:16.2513960Z    |                                                     help: consider borrowing here: `&Arc::new(Vec::new())`
2020-03-19T20:10:16.2514545Z    |
2020-03-19T20:10:16.2515831Z    = note: expected reference `&[(rustc::middle::exported_symbols::ExportedSymbol<'_>, rustc::middle::exported_symbols::SymbolExportLevel)]`
2020-03-19T20:10:16.2517252Z 
2020-03-19T20:10:16.2517685Z error: aborting due to previous error
2020-03-19T20:10:16.2518063Z 
2020-03-19T20:10:16.2518830Z For more information about this error, try `rustc --explain E0308`.
2020-03-19T20:10:16.2518830Z For more information about this error, try `rustc --explain E0308`.
2020-03-19T20:10:16.2520020Z make: *** [all] Error 1
2020-03-19T20:10:16.2521000Z ------------------------------------------
2020-03-19T20:10:16.2521346Z 
2020-03-19T20:10:16.2521573Z 
2020-03-19T20:10:16.2521815Z 
---
2020-03-19T20:10:16.2525433Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T20:10:16.2526035Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T20:10:16.2526403Z 
2020-03-19T20:10:16.2526639Z 
2020-03-19T20:10:16.2536989Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T20:10:16.2544401Z 
2020-03-19T20:10:16.2544620Z 
2020-03-19T20:10:16.2545382Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T20:10:16.2546468Z Build completed unsuccessfully in 1:51:07
2020-03-19T20:10:16.2546468Z Build completed unsuccessfully in 1:51:07
2020-03-19T20:10:16.9286323Z == clock drift check ==
2020-03-19T20:10:16.9325136Z   local time: Thu Mar 19 20:10:16 UTC 2020
2020-03-19T20:10:17.2615725Z   network time: Thu, 19 Mar 2020 20:10:17 GMT
2020-03-19T20:10:17.2617233Z == end clock drift check ==
2020-03-19T20:10:19.3266273Z 
2020-03-19T20:10:19.3347109Z ##[error]Bash exited with code '1'.
2020-03-19T20:10:19.3379266Z ##[section]Finishing: Run build
2020-03-19T20:10:19.3433952Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70161/merge to s
2020-03-19T20:10:19.3439409Z Task         : Get sources
2020-03-19T20:10:19.3439759Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T20:10:19.3440065Z Version      : 1.0.0
2020-03-19T20:10:19.3440294Z Author       : Microsoft
2020-03-19T20:10:19.3440294Z Author       : Microsoft
2020-03-19T20:10:19.3440656Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T20:10:19.3441046Z ==============================================================================
2020-03-19T20:10:19.7182137Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T20:10:19.7230689Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70161/merge to s
2020-03-19T20:10:19.7327498Z Cleaning up task key
2020-03-19T20:10:19.7329092Z Start cleaning up orphan processes.
2020-03-19T20:10:19.7539946Z Terminate orphan process: pid (3440) (python)
2020-03-19T20:10:19.7817230Z ##[section]Finishing: Finalize Job
