plain
2020-01-16T12:46:31.8283728Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T12:46:31.8390770Z ##[command]git config gc.auto 0
2020-01-16T12:46:31.8454164Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T12:46:31.8530434Z ##[command]git config --get-all http.proxy
2020-01-16T12:46:31.8666303Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68218/merge:refs/remotes/pull/68218/merge
---
2020-01-16T13:44:07.5459170Z .................................................................................................... 1700/9525
2020-01-16T13:44:16.0862024Z .................................................................................................... 1800/9525
2020-01-16T13:44:26.0639742Z ...........i........................................................................................ 1900/9525
2020-01-16T13:44:33.4731729Z .................................................................................................... 2000/9525
2020-01-16T13:44:50.1861399Z .iiiii.............................................................................................. 2100/9525
2020-01-16T13:44:59.3203481Z .................................................................................................... 2300/9525
2020-01-16T13:45:02.0228095Z .................................................................................................... 2400/9525
2020-01-16T13:45:07.8751511Z .................................................................................................... 2500/9525
2020-01-16T13:45:29.0451158Z .................................................................................................... 2600/9525
---
2020-01-16T13:48:16.2577620Z ............................................i...............i....................................... 4900/9525
2020-01-16T13:48:25.8413373Z .................................................................................................... 5000/9525
2020-01-16T13:48:32.8485857Z .......................................................................................i............ 5100/9525
2020-01-16T13:48:38.2924365Z .................................................................................................... 5200/9525
2020-01-16T13:48:49.3285347Z ...........................................................ii.ii...........i........................ 5300/9525
2020-01-16T13:48:59.2427188Z .................................................................................................... 5500/9525
2020-01-16T13:49:09.8104773Z .................................................................................................... 5600/9525
2020-01-16T13:49:16.3882848Z .............................................i...................................................... 5700/9525
2020-01-16T13:49:23.7394013Z .................................................................................................... 5800/9525
2020-01-16T13:49:23.7394013Z .................................................................................................... 5800/9525
2020-01-16T13:49:34.4821841Z .................................................................................................... 5900/9525
2020-01-16T13:49:43.7821781Z ....................................ii...i..ii...........i.......................................... 6000/9525
2020-01-16T13:50:04.5904846Z .................................................................................................... 6200/9525
2020-01-16T13:50:13.5291478Z .................................................................................................... 6300/9525
2020-01-16T13:50:13.5291478Z .................................................................................................... 6300/9525
2020-01-16T13:50:23.0533683Z ................................................................i..ii............................... 6400/9525
2020-01-16T13:50:52.7739564Z .................................................................................................... 6600/9525
2020-01-16T13:50:55.2071123Z ........................................i........................................................... 6700/9525
2020-01-16T13:50:57.6704337Z .................................................................................................... 6800/9525
2020-01-16T13:51:00.4403102Z ........................................i........................................................... 6900/9525
---
2020-01-16T13:52:42.5657076Z .................................................................................................... 7500/9525
2020-01-16T13:52:46.8717192Z .................................................................................................... 7600/9525
2020-01-16T13:52:53.2903373Z .................................................................................................... 7700/9525
2020-01-16T13:53:00.4004161Z .................................................................................................... 7800/9525
2020-01-16T13:53:11.1555998Z .........................................................................................iiii....... 7900/9525
2020-01-16T13:53:28.5890573Z .......................i......i..................................................................... 8100/9525
2020-01-16T13:53:33.9841041Z .................................................................................................... 8200/9525
2020-01-16T13:53:47.2310094Z .................................................................................................... 8300/9525
2020-01-16T13:53:58.5615837Z .................................................................................................... 8400/9525
---
2020-01-16T13:56:32.5299689Z  finished in 7.692
2020-01-16T13:56:32.5495914Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:56:32.7111418Z 
2020-01-16T13:56:32.7111654Z running 166 tests
2020-01-16T13:56:35.9654097Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-16T13:56:38.3648001Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-16T13:56:38.3652829Z 
2020-01-16T13:56:38.3659151Z  finished in 5.816
2020-01-16T13:56:38.3853683Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:56:38.5636816Z 
---
2020-01-16T13:56:40.6103227Z  finished in 2.225
2020-01-16T13:56:40.6304158Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:56:40.7891214Z 
2020-01-16T13:56:40.7891469Z running 9 tests
2020-01-16T13:56:40.7892382Z iiiiiiiii
2020-01-16T13:56:40.7892778Z 
2020-01-16T13:56:40.7898208Z  finished in 0.159
2020-01-16T13:56:40.8101440Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:56:40.9744840Z 
---
2020-01-16T13:57:02.9125744Z  finished in 22.102
2020-01-16T13:57:02.9322175Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:57:03.1142390Z 
2020-01-16T13:57:03.1142646Z running 116 tests
2020-01-16T13:57:29.1644233Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-16T13:57:32.7153817Z .....iiii.....ii
2020-01-16T13:57:32.7155090Z 
2020-01-16T13:57:32.7158313Z  finished in 29.783
2020-01-16T13:57:32.7165196Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T13:57:32.7165604Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-16T14:11:23.5566800Z 
2020-01-16T14:11:23.5567566Z    Doc-tests core
2020-01-16T14:11:28.4174932Z 
2020-01-16T14:11:28.4175682Z running 2442 tests
2020-01-16T14:11:38.1265574Z ......iiiii......................................................................................... 100/2442
2020-01-16T14:11:47.5461019Z ..................................................................................ii................ 200/2442
2020-01-16T14:12:09.7015813Z ................i................................................................................... 400/2442
2020-01-16T14:12:09.7015813Z ................i................................................................................... 400/2442
2020-01-16T14:12:19.7334877Z .................................................................i..i..................iiii......... 500/2442
2020-01-16T14:12:36.9025516Z .................................................................................................... 700/2442
2020-01-16T14:12:45.6509003Z .................................................................................................... 800/2442
2020-01-16T14:12:54.3159732Z .................................................................................................... 900/2442
2020-01-16T14:13:03.0950225Z .................................................................................................... 1000/2442
---
2020-01-16T14:16:49.0365464Z 
2020-01-16T14:16:49.0366293Z running 1003 tests
2020-01-16T14:17:09.4713823Z i................................................................................................... 100/1003
2020-01-16T14:17:20.1206134Z .................................................................................................... 200/1003
2020-01-16T14:17:27.6042055Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-16T14:17:33.0684907Z .................................................................................................... 400/1003
2020-01-16T14:17:40.2880943Z ..........................................i..i.....................................ii............... 500/1003
2020-01-16T14:17:53.6295838Z .................................................................................................... 700/1003
2020-01-16T14:17:53.6295838Z .................................................................................................... 700/1003
2020-01-16T14:18:00.5015078Z .............................iiii................................................................... 800/1003
2020-01-16T14:18:15.5337550Z .................................................................................................... 900/1003
2020-01-16T14:18:23.1532166Z ...................................................iiii............................................. 1000/1003
2020-01-16T14:18:23.2612656Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-16T14:18:23.2612949Z 
2020-01-16T14:18:23.2710978Z  finished in 181.204
2020-01-16T14:18:23.2725042Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-16T14:37:41.0356985Z  finished in 41.038
2020-01-16T14:37:41.0752603Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T14:37:41.3144319Z 
2020-01-16T14:37:41.3144630Z running 205 tests
2020-01-16T14:38:15.0659345Z ....................i...ii...................................F................................i..... 100/205
2020-01-16T14:38:46.8127225Z ..................................iiiiii......i............iii.iii.................................. 200/205
2020-01-16T14:38:51.1365734Z ..i..
2020-01-16T14:38:51.1367202Z 
2020-01-16T14:38:51.1370125Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-16T14:38:51.1370677Z 
2020-01-16T14:38:51.1371003Z error: make failed
2020-01-16T14:38:51.1371003Z error: make failed
2020-01-16T14:38:51.1371175Z status: exit code: 2
2020-01-16T14:38:51.1371520Z command: "make"
2020-01-16T14:38:51.1371849Z stdout:
2020-01-16T14:38:51.1372471Z ------------------------------------------
2020-01-16T14:38:51.1372876Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-16T14:38:51.1373065Z 
2020-01-16T14:38:51.1374749Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-16T14:38:51.1376461Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-16T14:38:51.1377261Z Makefile:4: recipe for target 'all' failed
2020-01-16T14:38:51.1378896Z ------------------------------------------
2020-01-16T14:38:51.1379115Z stderr:
2020-01-16T14:38:51.1379547Z ------------------------------------------
2020-01-16T14:38:51.1379547Z ------------------------------------------
2020-01-16T14:38:51.1379766Z error[E0412]: cannot find type `Lrc` in this scope
2020-01-16T14:38:51.1380471Z    --> the_backend.rs:78:16
2020-01-16T14:38:51.1380745Z     |
2020-01-16T14:38:51.1380910Z 78  |         sess: &Lrc<Session>,
2020-01-16T14:38:51.1381240Z     | 
2020-01-16T14:38:51.1381400Z    ::: /checkout/src/liballoc/sync.rs:196:1
2020-01-16T14:38:51.1381589Z     |
2020-01-16T14:38:51.1381589Z     |
2020-01-16T14:38:51.1381779Z 196 | pub struct Arc<T: ?Sized> {
2020-01-16T14:38:51.1382292Z     | ------------------------- similarly named struct `Arc` defined here
2020-01-16T14:38:51.1382698Z help: a struct with a similar name exists
2020-01-16T14:38:51.1382882Z     |
2020-01-16T14:38:51.1383067Z 78  |         sess: &Arc<Session>,
2020-01-16T14:38:51.1383229Z     |                ^^^
2020-01-16T14:38:51.1383229Z     |                ^^^
2020-01-16T14:38:51.1383405Z help: possible candidate is found in another module, you can import it into scope
2020-01-16T14:38:51.1383947Z     |
2020-01-16T14:38:51.1384107Z 12  | use rustc_data_structures::sync::Lrc;
2020-01-16T14:38:51.1384264Z     |
2020-01-16T14:38:51.1384418Z 
2020-01-16T14:38:51.1384901Z warning: ignoring --out-dir flag due to -o flag
2020-01-16T14:38:51.1385274Z error: aborting due to previous error
2020-01-16T14:38:51.1385412Z 
2020-01-16T14:38:51.1385851Z For more information about this error, try `rustc --explain E0412`.
2020-01-16T14:38:51.1385851Z For more information about this error, try `rustc --explain E0412`.
2020-01-16T14:38:51.1386080Z make: *** [all] Error 1
2020-01-16T14:38:51.1386923Z ------------------------------------------
2020-01-16T14:38:51.1387108Z 
2020-01-16T14:38:51.1387267Z 
2020-01-16T14:38:51.1387457Z 
---
2020-01-16T14:38:51.1390041Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-16T14:38:51.1390266Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-16T14:38:51.1393249Z 
2020-01-16T14:38:51.1393913Z 
2020-01-16T14:38:51.1399809Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-16T14:38:51.1400949Z 
2020-01-16T14:38:51.1400984Z 
2020-01-16T14:38:51.1461076Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-16T14:38:51.1461853Z Build completed unsuccessfully in 1:46:28
2020-01-16T14:38:51.1461853Z Build completed unsuccessfully in 1:46:28
2020-01-16T14:38:51.1467702Z == clock drift check ==
2020-01-16T14:38:51.6053422Z   local time: Thu Jan 16 14:38:51 UTC 2020
2020-01-16T14:38:51.7741938Z   network time: Thu, 16 Jan 2020 14:38:51 GMT
2020-01-16T14:38:51.7742657Z == end clock drift check ==
2020-01-16T14:38:52.6164061Z 
2020-01-16T14:38:52.6264424Z ##[error]Bash exited with code '1'.
2020-01-16T14:38:52.6307204Z ##[section]Starting: Checkout
2020-01-16T14:38:52.6309492Z ==============================================================================
2020-01-16T14:38:52.6309557Z Task         : Get sources
2020-01-16T14:38:52.6309609Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
