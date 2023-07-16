plain
2020-01-10T18:24:19.1569946Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T18:24:19.1669625Z ##[command]git config gc.auto 0
2020-01-10T18:24:19.1748663Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T18:24:19.1802722Z ##[command]git config --get-all http.proxy
2020-01-10T18:24:19.1961792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68096/merge:refs/remotes/pull/68096/merge
---
2020-01-10T19:25:49.0140549Z .............................i...............i...................................................... 4900/9503
2020-01-10T19:25:59.2100391Z .................................................................................................... 5000/9503
2020-01-10T19:26:06.0782528Z ..........................................................................i......................... 5100/9503
2020-01-10T19:26:12.5647149Z .................................................................................................... 5200/9503
2020-01-10T19:26:22.6118904Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T19:26:32.6365010Z .................................................................................................... 5500/9503
2020-01-10T19:26:42.8738584Z .................................................................................................... 5600/9503
2020-01-10T19:26:50.2894387Z .........................i.......................................................................... 5700/9503
2020-01-10T19:26:56.7287065Z .................................................................................................... 5800/9503
2020-01-10T19:26:56.7287065Z .................................................................................................... 5800/9503
2020-01-10T19:27:08.7050007Z .................................................................................................... 5900/9503
2020-01-10T19:27:19.9674724Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T19:27:38.7203929Z .................................................................................................... 6200/9503
2020-01-10T19:27:47.4671901Z .................................................................................................... 6300/9503
2020-01-10T19:27:47.4671901Z .................................................................................................... 6300/9503
2020-01-10T19:28:01.3508195Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T19:28:24.3597865Z .................................................................................................... 6600/9503
2020-01-10T19:28:26.5679644Z ..................i................................................................................. 6700/9503
2020-01-10T19:28:28.9997016Z .................................................................................................... 6800/9503
2020-01-10T19:28:31.6317433Z ..................i................................................................................. 6900/9503
---
2020-01-10T19:30:15.3358100Z .................................................................................................... 7500/9503
2020-01-10T19:30:19.5401159Z .................................................................................................... 7600/9503
2020-01-10T19:30:25.7630875Z .................................................................................................... 7700/9503
2020-01-10T19:30:34.7404598Z .................................................................................................... 7800/9503
2020-01-10T19:30:45.0397924Z .....................................................................iiii........................... 7900/9503
2020-01-10T19:31:01.2444861Z ...i......i......................................................................................... 8100/9503
2020-01-10T19:31:06.6881563Z .................................................................................................... 8200/9503
2020-01-10T19:31:22.5352353Z .................................................................................................... 8300/9503
2020-01-10T19:31:31.8076684Z .................................................................................................... 8400/9503
---
2020-01-10T19:34:05.8152048Z  finished in 7.196
2020-01-10T19:34:05.8336593Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:34:06.0483463Z 
2020-01-10T19:34:06.0483748Z running 166 tests
2020-01-10T19:34:09.2661597Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T19:34:11.6184214Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T19:34:11.6185783Z 
2020-01-10T19:34:11.6190237Z  finished in 5.785
2020-01-10T19:34:12.6346363Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:34:12.6411156Z 
---
2020-01-10T19:34:13.8797677Z  finished in 2.241
2020-01-10T19:34:13.8983118Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:34:14.0656329Z 
2020-01-10T19:34:14.0658699Z running 9 tests
2020-01-10T19:34:14.0659572Z iiiiiiiii
2020-01-10T19:34:14.0660003Z 
2020-01-10T19:34:14.0660049Z  finished in 0.166
2020-01-10T19:34:14.0879747Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:34:14.3046111Z 
---
2020-01-10T19:34:35.8261699Z  finished in 21.738
2020-01-10T19:34:35.8510774Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:34:36.0617132Z 
2020-01-10T19:34:36.0618803Z running 124 tests
2020-01-10T19:35:00.5763022Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T19:35:04.7449808Z .i.iii.....iiiiii.....ii
2020-01-10T19:35:04.7451191Z 
2020-01-10T19:35:04.7451482Z  finished in 28.894
2020-01-10T19:35:04.7460497Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T19:35:04.7460870Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-10T19:49:04.1195177Z 
2020-01-10T19:49:04.1199846Z    Doc-tests core
2020-01-10T19:49:09.1297787Z 
2020-01-10T19:49:09.1298334Z running 2441 tests
2020-01-10T19:49:19.0122865Z ......iiiii......................................................................................... 100/2441
2020-01-10T19:49:28.4046237Z ..................................................................................ii................ 200/2441
2020-01-10T19:49:50.7716740Z ................i................................................................................... 400/2441
2020-01-10T19:49:50.7716740Z ................i................................................................................... 400/2441
2020-01-10T19:50:00.9876575Z .................................................................i..i..................iiii......... 500/2441
2020-01-10T19:50:18.4054507Z .................................................................................................... 700/2441
2020-01-10T19:50:27.6499268Z .................................................................................................... 800/2441
2020-01-10T19:50:36.7469650Z .................................................................................................... 900/2441
2020-01-10T19:50:45.7630896Z .................................................................................................... 1000/2441
---
2020-01-10T19:54:24.0422993Z .................................................................................................... 500/760
2020-01-10T19:54:24.0829473Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-01-10T19:54:24.0847726Z ....thread '<unnamed>thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-01-10T19:54:24.0858731Z ' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-01-10T19:54:24.0880057Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-01-10T19:54:24.3673622Z ...........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-01-10T19:54:24.3700805Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs.:thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-01-10T19:54:24.3728143Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-01-10T19:54:24.3856544Z .................. 600/760
2020-01-10T19:54:26.4148262Z .......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-10T19:54:26.4150204Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-01-10T19:54:35.7745961Z 
2020-01-10T19:54:35.7746686Z running 1003 tests
2020-01-10T19:54:55.3957586Z i................................................................................................... 100/1003
2020-01-10T19:55:05.7912860Z .................................................................................................... 200/1003
2020-01-10T19:55:13.2622460Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-10T19:55:18.3685145Z .................................................................................................... 400/1003
2020-01-10T19:55:25.4676571Z ..........................................i..i.....................................ii............... 500/1003
2020-01-10T19:55:38.9447166Z .................................................................................................... 700/1003
2020-01-10T19:55:38.9447166Z .................................................................................................... 700/1003
2020-01-10T19:55:45.6734113Z .............................iiii................................................................... 800/1003
2020-01-10T19:56:00.3378546Z .................................................................................................... 900/1003
2020-01-10T19:56:07.5284598Z ...................................................iiii............................................. 1000/1003
2020-01-10T19:56:07.5950495Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-10T19:56:07.5950958Z 
2020-01-10T19:56:07.6055050Z  finished in 181.101
2020-01-10T19:56:07.6068660Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-10T20:15:48.6911489Z  finished in 40.383
2020-01-10T20:15:48.7293622Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T20:15:48.9552898Z 
2020-01-10T20:15:48.9553137Z running 206 tests
2020-01-10T20:16:22.9150795Z ....................i...ii....................................................................i...F. 100/206
2020-01-10T20:16:58.2492088Z ..................................iiiiii......i............iiii.iii................................. 200/206
2020-01-10T20:17:01.3228514Z failures:
2020-01-10T20:17:01.3235958Z 
2020-01-10T20:17:01.3237178Z ---- [run-make] run-make-fulldeps/libtest-json stdout ----
2020-01-10T20:17:01.3237398Z 
2020-01-10T20:17:01.3237398Z 
2020-01-10T20:17:01.3237553Z error: make failed
2020-01-10T20:17:01.3237715Z status: exit code: 2
2020-01-10T20:17:01.3237843Z command: "make" "make"
2020-01-10T20:17:01.3237962Z stdout:
2020-01-10T20:17:01.3238341Z ------------------------------------------
2020-01-10T20:17:01.3239279Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
2020-01-10T20:17:01.3240352Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json || true
2020-01-10T20:17:01.3241663Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json --show-output > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json || true
2020-01-10T20:17:01.3243160Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json | "/usr/bin/python2.7" validate_json.py
2020-01-10T20:17:01.3243934Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json | "/usr/bin/python2.7" validate_json.py
2020-01-10T20:17:01.3244183Z # Compare to output file
2020-01-10T20:17:01.3244723Z diff output-default.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json
2020-01-10T20:17:01.3245294Z diff output-stdout-success.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json
2020-01-10T20:17:01.3245506Z 5c5
2020-01-10T20:17:01.3246226Z < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
2020-01-10T20:17:01.3246607Z ---
2020-01-10T20:17:01.3247357Z > { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
2020-01-10T20:17:01.3247785Z Makefile:9: recipe for target 'all' failed
2020-01-10T20:17:01.3249220Z ------------------------------------------
2020-01-10T20:17:01.3249460Z stderr:
2020-01-10T20:17:01.3249859Z ------------------------------------------
2020-01-10T20:17:01.3249859Z ------------------------------------------
2020-01-10T20:17:01.3250041Z make: *** [all] Error 1
2020-01-10T20:17:01.3250540Z ------------------------------------------
2020-01-10T20:17:01.3250694Z 
2020-01-10T20:17:01.3250810Z 
2020-01-10T20:17:01.3250921Z 
---
2020-01-10T20:17:01.3257541Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T20:17:01.3257641Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T20:17:01.3264089Z 
2020-01-10T20:17:01.3264158Z 
2020-01-10T20:17:01.3274734Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T20:17:01.3275789Z 
2020-01-10T20:17:01.3275822Z 
2020-01-10T20:17:01.3287465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T20:17:01.3287540Z Build completed unsuccessfully in 1:47:01
2020-01-10T20:17:01.3287540Z Build completed unsuccessfully in 1:47:01
2020-01-10T20:17:01.3348895Z == clock drift check ==
2020-01-10T20:17:01.3371103Z   local time: Fri Jan 10 20:17:01 UTC 2020
2020-01-10T20:17:01.5056812Z   network time: Fri, 10 Jan 2020 20:17:01 GMT
2020-01-10T20:17:01.5060469Z == end clock drift check ==
2020-01-10T20:17:02.5745053Z 
2020-01-10T20:17:02.5883192Z ##[error]Bash exited with code '1'.
2020-01-10T20:17:02.5973916Z ##[section]Starting: Checkout
2020-01-10T20:17:02.5975742Z ==============================================================================
2020-01-10T20:17:02.5975800Z Task         : Get sources
2020-01-10T20:17:02.5975866Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
