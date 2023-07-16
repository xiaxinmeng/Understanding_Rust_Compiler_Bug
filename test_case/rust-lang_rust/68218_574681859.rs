plain
2020-01-15T12:30:00.4089842Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T12:30:00.4101395Z ##[command]git config gc.auto 0
2020-01-15T12:30:00.4111889Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T12:30:00.4114203Z ##[command]git config --get-all http.proxy
2020-01-15T12:30:00.4117248Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68218/merge:refs/remotes/pull/68218/merge
---
2020-01-15T13:29:00.3791509Z .................................................................................................... 1600/9520
2020-01-15T13:29:05.8889853Z .................................................................................................... 1700/9520
2020-01-15T13:29:15.2981717Z .................................................................................................... 1800/9520
2020-01-15T13:29:24.9115058Z ........i........................................................................................... 1900/9520
2020-01-15T13:29:32.1682979Z ..................................................................................................ii 2000/9520
2020-01-15T13:29:49.0118296Z iii................................................................................................. 2100/9520
2020-01-15T13:29:58.0373549Z .................................................................................................... 2300/9520
2020-01-15T13:30:00.8282023Z .................................................................................................... 2400/9520
2020-01-15T13:30:07.0087534Z .................................................................................................... 2500/9520
2020-01-15T13:30:27.9416487Z .................................................................................................... 2600/9520
---
2020-01-15T13:33:16.8572451Z .........................................i...............i.......................................... 4900/9520
2020-01-15T13:33:26.6481349Z .................................................................................................... 5000/9520
2020-01-15T13:33:33.5766137Z ....................................................................................i............... 5100/9520
2020-01-15T13:33:39.4123229Z .................................................................................................... 5200/9520
2020-01-15T13:33:50.3705210Z .......................................................ii.ii...........i............................ 5300/9520
2020-01-15T13:34:00.4917814Z .................................................................................................... 5500/9520
2020-01-15T13:34:11.3987029Z .................................................................................................... 5600/9520
2020-01-15T13:34:18.3962813Z .........................................i.......................................................... 5700/9520
2020-01-15T13:34:25.7488741Z .................................................................................................... 5800/9520
2020-01-15T13:34:25.7488741Z .................................................................................................... 5800/9520
2020-01-15T13:34:37.1727118Z .................................................................................................... 5900/9520
2020-01-15T13:34:47.4034848Z ................................ii...i..ii...........i.............................................. 6000/9520
2020-01-15T13:35:07.3684155Z .................................................................................................... 6200/9520
2020-01-15T13:35:16.2799404Z .................................................................................................... 6300/9520
2020-01-15T13:35:16.2799404Z .................................................................................................... 6300/9520
2020-01-15T13:35:28.1960061Z ............................................................i..ii................................... 6400/9520
2020-01-15T13:35:57.3848081Z .................................................................................................... 6600/9520
2020-01-15T13:35:59.9148307Z ....................................i............................................................... 6700/9520
2020-01-15T13:36:02.4529609Z .................................................................................................... 6800/9520
2020-01-15T13:36:05.3050668Z ....................................i............................................................... 6900/9520
---
2020-01-15T13:37:48.3313967Z .................................................................................................... 7500/9520
2020-01-15T13:37:53.1618823Z .................................................................................................... 7600/9520
2020-01-15T13:37:59.6244973Z .................................................................................................... 7700/9520
2020-01-15T13:38:07.1383193Z .................................................................................................... 7800/9520
2020-01-15T13:38:17.7496178Z ......................................................................................iiii.......... 7900/9520
2020-01-15T13:38:35.4307973Z ...................i......i......................................................................... 8100/9520
2020-01-15T13:38:41.1248457Z .................................................................................................... 8200/9520
2020-01-15T13:38:55.1131930Z .................................................................................................... 8300/9520
2020-01-15T13:39:05.6759440Z .................................................................................................... 8400/9520
---
2020-01-15T13:41:41.0704487Z  finished in 7.836
2020-01-15T13:41:41.0902395Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:41:41.2938052Z 
2020-01-15T13:41:41.2938784Z running 166 tests
2020-01-15T13:41:44.5978275Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-15T13:41:47.0475406Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-15T13:41:47.0480675Z 
2020-01-15T13:41:47.0484045Z  finished in 5.958
2020-01-15T13:41:47.5667313Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:41:47.5673931Z 
---
2020-01-15T13:41:49.3620093Z  finished in 2.293
2020-01-15T13:41:49.3827321Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:41:49.5638154Z 
2020-01-15T13:41:49.5638904Z running 9 tests
2020-01-15T13:41:49.5639828Z iiiiiiiii
2020-01-15T13:41:49.5640241Z 
2020-01-15T13:41:49.5640494Z  finished in 0.181
2020-01-15T13:41:49.5847971Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:41:50.5673404Z 
---
2020-01-15T13:42:13.0721772Z  finished in 22.602
2020-01-15T13:42:13.0722125Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:42:13.0722187Z 
2020-01-15T13:42:13.0722233Z running 116 tests
2020-01-15T13:42:39.0654401Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-15T13:42:42.7137748Z .....iiii.....ii
2020-01-15T13:42:42.7141572Z 
2020-01-15T13:42:42.7141669Z  finished in 30.503
2020-01-15T13:42:42.7151397Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T13:42:42.7151781Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-15T13:56:37.3887010Z 
2020-01-15T13:56:37.3893666Z    Doc-tests core
2020-01-15T13:56:42.3889735Z 
2020-01-15T13:56:42.3890796Z running 2442 tests
2020-01-15T13:56:52.4295855Z ......iiiii......................................................................................... 100/2442
2020-01-15T13:57:02.2339696Z ..................................................................................ii................ 200/2442
2020-01-15T13:57:24.6439920Z ................i................................................................................... 400/2442
2020-01-15T13:57:24.6439920Z ................i................................................................................... 400/2442
2020-01-15T13:57:34.8051597Z .................................................................i..i..................iiii......... 500/2442
2020-01-15T13:57:52.2975256Z .................................................................................................... 700/2442
2020-01-15T13:58:01.3548724Z .................................................................................................... 800/2442
2020-01-15T13:58:10.3888871Z .................................................................................................... 900/2442
2020-01-15T13:58:19.3454676Z .................................................................................................... 1000/2442
---
2020-01-15T14:01:56.9102794Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
2020-01-15T14:01:56.9109935Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-01-15T14:01:56.9110631Z   left: `1`,
2020-01-15T14:01:56.9111678Z  right: `2`', src/libstd/sync/mutex.rs:657:13
2020-01-15T14:01:56.9145730Z .............thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:792:13
2020-01-15T14:01:56.9152656Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2020-01-15T14:01:56.9168534Z thread '..<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:704:13
2020-01-15T14:01:56.9178844Z thread '<unnamed>.' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:632:13
2020-01-15T14:01:56.9186357Z thread '<unnamed>' panicked at 'explicit panic', .src/libstd/sync/rwlock.rs:644:13
2020-01-15T14:01:56.9193903Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:606:13
2020-01-15T14:01:56.9201479Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:619:13
2020-01-15T14:01:58.9813937Z .........................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:229:13
2020-01-15T14:01:58.9934287Z ................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-01-15T14:01:59.6007397Z ............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-01-15T14:01:59.6019106Z ..thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1692:13
2020-01-15T14:01:59.6019601Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
---
2020-01-15T14:02:06.2116533Z 
2020-01-15T14:02:06.2116779Z running 1003 tests
2020-01-15T14:02:25.5744412Z i................................................................................................... 100/1003
2020-01-15T14:02:36.0427289Z .................................................................................................... 200/1003
2020-01-15T14:02:43.4896339Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-15T14:02:48.8555420Z .................................................................................................... 400/1003
2020-01-15T14:02:55.9831476Z ..........................................i..i.....................................ii............... 500/1003
2020-01-15T14:03:09.4174441Z .................................................................................................... 700/1003
2020-01-15T14:03:09.4174441Z .................................................................................................... 700/1003
2020-01-15T14:03:16.2765799Z .............................iiii................................................................... 800/1003
2020-01-15T14:03:31.1355936Z .................................................................................................... 900/1003
2020-01-15T14:03:38.4478984Z ...................................................iiii............................................. 1000/1003
2020-01-15T14:03:38.5717757Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-15T14:03:38.5718360Z 
2020-01-15T14:03:38.5816386Z  finished in 178.924
2020-01-15T14:03:38.5830717Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-15T14:23:10.9392623Z  finished in 42.098
2020-01-15T14:23:10.9777693Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-15T14:23:11.2105412Z 
2020-01-15T14:23:11.2106399Z running 205 tests
2020-01-15T14:23:44.3982410Z ....................i...ii...................................F................................i..... 100/205
2020-01-15T14:24:20.0245657Z ..................................iiiiii......i............iii.iii.................................. 200/205
2020-01-15T14:24:21.8583006Z ..i..
2020-01-15T14:24:21.8592151Z 
2020-01-15T14:24:21.8595104Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-15T14:24:21.8595495Z 
2020-01-15T14:24:21.8595764Z error: make failed
2020-01-15T14:24:21.8595764Z error: make failed
2020-01-15T14:24:21.8595832Z status: exit code: 2
2020-01-15T14:24:21.8595877Z command: "make"
2020-01-15T14:24:21.8597498Z stdout:
2020-01-15T14:24:21.8598047Z ------------------------------------------
2020-01-15T14:24:21.8598312Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-15T14:24:21.8598445Z 
2020-01-15T14:24:21.8599525Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-15T14:24:21.8600307Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-15T14:24:21.8600685Z Makefile:4: recipe for target 'all' failed
2020-01-15T14:24:21.8601003Z ------------------------------------------
2020-01-15T14:24:21.8601054Z stderr:
2020-01-15T14:24:21.8601304Z ------------------------------------------
2020-01-15T14:24:21.8601304Z ------------------------------------------
2020-01-15T14:24:21.8601358Z error[E0412]: cannot find type `Lrc` in this scope
2020-01-15T14:24:21.8601586Z    --> the_backend.rs:78:16
2020-01-15T14:24:21.8601652Z     |
2020-01-15T14:24:21.8601711Z 78  |         sess: &Lrc<Session>,
2020-01-15T14:24:21.8601801Z     | 
2020-01-15T14:24:21.8601863Z    ::: /checkout/src/liballoc/sync.rs:196:1
2020-01-15T14:24:21.8601908Z     |
2020-01-15T14:24:21.8601908Z     |
2020-01-15T14:24:21.8601953Z 196 | pub struct Arc<T: ?Sized> {
2020-01-15T14:24:21.8602251Z     | ------------------------- similarly named struct `Arc` defined here
2020-01-15T14:24:21.8602347Z help: a struct with a similar name exists
2020-01-15T14:24:21.8602417Z     |
2020-01-15T14:24:21.8602463Z 78  |         sess: &Arc<Session>,
2020-01-15T14:24:21.8602781Z     |                ^^^
2020-01-15T14:24:21.8602781Z     |                ^^^
2020-01-15T14:24:21.8602838Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T14:24:21.8602902Z     |
2020-01-15T14:24:21.8602947Z 12  | use rustc_data_structures::sync::Lrc;
2020-01-15T14:24:21.8602989Z     |
2020-01-15T14:24:21.8603032Z 
2020-01-15T14:24:21.8603344Z warning: ignoring --out-dir flag due to -o flag
2020-01-15T14:24:21.8603435Z error: aborting due to previous error
2020-01-15T14:24:21.8603465Z 
2020-01-15T14:24:21.8603730Z For more information about this error, try `rustc --explain E0412`.
2020-01-15T14:24:21.8603730Z For more information about this error, try `rustc --explain E0412`.
2020-01-15T14:24:21.8603781Z make: *** [all] Error 1
2020-01-15T14:24:21.8604037Z ------------------------------------------
2020-01-15T14:24:21.8604069Z 
2020-01-15T14:24:21.8604095Z 
2020-01-15T14:24:21.8604119Z 
2020-01-15T14:24:21.8604119Z 
2020-01-15T14:24:21.8604159Z failures:
2020-01-15T14:24:21.8604657Z     [run-make] run-make-fulldeps/hotplug_codegen_backend
2020-01-15T14:24:21.8604692Z 
2020-01-15T14:24:21.8604962Z test result: FAILED. 186 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out
2020-01-15T14:24:21.8604999Z 
2020-01-15T14:24:21.8605040Z 
2020-01-15T14:24:21.8605066Z 
2020-01-15T14:24:21.8609879Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-15T14:24:21.8611073Z 
2020-01-15T14:24:21.8611133Z 
2020-01-15T14:24:21.8611597Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-15T14:24:21.8611673Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-15T14:24:21.8611673Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-15T14:24:21.8617630Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-15T14:24:21.8617822Z Build completed unsuccessfully in 1:47:44
2020-01-15T14:24:21.8667627Z == clock drift check ==
2020-01-15T14:24:21.8686154Z   local time: Wed Jan 15 14:24:21 UTC 2020
2020-01-15T14:24:22.4241222Z   network time: Wed, 15 Jan 2020 14:24:22 GMT
2020-01-15T14:24:22.4249275Z == end clock drift check ==
2020-01-15T14:24:23.5992189Z 
2020-01-15T14:24:23.6111474Z ##[error]Bash exited with code '1'.
2020-01-15T14:24:23.6150539Z ##[section]Starting: Checkout
2020-01-15T14:24:23.6152286Z ==============================================================================
2020-01-15T14:24:23.6152349Z Task         : Get sources
2020-01-15T14:24:23.6152402Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
