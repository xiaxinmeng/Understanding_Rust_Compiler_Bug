plain
2020-02-13T20:37:17.0471923Z ========================== Starting Command Output ===========================
2020-02-13T20:37:17.0474993Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1513ac0c-4b88-4f18-b39a-9fab6aadfad1.sh
2020-02-13T20:37:17.0475174Z 
2020-02-13T20:37:17.0479949Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-13T20:37:17.0486341Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69146/merge to s
2020-02-13T20:37:17.0505436Z Task         : Get sources
2020-02-13T20:37:17.0505468Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T20:37:17.0505499Z Version      : 1.0.0
2020-02-13T20:37:17.0505571Z Author       : Microsoft
---
2020-02-13T20:37:17.7968635Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-13T20:37:17.8062188Z ##[command]git config gc.auto 0
2020-02-13T20:37:17.8133262Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-13T20:37:17.8184297Z ##[command]git config --get-all http.proxy
2020-02-13T20:37:17.8331380Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69146/merge:refs/remotes/pull/69146/merge
---
2020-02-13T21:31:28.9944365Z .................................................................................................... 1700/9636
2020-02-13T21:31:33.3763720Z .................................................................................................... 1800/9636
2020-02-13T21:31:44.1800130Z ..............................i..................................................................... 1900/9636
2020-02-13T21:31:51.2501397Z .................................................................................................... 2000/9636
2020-02-13T21:32:04.3209042Z ....................iiiii........................................................................... 2100/9636
2020-02-13T21:32:13.6043429Z .................................................................................................... 2300/9636
2020-02-13T21:32:15.8570966Z .................................................................................................... 2400/9636
2020-02-13T21:32:20.4592392Z .................................................................................................... 2500/9636
2020-02-13T21:32:39.6172891Z .................................................................................................... 2600/9636
---
2020-02-13T21:35:03.8696364Z ........................................................................i...............i........... 4900/9636
2020-02-13T21:35:10.9648563Z .................................................................................................... 5000/9636
2020-02-13T21:35:18.2707665Z .................................................................................................... 5100/9636
2020-02-13T21:35:22.5365818Z ..............i..................................................................................... 5200/9636
2020-02-13T21:35:32.7173810Z ........................................................................................ii.ii....... 5300/9636
2020-02-13T21:35:36.4636094Z .i...i.............................................................................................. 5400/9636
2020-02-13T21:35:45.8210504Z .................................................................................................... 5600/9636
2020-02-13T21:35:54.9145003Z .............................................................................i...................... 5700/9636
2020-02-13T21:36:01.7301724Z .................................................................................................... 5800/9636
2020-02-13T21:36:07.3294397Z ...........................................................................i........................ 5900/9636
2020-02-13T21:36:07.3294397Z ...........................................................................i........................ 5900/9636
2020-02-13T21:36:16.3511634Z .....................................................................ii...i..ii...........i......... 6000/9636
2020-02-13T21:36:36.2852902Z .................................................................................................... 6200/9636
2020-02-13T21:36:43.4376181Z .................................................................................................... 6300/9636
2020-02-13T21:36:50.9801173Z .................................................................................................i.. 6400/9636
2020-02-13T21:37:05.5336402Z ii.................................................................................................. 6500/9636
---
2020-02-13T21:39:00.9123779Z .................................................................................................... 7600/9636
2020-02-13T21:39:04.9652766Z .................................................................................................... 7700/9636
2020-02-13T21:39:10.5920741Z .................................................................................................... 7800/9636
2020-02-13T21:39:17.8552753Z .................................................................................................... 7900/9636
2020-02-13T21:39:25.8641364Z ...........................................................................iiiiiii.i................ 8000/9636
2020-02-13T21:39:40.7577719Z ...............i......i............................................................................. 8200/9636
2020-02-13T21:39:45.9954294Z .................................................................................................... 8300/9636
2020-02-13T21:40:00.4305992Z .................................................................................................... 8400/9636
2020-02-13T21:40:08.7225810Z .................................................................................................... 8500/9636
---
2020-02-13T21:42:29.5178718Z  finished in 7.353
2020-02-13T21:42:29.5351256Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:42:29.6909387Z 
2020-02-13T21:42:29.6911092Z running 178 tests
2020-02-13T21:42:32.6523162Z iiii.......i..........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-13T21:42:34.9661493Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-13T21:42:34.9662100Z 
2020-02-13T21:42:34.9665986Z  finished in 5.431
2020-02-13T21:42:34.9852196Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:42:35.1367318Z 
---
2020-02-13T21:42:37.1099081Z  finished in 2.124
2020-02-13T21:42:37.1284126Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:42:37.2730422Z 
2020-02-13T21:42:37.2730712Z running 9 tests
2020-02-13T21:42:37.2732315Z iiiiiiiii
2020-02-13T21:42:37.2732772Z 
2020-02-13T21:42:37.2732822Z  finished in 0.144
2020-02-13T21:42:37.2911982Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:42:37.4415359Z 
---
2020-02-13T21:42:56.8634461Z  finished in 19.572
2020-02-13T21:42:56.8819082Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:42:57.0327750Z 
2020-02-13T21:42:57.0328644Z running 116 tests
2020-02-13T21:43:10.1255341Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-13T21:43:12.0072617Z ....iiii.....ii.
2020-02-13T21:43:12.0076906Z 
2020-02-13T21:43:12.0081204Z  finished in 15.126
2020-02-13T21:43:12.0086181Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T21:43:12.0089685Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-13T21:55:43.2413515Z 
2020-02-13T21:55:43.2414794Z    Doc-tests core
2020-02-13T21:55:47.6307152Z 
2020-02-13T21:55:47.6330460Z running 2471 tests
2020-02-13T21:55:56.1134828Z ......iiiii......................................................................................... 100/2471
2020-02-13T21:56:04.6376954Z ..................................................................................ii................ 200/2471
2020-02-13T21:56:24.2790573Z .................i.................................................................................. 400/2471
2020-02-13T21:56:24.2790573Z .................i.................................................................................. 400/2471
2020-02-13T21:56:33.4662919Z ......................................................................i..i..................iiii.... 500/2471
2020-02-13T21:56:49.5488172Z .................................................................................................... 700/2471
2020-02-13T21:56:57.6986841Z .................................................................................................... 800/2471
2020-02-13T21:57:05.7524636Z .................................................................................................... 900/2471
2020-02-13T21:57:13.8625347Z .................................................................................................... 1000/2471
---
2020-02-13T22:00:16.4052891Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-02-13T22:00:16.4061222Z ... 300/760
2020-02-13T22:00:16.4937562Z .................................................................................................... 400/760
2020-02-13T22:00:18.5601052Z .................................................................................................... 500/760
2020-02-13T22:00:18.5951275Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-13T22:00:18.5967038Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-13T22:00:18.5979408Z thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-02-13T22:00:18.5995902Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-02-13T22:00:18.8354835Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-13T22:00:18.8384706Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-02-13T22:00:18.8387146Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-13T22:00:18.8942175Z .......................... 600/760
2020-02-13T22:00:20.9351817Z .....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-13T22:00:20.9353329Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-02-13T22:00:29.8489018Z 
2020-02-13T22:00:29.8489412Z running 1009 tests
2020-02-13T22:00:46.0128850Z i................................................................................................... 100/1009
2020-02-13T22:00:55.1141229Z .................................................................................................... 200/1009
2020-02-13T22:01:01.3526261Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-13T22:01:05.8410368Z .................................................................................................... 400/1009
2020-02-13T22:01:11.8056667Z ............................................i..i.....................................ii............. 500/1009
2020-02-13T22:01:23.0449975Z .................................................................................................... 700/1009
2020-02-13T22:01:23.0449975Z .................................................................................................... 700/1009
2020-02-13T22:01:28.9780147Z ...................................iiii............................................................. 800/1009
2020-02-13T22:01:41.8166064Z .................................................................................................... 900/1009
2020-02-13T22:01:47.8503527Z .........................................................iiii....................................... 1000/1009
2020-02-13T22:01:48.2063624Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-13T22:01:48.2064013Z 
2020-02-13T22:01:48.2118315Z  finished in 152.820
2020-02-13T22:01:48.2129682Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-13T22:19:44.6596997Z  finished in 38.399
2020-02-13T22:19:44.6838826Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-13T22:19:44.8776476Z 
2020-02-13T22:19:44.8777108Z running 208 tests
2020-02-13T22:20:17.0320088Z ......................i...ii.....................................................................i.. 100/208
2020-02-13T22:20:52.9472320Z ..................F...................iiiiii......i..............iii................................ 200/208
2020-02-13T22:20:53.6459865Z failures:
2020-02-13T22:20:53.6465077Z 
2020-02-13T22:20:53.6465865Z ---- [run-make] run-make-fulldeps/min-global-align stdout ----
2020-02-13T22:20:53.6465930Z 
2020-02-13T22:20:53.6465930Z 
2020-02-13T22:20:53.6465990Z error: make failed
2020-02-13T22:20:53.6466146Z status: exit code: 2
2020-02-13T22:20:53.6466186Z command: "make"
2020-02-13T22:20:53.6466225Z stdout:
2020-02-13T22:20:53.6466520Z ------------------------------------------
2020-02-13T22:20:53.6469880Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align  --target=i686-unknown-linux-gnu --emit=llvm-ir min_global_align.rs
2020-02-13T22:20:53.6470379Z Makefile:15: recipe for target 'all' failed
2020-02-13T22:20:53.6470652Z ------------------------------------------
2020-02-13T22:20:53.6470715Z stderr:
2020-02-13T22:20:53.6470924Z ------------------------------------------
2020-02-13T22:20:53.6470977Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
---
2020-02-13T22:20:53.6471576Z 
2020-02-13T22:20:53.6471614Z error: aborting due to previous error
2020-02-13T22:20:53.6471640Z 
2020-02-13T22:20:53.6471897Z For more information about this error, try `rustc --explain E0492`.
2020-02-13T22:20:53.6471943Z make: *** [all] Error 1
2020-02-13T22:20:53.6472173Z ------------------------------------------
2020-02-13T22:20:53.6472218Z 
2020-02-13T22:20:53.6472242Z 
2020-02-13T22:20:53.6472272Z 
---
2020-02-13T22:20:53.6477253Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-13T22:20:53.6477344Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-13T22:20:53.6477415Z 
2020-02-13T22:20:53.6477439Z 
2020-02-13T22:20:53.6481297Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-13T22:20:53.6482074Z 
2020-02-13T22:20:53.6482101Z 
2020-02-13T22:20:53.6488540Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-13T22:20:53.6488614Z Build completed unsuccessfully in 1:37:38
2020-02-13T22:20:53.6488614Z Build completed unsuccessfully in 1:37:38
2020-02-13T22:20:53.6537165Z == clock drift check ==
2020-02-13T22:20:53.6556504Z   local time: Thu Feb 13 22:20:53 UTC 2020
2020-02-13T22:20:53.9452438Z   network time: Thu, 13 Feb 2020 22:20:53 GMT
2020-02-13T22:20:53.9455857Z == end clock drift check ==
2020-02-13T22:20:55.0348212Z 
2020-02-13T22:20:55.0407115Z ##[error]Bash exited with code '1'.
2020-02-13T22:20:55.0423763Z ##[section]Finishing: Run build
2020-02-13T22:20:55.0443658Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69146/merge to s
2020-02-13T22:20:55.0445543Z Task         : Get sources
2020-02-13T22:20:55.0445586Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T22:20:55.0445803Z Version      : 1.0.0
2020-02-13T22:20:55.0445839Z Author       : Microsoft
2020-02-13T22:20:55.0445839Z Author       : Microsoft
2020-02-13T22:20:55.0445880Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T22:20:55.0445941Z ==============================================================================
2020-02-13T22:20:55.4211325Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T22:20:55.4244446Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69146/merge to s
2020-02-13T22:20:55.4340911Z Cleaning up task key
2020-02-13T22:20:55.4341590Z Start cleaning up orphan processes.
2020-02-13T22:20:55.4433220Z Terminate orphan process: pid (3944) (python)
2020-02-13T22:20:55.4665993Z ##[section]Finishing: Finalize Job
