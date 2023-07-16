plain
2019-10-21T02:06:59.2952533Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T02:06:59.3153690Z ##[command]git config gc.auto 0
2019-10-21T02:06:59.3239282Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T02:06:59.3295561Z ##[command]git config --get-all http.proxy
2019-10-21T02:06:59.3434999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65646/merge:refs/remotes/pull/65646/merge
---
2019-10-21T03:11:47.8805798Z .................................................................................................... 1600/9203
2019-10-21T03:11:53.3804577Z .................................................................................................... 1700/9203
2019-10-21T03:12:06.8236866Z ................................i...............i................................................... 1800/9203
2019-10-21T03:12:14.6484032Z .................................................................................................... 1900/9203
2019-10-21T03:12:29.3956931Z ......................iiiii......................................................................... 2000/9203
2019-10-21T03:12:40.4175560Z .................................................................................................... 2200/9203
2019-10-21T03:12:43.1227602Z .................................................................................................... 2300/9203
2019-10-21T03:12:48.4511658Z .................................................................................................... 2400/9203
2019-10-21T03:13:11.6795647Z .................................................................................................... 2500/9203
---
2019-10-21T03:16:13.3154598Z .........................i...............i.......................................................... 4800/9203
2019-10-21T03:16:26.1185024Z .................................................................................................... 4900/9203
2019-10-21T03:16:32.7373337Z .................................................................................................... 5000/9203
2019-10-21T03:16:42.6297719Z .................................................................................................... 5100/9203
2019-10-21T03:16:50.3816650Z .........................ii.ii...................................................................... 5200/9203
2019-10-21T03:17:00.9372135Z .................................................................................................... 5400/9203
2019-10-21T03:17:11.7751115Z ...........................................................................................i........ 5500/9203
2019-10-21T03:17:20.2922580Z .................................................................................................... 5600/9203
2019-10-21T03:17:25.3780642Z .................................................................................................... 5700/9203
2019-10-21T03:17:25.3780642Z .................................................................................................... 5700/9203
2019-10-21T03:17:36.5087520Z ........................................................................................ii...i..ii.. 5800/9203
2019-10-21T03:18:04.0082965Z .................................................................................................... 6000/9203
2019-10-21T03:18:13.6486558Z .................................................................................................... 6100/9203
2019-10-21T03:18:20.4833514Z .................................................................................................... 6200/9203
2019-10-21T03:18:20.4833514Z .................................................................................................... 6200/9203
2019-10-21T03:18:35.0330796Z ..........i..ii..................................................................................... 6300/9203
2019-10-21T03:18:55.7234890Z ......................................................................i............................. 6500/9203
2019-10-21T03:18:58.0012349Z .................................................................................................... 6600/9203
2019-10-21T03:19:00.5352260Z .............................................i...................................................... 6700/9203
2019-10-21T03:19:04.2687111Z .................................................................................................... 6800/9203
---
2019-10-21T03:23:47.5708693Z  finished in 5.784
2019-10-21T03:23:47.5936993Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:23:47.7672772Z 
2019-10-21T03:23:47.7673105Z running 153 tests
2019-10-21T03:23:51.0524904Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-21T03:23:53.1329109Z i..iiii..............i.........iii.i.........ii......
2019-10-21T03:23:53.1334659Z 
2019-10-21T03:23:53.1340601Z  finished in 5.540
2019-10-21T03:23:53.1539439Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:23:53.3137710Z 
---
2019-10-21T03:23:55.4579815Z  finished in 2.303
2019-10-21T03:23:55.4797334Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:23:55.6490571Z 
2019-10-21T03:23:55.6491414Z running 9 tests
2019-10-21T03:23:55.6500772Z iiiiiiiii
2019-10-21T03:23:55.6502466Z 
2019-10-21T03:23:55.6503417Z  finished in 0.171
2019-10-21T03:23:55.6724678Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:23:55.8428345Z 
---
2019-10-21T03:24:14.3528701Z  finished in 18.680
2019-10-21T03:24:14.3754553Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:24:14.5472437Z 
2019-10-21T03:24:14.5473451Z running 123 tests
2019-10-21T03:24:39.2170676Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-21T03:24:44.0015831Z i.i.i......iii.i.....ii
2019-10-21T03:24:44.0018619Z 
2019-10-21T03:24:44.0022756Z  finished in 29.626
2019-10-21T03:24:44.0034422Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:24:44.0034965Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-21T03:38:01.7207366Z 
2019-10-21T03:38:01.7209547Z    Doc-tests core
2019-10-21T03:38:07.0535172Z 
2019-10-21T03:38:07.0536043Z running 2406 tests
2019-10-21T03:38:18.5524418Z ......iiiii......................................................................................... 100/2406
2019-10-21T03:38:29.6716161Z ................................................................................ii.................. 200/2406
2019-10-21T03:38:55.9268234Z ..i................................................................................................. 400/2406
2019-10-21T03:38:55.9268234Z ..i................................................................................................. 400/2406
2019-10-21T03:39:06.8448525Z .................................................i..i.................iiii.......................... 500/2406
2019-10-21T03:39:28.5065038Z .................................................................................................... 700/2406
2019-10-21T03:39:39.2278640Z .................................................................................................... 800/2406
2019-10-21T03:39:50.0314470Z .................................................................................................... 900/2406
2019-10-21T03:40:00.7744277Z .................................................................................................... 1000/2406
---
2019-10-21T03:44:25.5682766Z 
2019-10-21T03:44:25.5683048Z running 994 tests
2019-10-21T03:44:47.3650329Z i................................................................................................... 100/994
2019-10-21T03:44:59.6149551Z .................................................................................................... 200/994
2019-10-21T03:45:08.2844196Z ...................iii......i......i...i......i..................................................... 300/994
2019-10-21T03:45:14.1670354Z .................................................................................................... 400/994
2019-10-21T03:45:22.1517858Z .....................................i..i.................................ii........................ 500/994
2019-10-21T03:45:37.6466938Z .................................................................................................... 700/994
2019-10-21T03:45:37.6466938Z .................................................................................................... 700/994
2019-10-21T03:45:46.0464065Z ....................iiii............................................................................ 800/994
2019-10-21T03:46:01.6440095Z .................................................................................................... 900/994
2019-10-21T03:46:09.5723136Z ..........................................iiii................................................
2019-10-21T03:46:09.5725367Z 
2019-10-21T03:46:09.5844763Z  finished in 205.226
2019-10-21T03:46:09.5861650Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T03:46:09.8169652Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-21T04:02:55.3894252Z  finished in 41.987
2019-10-21T04:02:55.4276329Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T04:02:55.5917037Z 
2019-10-21T04:02:55.5917281Z running 202 tests
2019-10-21T04:03:32.0876461Z ....................i...ii...........................F......................................i....... 100/202
2019-10-21T04:04:24.1046321Z ................................iiii.......i...........iiii.iii....................................i 200/202
2019-10-21T04:04:24.4294056Z failures:
2019-10-21T04:04:24.4303683Z 
2019-10-21T04:04:24.4305349Z ---- [run-make] run-make-fulldeps/foreign-exceptions stdout ----
2019-10-21T04:04:24.4306170Z 
2019-10-21T04:04:24.4306170Z 
2019-10-21T04:04:24.4306256Z error: make failed
2019-10-21T04:04:24.4306320Z status: exit code: 2
2019-10-21T04:04:24.4306357Z command: "make"
2019-10-21T04:04:24.4306410Z stdout:
2019-10-21T04:04:24.4306700Z ------------------------------------------
2019-10-21T04:04:24.4307033Z c++ -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o foo.cpp
2019-10-21T04:04:24.4307436Z ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o
2019-10-21T04:04:24.4315226Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions  foo.rs -lfoo -lstdc++
2019-10-21T04:04:24.4315783Z Makefile:7: recipe for target 'foo' failed
2019-10-21T04:04:24.4316063Z ------------------------------------------
2019-10-21T04:04:24.4316109Z stderr:
2019-10-21T04:04:24.4316300Z ------------------------------------------
2019-10-21T04:04:24.4316300Z ------------------------------------------
2019-10-21T04:04:24.4316732Z ar: `u' modifier ignored since `D' is the default (see `U')
2019-10-21T04:04:24.4316805Z error[E0658]: the `#[unwind]` attribute is an experimental feature
2019-10-21T04:04:24.4317102Z    |
2019-10-21T04:04:24.4317143Z 15 |     #[unwind(allowed)]
2019-10-21T04:04:24.4317182Z    |     ^^^^^^^^^^^^^^^^^^
2019-10-21T04:04:24.4317348Z    |
2019-10-21T04:04:24.4317348Z    |
2019-10-21T04:04:24.4317769Z    = note: for more information, see ***/issues/58760
2019-10-21T04:04:24.4317825Z    = help: add `#![feature(unwind_attributes)]` to the crate attributes to enable
2019-10-21T04:04:24.4317858Z 
2019-10-21T04:04:24.4317921Z error[E0658]: the `#[unwind]` attribute is an experimental feature
2019-10-21T04:04:24.4318498Z    |
2019-10-21T04:04:24.4318555Z 18 |     #[unwind(allowed)]
2019-10-21T04:04:24.4318593Z    |     ^^^^^^^^^^^^^^^^^^
2019-10-21T04:04:24.4318629Z    |
2019-10-21T04:04:24.4318629Z    |
2019-10-21T04:04:24.4318916Z    = note: for more information, see ***/issues/58760
2019-10-21T04:04:24.4318982Z    = help: add `#![feature(unwind_attributes)]` to the crate attributes to enable
2019-10-21T04:04:24.4319069Z error: aborting due to 2 previous errors
2019-10-21T04:04:24.4319096Z 
2019-10-21T04:04:24.4319322Z For more information about this error, try `rustc --explain E0658`.
2019-10-21T04:04:24.4319322Z For more information about this error, try `rustc --explain E0658`.
2019-10-21T04:04:24.4319367Z make: *** [foo] Error 1
2019-10-21T04:04:24.4319609Z ------------------------------------------
2019-10-21T04:04:24.4319638Z 
2019-10-21T04:04:24.4319661Z 
2019-10-21T04:04:24.4319683Z 
---
2019-10-21T04:04:24.4321210Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-21T04:04:24.4321284Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-21T04:04:24.4321335Z 
2019-10-21T04:04:24.4321361Z 
2019-10-21T04:04:24.4325623Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-21T04:04:24.4326467Z 
2019-10-21T04:04:24.4326495Z 
2019-10-21T04:04:24.4362199Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-21T04:04:24.4362281Z Build completed unsuccessfully in 1:50:41
2019-10-21T04:04:24.4362281Z Build completed unsuccessfully in 1:50:41
2019-10-21T04:04:24.4392560Z == clock drift check ==
2019-10-21T04:04:24.4416648Z   local time: Mon Oct 21 04:04:24 UTC 2019
2019-10-21T04:04:24.7074150Z   network time: Mon, 21 Oct 2019 04:04:24 GMT
2019-10-21T04:04:24.7076416Z == end clock drift check ==
2019-10-21T04:04:31.7533944Z 
2019-10-21T04:04:31.7702996Z ##[error]Bash exited with code '1'.
2019-10-21T04:04:31.7762650Z ##[section]Starting: Checkout
2019-10-21T04:04:31.7765861Z ==============================================================================
2019-10-21T04:04:31.7765914Z Task         : Get sources
2019-10-21T04:04:31.7765984Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
