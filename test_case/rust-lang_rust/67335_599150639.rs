plain
2020-03-14T22:20:54.7395104Z ========================== Starting Command Output ===========================
2020-03-14T22:20:54.7400958Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/29976989-c722-48be-8fe6-b9d9e5dceb70.sh
2020-03-14T22:20:54.7401271Z 
2020-03-14T22:20:54.7407425Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T22:20:54.7430103Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67335/merge to s
2020-03-14T22:20:54.7434232Z Task         : Get sources
2020-03-14T22:20:54.7434564Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T22:20:54.7434912Z Version      : 1.0.0
2020-03-14T22:20:54.7435137Z Author       : Microsoft
---
2020-03-14T22:20:55.7256193Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T22:20:55.7262479Z ##[command]git config gc.auto 0
2020-03-14T22:20:55.7267361Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T22:20:55.7271319Z ##[command]git config --get-all http.proxy
2020-03-14T22:20:55.7278730Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67335/merge:refs/remotes/pull/67335/merge
---
2020-03-14T23:21:07.3187518Z .................................................................................................... 1700/9771
2020-03-14T23:21:11.5758163Z .................................................................................................... 1800/9771
2020-03-14T23:21:23.0941939Z ...................................................................i................................ 1900/9771
2020-03-14T23:21:29.7781002Z .................................................................................................... 2000/9771
2020-03-14T23:21:43.6848277Z .........................................................iiiii...................................... 2100/9771
2020-03-14T23:21:54.1641287Z .................................................................................................... 2300/9771
2020-03-14T23:21:56.2910818Z .................................................................................................... 2400/9771
2020-03-14T23:21:59.2948631Z .................................................................................................... 2500/9771
2020-03-14T23:22:20.9513919Z .................................................................................................... 2600/9771
---
2020-03-14T23:25:00.1342210Z .............................i...............i...................................................... 5000/9771
2020-03-14T23:25:09.4337727Z .................................................................................................... 5100/9771
2020-03-14T23:25:15.4727928Z ........................................................................i........................... 5200/9771
2020-03-14T23:25:20.9949017Z .................................................................................................... 5300/9771
2020-03-14T23:25:30.4691758Z .....................................................ii.ii........i...i............................. 5400/9771
2020-03-14T23:25:38.2286339Z .................................................................................................... 5600/9771
2020-03-14T23:25:47.6769776Z .................................................................................................... 5700/9771
2020-03-14T23:25:53.7431994Z .............................................i...................................................... 5800/9771
2020-03-14T23:26:00.0170309Z .................................................................................................... 5900/9771
2020-03-14T23:26:00.0170309Z .................................................................................................... 5900/9771
2020-03-14T23:26:10.0444453Z .................................................................................................... 6000/9771
2020-03-14T23:26:15.7797661Z .......................................ii...i..ii...........i....................................... 6100/9771
2020-03-14T23:26:35.9387543Z .................................................................................................... 6300/9771
2020-03-14T23:26:42.6612137Z .................................................................................................... 6400/9771
2020-03-14T23:26:42.6612137Z .................................................................................................... 6400/9771
2020-03-14T23:26:52.1834410Z .....................................................................i..ii.......................... 6500/9771
2020-03-14T23:27:15.0019321Z .................................................................................................... 6700/9771
2020-03-14T23:27:23.9703643Z ...................................................................i................................ 6800/9771
2020-03-14T23:27:26.0036398Z .................................................................................................... 6900/9771
2020-03-14T23:27:28.1821772Z .................................................................................................... 7000/9771
---
2020-03-14T23:29:12.6444431Z .................................................................................................... 7800/9771
2020-03-14T23:29:18.3746756Z .................................................................................................... 7900/9771
2020-03-14T23:29:24.1717333Z ...................................................i................................................ 8000/9771
2020-03-14T23:29:34.3936556Z .................................................................................................... 8100/9771
2020-03-14T23:29:39.6760547Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-14T23:29:53.1185037Z .................................................................................................... 8400/9771
2020-03-14T23:30:03.7374692Z .................................................................................................... 8500/9771
2020-03-14T23:30:16.1949866Z .................................................................................................... 8600/9771
2020-03-14T23:30:21.7779439Z .................................................................................................... 8700/9771
---
2020-03-14T23:32:40.8931163Z  finished in 7.642
2020-03-14T23:32:40.9121517Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:32:41.0842849Z 
2020-03-14T23:32:41.0844006Z running 181 tests
2020-03-14T23:32:44.0105067Z iiii......i............ii..iiii....i....i...........i............i..i..................i....i....... 100/181
2020-03-14T23:32:46.5368509Z .....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-14T23:32:46.5374191Z 
2020-03-14T23:32:46.5374377Z  finished in 5.625
2020-03-14T23:32:46.5567873Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:32:46.7354602Z 
---
2020-03-14T23:32:48.7134308Z  finished in 2.156
2020-03-14T23:32:48.7333076Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:32:48.8782369Z 
2020-03-14T23:32:48.8796665Z running 9 tests
2020-03-14T23:32:48.8798310Z iiiiiiiii
2020-03-14T23:32:48.8799460Z 
2020-03-14T23:32:48.8799656Z  finished in 0.144
2020-03-14T23:32:48.8955376Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:32:49.0435176Z 
---
2020-03-14T23:33:09.1409718Z  finished in 20.245
2020-03-14T23:33:09.1612636Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:33:09.3421634Z 
2020-03-14T23:33:09.3422111Z running 115 tests
2020-03-14T23:33:22.4999572Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-14T23:33:24.1323512Z ...iiii.....ii.
2020-03-14T23:33:24.1326007Z 
2020-03-14T23:33:24.1326220Z  finished in 14.971
2020-03-14T23:33:24.1333736Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-14T23:33:24.1334503Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-14T23:46:22.7245272Z 
2020-03-14T23:46:22.7246452Z    Doc-tests core
2020-03-14T23:46:27.5095393Z 
2020-03-14T23:46:27.5096345Z running 2480 tests
2020-03-14T23:46:36.8526077Z ......iiiii......................................................................................... 100/2480
2020-03-14T23:46:46.3069420Z ....................................................................................ii.............. 200/2480
2020-03-14T23:47:08.4952914Z ...................i................................................................................ 400/2480
2020-03-14T23:47:08.4952914Z ...................i................................................................................ 400/2480
2020-03-14T23:47:18.6729265Z ........................................................................i..i..................iiii.. 500/2480
2020-03-14T23:47:35.9953149Z .................................................................................................... 700/2480
2020-03-14T23:47:44.6333035Z .................................................................................................... 800/2480
2020-03-14T23:47:53.2352273Z .................................................................................................... 900/2480
2020-03-14T23:48:01.8436310Z .................................................................................................... 1000/2480
---
2020-03-14T23:51:34.4963060Z 
2020-03-14T23:51:34.4964063Z running 1010 tests
2020-03-14T23:51:51.9937887Z i................................................................................................... 100/1010
2020-03-14T23:52:01.9224233Z .................................................................................................... 200/1010
2020-03-14T23:52:08.8453676Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-14T23:52:13.8984638Z .................................................................................................... 400/1010
2020-03-14T23:52:20.4764168Z ............................................i..i......................................ii............ 500/1010
2020-03-14T23:52:32.6873371Z .................................................................................................... 700/1010
2020-03-14T23:52:32.6873371Z .................................................................................................... 700/1010
2020-03-14T23:52:39.2756071Z ....................................iiii............................................................ 800/1010
2020-03-14T23:52:53.1551241Z .................................................................................................... 900/1010
2020-03-14T23:52:59.7463223Z ..........................................................iiii...................................... 1000/1010
2020-03-14T23:53:00.1658943Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-14T23:53:00.1659210Z 
2020-03-14T23:53:00.1747641Z  finished in 163.742
2020-03-14T23:53:00.1762539Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-15T00:11:03.3875786Z  finished in 39.637
2020-03-15T00:11:03.4130834Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T00:11:03.5701689Z 
2020-03-15T00:11:03.5702117Z running 210 tests
2020-03-15T00:11:35.1103380Z ......................i...ii.......................................................................i 100/210
2020-03-15T00:12:13.5140219Z ....................F...................iiiiii......i..............iii.............................. 200/210
2020-03-15T00:12:14.4236508Z failures:
2020-03-15T00:12:14.4241112Z 
2020-03-15T00:12:14.4241686Z ---- [run-make] run-make-fulldeps/min-global-align stdout ----
2020-03-15T00:12:14.4241878Z 
2020-03-15T00:12:14.4241878Z 
2020-03-15T00:12:14.4242037Z error: make failed
2020-03-15T00:12:14.4242212Z status: exit code: 2
2020-03-15T00:12:14.4242379Z command: "make"
2020-03-15T00:12:14.4242518Z stdout:
2020-03-15T00:12:14.4243034Z ------------------------------------------
2020-03-15T00:12:14.4246016Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align  --target=i686-unknown-linux-gnu --emit=llvm-ir min_global_align.rs
2020-03-15T00:12:14.4248658Z Makefile:15: recipe for target 'all' failed
2020-03-15T00:12:14.4249709Z ------------------------------------------
2020-03-15T00:12:14.4250063Z stderr:
2020-03-15T00:12:14.4251038Z ------------------------------------------
2020-03-15T00:12:14.4251398Z error[E0119]: conflicting implementations of trait `Freeze` for type `bool`:
2020-03-15T00:12:14.4251398Z error[E0119]: conflicting implementations of trait `Freeze` for type `bool`:
2020-03-15T00:12:14.4252245Z   --> min_global_align.rs:23:1
2020-03-15T00:12:14.4252847Z    |
2020-03-15T00:12:14.4253022Z 20 | impl Freeze for bool {}
2020-03-15T00:12:14.4253677Z    | -------------------- first implementation here
2020-03-15T00:12:14.4253875Z ...
2020-03-15T00:12:14.4254457Z 23 | impl<T: ?Sized> Freeze for T {}
2020-03-15T00:12:14.4255076Z 
2020-03-15T00:12:14.4255241Z error: aborting due to previous error
2020-03-15T00:12:14.4255408Z 
2020-03-15T00:12:14.4256316Z For more information about this error, try `rustc --explain E0119`.
2020-03-15T00:12:14.4256316Z For more information about this error, try `rustc --explain E0119`.
2020-03-15T00:12:14.4256565Z make: *** [all] Error 1
2020-03-15T00:12:14.4257238Z ------------------------------------------
2020-03-15T00:12:14.4257395Z 
2020-03-15T00:12:14.4257481Z 
2020-03-15T00:12:14.4257576Z 
---
2020-03-15T00:12:14.4261118Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-15T00:12:14.4261500Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-15T00:12:14.4261708Z 
2020-03-15T00:12:14.4261819Z 
2020-03-15T00:12:14.4271638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-15T00:12:14.4278238Z 
2020-03-15T00:12:14.4278341Z 
2020-03-15T00:12:14.4278563Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-15T00:12:14.4278939Z Build completed unsuccessfully in 1:45:31
2020-03-15T00:12:14.4278939Z Build completed unsuccessfully in 1:45:31
2020-03-15T00:12:14.4373173Z == clock drift check ==
2020-03-15T00:12:14.4398499Z   local time: Sun Mar 15 00:12:14 UTC 2020
2020-03-15T00:12:14.6120443Z   network time: Sun, 15 Mar 2020 00:12:14 GMT
2020-03-15T00:12:14.6122421Z == end clock drift check ==
2020-03-15T00:12:16.7886171Z 
2020-03-15T00:12:16.7972262Z ##[error]Bash exited with code '1'.
2020-03-15T00:12:16.7988108Z ##[section]Finishing: Run build
2020-03-15T00:12:16.8039524Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67335/merge to s
2020-03-15T00:12:16.8044918Z Task         : Get sources
2020-03-15T00:12:16.8045229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T00:12:16.8045510Z Version      : 1.0.0
2020-03-15T00:12:16.8045749Z Author       : Microsoft
2020-03-15T00:12:16.8045749Z Author       : Microsoft
2020-03-15T00:12:16.8046064Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-15T00:12:16.8046421Z ==============================================================================
2020-03-15T00:12:17.1559005Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-15T00:12:17.1609870Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67335/merge to s
2020-03-15T00:12:17.1707979Z Cleaning up task key
2020-03-15T00:12:17.1715832Z Start cleaning up orphan processes.
2020-03-15T00:12:17.1939342Z Terminate orphan process: pid (4028) (python)
2020-03-15T00:12:17.2202251Z ##[section]Finishing: Finalize Job
