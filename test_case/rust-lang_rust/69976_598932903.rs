plain
2020-03-13T20:11:41.2942159Z ========================== Starting Command Output ===========================
2020-03-13T20:11:41.2946465Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd34dde3-6a3b-49d1-93dc-9669ff531565.sh
2020-03-13T20:11:41.2946702Z 
2020-03-13T20:11:41.2950893Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T20:11:41.2969785Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69976/merge to s
2020-03-13T20:11:41.2973094Z Task         : Get sources
2020-03-13T20:11:41.2973354Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:11:41.2973604Z Version      : 1.0.0
2020-03-13T20:11:41.2973822Z Author       : Microsoft
---
2020-03-13T20:11:42.8988683Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T20:11:42.8999066Z ##[command]git config gc.auto 0
2020-03-13T20:11:42.9005592Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T20:11:42.9012108Z ##[command]git config --get-all http.proxy
2020-03-13T20:11:42.9022793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69976/merge:refs/remotes/pull/69976/merge
---
2020-03-13T21:09:07.9587960Z .................................................................................................... 1700/9766
2020-03-13T21:09:12.0161510Z .................................................................................................... 1800/9766
2020-03-13T21:09:22.5266749Z ................................................................i................................... 1900/9766
2020-03-13T21:09:28.8641769Z .................................................................................................... 2000/9766
2020-03-13T21:09:42.2367597Z ......................................................iiiii......................................... 2100/9766
2020-03-13T21:09:51.6608988Z .................................................................................................... 2300/9766
2020-03-13T21:09:53.7421714Z .................................................................................................... 2400/9766
2020-03-13T21:09:56.6355674Z .................................................................................................... 2500/9766
2020-03-13T21:10:16.2478504Z .................................................................................................... 2600/9766
---
2020-03-13T21:12:42.1764952Z .........................i...............i.......................................................... 5000/9766
2020-03-13T21:12:50.8785200Z .................................................................................................... 5100/9766
2020-03-13T21:12:56.0111958Z ....................................................................i............................... 5200/9766
2020-03-13T21:13:01.4934209Z .................................................................................................... 5300/9766
2020-03-13T21:13:09.8873989Z .................................................ii.ii........i...i................................. 5400/9766
2020-03-13T21:13:17.2089528Z .................................................................................................... 5600/9766
2020-03-13T21:13:25.8915696Z .................................................................................................... 5700/9766
2020-03-13T21:13:31.5419328Z ........................................i........................................................... 5800/9766
2020-03-13T21:13:37.2329092Z .................................................................................................... 5900/9766
2020-03-13T21:13:37.2329092Z .................................................................................................... 5900/9766
2020-03-13T21:13:47.1544149Z .................................................................................................... 6000/9766
2020-03-13T21:13:55.3928945Z .................................ii...i..ii...........i............................................. 6100/9766
2020-03-13T21:14:11.3685675Z .................................................................................................... 6300/9766
2020-03-13T21:14:17.7153216Z .................................................................................................... 6400/9766
2020-03-13T21:14:17.7153216Z .................................................................................................... 6400/9766
2020-03-13T21:14:28.0132913Z ................................................................i..ii............................... 6500/9766
2020-03-13T21:14:51.7451727Z .................................................................................................... 6700/9766
2020-03-13T21:14:56.5249774Z ..............................................................i..................................... 6800/9766
2020-03-13T21:14:58.4644008Z .................................................................................................... 6900/9766
2020-03-13T21:15:00.3933700Z ................................................................................................i... 7000/9766
---
2020-03-13T21:16:33.2984358Z .................................................................................................... 7700/9766
2020-03-13T21:16:37.5528926Z .................................................................................................... 7800/9766
2020-03-13T21:16:43.0057857Z .................................................................................................... 7900/9766
2020-03-13T21:16:48.7354138Z ..............................................i..................................................... 8000/9766
2020-03-13T21:16:58.2756725Z ...............................................................................................iiiii 8100/9766
2020-03-13T21:17:04.1222822Z iiiii.i............................................................................................. 8200/9766
2020-03-13T21:17:17.9363806Z .................................................................................................... 8400/9766
2020-03-13T21:17:28.2182945Z .................................................................................................... 8500/9766
2020-03-13T21:17:39.4489142Z .................................................................................................... 8600/9766
2020-03-13T21:17:44.5536525Z .................................................................................................... 8700/9766
---
2020-03-13T21:19:51.6722583Z  finished in 6.839
2020-03-13T21:19:51.6959684Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:19:51.8542653Z 
2020-03-13T21:19:51.8542926Z running 179 tests
2020-03-13T21:19:54.5888404Z iiii......i...........ii..iiii....i....i...........i............i..i..................i....i........ 100/179
2020-03-13T21:19:56.7329922Z ....i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-03-13T21:19:56.7332618Z 
2020-03-13T21:19:56.7339273Z  finished in 5.038
2020-03-13T21:19:56.7517511Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:19:56.8932943Z 
---
2020-03-13T21:19:58.6593350Z  finished in 1.907
2020-03-13T21:19:58.6755575Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:19:58.8059399Z 
2020-03-13T21:19:58.8059889Z running 9 tests
2020-03-13T21:19:58.8064023Z iiiiiiiii
2020-03-13T21:19:58.8064880Z 
2020-03-13T21:19:58.8065062Z  finished in 0.130
2020-03-13T21:19:58.8235865Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:19:58.9597510Z 
---
2020-03-13T21:20:16.8873351Z  finished in 18.065
2020-03-13T21:20:16.9035025Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:20:17.0370624Z 
2020-03-13T21:20:17.0370814Z running 115 tests
2020-03-13T21:20:29.3507967Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-13T21:20:31.0487199Z ...iiii.....ii.
2020-03-13T21:20:31.0488252Z 
2020-03-13T21:20:31.0492070Z  finished in 14.145
2020-03-13T21:20:31.0497764Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:20:31.0501350Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-13T21:32:33.7683931Z 
2020-03-13T21:32:33.7687917Z    Doc-tests core
2020-03-13T21:32:38.0675617Z 
2020-03-13T21:32:38.0676917Z running 2480 tests
2020-03-13T21:32:46.4717032Z ......iiiii......................................................................................... 100/2480
2020-03-13T21:32:54.7956060Z ....................................................................................ii.............. 200/2480
2020-03-13T21:33:13.7682082Z ...................i................................................................................ 400/2480
2020-03-13T21:33:13.7682082Z ...................i................................................................................ 400/2480
2020-03-13T21:33:22.5995208Z ........................................................................i..i..................iiii.. 500/2480
2020-03-13T21:33:37.5154476Z .................................................................................................... 700/2480
2020-03-13T21:33:45.4486772Z .................................................................................................... 800/2480
2020-03-13T21:33:53.1652514Z .................................................................................................... 900/2480
2020-03-13T21:34:00.9501209Z .................................................................................................... 1000/2480
---
2020-03-13T21:37:07.3523632Z .................................................................................................... 500/760
2020-03-13T21:37:07.3836594Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-13T21:37:07.3851809Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-03-13T21:37:07.3853284Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-03-13T21:37:07.3885364Z .........thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', .src/libstd/sync/mpsc/mod.rs:2645:13..
2020-03-13T21:37:07.6385873Z ....................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-13T21:37:07.6410677Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-03-13T21:37:07.6438851Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-13T21:37:09.7093550Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-13T21:37:09.7098520Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-03-13T21:37:09.7108037Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-03-13T21:37:09.7108632Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-03-13T21:37:18.5898733Z 
2020-03-13T21:37:18.5900366Z running 1010 tests
2020-03-13T21:37:34.9844414Z i................................................................................................... 100/1010
2020-03-13T21:37:44.2068388Z .................................................................................................... 200/1010
2020-03-13T21:37:50.7035021Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-13T21:37:55.3327693Z .................................................................................................... 400/1010
2020-03-13T21:38:01.5587735Z ............................................i..i......................................ii............ 500/1010
2020-03-13T21:38:13.1410563Z .................................................................................................... 700/1010
2020-03-13T21:38:13.1410563Z .................................................................................................... 700/1010
2020-03-13T21:38:19.3185492Z ....................................iiii............................................................ 800/1010
2020-03-13T21:38:32.3384990Z .................................................................................................... 900/1010
2020-03-13T21:38:38.6484737Z ..........................................................iiii...................................... 1000/1010
2020-03-13T21:38:39.0882469Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-13T21:38:39.0883038Z 
2020-03-13T21:38:39.0938374Z  finished in 153.513
2020-03-13T21:38:39.0951025Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-13T21:55:38.4010524Z  finished in 38.354
2020-03-13T21:55:38.4278679Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-13T21:55:38.5731515Z 
2020-03-13T21:55:38.5731918Z running 210 tests
2020-03-13T21:56:08.2281504Z ......................i...ii.......................................................................i 100/210
2020-03-13T21:56:44.8287827Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-13T21:56:46.2215260Z ......Fi..
2020-03-13T21:56:46.2216141Z 
2020-03-13T21:56:46.2229446Z ---- [run-make] run-make-fulldeps/sysroot-crates-are-unstable stdout ----
2020-03-13T21:56:46.2230047Z 
2020-03-13T21:56:46.2230312Z error: make failed
---
2020-03-13T21:56:46.2232842Z python2.7 test.py
2020-03-13T21:56:46.2233196Z verifying if byteorder is an unstable crate
2020-03-13T21:56:46.2233509Z verifying if backtrace is an unstable crate
2020-03-13T21:56:46.2233828Z verifying if parking_lot_core is an unstable crate
2020-03-13T21:56:46.2234158Z verifying if arena is an unstable crate
2020-03-13T21:56:46.2234474Z verifying if crc32fast is an unstable crate
2020-03-13T21:56:46.2234831Z verifying if rustc_error_codes is an unstable crate
2020-03-13T21:56:46.2235142Z verifying if log is an unstable crate
2020-03-13T21:56:46.2235604Z verifying if cc is an unstable crate
2020-03-13T21:56:46.2236597Z crate cc "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcc-82c555efc31f9ff7.rlib" is not unstable
2020-03-13T21:56:46.2237150Z error[E0514]: found crate `cc` compiled by an incompatible version of rustc
2020-03-13T21:56:46.2237661Z  --> <anon>:1:1
2020-03-13T21:56:46.2238136Z 1 | extern crate cc;
2020-03-13T21:56:46.2239700Z   | ^^^^^^^^^^^^^^^^
2020-03-13T21:56:46.2240036Z   |
2020-03-13T21:56:46.2240036Z   |
2020-03-13T21:56:46.2240859Z   = help: please recompile that crate using this compiler (rustc 1.43.0-nightly (d602e0a9c 2020-03-13))
2020-03-13T21:56:46.2241518Z   = note: the following crate versions were found:
2020-03-13T21:56:46.2242429Z           crate `cc` compiled by rustc 1.42.0-beta.5 (4e1c5f0e9 2020-02-28): /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcc-82c555efc31f9ff7.rlib
2020-03-13T21:56:46.2243220Z error: aborting due to previous error
2020-03-13T21:56:46.2243455Z 
2020-03-13T21:56:46.2243630Z 
2020-03-13T21:56:46.2243816Z 
---
2020-03-13T21:56:46.2250635Z verifying if rustc_builtin_macros is an unstable crate
2020-03-13T21:56:46.2251115Z verifying if crossbeam_queue is an unstable crate
2020-03-13T21:56:46.2251344Z verifying if rustc_demangle is an unstable crate
2020-03-13T21:56:46.2251688Z verifying if build_helper is an unstable crate
2020-03-13T21:56:46.2252047Z verifying if rustc_hir is an unstable crate
2020-03-13T21:56:46.2252254Z verifying if itoa is an unstable crate
2020-03-13T21:56:46.2252519Z verifying if datafrog is an unstable crate
2020-03-13T21:56:46.2252730Z verifying if rustc_llvm is an unstable crate
2020-03-13T21:56:46.2252963Z verifying if rustc_infer is an unstable crate
2020-03-13T21:56:46.2253170Z verifying if libc is an unstable crate
2020-03-13T21:56:46.2253372Z verifying if autocfg is an unstable crate
2020-03-13T21:56:46.2254445Z crate autocfg "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libautocfg-a7bdb82c04c44ac6.rlib" is not unstable
2020-03-13T21:56:46.2254851Z error[E0514]: found crate `autocfg` compiled by an incompatible version of rustc
2020-03-13T21:56:46.2255219Z  --> <anon>:1:1
2020-03-13T21:56:46.2255459Z 1 | extern crate autocfg;
2020-03-13T21:56:46.2255614Z   | ^^^^^^^^^^^^^^^^^^^^^
2020-03-13T21:56:46.2255751Z   |
2020-03-13T21:56:46.2255751Z   |
2020-03-13T21:56:46.2256170Z   = help: please recompile that crate using this compiler (rustc 1.43.0-nightly (d602e0a9c 2020-03-13))
2020-03-13T21:56:46.2256465Z   = note: the following crate versions were found:
2020-03-13T21:56:46.2257197Z           crate `autocfg` compiled by rustc 1.42.0-beta.5 (4e1c5f0e9 2020-02-28): /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libautocfg-a7bdb82c04c44ac6.rlib
2020-03-13T21:56:46.2257709Z error: aborting due to previous error
2020-03-13T21:56:46.2257849Z 
2020-03-13T21:56:46.2257922Z 
2020-03-13T21:56:46.2257992Z 
---
2020-03-13T21:56:46.2282662Z 
2020-03-13T21:56:46.2282957Z ------------------------------------------
2020-03-13T21:56:46.2283137Z stderr:
2020-03-13T21:56:46.2283444Z ------------------------------------------
2020-03-13T21:56:46.2283633Z make: *** [all] Error 1
2020-03-13T21:56:46.2284059Z ------------------------------------------
2020-03-13T21:56:46.2284196Z 
2020-03-13T21:56:46.2284274Z 
2020-03-13T21:56:46.2284350Z 
---
2020-03-13T21:56:46.2286212Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T21:56:46.2286537Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T21:56:46.2286899Z 
2020-03-13T21:56:46.2287156Z 
2020-03-13T21:56:46.2295797Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T21:56:46.2301899Z 
2020-03-13T21:56:46.2301983Z 
2020-03-13T21:56:46.2302178Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T21:56:46.2302474Z Build completed unsuccessfully in 1:38:43
2020-03-13T21:56:46.2302474Z Build completed unsuccessfully in 1:38:43
2020-03-13T21:56:46.2339276Z == clock drift check ==
2020-03-13T21:56:46.2373015Z   local time: Fri Mar 13 21:56:46 UTC 2020
2020-03-13T21:56:46.3447614Z   network time: Fri, 13 Mar 2020 21:56:46 GMT
2020-03-13T21:56:46.3451973Z == end clock drift check ==
2020-03-13T21:56:49.0172349Z 
2020-03-13T21:56:49.0240619Z ##[error]Bash exited with code '1'.
2020-03-13T21:56:49.0251742Z ##[section]Finishing: Run build
2020-03-13T21:56:49.0300713Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69976/merge to s
2020-03-13T21:56:49.0305500Z Task         : Get sources
2020-03-13T21:56:49.0305772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T21:56:49.0306038Z Version      : 1.0.0
2020-03-13T21:56:49.0306214Z Author       : Microsoft
2020-03-13T21:56:49.0306214Z Author       : Microsoft
2020-03-13T21:56:49.0306487Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T21:56:49.0306827Z ==============================================================================
2020-03-13T21:56:49.3441269Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T21:56:49.3501592Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69976/merge to s
2020-03-13T21:56:49.3582229Z Cleaning up task key
2020-03-13T21:56:49.3583304Z Start cleaning up orphan processes.
2020-03-13T21:56:49.3758240Z Terminate orphan process: pid (4140) (python)
2020-03-13T21:56:49.4011571Z ##[section]Finishing: Finalize Job
