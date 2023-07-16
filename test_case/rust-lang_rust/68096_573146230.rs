plain
2020-01-10T16:27:21.8201072Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T16:27:21.8295699Z ##[command]git config gc.auto 0
2020-01-10T16:27:21.8372298Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T16:27:21.8438762Z ##[command]git config --get-all http.proxy
2020-01-10T16:27:21.8591348Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68096/merge:refs/remotes/pull/68096/merge
---
2020-01-10T17:24:51.0368023Z .............................i...............i...................................................... 4900/9503
2020-01-10T17:25:00.5656168Z .................................................................................................... 5000/9503
2020-01-10T17:25:06.9600441Z ..........................................................................i......................... 5100/9503
2020-01-10T17:25:12.9701158Z .................................................................................................... 5200/9503
2020-01-10T17:25:22.5015155Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T17:25:31.5101712Z .................................................................................................... 5500/9503
2020-01-10T17:25:40.9691027Z .................................................................................................... 5600/9503
2020-01-10T17:25:47.8913178Z .........................i.......................................................................... 5700/9503
2020-01-10T17:25:53.9203934Z .................................................................................................... 5800/9503
2020-01-10T17:25:53.9203934Z .................................................................................................... 5800/9503
2020-01-10T17:26:05.3265808Z .................................................................................................... 5900/9503
2020-01-10T17:26:15.7836061Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T17:26:32.8352056Z .................................................................................................... 6200/9503
2020-01-10T17:26:40.5554384Z .................................................................................................... 6300/9503
2020-01-10T17:26:40.5554384Z .................................................................................................... 6300/9503
2020-01-10T17:26:55.1482496Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T17:27:16.6403532Z .................................................................................................... 6600/9503
2020-01-10T17:27:18.7558119Z ..................i................................................................................. 6700/9503
2020-01-10T17:27:21.0122314Z .................................................................................................... 6800/9503
2020-01-10T17:27:23.5085390Z ..................i................................................................................. 6900/9503
---
2020-01-10T17:29:02.0203651Z .................................................................................................... 7500/9503
2020-01-10T17:29:05.7396634Z .................................................................................................... 7600/9503
2020-01-10T17:29:11.5952580Z .................................................................................................... 7700/9503
2020-01-10T17:29:19.8802766Z .................................................................................................... 7800/9503
2020-01-10T17:29:29.4930828Z .....................................................................iiii........................... 7900/9503
2020-01-10T17:29:44.8644755Z ...i......i......................................................................................... 8100/9503
2020-01-10T17:29:49.9651097Z .................................................................................................... 8200/9503
2020-01-10T17:30:04.9801462Z .................................................................................................... 8300/9503
2020-01-10T17:30:13.4496452Z .................................................................................................... 8400/9503
---
2020-01-10T17:32:36.7482231Z  finished in 6.944
2020-01-10T17:32:36.7675259Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:32:36.9299603Z 
2020-01-10T17:32:36.9300603Z running 166 tests
2020-01-10T17:32:39.9043045Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T17:32:42.1384039Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T17:32:42.1384809Z 
2020-01-10T17:32:42.1387005Z  finished in 5.370
2020-01-10T17:32:42.1568466Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:32:42.3088978Z 
---
2020-01-10T17:32:44.2371969Z  finished in 2.080
2020-01-10T17:32:44.2553215Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:32:44.4042338Z 
2020-01-10T17:32:44.4043081Z running 9 tests
2020-01-10T17:32:44.4044042Z iiiiiiiii
2020-01-10T17:32:44.4044440Z 
2020-01-10T17:32:44.4044508Z  finished in 0.149
2020-01-10T17:32:44.4221135Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:32:44.5743001Z 
---
2020-01-10T17:33:04.4281908Z  finished in 20.006
2020-01-10T17:33:04.4469631Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:33:04.6142611Z 
2020-01-10T17:33:04.6142833Z running 124 tests
2020-01-10T17:33:28.2574288Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T17:33:32.3496044Z .i.iii.....iiiiii.....ii
2020-01-10T17:33:32.3496514Z 
2020-01-10T17:33:32.3503245Z  finished in 27.903
2020-01-10T17:33:32.3509471Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:33:32.3509846Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-10T17:46:44.5406578Z 
2020-01-10T17:46:44.5407336Z    Doc-tests core
2020-01-10T17:46:49.3039591Z 
2020-01-10T17:46:49.3040376Z running 2441 tests
2020-01-10T17:46:58.2926820Z ......iiiii......................................................................................... 100/2441
2020-01-10T17:47:07.1841206Z ..................................................................................ii................ 200/2441
2020-01-10T17:47:28.0017116Z ................i................................................................................... 400/2441
2020-01-10T17:47:28.0017116Z ................i................................................................................... 400/2441
2020-01-10T17:47:37.4574089Z .................................................................i..i..................iiii......... 500/2441
2020-01-10T17:47:53.4806877Z .................................................................................................... 700/2441
2020-01-10T17:48:02.0593005Z .................................................................................................... 800/2441
2020-01-10T17:48:10.5086637Z .................................................................................................... 900/2441
2020-01-10T17:48:19.0132017Z .................................................................................................... 1000/2441
---
2020-01-10T17:51:58.0502915Z 
2020-01-10T17:51:58.0503173Z running 1003 tests
2020-01-10T17:52:16.5478823Z i................................................................................................... 100/1003
2020-01-10T17:52:26.4293174Z .................................................................................................... 200/1003
2020-01-10T17:52:33.5545924Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-10T17:52:38.4374453Z .................................................................................................... 400/1003
2020-01-10T17:52:45.1922628Z ..........................................i..i.....................................ii............... 500/1003
2020-01-10T17:52:57.9606128Z .................................................................................................... 700/1003
2020-01-10T17:52:57.9606128Z .................................................................................................... 700/1003
2020-01-10T17:53:04.3121637Z .............................iiii................................................................... 800/1003
2020-01-10T17:53:18.1173337Z .................................................................................................... 900/1003
2020-01-10T17:53:25.0300605Z ...................................................iiii............................................. 1000/1003
2020-01-10T17:53:25.0975895Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-10T17:53:25.0976200Z 
2020-01-10T17:53:25.1075720Z  finished in 172.434
2020-01-10T17:53:25.1087420Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-10T18:11:48.0417708Z  finished in 37.728
2020-01-10T18:11:48.0788278Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T18:11:48.3058488Z 
2020-01-10T18:11:48.3059305Z running 206 tests
2020-01-10T18:12:19.7009005Z ....................i...ii....................................................................i...F. 100/206
2020-01-10T18:12:52.2470356Z ..................................iiiiii......i............iiii.iii................................. 200/206
2020-01-10T18:12:54.3288210Z failures:
2020-01-10T18:12:54.3296383Z 
2020-01-10T18:12:54.3296928Z ---- [run-make] run-make-fulldeps/libtest-json stdout ----
2020-01-10T18:12:54.3296971Z 
2020-01-10T18:12:54.3296971Z 
2020-01-10T18:12:54.3297017Z error: make failed
2020-01-10T18:12:54.3297062Z status: exit code: 2
2020-01-10T18:12:54.3297125Z command: "make" "make"
2020-01-10T18:12:54.3297168Z stdout:
2020-01-10T18:12:54.3297416Z ------------------------------------------
2020-01-10T18:12:54.3298284Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
2020-01-10T18:12:54.3300226Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json || true
2020-01-10T18:12:54.3301036Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json --show-output > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json || true
2020-01-10T18:12:54.3301396Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json | "/usr/bin/python2.7" validate_json.py
2020-01-10T18:12:54.3301744Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json | "/usr/bin/python2.7" validate_json.py
2020-01-10T18:12:54.3301799Z # Compare to output file
2020-01-10T18:12:54.3302286Z diff output-default.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json
2020-01-10T18:12:54.3302506Z 5c5
2020-01-10T18:12:54.3303075Z < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
2020-01-10T18:12:54.3303257Z ---
2020-01-10T18:12:54.3304106Z > { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
2020-01-10T18:12:54.3304413Z Makefile:9: recipe for target 'all' failed
2020-01-10T18:12:54.3305501Z ------------------------------------------
2020-01-10T18:12:54.3305583Z stderr:
2020-01-10T18:12:54.3305817Z ------------------------------------------
2020-01-10T18:12:54.3305817Z ------------------------------------------
2020-01-10T18:12:54.3305867Z make: *** [all] Error 1
2020-01-10T18:12:54.3306294Z ------------------------------------------
2020-01-10T18:12:54.3306326Z 
2020-01-10T18:12:54.3306351Z 
2020-01-10T18:12:54.3306376Z 
---
2020-01-10T18:12:54.3307312Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T18:12:54.3307379Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T18:12:54.3326858Z 
2020-01-10T18:12:54.3369591Z 
2020-01-10T18:12:54.3389308Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T18:12:54.3390657Z 
2020-01-10T18:12:54.3390683Z 
2020-01-10T18:12:54.3390722Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T18:12:54.3390779Z Build completed unsuccessfully in 1:40:19
2020-01-10T18:12:54.3390779Z Build completed unsuccessfully in 1:40:19
2020-01-10T18:12:54.3410110Z == clock drift check ==
2020-01-10T18:12:54.3410205Z   local time: Fri Jan 10 18:12:54 UTC 2020
2020-01-10T18:12:54.6431954Z   network time: Fri, 10 Jan 2020 18:12:54 GMT
2020-01-10T18:12:54.6432357Z == end clock drift check ==
2020-01-10T18:12:55.6071257Z 
2020-01-10T18:12:55.6163198Z ##[error]Bash exited with code '1'.
2020-01-10T18:12:55.6203884Z ##[section]Starting: Checkout
2020-01-10T18:12:55.6206182Z ==============================================================================
2020-01-10T18:12:55.6206239Z Task         : Get sources
2020-01-10T18:12:55.6206287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
