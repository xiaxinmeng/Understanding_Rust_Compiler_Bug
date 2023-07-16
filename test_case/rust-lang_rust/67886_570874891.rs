plain
2020-01-05T04:36:49.6792233Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T04:36:50.4733727Z ##[command]git config gc.auto 0
2020-01-05T04:36:50.4736162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T04:36:50.4737963Z ##[command]git config --get-all http.proxy
2020-01-05T04:36:50.4740383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67886/merge:refs/remotes/pull/67886/merge
---
2020-01-05T05:32:32.8755659Z .................................................................................................... 1500/9475
2020-01-05T05:32:39.0907155Z .................................................................................................... 1600/9475
2020-01-05T05:32:44.3093840Z .................................................................................................... 1700/9475
2020-01-05T05:32:54.2397307Z .................................................................................................... 1800/9475
2020-01-05T05:33:02.6923032Z i................................................................................................... 1900/9475
2020-01-05T05:33:09.5996171Z ........................................................................................iiiii....... 2000/9475
2020-01-05T05:33:32.4358667Z .................................................................................................... 2200/9475
2020-01-05T05:33:34.9937315Z .................................................................................................... 2300/9475
2020-01-05T05:33:37.6795516Z .................................................................................................... 2400/9475
2020-01-05T05:33:44.1963378Z .................................................................................................... 2500/9475
---
2020-01-05T05:36:51.8413717Z ....................i...............i............................................................... 4900/9475
2020-01-05T05:37:02.8043421Z .................................................................................................... 5000/9475
2020-01-05T05:37:08.3083612Z .................................................................i.................................. 5100/9475
2020-01-05T05:37:16.5882862Z .................................................................................................... 5200/9475
2020-01-05T05:37:24.5708099Z ................................ii.ii...........i................................................... 5300/9475
2020-01-05T05:37:34.2245675Z .................................................................................................... 5500/9475
2020-01-05T05:37:44.3322676Z .................................................................................................... 5600/9475
2020-01-05T05:37:51.8013916Z ................i................................................................................... 5700/9475
2020-01-05T05:37:58.1789730Z .................................................................................................... 5800/9475
2020-01-05T05:37:58.1789730Z .................................................................................................... 5800/9475
2020-01-05T05:38:09.6483012Z .................................................................................................... 5900/9475
2020-01-05T05:38:21.5634129Z .....ii...i..ii...........i......................................................................... 6000/9475
2020-01-05T05:38:39.5803407Z .................................................................................................... 6200/9475
2020-01-05T05:38:47.6880511Z .................................................................................................... 6300/9475
2020-01-05T05:38:47.6880511Z .................................................................................................... 6300/9475
2020-01-05T05:39:05.5622617Z ................................i..ii............................................................... 6400/9475
2020-01-05T05:39:27.0611524Z .................................................................................................... 6600/9475
2020-01-05T05:39:29.3550823Z .......i............................................................................................ 6700/9475
2020-01-05T05:39:31.6828877Z .................................................................................................... 6800/9475
2020-01-05T05:39:34.3820960Z .......i............................................................................................ 6900/9475
---
2020-01-05T05:41:15.9233698Z .................................................................................................... 7500/9475
2020-01-05T05:41:20.5682043Z .................................................................................................... 7600/9475
2020-01-05T05:41:26.2270458Z .................................................................................................... 7700/9475
2020-01-05T05:41:37.8507833Z .................................................................................................... 7800/9475
2020-01-05T05:41:46.5942217Z ...........................................iiii..................................................... 7900/9475
2020-01-05T05:42:02.5629280Z .................................................................................................... 8100/9475
2020-01-05T05:42:11.3706597Z .................................................................................................... 8200/9475
2020-01-05T05:42:26.1839230Z .................................................................................................... 8300/9475
2020-01-05T05:42:34.1479417Z .................................................................................................... 8400/9475
---
2020-01-05T05:45:01.0205546Z  finished in 7.008
2020-01-05T05:45:01.0404332Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:45:01.2062184Z 
2020-01-05T05:45:01.2062422Z running 166 tests
2020-01-05T05:45:04.4642846Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-05T05:45:06.7837943Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-05T05:45:06.7842150Z 
2020-01-05T05:45:06.7845226Z  finished in 5.744
2020-01-05T05:45:06.8050055Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:45:06.9699228Z 
---
2020-01-05T05:45:09.0165870Z  finished in 2.211
2020-01-05T05:45:09.0353289Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:45:09.1953149Z 
2020-01-05T05:45:09.1954177Z running 9 tests
2020-01-05T05:45:09.1955265Z iiiiiiiii
2020-01-05T05:45:09.1956045Z 
2020-01-05T05:45:09.1956210Z  finished in 0.159
2020-01-05T05:45:09.2154619Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:45:09.3843937Z 
---
2020-01-05T05:45:30.3380477Z  finished in 21.122
2020-01-05T05:45:30.3599182Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:45:30.5279774Z 
2020-01-05T05:45:30.5279976Z running 124 tests
2020-01-05T05:45:55.7414341Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-05T05:46:00.0735580Z .i.iii.....iiiiii.....ii
2020-01-05T05:46:00.0736712Z 
2020-01-05T05:46:00.0736824Z  finished in 29.713
2020-01-05T05:46:00.0745387Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T05:46:00.0745766Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-05T05:59:21.9376573Z 
2020-01-05T05:59:21.9381670Z    Doc-tests core
2020-01-05T05:59:27.0472582Z 
2020-01-05T05:59:27.0473341Z running 2440 tests
2020-01-05T05:59:36.7102667Z ......iiiii......................................................................................... 100/2440
2020-01-05T05:59:46.0086482Z ..................................................................................ii................ 200/2440
2020-01-05T06:00:08.1036934Z ................i................................................................................... 400/2440
2020-01-05T06:00:08.1036934Z ................i................................................................................... 400/2440
2020-01-05T06:00:18.0684504Z .................................................................i..i..................iiii......... 500/2440
2020-01-05T06:00:35.2211001Z .................................................................................................... 700/2440
2020-01-05T06:00:44.3272603Z .................................................................................................... 800/2440
2020-01-05T06:00:53.6005993Z .................................................................................................... 900/2440
2020-01-05T06:01:03.1433479Z .................................................................................................... 1000/2440
---
2020-01-05T06:04:43.9076530Z .................................................................................................... 500/760
2020-01-05T06:04:43.9381137Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:43.9409679Z ......thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:43.9414458Z <unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1192:5
2020-01-05T06:04:43.9447863Z .......thread '.<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:44.1360615Z ......................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:44.1390129Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:44.1475723Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-05T06:04:44.1985135Z ................... 600/760
2020-01-05T06:04:46.2563393Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-05T06:04:46.2563889Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-01-05T06:04:55.5866353Z 
2020-01-05T06:04:55.5867159Z running 1002 tests
2020-01-05T06:05:14.8209976Z i................................................................................................... 100/1002
2020-01-05T06:05:25.3319242Z .................................................................................................... 200/1002
2020-01-05T06:05:32.9977001Z .................iii......i......i...i......i....................................................... 300/1002
2020-01-05T06:05:38.2195950Z .................................................................................................... 400/1002
2020-01-05T06:05:45.4973208Z .........................................i..i.....................................ii................ 500/1002
2020-01-05T06:05:59.0520135Z .................................................................................................... 700/1002
2020-01-05T06:05:59.0520135Z .................................................................................................... 700/1002
2020-01-05T06:06:06.2992558Z ............................iiii.................................................................... 800/1002
2020-01-05T06:06:20.8517598Z .................................................................................................... 900/1002
2020-01-05T06:06:28.1558301Z ..................................................iiii.............................................. 1000/1002
2020-01-05T06:06:28.2199694Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-05T06:06:28.2199931Z 
2020-01-05T06:06:28.2326188Z  finished in 179.834
2020-01-05T06:06:28.2339957Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-05T06:25:51.7277557Z  finished in 53.216
2020-01-05T06:25:51.7663231Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T06:25:52.0410362Z 
2020-01-05T06:25:52.0422721Z running 206 tests
2020-01-05T06:26:30.2244578Z ....................i...ii..................................F.................................i..... 100/206
2020-01-05T06:27:04.3787548Z ..................................iiiiii......i............iiii.iii................................. 200/206
2020-01-05T06:27:06.9564121Z failures:
2020-01-05T06:27:06.9564153Z 
2020-01-05T06:27:06.9566488Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-05T06:27:06.9566923Z 
2020-01-05T06:27:06.9566923Z 
2020-01-05T06:27:06.9566981Z error: make failed
2020-01-05T06:27:06.9567107Z status: exit code: 2
2020-01-05T06:27:06.9569851Z command: "make" "make"
2020-01-05T06:27:06.9575671Z stdout:
2020-01-05T06:27:06.9577020Z ------------------------------------------
2020-01-05T06:27:06.9577144Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-05T06:27:06.9577182Z 
2020-01-05T06:27:06.9578095Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-05T06:27:06.9578517Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-05T06:27:06.9578760Z Makefile:4: recipe for target 'all' failed
2020-01-05T06:27:06.9578999Z ------------------------------------------
2020-01-05T06:27:06.9579042Z stderr:
2020-01-05T06:27:06.9579265Z ------------------------------------------
2020-01-05T06:27:06.9579313Z error[E0432]: unresolved import `rustc::hir::def_id`
2020-01-05T06:27:06.9579313Z error[E0432]: unresolved import `rustc::hir::def_id`
2020-01-05T06:27:06.9579508Z   --> the_backend.rs:68:25
2020-01-05T06:27:06.9579567Z    |
2020-01-05T06:27:06.9579614Z 68 |         use rustc::hir::def_id::LOCAL_CRATE;
2020-01-05T06:27:06.9579665Z    |                         ^^^^^^ could not find `def_id` in `hir`
2020-01-05T06:27:06.9579699Z 
2020-01-05T06:27:06.9579980Z warning: ignoring --out-dir flag due to -o flag
2020-01-05T06:27:06.9580057Z error: aborting due to previous error
2020-01-05T06:27:06.9580086Z 
2020-01-05T06:27:06.9580375Z For more information about this error, try `rustc --explain E0432`.
2020-01-05T06:27:06.9580375Z For more information about this error, try `rustc --explain E0432`.
2020-01-05T06:27:06.9580426Z make: *** [all] Error 1
2020-01-05T06:27:06.9580683Z ------------------------------------------
2020-01-05T06:27:06.9580732Z 
2020-01-05T06:27:06.9580757Z 
2020-01-05T06:27:06.9580782Z 
---
2020-01-05T06:27:06.9584551Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-05T06:27:06.9584877Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T06:27:06.9593220Z 
2020-01-05T06:27:06.9593300Z 
2020-01-05T06:27:06.9598112Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-05T06:27:06.9599085Z 
2020-01-05T06:27:06.9599114Z 
2020-01-05T06:27:06.9602520Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-05T06:27:06.9602583Z Build completed unsuccessfully in 1:43:33
2020-01-05T06:27:06.9602583Z Build completed unsuccessfully in 1:43:33
2020-01-05T06:27:06.9655528Z == clock drift check ==
2020-01-05T06:27:06.9673059Z   local time: Sun Jan  5 06:27:06 UTC 2020
2020-01-05T06:27:07.1332754Z   network time: Sun, 05 Jan 2020 06:27:07 GMT
2020-01-05T06:27:07.1332923Z == end clock drift check ==
2020-01-05T06:27:14.2227135Z 
2020-01-05T06:27:14.2367534Z ##[error]Bash exited with code '1'.
2020-01-05T06:27:14.2400338Z ##[section]Starting: Checkout
2020-01-05T06:27:14.2402038Z ==============================================================================
2020-01-05T06:27:14.2402255Z Task         : Get sources
2020-01-05T06:27:14.2402300Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
