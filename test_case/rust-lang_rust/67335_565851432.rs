plain
2019-12-15T20:20:31.0022179Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-15T20:20:32.0007937Z ##[command]git config gc.auto 0
2019-12-15T20:20:32.0013431Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-15T20:20:32.0018482Z ##[command]git config --get-all http.proxy
2019-12-15T20:20:32.0023956Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67335/merge:refs/remotes/pull/67335/merge
---
2019-12-15T21:15:18.5487852Z .................................................................................................... 1600/9380
2019-12-15T21:15:22.4174765Z .................................................................................................... 1700/9380
2019-12-15T21:15:33.2263569Z ...................................................................i................................ 1800/9380
2019-12-15T21:15:39.6419699Z .................................................................................................... 1900/9380
2019-12-15T21:15:53.0282296Z ....................................................iiiii........................................... 2000/9380
2019-12-15T21:16:02.2267969Z .................................................................................................... 2200/9380
2019-12-15T21:16:04.4272270Z .................................................................................................... 2300/9380
2019-12-15T21:16:07.5177437Z .................................................................................................... 2400/9380
2019-12-15T21:16:27.9083223Z .................................................................................................... 2500/9380
---
2019-12-15T21:18:52.9921820Z .............................................................i...............i...................... 4800/9380
2019-12-15T21:18:59.8873554Z .................................................................................................... 4900/9380
2019-12-15T21:19:07.3262094Z .................................................................................................... 5000/9380
2019-12-15T21:19:11.8723706Z .....i.............................................................................................. 5100/9380
2019-12-15T21:19:21.0909077Z .......................................................................ii.ii...........i............ 5200/9380
2019-12-15T21:19:28.9141635Z .......i............................................................................................ 5400/9380
2019-12-15T21:19:37.8864957Z .................................................................................................... 5500/9380
2019-12-15T21:19:43.6873200Z .....................................................i.............................................. 5600/9380
2019-12-15T21:19:49.8559176Z .................................................................................................... 5700/9380
2019-12-15T21:19:49.8559176Z .................................................................................................... 5700/9380
2019-12-15T21:19:58.7887550Z .................................................................................................... 5800/9380
2019-12-15T21:20:05.1930297Z .........................................ii...i...ii..........i..................................... 5900/9380
2019-12-15T21:20:24.8251660Z .................................................................................................... 6100/9380
2019-12-15T21:20:32.0309378Z .................................................................................................... 6200/9380
2019-12-15T21:20:32.0309378Z .................................................................................................... 6200/9380
2019-12-15T21:20:41.3711195Z ..................................................................i..ii............................. 6300/9380
2019-12-15T21:21:06.8889876Z .................................................................................................... 6500/9380
2019-12-15T21:21:08.7772526Z ......................................i............................................................. 6600/9380
2019-12-15T21:21:10.7868414Z .................................................................................................... 6700/9380
2019-12-15T21:21:12.9044881Z ..............................i..................................................................... 6800/9380
---
2019-12-15T21:22:38.7080428Z .................................................................................................... 7400/9380
2019-12-15T21:22:42.9596910Z .................................................................................................... 7500/9380
2019-12-15T21:22:48.0864650Z .................................................................................................... 7600/9380
2019-12-15T21:22:56.4299029Z .................................................................................................... 7700/9380
2019-12-15T21:23:04.5001790Z ...................................................iiii............................................. 7800/9380
2019-12-15T21:23:17.4332081Z .................................................................................................... 8000/9380
2019-12-15T21:23:23.0361044Z .................................................................................................... 8100/9380
2019-12-15T21:23:36.7972039Z .................................................................................................... 8200/9380
2019-12-15T21:23:43.7273997Z .................................................................................................... 8300/9380
---
2019-12-15T21:25:53.1760991Z  finished in 5.689
2019-12-15T21:25:53.1933095Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:25:53.3904344Z 
2019-12-15T21:25:53.3905311Z running 166 tests
2019-12-15T21:25:56.1709212Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-15T21:25:58.1191769Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-15T21:25:58.1196067Z 
2019-12-15T21:25:58.1200257Z  finished in 4.926
2019-12-15T21:25:58.1372822Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:25:58.2870597Z 
---
2019-12-15T21:26:00.0644001Z  finished in 1.927
2019-12-15T21:26:00.0836571Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:26:00.2323684Z 
2019-12-15T21:26:00.2323815Z running 9 tests
2019-12-15T21:26:00.2324614Z iiiiiiiii
2019-12-15T21:26:00.2325051Z 
2019-12-15T21:26:00.2325100Z  finished in 0.148
2019-12-15T21:26:00.2482972Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:26:00.4020874Z 
---
2019-12-15T21:26:17.5548360Z  finished in 17.306
2019-12-15T21:26:17.5781900Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:26:17.7402320Z 
2019-12-15T21:26:17.7403370Z running 124 tests
2019-12-15T21:26:40.7027681Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-15T21:26:44.6164521Z .i.iii.....iiiiii.....ii
2019-12-15T21:26:44.6166070Z 
2019-12-15T21:26:44.6169652Z  finished in 27.038
2019-12-15T21:26:44.6176341Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T21:26:44.6178019Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-15T21:38:40.4495432Z 
2019-12-15T21:38:40.4499098Z    Doc-tests core
2019-12-15T21:38:44.4443732Z 
2019-12-15T21:38:44.4444556Z running 2437 tests
2019-12-15T21:38:52.5055570Z ......iiiii......................................................................................... 100/2437
2019-12-15T21:39:00.3849424Z ..................................................................................ii................ 200/2437
2019-12-15T21:39:19.0031670Z ................i................................................................................... 400/2437
2019-12-15T21:39:19.0031670Z ................i................................................................................... 400/2437
2019-12-15T21:39:27.2873265Z ................................................................i..i..................iiii.......... 500/2437
2019-12-15T21:39:41.7887027Z .................................................................................................... 700/2437
2019-12-15T21:39:49.3029604Z .................................................................................................... 800/2437
2019-12-15T21:39:57.2657245Z .................................................................................................... 900/2437
2019-12-15T21:40:05.3455855Z .................................................................................................... 1000/2437
---
2019-12-15T21:43:18.4289553Z .................................................................................................... 500/756
2019-12-15T21:43:18.4583406Z .............thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.4602910Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.4610246Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.4637436Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.7574649Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:.1189:5
2019-12-15T21:43:18.7588859Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.7597182Z <unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-15T21:43:18.7621208Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-15T21:43:20.8094997Z ...............thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:627:13
2019-12-15T21:43:20.8097791Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:582:13
2019-12-15T21:43:20.8104661Z ......thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:559:13
2019-12-15T21:43:20.8106999Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:687:13
2019-12-15T21:43:20.8106999Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:687:13
2019-12-15T21:43:20.8124209Z ...thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-12-15T21:43:20.8124601Z   left: `1`,
2019-12-15T21:43:20.8126325Z  right: `2`', src/libstd/sync/mutex.rs:651:13
2019-12-15T21:43:20.8185702Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:791:13
2019-12-15T21:43:20.8198867Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2019-12-15T21:43:20.8212656Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-12-15T21:43:20.8223324Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:635:13
2019-12-15T21:43:20.8234442Z thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:646:13
2019-12-15T21:43:20.8242789Z thread '<unnamed>' panicked at 'explicit panic', .src/libstd/sync/rwlock.rs:611:13
2019-12-15T21:43:20.8251247Z ....thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-12-15T21:43:22.8670334Z .......................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-12-15T21:43:22.8822511Z .............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1544:13
2019-12-15T21:43:23.4929337Z ...........thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1676:13
2019-12-15T21:43:23.4937704Z .thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1662:13
2019-12-15T21:43:23.4946656Z .thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1648:13
---
2019-12-15T21:43:30.0193572Z 
2019-12-15T21:43:30.0194933Z running 1003 tests
2019-12-15T21:43:45.7130298Z i................................................................................................... 100/1003
2019-12-15T21:43:55.0613791Z .................................................................................................... 200/1003
2019-12-15T21:44:01.6008666Z ..................iii......i......i...i......i...................................................... 300/1003
2019-12-15T21:44:06.0000878Z .................................................................................................... 400/1003
2019-12-15T21:44:12.2881748Z ..........................................i..i.....................................ii............... 500/1003
2019-12-15T21:44:24.3129375Z .................................................................................................... 700/1003
2019-12-15T21:44:24.3129375Z .................................................................................................... 700/1003
2019-12-15T21:44:30.4364220Z .............................iiii................................................................... 800/1003
2019-12-15T21:44:43.1504209Z .................................................................................................... 900/1003
2019-12-15T21:44:49.2916615Z ...................................................iiii............................................. 1000/1003
2019-12-15T21:44:49.3910426Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-12-15T21:44:49.3910693Z 
2019-12-15T21:44:49.4029278Z  finished in 163.872
2019-12-15T21:44:49.4042879Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-12-15T22:01:42.8977171Z  finished in 34.496
2019-12-15T22:01:42.9300904Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-15T22:01:43.1145688Z 
2019-12-15T22:01:43.1145945Z running 206 tests
2019-12-15T22:02:11.2435721Z ....................i...ii....................................................................i..... 100/206
2019-12-15T22:02:40.8974084Z ...............F..................iiiiii......i............iiii.iii................................. 200/206
2019-12-15T22:02:41.6970713Z failures:
2019-12-15T22:02:41.6970852Z 
2019-12-15T22:02:41.6971107Z ---- [run-make] run-make-fulldeps/min-global-align stdout ----
2019-12-15T22:02:41.6971136Z 
2019-12-15T22:02:41.6971136Z 
2019-12-15T22:02:41.6971175Z error: make failed
2019-12-15T22:02:41.6971235Z status: exit code: 2
2019-12-15T22:02:41.6971272Z command: "make"
2019-12-15T22:02:41.6971307Z stdout:
2019-12-15T22:02:41.6971527Z ------------------------------------------
2019-12-15T22:02:41.6972487Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/min-global-align/min-global-align  --target=i686-unknown-linux-gnu --emit=llvm-ir min_global_align.rs
2019-12-15T22:02:41.6972830Z Makefile:15: recipe for target 'all' failed
2019-12-15T22:02:41.6973055Z ------------------------------------------
2019-12-15T22:02:41.6973115Z stderr:
2019-12-15T22:02:41.6973298Z ------------------------------------------
2019-12-15T22:02:41.6973346Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-12-15T22:02:41.6973346Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-12-15T22:02:41.6973696Z   --> min_global_align.rs:10:44
2019-12-15T22:02:41.6973738Z    |
2019-12-15T22:02:41.6973957Z 10 | pub static CONST_BOOL_REF: &'static bool = &CONST_BOOL;
2019-12-15T22:02:41.6974051Z 
2019-12-15T22:02:41.6974087Z error: aborting due to previous error
2019-12-15T22:02:41.6974111Z 
2019-12-15T22:02:41.6974344Z For more information about this error, try `rustc --explain E0492`.
2019-12-15T22:02:41.6974344Z For more information about this error, try `rustc --explain E0492`.
2019-12-15T22:02:41.6974387Z make: *** [all] Error 1
2019-12-15T22:02:41.6974593Z ------------------------------------------
2019-12-15T22:02:41.6974640Z 
2019-12-15T22:02:41.6974662Z 
2019-12-15T22:02:41.6974683Z 
---
2019-12-15T22:02:41.6981380Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-15T22:02:41.6981458Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-15T22:02:41.6987465Z 
2019-12-15T22:02:41.6987566Z 
2019-12-15T22:02:41.6997634Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-15T22:02:41.6998521Z 
2019-12-15T22:02:41.6998550Z 
2019-12-15T22:02:41.7001603Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-15T22:02:41.7001664Z Build completed unsuccessfully in 1:36:22
2019-12-15T22:02:41.7001664Z Build completed unsuccessfully in 1:36:22
2019-12-15T22:02:41.7051564Z == clock drift check ==
2019-12-15T22:02:41.7993679Z   local time: Sun Dec 15 22:02:41 UTC 2019
2019-12-15T22:02:43.0278312Z   network time: Sun, 15 Dec 2019 22:02:43 GMT
2019-12-15T22:02:43.0288037Z == end clock drift check ==
2019-12-15T22:02:51.4357130Z 
2019-12-15T22:02:51.4471903Z ##[error]Bash exited with code '1'.
2019-12-15T22:02:51.4541638Z ##[section]Starting: Checkout
2019-12-15T22:02:51.4544070Z ==============================================================================
2019-12-15T22:02:51.4544164Z Task         : Get sources
2019-12-15T22:02:51.4544218Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
