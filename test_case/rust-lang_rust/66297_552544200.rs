plain
2019-11-11T15:58:22.5568065Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T15:58:22.5781574Z ##[command]git config gc.auto 0
2019-11-11T15:58:22.5875127Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T15:58:22.5932591Z ##[command]git config --get-all http.proxy
2019-11-11T15:58:22.6080093Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66297/merge:refs/remotes/pull/66297/merge
---
2019-11-11T16:57:14.4829283Z .................................................................................................... 1400/9228
2019-11-11T16:57:21.1632356Z .................................................................................................... 1500/9228
2019-11-11T16:57:27.6208706Z .................................................................................................... 1600/9228
2019-11-11T16:57:37.2701601Z .................................................................................................... 1700/9228
2019-11-11T16:57:46.1326280Z ..i................................................................................................. 1800/9228
2019-11-11T16:57:53.5465400Z ......................................................................................iiiii......... 1900/9228
2019-11-11T16:58:15.6840749Z .................................................................................................... 2100/9228
2019-11-11T16:58:18.2461238Z .................................................................................................... 2200/9228
2019-11-11T16:58:20.9353574Z .................................................................................................... 2300/9228
2019-11-11T16:58:30.9949624Z .................................................................................................... 2400/9228
---
2019-11-11T17:01:29.4555439Z ..................................................................................i...............i. 4700/9228
2019-11-11T17:01:36.5756564Z .................................................................................................... 4800/9228
2019-11-11T17:01:45.4924955Z .................................................................................................... 4900/9228
2019-11-11T17:01:50.6845937Z .................................................................................................... 5000/9228
2019-11-11T17:02:01.7837483Z .....................................................................................ii.ii.......... 5100/9228
2019-11-11T17:02:05.6046607Z .i.................................................................................................. 5200/9228
2019-11-11T17:02:19.7587949Z .................................................................................................... 5400/9228
2019-11-11T17:02:26.6091617Z ...................................................................i................................ 5500/9228
2019-11-11T17:02:33.8334110Z .................................................................................................... 5600/9228
2019-11-11T17:02:41.5219164Z .................................................................................................... 5700/9228
2019-11-11T17:02:41.5219164Z .................................................................................................... 5700/9228
2019-11-11T17:02:50.5113883Z ....................................................ii...i..ii...........i.......................... 5800/9228
2019-11-11T17:03:12.5027250Z .................................................................................................... 6000/9228
2019-11-11T17:03:20.0754274Z .................................................................................................... 6100/9228
2019-11-11T17:03:20.0754274Z .................................................................................................... 6100/9228
2019-11-11T17:03:24.9332066Z .......................................................................i..ii........................ 6200/9228
2019-11-11T17:03:53.5445233Z .................................................................................................... 6400/9228
2019-11-11T17:03:55.6029666Z .......................................i............................................................ 6500/9228
2019-11-11T17:03:57.7428862Z .................................................................................................... 6600/9228
2019-11-11T17:03:59.9879980Z .......................i............................................................................ 6700/9228
---
2019-11-11T17:09:07.0054596Z  finished in 6.342
2019-11-11T17:09:07.0245031Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:09:07.2068189Z 
2019-11-11T17:09:07.2068378Z running 156 tests
2019-11-11T17:09:10.0907316Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-11T17:09:11.9566524Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-11T17:09:11.9567256Z 
2019-11-11T17:09:11.9567304Z  finished in 4.931
2019-11-11T17:09:11.9745037Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:09:12.1316955Z 
---
2019-11-11T17:09:14.0884230Z  finished in 2.113
2019-11-11T17:09:14.1067116Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:09:14.2722422Z 
2019-11-11T17:09:14.2723380Z running 9 tests
2019-11-11T17:09:14.2724548Z iiiiiiiii
2019-11-11T17:09:14.2725040Z 
2019-11-11T17:09:14.2729035Z  finished in 0.166
2019-11-11T17:09:14.2929544Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:09:14.4699775Z 
---
2019-11-11T17:09:33.6199035Z  finished in 19.328
2019-11-11T17:09:33.6390877Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:09:33.8227686Z 
2019-11-11T17:09:33.8227836Z running 123 tests
2019-11-11T17:09:58.3093961Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-11T17:10:03.1747740Z i.i.i......iii.i.....ii
2019-11-11T17:10:03.1749113Z 
2019-11-11T17:10:03.1751493Z  finished in 29.535
2019-11-11T17:10:03.1759975Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:10:03.1760314Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-11T17:21:41.7270340Z 
2019-11-11T17:21:41.7271246Z    Doc-tests core
2019-11-11T17:21:46.4928941Z 
2019-11-11T17:21:46.4929803Z running 2418 tests
2019-11-11T17:21:56.8313782Z ......iiiii......................................................................................... 100/2418
2019-11-11T17:22:07.1238822Z ................................................................................ii.................. 200/2418
2019-11-11T17:22:30.9921928Z ..i................................................................................................. 400/2418
2019-11-11T17:22:30.9921928Z ..i................................................................................................. 400/2418
2019-11-11T17:22:40.7338164Z ..................................................i..i.................iiii......................... 500/2418
2019-11-11T17:22:59.2915365Z .................................................................................................... 700/2418
2019-11-11T17:23:08.9641647Z .................................................................................................... 800/2418
2019-11-11T17:23:18.5019039Z .................................................................................................... 900/2418
2019-11-11T17:23:28.2310900Z .................................................................................................... 1000/2418
---
2019-11-11T17:27:21.4676568Z ............................................... 300/763
2019-11-11T17:27:21.4699701Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-11-11T17:27:21.5517330Z .................................................................................................... 400/763
2019-11-11T17:27:23.6213709Z .................................................................................................... 500/763
2019-11-11T17:27:24.4858916Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4887413Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4887904Z .thread '<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4888476Z ...thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4888924Z .........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4889316Z ...........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:24.4890909Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-11T17:27:25.9187476Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-11-11T17:27:25.9188666Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-11-11T17:27:25.9189196Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-11-11T17:27:25.9189807Z .......thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-11-11T17:27:35.1412185Z 
2019-11-11T17:27:35.1413121Z running 1000 tests
2019-11-11T17:27:55.4191751Z i................................................................................................... 100/1000
2019-11-11T17:28:06.6975634Z .................................................................................................... 200/1000
2019-11-11T17:28:14.5389461Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-11T17:28:20.0405430Z .................................................................................................... 400/1000
2019-11-11T17:28:27.4506579Z ...........................................i..i.................................ii.................. 500/1000
2019-11-11T17:28:41.3465318Z .................................................................................................... 700/1000
2019-11-11T17:28:41.3465318Z .................................................................................................... 700/1000
2019-11-11T17:28:48.7990678Z ..........................iiii...................................................................... 800/1000
2019-11-11T17:29:03.9081915Z .................................................................................................... 900/1000
2019-11-11T17:29:11.3249920Z ................................................iiii................................................ 1000/1000
2019-11-11T17:29:11.3289855Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-11T17:29:11.3289885Z 
2019-11-11T17:29:11.3350457Z  finished in 188.807
2019-11-11T17:29:11.3364809Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-11T17:46:27.2383896Z  finished in 41.652
2019-11-11T17:46:27.2727803Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-11T17:46:27.4719787Z 
2019-11-11T17:46:27.4721259Z running 203 tests
2019-11-11T17:46:55.3001322Z ....................i...ii............................................F......................i...... 100/203
2019-11-11T17:47:28.1359705Z .................................iiii.......i...........iiii.iii.................................... 200/203
2019-11-11T17:47:28.7062091Z i..
2019-11-11T17:47:28.7072659Z 
2019-11-11T17:47:28.7072659Z 
2019-11-11T17:47:28.7073410Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2019-11-11T17:47:28.7073734Z error: make failed
2019-11-11T17:47:28.7073780Z status: exit code: 2
2019-11-11T17:47:28.7073821Z command: "make"
2019-11-11T17:47:28.7073862Z stdout:
2019-11-11T17:47:28.7073862Z stdout:
2019-11-11T17:47:28.7074160Z ------------------------------------------
2019-11-11T17:47:28.7074951Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2019-11-11T17:47:28.7075254Z Makefile:8: recipe for target 'all' failed
2019-11-11T17:47:28.7075507Z ------------------------------------------
2019-11-11T17:47:28.7075571Z stderr:
2019-11-11T17:47:28.7075782Z ------------------------------------------
2019-11-11T17:47:28.7075836Z error[E0063]: missing field `override_queries` in initializer of `rustc_interface::Config`
---
2019-11-11T17:47:28.7076232Z 
2019-11-11T17:47:28.7076275Z error: aborting due to previous error
2019-11-11T17:47:28.7076303Z 
2019-11-11T17:47:28.7076558Z For more information about this error, try `rustc --explain E0063`.
2019-11-11T17:47:28.7076607Z make: *** [all] Error 1
2019-11-11T17:47:28.7076840Z ------------------------------------------
2019-11-11T17:47:28.7076896Z 
2019-11-11T17:47:28.7076920Z 
2019-11-11T17:47:28.7076945Z 
---
2019-11-11T17:47:28.7077991Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-11T17:47:28.7078050Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-11T17:47:28.7082114Z 
2019-11-11T17:47:28.7082285Z 
2019-11-11T17:47:28.7087853Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-11T17:47:28.7088993Z 
2019-11-11T17:47:28.7089323Z 
2019-11-11T17:47:28.7096936Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-11T17:47:28.7097035Z Build completed unsuccessfully in 1:41:56
2019-11-11T17:47:28.7097035Z Build completed unsuccessfully in 1:41:56
2019-11-11T17:47:28.7152791Z == clock drift check ==
2019-11-11T17:47:28.8358771Z   local time: Mon Nov 11 17:47:28 UTC 2019
2019-11-11T17:47:29.2094512Z   network time: Mon, 11 Nov 2019 17:47:29 GMT
2019-11-11T17:47:29.2097920Z == end clock drift check ==
2019-11-11T17:47:38.6646441Z 
2019-11-11T17:47:38.6718866Z ##[error]Bash exited with code '1'.
2019-11-11T17:47:38.6758061Z ##[section]Starting: Checkout
2019-11-11T17:47:38.6760310Z ==============================================================================
2019-11-11T17:47:38.6760383Z Task         : Get sources
2019-11-11T17:47:38.6760432Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
