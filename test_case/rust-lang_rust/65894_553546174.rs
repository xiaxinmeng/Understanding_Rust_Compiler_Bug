plain
2019-11-13T16:57:25.6983622Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T16:57:25.7186746Z ##[command]git config gc.auto 0
2019-11-13T16:57:25.7265503Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T16:57:25.7327070Z ##[command]git config --get-all http.proxy
2019-11-13T16:57:25.7471126Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65894/merge:refs/remotes/pull/65894/merge
---
2019-11-13T17:57:06.9908476Z .................................................................................................... 1500/9239
2019-11-13T17:57:12.9611139Z .................................................................................................... 1600/9239
2019-11-13T17:57:21.9406685Z .................................................................................................... 1700/9239
2019-11-13T17:57:30.3574910Z ....i............................................................................................... 1800/9239
2019-11-13T17:57:37.0192322Z ........................................................................................iiiii....... 1900/9239
2019-11-13T17:57:58.4707749Z .................................................................................................... 2100/9239
2019-11-13T17:58:00.8677278Z .................................................................................................... 2200/9239
2019-11-13T17:58:03.3561400Z .................................................................................................... 2300/9239
2019-11-13T17:58:10.1492492Z .................................................................................................... 2400/9239
---
2019-11-13T18:01:04.4220583Z .......................................................................................i............ 4700/9239
2019-11-13T18:01:11.0905323Z ...i................................................................................................ 4800/9239
2019-11-13T18:01:20.6156772Z .................................................................................................... 4900/9239
2019-11-13T18:01:26.0770172Z .................................................................................................... 5000/9239
2019-11-13T18:01:37.2890400Z ..........................................................................................ii.ii..... 5100/9239
2019-11-13T18:01:45.8315020Z .........................i.......................................................................... 5300/9239
2019-11-13T18:01:54.4600377Z .................................................................................................... 5400/9239
2019-11-13T18:02:02.9885094Z ........................................................................i........................... 5500/9239
2019-11-13T18:02:10.4316136Z .................................................................................................... 5600/9239
2019-11-13T18:02:10.4316136Z .................................................................................................... 5600/9239
2019-11-13T18:02:17.4781355Z .................................................................................................... 5700/9239
2019-11-13T18:02:27.1210067Z ..........................................................ii...i..ii...........i.................... 5800/9239
2019-11-13T18:02:49.8347088Z .................................................................................................... 6000/9239
2019-11-13T18:02:58.2133009Z .................................................................................................... 6100/9239
2019-11-13T18:02:58.2133009Z .................................................................................................... 6100/9239
2019-11-13T18:03:03.4559917Z .............................................................................i..ii.................. 6200/9239
2019-11-13T18:03:32.2495562Z .................................................................................................... 6400/9239
2019-11-13T18:03:35.0659375Z .............................................i...................................................... 6500/9239
2019-11-13T18:03:37.3021700Z .................................................................................................... 6600/9239
2019-11-13T18:03:39.7376266Z .............................i...................................................................... 6700/9239
---
2019-11-13T18:08:56.5435876Z  finished in 5.740
2019-11-13T18:08:56.5522663Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:08:56.7514499Z 
2019-11-13T18:08:56.7514736Z running 156 tests
2019-11-13T18:08:59.6308714Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-13T18:09:01.6348029Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-13T18:09:01.6351609Z 
2019-11-13T18:09:01.6356445Z  finished in 5.083
2019-11-13T18:09:01.6562695Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:09:01.8254900Z 
---
2019-11-13T18:09:03.7708355Z  finished in 2.114
2019-11-13T18:09:03.7892157Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:09:03.9676276Z 
2019-11-13T18:09:03.9676415Z running 9 tests
2019-11-13T18:09:03.9677332Z iiiiiiiii
2019-11-13T18:09:03.9677775Z 
2019-11-13T18:09:03.9677849Z  finished in 0.178
2019-11-13T18:09:03.9862755Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:09:04.1728395Z 
---
2019-11-13T18:09:23.5884550Z  finished in 19.601
2019-11-13T18:09:23.6155672Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:09:24.3485787Z 
2019-11-13T18:09:24.3486726Z running 123 tests
2019-11-13T18:09:48.9541909Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-13T18:09:53.8349842Z i.i.i......iii.i.....ii
2019-11-13T18:09:53.8351604Z 
2019-11-13T18:09:53.8354965Z  finished in 30.219
2019-11-13T18:09:53.8361528Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:09:53.8365295Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-13T18:21:58.8926768Z 
2019-11-13T18:21:58.8937206Z    Doc-tests core
2019-11-13T18:22:03.9566332Z 
2019-11-13T18:22:03.9567928Z running 2420 tests
2019-11-13T18:22:14.6155691Z ......iiiii......................................................................................... 100/2420
2019-11-13T18:22:24.9817687Z .................................................................................ii................. 200/2420
2019-11-13T18:22:49.3771439Z ..i................................................................................................. 400/2420
2019-11-13T18:22:49.3771439Z ..i................................................................................................. 400/2420
2019-11-13T18:22:59.4716461Z .....................................................ii.................iiii........................ 500/2420
2019-11-13T18:23:18.4580051Z .................................................................................................... 700/2420
2019-11-13T18:23:28.3718296Z .................................................................................................... 800/2420
2019-11-13T18:23:38.2658840Z .................................................................................................... 900/2420
2019-11-13T18:23:48.2003818Z .................................................................................................... 1000/2420
---
2019-11-13T18:27:53.5348937Z 
2019-11-13T18:27:53.5367911Z running 1000 tests
2019-11-13T18:28:14.0714778Z i................................................................................................... 100/1000
2019-11-13T18:28:25.3486280Z .................................................................................................... 200/1000
2019-11-13T18:28:33.2307541Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-13T18:28:38.4794568Z .................................................................................................... 400/1000
2019-11-13T18:28:45.9518321Z ...........................................i..i.................................ii.................. 500/1000
2019-11-13T18:28:59.8037556Z .................................................................................................... 700/1000
2019-11-13T18:28:59.8037556Z .................................................................................................... 700/1000
2019-11-13T18:29:07.2659786Z ..........................iiii...................................................................... 800/1000
2019-11-13T18:29:22.6663856Z .................................................................................................... 900/1000
2019-11-13T18:29:30.3895818Z ................................................iiii................................................ 1000/1000
2019-11-13T18:29:30.3899329Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-13T18:29:30.3899715Z 
2019-11-13T18:29:30.4000698Z  finished in 190.430
2019-11-13T18:29:30.4017572Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-13T18:47:32.9761712Z  finished in 41.906
2019-11-13T18:47:33.0087778Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-13T18:47:33.2218817Z 
2019-11-13T18:47:33.2219548Z running 203 tests
2019-11-13T18:48:04.1091070Z ....................i...ii...................................................................i...... 100/203
2019-11-13T18:48:38.7579367Z .................................iiii.......i.........F.iiii.iii.................................... 200/203
2019-11-13T18:48:39.2424655Z i..
2019-11-13T18:48:39.2434853Z 
2019-11-13T18:48:39.2435338Z ---- [run-make] run-make-fulldeps/rustdoc-error-lines stdout ----
2019-11-13T18:48:39.2435395Z 
2019-11-13T18:48:39.2435440Z error: make failed
2019-11-13T18:48:39.2435440Z error: make failed
2019-11-13T18:48:39.2435483Z status: exit code: 2
2019-11-13T18:48:39.2435525Z command: "make"
2019-11-13T18:48:39.2435580Z stdout:
2019-11-13T18:48:39.2435826Z ------------------------------------------
2019-11-13T18:48:39.2436670Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --test input.rs > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output || true
2019-11-13T18:48:39.2438265Z "/checkout/src/etc/cat-and-grep.sh" 'input.rs - foo (line 5)' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
2019-11-13T18:48:39.2438759Z [[[ begin stdout ]]]
2019-11-13T18:48:39.2440542Z running 3 tests
2019-11-13T18:48:39.2440809Z test input.rs - bar (line 15) ... FAILED
2019-11-13T18:48:39.2441732Z test input.rs - bar (line 22) ... FAILED
2019-11-13T18:48:39.2442077Z test input.rs - foo (line 5) ... FAILED
---
2019-11-13T18:48:39.2451669Z 
2019-11-13T18:48:39.2451722Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T18:48:39.2451756Z 
2019-11-13T18:48:39.2452795Z 
2019-11-13T18:48:39.2452883Z [[[ end stdout ]]]
2019-11-13T18:48:39.2453915Z "/checkout/src/etc/cat-and-grep.sh" 'input.rs:7:15' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
2019-11-13T18:48:39.2454025Z [[[ begin stdout ]]]
2019-11-13T18:48:39.2455024Z running 3 tests
2019-11-13T18:48:39.2455886Z test input.rs - bar (line 15) ... FAILED
2019-11-13T18:48:39.2456168Z test input.rs - bar (line 22) ... FAILED
2019-11-13T18:48:39.2457218Z test input.rs - foo (line 5) ... FAILED
---
2019-11-13T18:48:39.2464234Z 
2019-11-13T18:48:39.2464296Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T18:48:39.2464327Z 
2019-11-13T18:48:39.2465211Z 
2019-11-13T18:48:39.2465272Z [[[ end stdout ]]]
2019-11-13T18:48:39.2465862Z "/checkout/src/etc/cat-and-grep.sh" 'input.rs - bar (line 15)' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
2019-11-13T18:48:39.2465926Z [[[ begin stdout ]]]
2019-11-13T18:48:39.2466182Z running 3 tests
2019-11-13T18:48:39.2466410Z test input.rs - bar (line 15) ... FAILED
2019-11-13T18:48:39.2466634Z test input.rs - bar (line 22) ... FAILED
2019-11-13T18:48:39.2466875Z test input.rs - foo (line 5) ... FAILED
---
2019-11-13T18:48:39.2472766Z 
2019-11-13T18:48:39.2472832Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T18:48:39.2472865Z 
2019-11-13T18:48:39.2473058Z 
2019-11-13T18:48:39.2473104Z [[[ end stdout ]]]
2019-11-13T18:48:39.2473477Z "/checkout/src/etc/cat-and-grep.sh" 'input.rs:17:15' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
2019-11-13T18:48:39.2473557Z [[[ begin stdout ]]]
2019-11-13T18:48:39.2473818Z running 3 tests
2019-11-13T18:48:39.2474047Z test input.rs - bar (line 15) ... FAILED
2019-11-13T18:48:39.2474277Z test input.rs - bar (line 22) ... FAILED
2019-11-13T18:48:39.2474518Z test input.rs - foo (line 5) ... FAILED
---
2019-11-13T18:48:39.2481126Z 
2019-11-13T18:48:39.2481188Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T18:48:39.2481220Z 
2019-11-13T18:48:39.2481408Z 
2019-11-13T18:48:39.2481452Z [[[ end stdout ]]]
2019-11-13T18:48:39.2481834Z "/checkout/src/etc/cat-and-grep.sh" 'input.rs - bar (line 24)' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-error-lines/rustdoc-error-lines/output
2019-11-13T18:48:39.2481904Z [[[ begin stdout ]]]
2019-11-13T18:48:39.2482163Z running 3 tests
2019-11-13T18:48:39.2482395Z test input.rs - bar (line 15) ... FAILED
2019-11-13T18:48:39.2483508Z test input.rs - bar (line 22) ... FAILED
2019-11-13T18:48:39.2483859Z test input.rs - foo (line 5) ... FAILED
---
2019-11-13T18:48:39.2489233Z 
2019-11-13T18:48:39.2489281Z test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-13T18:48:39.2489327Z 
2019-11-13T18:48:39.2489522Z 
2019-11-13T18:48:39.2489567Z [[[ end stdout ]]]
2019-11-13T18:48:39.2489817Z Error: cannot match: input.rs - bar (line 24)
2019-11-13T18:48:39.2490072Z Makefile:7: recipe for target 'all' failed
2019-11-13T18:48:39.2490329Z ------------------------------------------
2019-11-13T18:48:39.2490389Z stderr:
2019-11-13T18:48:39.2490635Z ------------------------------------------
2019-11-13T18:48:39.2490635Z ------------------------------------------
2019-11-13T18:48:39.2490807Z make: *** [all] Error 1
2019-11-13T18:48:39.2491126Z ------------------------------------------
2019-11-13T18:48:39.2491159Z 
2019-11-13T18:48:39.2491184Z 
2019-11-13T18:48:39.2491262Z 
---
2019-11-13T18:48:39.2492228Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-13T18:48:39.2492286Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-13T18:48:39.2492319Z 
2019-11-13T18:48:39.2492358Z 
2019-11-13T18:48:39.2499147Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-13T18:48:39.2500464Z 
2019-11-13T18:48:39.2500594Z 
2019-11-13T18:48:39.2500753Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-13T18:48:39.2500913Z Build completed unsuccessfully in 1:44:57
2019-11-13T18:48:39.2500913Z Build completed unsuccessfully in 1:44:57
2019-11-13T18:48:39.2543152Z == clock drift check ==
2019-11-13T18:48:39.4584544Z   local time: Wed Nov 13 18:48:39 UTC 2019
2019-11-13T18:48:39.6994003Z   network time: Wed, 13 Nov 2019 18:48:39 GMT
2019-11-13T18:48:39.6994157Z == end clock drift check ==
2019-11-13T18:48:47.3619473Z 
2019-11-13T18:48:47.3744015Z ##[error]Bash exited with code '1'.
2019-11-13T18:48:47.3784867Z ##[section]Starting: Checkout
2019-11-13T18:48:47.3786554Z ==============================================================================
2019-11-13T18:48:47.3786615Z Task         : Get sources
2019-11-13T18:48:47.3786684Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
