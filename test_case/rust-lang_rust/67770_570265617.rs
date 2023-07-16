plain
2020-01-02T14:49:41.2219000Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-02T14:49:41.2483991Z ##[command]git config gc.auto 0
2020-01-02T14:49:41.2550641Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-02T14:49:41.2616379Z ##[command]git config --get-all http.proxy
2020-01-02T14:49:41.2790313Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67770/merge:refs/remotes/pull/67770/merge
---
2020-01-02T15:51:32.7011440Z .................................................................................................... 1500/9468
2020-01-02T15:51:38.6241623Z .................................................................................................... 1600/9468
2020-01-02T15:51:43.5832372Z .................................................................................................... 1700/9468
2020-01-02T15:51:52.9750276Z .................................................................................................... 1800/9468
2020-01-02T15:52:01.3451548Z .i.................................................................................................. 1900/9468
2020-01-02T15:52:08.0778457Z .......................................................................................iiiii........ 2000/9468
2020-01-02T15:52:31.0231452Z .................................................................................................... 2200/9468
2020-01-02T15:52:32.5299838Z .................................................................................................... 2300/9468
2020-01-02T15:52:34.9879091Z .................................................................................................... 2400/9468
2020-01-02T15:52:41.0894831Z .................................................................................................... 2500/9468
---
2020-01-02T15:55:40.5667540Z ..................i...............i................................................................. 4900/9468
2020-01-02T15:55:50.4874154Z .................................................................................................... 5000/9468
2020-01-02T15:55:56.3306962Z ...............................................................i.................................... 5100/9468
2020-01-02T15:56:04.5219720Z .................................................................................................... 5200/9468
2020-01-02T15:56:12.0845944Z ..............................ii.ii...........i..................................................... 5300/9468
2020-01-02T15:56:21.4433991Z .................................................................................................... 5500/9468
2020-01-02T15:56:31.7072344Z .................................................................................................... 5600/9468
2020-01-02T15:56:38.8067690Z .............i...................................................................................... 5700/9468
2020-01-02T15:56:44.9816718Z .................................................................................................... 5800/9468
2020-01-02T15:56:44.9816718Z .................................................................................................... 5800/9468
2020-01-02T15:56:55.7585276Z .................................................................................................... 5900/9468
2020-01-02T15:57:07.6563902Z .ii...i..ii...........i............................................................................. 6000/9468
2020-01-02T15:57:25.2808996Z .................................................................................................... 6200/9468
2020-01-02T15:57:32.6272291Z .................................................................................................... 6300/9468
2020-01-02T15:57:32.6272291Z .................................................................................................... 6300/9468
2020-01-02T15:57:49.9571442Z ............................i..ii................................................................... 6400/9468
2020-01-02T15:58:10.3314289Z .................................................................................................... 6600/9468
2020-01-02T15:58:12.5155954Z ...i................................................................................................ 6700/9468
2020-01-02T15:58:14.9532553Z .................................................................................................... 6800/9468
2020-01-02T15:58:17.5662101Z ...i................................................................................................ 6900/9468
---
2020-01-02T15:59:57.5574082Z .................................................................................................... 7500/9468
2020-01-02T16:00:02.4440556Z .................................................................................................... 7600/9468
2020-01-02T16:00:08.0946537Z .................................................................................................... 7700/9468
2020-01-02T16:00:18.4147552Z .................................................................................................... 7800/9468
2020-01-02T16:00:26.0894191Z .....................................iiii........................................................... 7900/9468
2020-01-02T16:00:41.1590369Z .................................................................................................... 8100/9468
2020-01-02T16:00:49.9842422Z .................................................................................................... 8200/9468
2020-01-02T16:01:04.4083825Z .................................................................................................... 8300/9468
2020-01-02T16:01:12.4436234Z .................................................................................................... 8400/9468
---
2020-01-02T16:03:35.6605093Z  finished in 6.879
2020-01-02T16:03:35.6812001Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:03:35.8523322Z 
2020-01-02T16:03:35.8523594Z running 166 tests
2020-01-02T16:03:38.9912272Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-02T16:03:41.0952933Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-02T16:03:41.0954737Z 
2020-01-02T16:03:41.0957728Z  finished in 5.414
2020-01-02T16:03:41.1169370Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:03:41.2861450Z 
---
2020-01-02T16:03:43.2683685Z  finished in 2.151
2020-01-02T16:03:43.2897617Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:03:43.4602361Z 
2020-01-02T16:03:43.4603130Z running 9 tests
2020-01-02T16:03:43.4604294Z iiiiiiiii
2020-01-02T16:03:43.4605123Z 
2020-01-02T16:03:43.4605263Z  finished in 0.170
2020-01-02T16:03:43.4816764Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:03:43.6517820Z 
---
2020-01-02T16:04:04.0790451Z  finished in 20.597
2020-01-02T16:04:04.1004449Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:04:04.2694680Z 
2020-01-02T16:04:04.2694923Z running 124 tests
2020-01-02T16:04:29.3092683Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-02T16:04:33.5469171Z .i.iii.....iiiiii.....ii
2020-01-02T16:04:33.5469688Z 
2020-01-02T16:04:33.5473692Z  finished in 29.446
2020-01-02T16:04:33.5474145Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:04:33.5474483Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-02T16:18:12.4727005Z 
2020-01-02T16:18:12.4730822Z    Doc-tests core
2020-01-02T16:18:17.0453021Z 
2020-01-02T16:18:17.0454616Z running 2440 tests
2020-01-02T16:18:26.2466182Z ......iiiii......................................................................................... 100/2440
2020-01-02T16:18:35.3102634Z ..................................................................................ii................ 200/2440
2020-01-02T16:18:56.5362782Z ................i................................................................................... 400/2440
2020-01-02T16:18:56.5362782Z ................i................................................................................... 400/2440
2020-01-02T16:19:06.1948331Z .................................................................i..i..................iiii......... 500/2440
2020-01-02T16:19:22.9793761Z .................................................................................................... 700/2440
2020-01-02T16:19:31.6587972Z .................................................................................................... 800/2440
2020-01-02T16:19:40.3482466Z .................................................................................................... 900/2440
2020-01-02T16:19:48.9978523Z .................................................................................................... 1000/2440
---
2020-01-02T16:23:26.1578706Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-01-02T16:23:26.1588415Z ... 300/760
2020-01-02T16:23:26.3100982Z .................................................................................................... 400/760
2020-01-02T16:23:28.3794595Z .................................................................................................... 500/760
2020-01-02T16:23:28.4017572Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.4034837Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.4047360Z thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.4091259Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.6809385Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.6858165Z ...........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T16:23:28.6881372Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192.:5
2020-01-02T16:23:30.7382436Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-02T16:23:30.7398663Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-01-02T16:23:30.7399481Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-01-02T16:23:30.7400321Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-01-02T16:23:40.0052197Z 
2020-01-02T16:23:40.0052457Z running 1002 tests
2020-01-02T16:23:58.0552031Z i................................................................................................... 100/1002
2020-01-02T16:24:08.1121328Z .................................................................................................... 200/1002
2020-01-02T16:24:15.2388433Z .................iii......i......i...i......i....................................................... 300/1002
2020-01-02T16:24:20.1331565Z .................................................................................................... 400/1002
2020-01-02T16:24:27.0795003Z .........................................i..i.....................................ii................ 500/1002
2020-01-02T16:24:39.8013071Z .................................................................................................... 700/1002
2020-01-02T16:24:39.8013071Z .................................................................................................... 700/1002
2020-01-02T16:24:46.4275695Z ............................iiii.................................................................... 800/1002
2020-01-02T16:25:00.6984262Z .................................................................................................... 900/1002
2020-01-02T16:25:07.7603082Z ..................................................iiii.............................................. 1000/1002
2020-01-02T16:25:07.8143931Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-02T16:25:07.8144066Z 
2020-01-02T16:25:07.8247548Z  finished in 182.605
2020-01-02T16:25:07.8262851Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-02T16:44:37.7806448Z  finished in 39.491
2020-01-02T16:44:37.8219207Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T16:44:38.0326496Z 
2020-01-02T16:44:38.0326798Z running 206 tests
2020-01-02T16:45:10.6051446Z ....................i...ii....................................................................i..... 100/206
2020-01-02T16:45:44.6346416Z ..................................iiiiii......i............iiii.iii.......F......................... 200/206
2020-01-02T16:45:48.3287544Z failures:
2020-01-02T16:45:48.3289216Z 
2020-01-02T16:45:48.3289781Z ---- [run-make] run-make-fulldeps/simd-ffi stdout ----
2020-01-02T16:45:48.3289902Z 
2020-01-02T16:45:48.3289902Z 
2020-01-02T16:45:48.3289947Z error: make failed
2020-01-02T16:45:48.3289991Z status: exit code: 2
2020-01-02T16:45:48.3290049Z command: "make" "make"
2020-01-02T16:45:48.3290089Z stdout:
2020-01-02T16:45:48.3290354Z ------------------------------------------
2020-01-02T16:45:48.3291277Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-linux-androideabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-linux-androideabi
2020-01-02T16:45:48.3291608Z Makefile:47: recipe for target 'arm-linux-androideabi' failed
2020-01-02T16:45:48.3291863Z ------------------------------------------
2020-01-02T16:45:48.3291916Z stderr:
2020-01-02T16:45:48.3293359Z ------------------------------------------
2020-01-02T16:45:48.3293359Z ------------------------------------------
2020-01-02T16:45:48.3293728Z '+sse2' is not a recognized feature for this target (ignoring feature)
2020-01-02T16:45:48.3293786Z error[E0566]: conflicting representation hints
2020-01-02T16:45:48.3294014Z   --> simd.rs:11:8
2020-01-02T16:45:48.3294099Z 11 | #[repr(C)]
2020-01-02T16:45:48.3294155Z    |        ^
2020-01-02T16:45:48.3294200Z 12 | #[derive(Copy)]
2020-01-02T16:45:48.3294200Z 12 | #[derive(Copy)]
2020-01-02T16:45:48.3294242Z 13 | #[repr(simd)]
2020-01-02T16:45:48.3294585Z 
2020-01-02T16:45:48.3294632Z error[E0566]: conflicting representation hints
2020-01-02T16:45:48.3294911Z   --> simd.rs:26:8
2020-01-02T16:45:48.3294972Z    |
2020-01-02T16:45:48.3294972Z    |
2020-01-02T16:45:48.3296123Z 26 | #[repr(C)]
2020-01-02T16:45:48.3296192Z    |        ^
2020-01-02T16:45:48.3296233Z 27 | #[derive(Copy)]
2020-01-02T16:45:48.3296328Z 28 | #[repr(simd)]
2020-01-02T16:45:48.3296398Z 
2020-01-02T16:45:48.3296454Z error: aborting due to 2 previous errors
2020-01-02T16:45:48.3296501Z 
2020-01-02T16:45:48.3296864Z For more information about this error, try `rustc --explain E0566`.
2020-01-02T16:45:48.3296864Z For more information about this error, try `rustc --explain E0566`.
2020-01-02T16:45:48.3297087Z make: *** [arm-linux-androideabi] Error 1
2020-01-02T16:45:48.3297349Z ------------------------------------------
2020-01-02T16:45:48.3297380Z 
2020-01-02T16:45:48.3297405Z 
2020-01-02T16:45:48.3297429Z 
---
2020-01-02T16:45:48.3300271Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-02T16:45:48.3300330Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-02T16:45:48.3305456Z 
2020-01-02T16:45:48.3305688Z 
2020-01-02T16:45:48.3313002Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-02T16:45:48.3314613Z 
2020-01-02T16:45:48.3314760Z 
2020-01-02T16:45:48.3314992Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-02T16:45:48.3315152Z Build completed unsuccessfully in 1:49:15
2020-01-02T16:45:48.3315152Z Build completed unsuccessfully in 1:49:15
2020-01-02T16:45:48.3368208Z == clock drift check ==
2020-01-02T16:45:48.3386902Z   local time: Thu Jan  2 16:45:48 UTC 2020
2020-01-02T16:45:48.6318040Z   network time: Thu, 02 Jan 2020 16:45:48 GMT
2020-01-02T16:45:48.6321146Z == end clock drift check ==
2020-01-02T16:45:54.2013381Z 
2020-01-02T16:45:54.2160794Z ##[error]Bash exited with code '1'.
2020-01-02T16:45:54.2201695Z ##[section]Starting: Checkout
2020-01-02T16:45:54.2203760Z ==============================================================================
2020-01-02T16:45:54.2203838Z Task         : Get sources
2020-01-02T16:45:54.2203890Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
