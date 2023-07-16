plain
2019-11-15T23:09:50.3418873Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-15T23:09:50.3596640Z ##[command]git config gc.auto 0
2019-11-15T23:09:50.3668495Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-15T23:09:50.3727086Z ##[command]git config --get-all http.proxy
2019-11-15T23:09:50.3849015Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66456/merge:refs/remotes/pull/66456/merge
---
2019-11-15T23:59:33.5576024Z .................................................................................................... 1500/9244
2019-11-15T23:59:39.1745163Z .................................................................................................... 1600/9244
2019-11-15T23:59:47.6535801Z .................................................................................................... 1700/9244
2019-11-15T23:59:55.6576863Z .....i.............................................................................................. 1800/9244
2019-11-16T00:00:01.8691524Z .........................................................................................iiiii...... 1900/9244
2019-11-16T00:00:21.1460620Z .................................................................................................... 2100/9244
2019-11-16T00:00:23.2845723Z .................................................................................................... 2200/9244
2019-11-16T00:00:25.4936107Z .................................................................................................... 2300/9244
2019-11-16T00:00:31.2218121Z .................................................................................................... 2400/9244
---
2019-11-16T00:03:08.8517711Z ........................................................................................i........... 4700/9244
2019-11-16T00:03:14.3699873Z ....i............................................................................................... 4800/9244
2019-11-16T00:03:21.5696003Z .................................................................................................... 4900/9244
2019-11-16T00:03:25.7255733Z .................................................................................................... 5000/9244
2019-11-16T00:03:33.9534076Z ............................................................................................ii.ii... 5100/9244
2019-11-16T00:03:40.7670187Z ............................i....................................................................... 5300/9244
2019-11-16T00:03:47.5525443Z .................................................................................................... 5400/9244
2019-11-16T00:03:54.3511028Z ..........................................................................i......................... 5500/9244
2019-11-16T00:04:00.6354028Z .................................................................................................... 5600/9244
2019-11-16T00:04:00.6354028Z .................................................................................................... 5600/9244
2019-11-16T00:04:06.4201096Z .................................................................................................... 5700/9244
2019-11-16T00:04:15.2870157Z ............................................................ii...i..ii...........i.................. 5800/9244
2019-11-16T00:04:34.0973685Z .................................................................................................... 6000/9244
2019-11-16T00:04:39.8623535Z .................................................................................................... 6100/9244
2019-11-16T00:04:39.8623535Z .................................................................................................... 6100/9244
2019-11-16T00:04:43.1549706Z ...............................................................................i..ii................ 6200/9244
2019-11-16T00:05:05.5914974Z .................................................................................................... 6400/9244
2019-11-16T00:05:08.1914916Z ...............................................i.................................................... 6500/9244
2019-11-16T00:05:09.8291358Z .................................................................................................... 6600/9244
2019-11-16T00:05:11.5837423Z ..................................i................................................................. 6700/9244
---
2019-11-16T00:09:14.7620232Z  finished in 5.060
2019-11-16T00:09:14.7752530Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:09:14.8966453Z 
2019-11-16T00:09:14.8966918Z running 156 tests
2019-11-16T00:09:17.1450080Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-16T00:09:18.6522643Z .i.i...iiii.............i.........iii.i.........ii......
2019-11-16T00:09:18.6523852Z 
2019-11-16T00:09:18.6526445Z  finished in 3.877
2019-11-16T00:09:18.6691769Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:09:18.7950766Z 
---
2019-11-16T00:09:20.4082306Z  finished in 1.738
2019-11-16T00:09:20.4270030Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:09:20.5662290Z 
2019-11-16T00:09:20.5662454Z running 9 tests
2019-11-16T00:09:20.5663172Z iiiiiiiii
2019-11-16T00:09:20.5663512Z 
2019-11-16T00:09:20.5663567Z  finished in 0.139
2019-11-16T00:09:20.5802636Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:09:20.7294949Z 
---
2019-11-16T00:09:36.8904118Z  finished in 16.310
2019-11-16T00:09:36.9036115Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:09:37.0287918Z 
2019-11-16T00:09:37.0289067Z running 123 tests
2019-11-16T00:09:56.5427986Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-16T00:10:00.4717068Z i.i.i......iii.i.....ii
2019-11-16T00:10:00.4717470Z 
2019-11-16T00:10:00.4717531Z  finished in 23.568
2019-11-16T00:10:00.4725226Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:10:00.4725558Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-16T00:19:48.0248738Z 
2019-11-16T00:19:48.0258222Z    Doc-tests core
2019-11-16T00:19:52.5235696Z 
2019-11-16T00:19:52.5237185Z running 2420 tests
2019-11-16T00:20:02.1256326Z ......iiiii......................................................................................... 100/2420
2019-11-16T00:20:11.3653253Z .................................................................................ii................. 200/2420
2019-11-16T00:20:33.7616914Z ...i................................................................................................ 400/2420
2019-11-16T00:20:33.7616914Z ...i................................................................................................ 400/2420
2019-11-16T00:20:43.1986553Z ...................................................i..i.................iiii........................ 500/2420
2019-11-16T00:21:01.2556383Z .................................................................................................... 700/2420
2019-11-16T00:21:10.5787116Z .................................................................................................... 800/2420
2019-11-16T00:21:19.7881669Z .................................................................................................... 900/2420
2019-11-16T00:21:29.3347743Z .................................................................................................... 1000/2420
---
2019-11-16T00:25:03.9894573Z .................................................................................................... 500/763
2019-11-16T00:25:04.0165361Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-16T00:25:04.0177620Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-16T00:25:04.0182126Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1187:5
2019-11-16T00:25:04.0210801Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-16T00:25:04.2413135Z ...........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:.1187:5
2019-11-16T00:25:04.2434972Z ....thread '<unnamed>..' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-16T00:25:04.2447726Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-16T00:25:06.2840448Z .............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-11-16T00:25:06.2840922Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-11-16T00:25:06.2841271Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-11-16T00:25:06.2841589Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-11-16T00:25:15.3849578Z 
2019-11-16T00:25:15.3860211Z running 999 tests
2019-11-16T00:25:32.7218558Z i................................................................................................... 100/999
2019-11-16T00:25:41.9889657Z .................................................................................................... 200/999
2019-11-16T00:25:48.4025852Z ..................iii......i......i...i......i...................................................... 300/999
2019-11-16T00:25:52.7338557Z .................................................................................................... 400/999
2019-11-16T00:25:58.8935537Z ..........................................i..i.................................ii................... 500/999
2019-11-16T00:26:09.9723047Z .................................................................................................... 700/999
2019-11-16T00:26:09.9723047Z .................................................................................................... 700/999
2019-11-16T00:26:15.9914641Z .........................iiii....................................................................... 800/999
2019-11-16T00:26:29.3901910Z .................................................................................................... 900/999
2019-11-16T00:26:35.7807148Z ...............................................iiii................................................
2019-11-16T00:26:35.7808014Z 
2019-11-16T00:26:35.7856989Z  finished in 167.153
2019-11-16T00:26:35.7868567Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:26:35.9740617Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-16T00:41:23.9028536Z  finished in 31.637
2019-11-16T00:41:23.9312921Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-16T00:41:24.1103288Z 
2019-11-16T00:41:24.1104808Z running 203 tests
2019-11-16T00:41:46.6989742Z ....................i...ii............................................F......................i...... 100/203
2019-11-16T00:42:17.4020704Z .................................iiii.......i...........iiii.iii.................................... 200/203
2019-11-16T00:42:17.7099238Z i..
2019-11-16T00:42:17.7103180Z 
2019-11-16T00:42:17.7103180Z 
2019-11-16T00:42:17.7103742Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2019-11-16T00:42:17.7104222Z error: make failed
2019-11-16T00:42:17.7104344Z status: exit code: 2
2019-11-16T00:42:17.7104441Z command: "make"
2019-11-16T00:42:17.7104535Z stdout:
2019-11-16T00:42:17.7104535Z stdout:
2019-11-16T00:42:17.7104862Z ------------------------------------------
2019-11-16T00:42:17.7106159Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2019-11-16T00:42:17.7106496Z Makefile:8: recipe for target 'all' failed
2019-11-16T00:42:17.7106734Z ------------------------------------------
2019-11-16T00:42:17.7106773Z stderr:
2019-11-16T00:42:17.7106954Z ------------------------------------------
2019-11-16T00:42:17.7107013Z error[E0063]: missing field `registry` in initializer of `rustc_interface::Config`
---
2019-11-16T00:42:17.7107470Z 
2019-11-16T00:42:17.7107502Z error: aborting due to previous error
2019-11-16T00:42:17.7107523Z 
2019-11-16T00:42:17.7107769Z For more information about this error, try `rustc --explain E0063`.
2019-11-16T00:42:17.7107810Z make: *** [all] Error 1
2019-11-16T00:42:17.7108024Z ------------------------------------------
2019-11-16T00:42:17.7108049Z 
2019-11-16T00:42:17.7108067Z 
2019-11-16T00:42:17.7108086Z 
---
2019-11-16T00:42:17.7108821Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-16T00:42:17.7108867Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-16T00:42:17.7111971Z 
2019-11-16T00:42:17.7112056Z 
2019-11-16T00:42:17.7116828Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-16T00:42:17.7119009Z 
2019-11-16T00:42:17.7119039Z 
2019-11-16T00:42:17.7121733Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-16T00:42:17.7121811Z Build completed unsuccessfully in 1:26:41
2019-11-16T00:42:17.7121811Z Build completed unsuccessfully in 1:26:41
2019-11-16T00:42:17.7165334Z == clock drift check ==
2019-11-16T00:42:17.7177988Z   local time: Sat Nov 16 00:42:17 UTC 2019
2019-11-16T00:42:18.2530010Z   network time: Sat, 16 Nov 2019 00:42:18 GMT
2019-11-16T00:42:18.2532049Z == end clock drift check ==
2019-11-16T00:42:22.0086909Z 
2019-11-16T00:42:22.0170235Z ##[error]Bash exited with code '1'.
2019-11-16T00:42:22.0221844Z ##[section]Starting: Checkout
2019-11-16T00:42:22.0224238Z ==============================================================================
2019-11-16T00:42:22.0224290Z Task         : Get sources
2019-11-16T00:42:22.0224491Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
