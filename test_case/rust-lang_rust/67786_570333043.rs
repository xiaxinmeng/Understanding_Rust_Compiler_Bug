plain
2020-01-02T18:32:29.8909477Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-02T18:32:29.9141961Z ##[command]git config gc.auto 0
2020-01-02T18:32:29.9233407Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-02T18:32:29.9288912Z ##[command]git config --get-all http.proxy
2020-01-02T18:32:30.9288168Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67786/merge:refs/remotes/pull/67786/merge
---
2020-01-02T19:31:43.6377565Z .................................................................................................... 1500/9468
2020-01-02T19:31:49.4376585Z .................................................................................................... 1600/9468
2020-01-02T19:31:54.3635067Z .................................................................................................... 1700/9468
2020-01-02T19:32:03.6310929Z .................................................................................................... 1800/9468
2020-01-02T19:32:11.8198076Z .i.................................................................................................. 1900/9468
2020-01-02T19:32:18.4652072Z .......................................................................................iiiii........ 2000/9468
2020-01-02T19:32:40.2519798Z .................................................................................................... 2200/9468
2020-01-02T19:32:42.6702971Z .................................................................................................... 2300/9468
2020-01-02T19:32:45.0968392Z .................................................................................................... 2400/9468
2020-01-02T19:32:51.0824527Z .................................................................................................... 2500/9468
---
2020-01-02T19:35:46.4908959Z ..................i...............i................................................................. 4900/9468
2020-01-02T19:35:56.2300532Z .................................................................................................... 5000/9468
2020-01-02T19:36:01.6823486Z ...............................................................i.................................... 5100/9468
2020-01-02T19:36:09.7349658Z .................................................................................................... 5200/9468
2020-01-02T19:36:17.0828335Z ..............................ii.ii...........i..................................................... 5300/9468
2020-01-02T19:36:26.2132959Z .................................................................................................... 5500/9468
2020-01-02T19:36:36.1581210Z .................................................................................................... 5600/9468
2020-01-02T19:36:43.0939733Z .............i...................................................................................... 5700/9468
2020-01-02T19:36:49.1815056Z .................................................................................................... 5800/9468
2020-01-02T19:36:49.1815056Z .................................................................................................... 5800/9468
2020-01-02T19:36:59.8345354Z .................................................................................................... 5900/9468
2020-01-02T19:37:11.5877353Z .ii...i..ii...........i............................................................................. 6000/9468
2020-01-02T19:37:29.2328338Z .................................................................................................... 6200/9468
2020-01-02T19:37:36.4548723Z .................................................................................................... 6300/9468
2020-01-02T19:37:36.4548723Z .................................................................................................... 6300/9468
2020-01-02T19:37:53.4795239Z ............................i..ii................................................................... 6400/9468
2020-01-02T19:38:13.1193711Z .................................................................................................... 6600/9468
2020-01-02T19:38:15.1975097Z ...i................................................................................................ 6700/9468
2020-01-02T19:38:17.5587691Z .................................................................................................... 6800/9468
2020-01-02T19:38:20.0544111Z ...i................................................................................................ 6900/9468
---
2020-01-02T19:39:57.2459296Z .................................................................................................... 7500/9468
2020-01-02T19:40:02.0440322Z .................................................................................................... 7600/9468
2020-01-02T19:40:07.5795137Z .................................................................................................... 7700/9468
2020-01-02T19:40:17.5681385Z .................................................................................................... 7800/9468
2020-01-02T19:40:25.0830489Z .....................................iiii........................................................... 7900/9468
2020-01-02T19:40:39.5653420Z .................................................................................................... 8100/9468
2020-01-02T19:40:48.0503572Z .................................................................................................... 8200/9468
2020-01-02T19:41:02.1019932Z .................................................................................................... 8300/9468
2020-01-02T19:41:10.0037265Z .................................................................................................... 8400/9468
---
2020-01-02T19:43:29.1658557Z  finished in 6.560
2020-01-02T19:43:29.1851850Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:43:29.3785188Z 
2020-01-02T19:43:29.3787583Z running 166 tests
2020-01-02T19:43:32.3545299Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-02T19:43:34.3921947Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-02T19:43:34.3922905Z 
2020-01-02T19:43:34.3927377Z  finished in 5.207
2020-01-02T19:43:34.4114734Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:43:34.5786917Z 
---
2020-01-02T19:43:36.5069080Z  finished in 2.095
2020-01-02T19:43:36.5254607Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:43:36.6891997Z 
2020-01-02T19:43:36.6892852Z running 9 tests
2020-01-02T19:43:36.6893753Z iiiiiiiii
2020-01-02T19:43:36.6894983Z 
2020-01-02T19:43:36.6895175Z  finished in 0.164
2020-01-02T19:43:36.7109678Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:43:36.9123346Z 
---
2020-01-02T19:43:56.0825775Z  finished in 19.373
2020-01-02T19:43:56.1079613Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:43:56.2973307Z 
2020-01-02T19:43:56.2973545Z running 124 tests
2020-01-02T19:44:21.4495384Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-02T19:44:25.5631081Z .i.iii.....iiiiii.....ii
2020-01-02T19:44:25.5632826Z 
2020-01-02T19:44:25.5632985Z  finished in 29.455
2020-01-02T19:44:25.5640766Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T19:44:25.5641638Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-02T19:57:28.8993311Z 
2020-01-02T19:57:28.9000024Z    Doc-tests core
2020-01-02T19:57:33.4004609Z 
2020-01-02T19:57:33.4005943Z running 2440 tests
2020-01-02T19:57:42.7711459Z ......iiiii......................................................................................... 100/2440
2020-01-02T19:57:51.9124003Z ..................................................................................ii................ 200/2440
2020-01-02T19:58:13.4187958Z ................i................................................................................... 400/2440
2020-01-02T19:58:13.4187958Z ................i................................................................................... 400/2440
2020-01-02T19:58:23.2705183Z .................................................................i..i..................iiii......... 500/2440
2020-01-02T19:58:40.3122638Z .................................................................................................... 700/2440
2020-01-02T19:58:49.3465487Z .................................................................................................... 800/2440
2020-01-02T19:58:58.1375957Z .................................................................................................... 900/2440
2020-01-02T19:59:06.6665807Z .................................................................................................... 1000/2440
---
2020-01-02T20:02:43.2949147Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-01-02T20:02:43.2953808Z ... 300/760
2020-01-02T20:02:43.3729847Z .................................................................................................... 400/760
2020-01-02T20:02:45.4495239Z .................................................................................................... 500/760
2020-01-02T20:02:45.4922050Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T20:02:45.4937959Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1192:5
2020-01-02T20:02:45.4966863Z ..thread '.<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', .src/libcore/result.rs.:1192.:5
2020-01-02T20:02:45.8486710Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T20:02:45.8522251Z ............thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T20:02:45.8528770Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1192:5
2020-01-02T20:02:45.8671502Z .................. 600/760
---
2020-01-02T20:02:57.0968385Z 
2020-01-02T20:02:57.0968649Z running 1002 tests
2020-01-02T20:03:15.5779130Z i................................................................................................... 100/1002
2020-01-02T20:03:25.9797525Z .................................................................................................... 200/1002
2020-01-02T20:03:33.3550814Z .................iii......i......i...i......i....................................................... 300/1002
2020-01-02T20:03:38.4229783Z .................................................................................................... 400/1002
2020-01-02T20:03:45.6110702Z .........................................i..i.....................................ii................ 500/1002
2020-01-02T20:03:58.8513758Z .................................................................................................... 700/1002
2020-01-02T20:03:58.8513758Z .................................................................................................... 700/1002
2020-01-02T20:04:05.7673284Z ............................iiii.................................................................... 800/1002
2020-01-02T20:04:19.9972214Z .................................................................................................... 900/1002
2020-01-02T20:04:27.1362294Z ..................................................iiii.............................................. 1000/1002
2020-01-02T20:04:27.1987813Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-02T20:04:27.1987903Z 
2020-01-02T20:04:27.2121853Z  finished in 184.309
2020-01-02T20:04:27.2139359Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-02T20:24:03.6960224Z  finished in 41.198
2020-01-02T20:24:03.7402732Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-02T20:24:03.9704526Z 
2020-01-02T20:24:03.9705750Z running 206 tests
2020-01-02T20:24:30.9406272Z ....................i...ii..................................F..........F......................i..... 100/206
2020-01-02T20:25:05.7014791Z ..................................iiiiii......i............iiii.iii................................. 200/206
2020-01-02T20:25:06.8959395Z failures:
2020-01-02T20:25:06.8969521Z 
2020-01-02T20:25:06.8970075Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-02T20:25:06.8970116Z 
2020-01-02T20:25:06.8970116Z 
2020-01-02T20:25:06.8970194Z error: make failed
2020-01-02T20:25:06.8970234Z status: exit code: 2
2020-01-02T20:25:06.8970271Z command: "make" "make"
2020-01-02T20:25:06.8970328Z stdout:
2020-01-02T20:25:06.8970554Z ------------------------------------------
2020-01-02T20:25:06.8970602Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-02T20:25:06.8970631Z 
2020-01-02T20:25:06.8971924Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-02T20:25:06.8972442Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-02T20:25:06.8972652Z Makefile:4: recipe for target 'all' failed
2020-01-02T20:25:06.8972891Z ------------------------------------------
2020-01-02T20:25:06.8972932Z stderr:
2020-01-02T20:25:06.8973119Z ------------------------------------------
2020-01-02T20:25:06.8973163Z error[E0432]: unresolved import `syntax::symbol`
2020-01-02T20:25:06.8973163Z error[E0432]: unresolved import `syntax::symbol`
2020-01-02T20:25:06.8973364Z   --> the_backend.rs:14:13
2020-01-02T20:25:06.8973414Z    |
2020-01-02T20:25:06.8973452Z 14 | use syntax::symbol::Symbol;
2020-01-02T20:25:06.8973523Z    |             ^^^^^^ could not find `symbol` in `syntax`
2020-01-02T20:25:06.8973550Z 
2020-01-02T20:25:06.8973745Z warning: ignoring --out-dir flag due to -o flag
2020-01-02T20:25:06.8973830Z error: aborting due to previous error
2020-01-02T20:25:06.8973856Z 
2020-01-02T20:25:06.8974069Z For more information about this error, try `rustc --explain E0432`.
2020-01-02T20:25:06.8974069Z For more information about this error, try `rustc --explain E0432`.
2020-01-02T20:25:06.8974111Z make: *** [all] Error 1
2020-01-02T20:25:06.8974340Z ------------------------------------------
2020-01-02T20:25:06.8974368Z 
2020-01-02T20:25:06.8974390Z 
2020-01-02T20:25:06.8974610Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2020-01-02T20:25:06.8974610Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2020-01-02T20:25:06.8974638Z 
2020-01-02T20:25:06.8974675Z error: make failed
2020-01-02T20:25:06.8974712Z status: exit code: 2
2020-01-02T20:25:06.8974768Z command: "make" "make"
2020-01-02T20:25:06.8974803Z stdout:
2020-01-02T20:25:06.8975004Z ------------------------------------------
2020-01-02T20:25:06.8975724Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2020-01-02T20:25:06.8975960Z Makefile:8: recipe for target 'all' failed
2020-01-02T20:25:06.8976200Z ------------------------------------------
2020-01-02T20:25:06.8976237Z stderr:
2020-01-02T20:25:06.8976440Z ------------------------------------------
2020-01-02T20:25:06.8976483Z error[E0432]: unresolved import `syntax::source_map`
---
2020-01-02T20:25:06.8976838Z 
2020-01-02T20:25:06.8976897Z error: aborting due to previous error
2020-01-02T20:25:06.8976922Z 
2020-01-02T20:25:06.8977137Z For more information about this error, try `rustc --explain E0432`.
2020-01-02T20:25:06.8977180Z make: *** [all] Error 1
2020-01-02T20:25:06.8977408Z ------------------------------------------
2020-01-02T20:25:06.8977434Z 
2020-01-02T20:25:06.8977457Z 
2020-01-02T20:25:06.8977479Z 
---
2020-01-02T20:25:06.8986140Z test result: FAILED. 185 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out
2020-01-02T20:25:06.8986182Z 
2020-01-02T20:25:06.8986208Z 
2020-01-02T20:25:06.8986233Z 
2020-01-02T20:25:06.8999000Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-02T20:25:06.9044877Z 
2020-01-02T20:25:06.9044914Z 
2020-01-02T20:25:06.9049088Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-02T20:25:06.9049151Z Build completed unsuccessfully in 1:46:09
2020-01-02T20:25:06.9049151Z Build completed unsuccessfully in 1:46:09
2020-01-02T20:25:06.9061897Z == clock drift check ==
2020-01-02T20:25:06.9080087Z   local time: Thu Jan  2 20:25:06 UTC 2020
2020-01-02T20:25:07.1997109Z   network time: Thu, 02 Jan 2020 20:25:07 GMT
2020-01-02T20:25:07.1997746Z == end clock drift check ==
2020-01-02T20:25:13.2396253Z 
2020-01-02T20:25:13.2481729Z ##[error]Bash exited with code '1'.
2020-01-02T20:25:13.2540187Z ##[section]Starting: Checkout
2020-01-02T20:25:13.2541890Z ==============================================================================
2020-01-02T20:25:13.2541940Z Task         : Get sources
2020-01-02T20:25:13.2542004Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
