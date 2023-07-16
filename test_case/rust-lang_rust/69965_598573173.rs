plain
2020-03-13T04:37:43.7945752Z ========================== Starting Command Output ===========================
2020-03-13T04:37:43.7948728Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/96d275d6-a7fb-4823-ba8b-d9b708f52267.sh
2020-03-13T04:37:43.7949666Z 
2020-03-13T04:37:43.7955777Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T04:37:43.7981391Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-13T04:37:43.7988314Z Task         : Get sources
2020-03-13T04:37:43.7988657Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T04:37:43.7988997Z Version      : 1.0.0
2020-03-13T04:37:43.7989253Z Author       : Microsoft
---
2020-03-13T04:37:44.7800405Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T04:37:44.7809470Z ##[command]git config gc.auto 0
2020-03-13T04:37:44.7817244Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T04:37:44.7824076Z ##[command]git config --get-all http.proxy
2020-03-13T04:37:44.7835006Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69965/merge:refs/remotes/pull/69965/merge
---
2020-03-13T05:39:51.4334726Z .................................................................................................... 1700/9766
2020-03-13T05:39:55.5015613Z .................................................................................................... 1800/9766
2020-03-13T05:40:07.2162274Z ................................................................i................................... 1900/9766
2020-03-13T05:40:14.2631541Z .................................................................................................... 2000/9766
2020-03-13T05:40:29.3061540Z ......................................................iiiii......................................... 2100/9766
2020-03-13T05:40:39.6322024Z .................................................................................................... 2300/9766
2020-03-13T05:40:41.8801421Z .................................................................................................... 2400/9766
2020-03-13T05:40:45.0889769Z .................................................................................................... 2500/9766
2020-03-13T05:41:07.4718492Z .................................................................................................... 2600/9766
---
2020-03-13T05:43:51.8872957Z .........................i...............i.......................................................... 5000/9766
2020-03-13T05:44:01.6416604Z .................................................................................................... 5100/9766
2020-03-13T05:44:07.4328706Z ....................................................................i............................... 5200/9766
2020-03-13T05:44:13.7607784Z .................................................................................................... 5300/9766
2020-03-13T05:44:23.1299995Z .................................................ii.ii........i...i................................. 5400/9766
2020-03-13T05:44:31.3444812Z .................................................................................................... 5600/9766
2020-03-13T05:44:41.0525336Z .................................................................................................... 5700/9766
2020-03-13T05:44:47.4731309Z ........................................i........................................................... 5800/9766
2020-03-13T05:44:53.7026925Z .................................................................................................... 5900/9766
2020-03-13T05:44:53.7026925Z .................................................................................................... 5900/9766
2020-03-13T05:45:04.2951246Z .................................................................................................... 6000/9766
2020-03-13T05:45:13.1214833Z .................................ii...i..ii...........i............................................. 6100/9766
2020-03-13T05:45:30.6365018Z .................................................................................................... 6300/9766
2020-03-13T05:45:37.4796800Z .................................................................................................... 6400/9766
2020-03-13T05:45:37.4796800Z .................................................................................................... 6400/9766
2020-03-13T05:45:48.5253346Z ................................................................i..ii............................... 6500/9766
2020-03-13T05:46:18.0769440Z .................................................................................................... 6700/9766
2020-03-13T05:46:23.4674803Z ..............................................................i..................................... 6800/9766
2020-03-13T05:46:25.5514808Z .................................................................................................... 6900/9766
2020-03-13T05:46:27.6163875Z ................................................................................................i... 7000/9766
---
2020-03-13T05:48:08.2901323Z .................................................................................................... 7700/9766
2020-03-13T05:48:12.7968601Z .................................................................................................... 7800/9766
2020-03-13T05:48:18.8687434Z .................................................................................................... 7900/9766
2020-03-13T05:48:25.0819761Z ..............................................i..................................................... 8000/9766
2020-03-13T05:48:35.2923336Z ...............................................................................................iiiii 8100/9766
2020-03-13T05:48:41.2858207Z iiiii.i............................................................................................. 8200/9766
2020-03-13T05:48:55.8238096Z .................................................................................................... 8400/9766
2020-03-13T05:49:06.8870782Z .................................................................................................... 8500/9766
2020-03-13T05:49:18.9533034Z .................................................................................................... 8600/9766
2020-03-13T05:49:24.6679369Z .................................................................................................... 8700/9766
---
2020-03-13T05:51:45.6415070Z  finished in 7.600
2020-03-13T05:51:45.6415660Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:51:45.6415944Z 
2020-03-13T05:51:45.6416113Z running 179 tests
2020-03-13T05:51:48.4652260Z iiii......i...........ii..iiii....i....i...........i............i..i..................i....i........ 100/179
2020-03-13T05:51:50.8712995Z ....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-03-13T05:51:50.8720792Z 
2020-03-13T05:51:50.8726748Z  finished in 5.517
2020-03-13T05:51:50.8906459Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:51:51.0589478Z 
---
2020-03-13T05:51:52.9864527Z  finished in 2.095
2020-03-13T05:51:53.0059714Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:51:53.1657156Z 
2020-03-13T05:51:53.1657978Z running 9 tests
2020-03-13T05:51:53.1659339Z iiiiiiiii
2020-03-13T05:51:53.1660900Z 
2020-03-13T05:51:53.1661210Z  finished in 0.159
2020-03-13T05:51:53.1849208Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:51:53.3861764Z 
---
2020-03-13T05:52:13.4741888Z  finished in 20.289
2020-03-13T05:52:13.4965413Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:52:13.7059934Z 
2020-03-13T05:52:13.7060492Z running 115 tests
2020-03-13T05:52:27.7131847Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-13T05:52:29.4004056Z ...iiii.....ii.
2020-03-13T05:52:29.4005579Z 
2020-03-13T05:52:29.4005736Z  finished in 15.904
2020-03-13T05:52:29.4011973Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T05:52:29.4013219Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-13T06:06:00.7380474Z 
2020-03-13T06:06:00.7381849Z    Doc-tests core
2020-03-13T06:06:05.7043167Z 
2020-03-13T06:06:05.7044828Z running 2480 tests
2020-03-13T06:06:14.9020590Z ......iiiii......................................................................................... 100/2480
2020-03-13T06:06:24.0334214Z ....................................................................................ii.............. 200/2480
2020-03-13T06:06:44.7637834Z ...................i................................................................................ 400/2480
2020-03-13T06:06:44.7637834Z ...................i................................................................................ 400/2480
2020-03-13T06:06:54.5040208Z ........................................................................i..i..................iiii.. 500/2480
2020-03-13T06:07:10.9972177Z .................................................................................................... 700/2480
2020-03-13T06:07:19.4264957Z .................................................................................................... 800/2480
2020-03-13T06:07:27.8232240Z .................................................................................................... 900/2480
2020-03-13T06:07:36.4059508Z .................................................................................................... 1000/2480
---
2020-03-13T06:11:04.0093394Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-03-13T06:11:04.0103577Z ... 300/760
2020-03-13T06:11:04.1182135Z .................................................................................................... 400/760
2020-03-13T06:11:06.1843949Z .................................................................................................... 500/760
2020-03-13T06:11:06.2250876Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-13T06:11:06.2270419Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-03-13T06:11:06.2289336Z ....thread '.<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError.', thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-03-13T06:11:06.2304647Z ......src/libstd/sync/mpsc/mod.rs:.2778:.21.
2020-03-13T06:11:06.4744037Z .................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-13T06:11:06.4772350Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-03-13T06:11:06.4817417Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-13T06:11:06.4945819Z ................... 600/760
2020-03-13T06:11:08.5253219Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-13T06:11:08.5254795Z .......thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-03-13T06:11:17.5333313Z 
2020-03-13T06:11:17.5333663Z running 1010 tests
2020-03-13T06:11:35.9068568Z i................................................................................................... 100/1010
2020-03-13T06:11:46.3986599Z .................................................................................................... 200/1010
2020-03-13T06:11:53.8557812Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-13T06:11:59.1024968Z .................................................................................................... 400/1010
2020-03-13T06:12:06.1633557Z ............................................i..i......................................ii............ 500/1010
2020-03-13T06:12:19.0458562Z .................................................................................................... 700/1010
2020-03-13T06:12:19.0458562Z .................................................................................................... 700/1010
2020-03-13T06:12:26.0692241Z ....................................iiii............................................................ 800/1010
2020-03-13T06:12:40.4086446Z .................................................................................................... 900/1010
2020-03-13T06:12:47.3238355Z ..........................................................iiii...................................... 1000/1010
2020-03-13T06:12:47.7976542Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-13T06:12:47.7976822Z 
2020-03-13T06:12:47.8078334Z  finished in 171.825
2020-03-13T06:12:47.8093012Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-13T06:31:52.6929402Z  finished in 43.350
2020-03-13T06:31:52.7187867Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T06:31:52.9332467Z 
2020-03-13T06:31:52.9332869Z running 210 tests
2020-03-13T06:32:27.1565511Z ......................i...ii.................................F.....................................i 100/210
2020-03-13T06:33:08.3962724Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-13T06:33:12.9783043Z failures:
2020-03-13T06:33:12.9790105Z 
2020-03-13T06:33:12.9792234Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-13T06:33:12.9792505Z 
2020-03-13T06:33:12.9792505Z 
2020-03-13T06:33:12.9792682Z error: make failed
2020-03-13T06:33:12.9792905Z status: exit code: 2
2020-03-13T06:33:12.9793146Z command: "make"
2020-03-13T06:33:12.9793352Z stdout:
2020-03-13T06:33:12.9793816Z ------------------------------------------
2020-03-13T06:33:12.9794181Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-13T06:33:12.9794413Z 
2020-03-13T06:33:12.9796906Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-13T06:33:12.9799603Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-13T06:33:12.9800549Z Makefile:4: recipe for target 'all' failed
2020-03-13T06:33:12.9801422Z ------------------------------------------
2020-03-13T06:33:12.9801665Z stderr:
2020-03-13T06:33:12.9802110Z ------------------------------------------
2020-03-13T06:33:12.9802497Z error[E0432]: unresolved import `rustc_codegen_utils::codegen_backend`
2020-03-13T06:33:12.9802497Z error[E0432]: unresolved import `rustc_codegen_utils::codegen_backend`
2020-03-13T06:33:12.9803038Z   --> the_backend.rs:23:26
2020-03-13T06:33:12.9803265Z    |
2020-03-13T06:33:12.9803555Z 23 | use rustc_codegen_utils::codegen_backend::CodegenBackend;
2020-03-13T06:33:12.9804041Z    |                          ^^^^^^^^^^^^^^^ could not find `codegen_backend` in `rustc_codegen_utils`
2020-03-13T06:33:12.9804374Z 
2020-03-13T06:33:12.9804798Z warning: ignoring --out-dir flag due to -o flag
2020-03-13T06:33:12.9805220Z error: aborting due to previous error
2020-03-13T06:33:12.9805437Z 
2020-03-13T06:33:12.9805928Z For more information about this error, try `rustc --explain E0432`.
2020-03-13T06:33:12.9805928Z For more information about this error, try `rustc --explain E0432`.
2020-03-13T06:33:12.9806246Z make: *** [all] Error 1
2020-03-13T06:33:12.9806828Z ------------------------------------------
2020-03-13T06:33:12.9807026Z 
2020-03-13T06:33:12.9807136Z 
2020-03-13T06:33:12.9807244Z 
2020-03-13T06:33:12.9807244Z 
2020-03-13T06:33:12.9807406Z failures:
2020-03-13T06:33:12.9807869Z     [run-make] run-make-fulldeps/hotplug_codegen_backend
2020-03-13T06:33:12.9808095Z 
2020-03-13T06:33:12.9808659Z test result: FAILED. 194 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
2020-03-13T06:33:12.9808982Z 
2020-03-13T06:33:12.9813171Z 
2020-03-13T06:33:12.9813375Z 
2020-03-13T06:33:12.9824699Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T06:33:12.9832384Z 
2020-03-13T06:33:12.9832499Z 
2020-03-13T06:33:12.9832768Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T06:33:12.9833145Z Build completed unsuccessfully in 1:49:55
2020-03-13T06:33:12.9833145Z Build completed unsuccessfully in 1:49:55
2020-03-13T06:33:12.9833911Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T06:33:12.9834393Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T06:33:12.9884940Z == clock drift check ==
2020-03-13T06:33:12.9897443Z   local time: Fri Mar 13 06:33:12 UTC 2020
2020-03-13T06:33:13.2724725Z   network time: Fri, 13 Mar 2020 06:33:13 GMT
2020-03-13T06:33:13.2725217Z == end clock drift check ==
2020-03-13T06:33:15.3799965Z 
2020-03-13T06:33:15.3859795Z ##[error]Bash exited with code '1'.
2020-03-13T06:33:15.3908738Z ##[section]Finishing: Run build
2020-03-13T06:33:15.3974500Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-13T06:33:15.3980188Z Task         : Get sources
2020-03-13T06:33:15.3980563Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T06:33:15.3980889Z Version      : 1.0.0
2020-03-13T06:33:15.3981116Z Author       : Microsoft
2020-03-13T06:33:15.3981116Z Author       : Microsoft
2020-03-13T06:33:15.3981503Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T06:33:15.3981920Z ==============================================================================
2020-03-13T06:33:15.7636869Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T06:33:15.7681783Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-13T06:33:15.7775336Z Cleaning up task key
2020-03-13T06:33:15.7776615Z Start cleaning up orphan processes.
2020-03-13T06:33:15.7955844Z Terminate orphan process: pid (3783) (python)
2020-03-13T06:33:15.8605163Z ##[section]Finishing: Finalize Job
