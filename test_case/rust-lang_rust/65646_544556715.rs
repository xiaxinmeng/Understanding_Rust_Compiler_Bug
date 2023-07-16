plain
2019-10-21T13:04:43.2845546Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T13:04:43.3051688Z ##[command]git config gc.auto 0
2019-10-21T13:04:43.3123209Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T13:04:43.3183328Z ##[command]git config --get-all http.proxy
2019-10-21T13:04:43.3334657Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65646/merge:refs/remotes/pull/65646/merge
---
2019-10-21T14:09:39.5400699Z .................................................................................................... 1600/9203
2019-10-21T14:09:45.2728321Z .................................................................................................... 1700/9203
2019-10-21T14:09:58.8159245Z ................................i...............i................................................... 1800/9203
2019-10-21T14:10:06.6433508Z .................................................................................................... 1900/9203
2019-10-21T14:10:21.4055545Z ......................iiiii......................................................................... 2000/9203
2019-10-21T14:10:32.5697213Z .................................................................................................... 2200/9203
2019-10-21T14:10:35.2341630Z .................................................................................................... 2300/9203
2019-10-21T14:10:40.6209507Z .................................................................................................... 2400/9203
2019-10-21T14:11:03.6339514Z .................................................................................................... 2500/9203
---
2019-10-21T14:14:07.4597112Z .........................i...............i.......................................................... 4800/9203
2019-10-21T14:14:20.0086324Z .................................................................................................... 4900/9203
2019-10-21T14:14:26.6666822Z .................................................................................................... 5000/9203
2019-10-21T14:14:36.4961803Z .................................................................................................... 5100/9203
2019-10-21T14:14:44.5140913Z .........................ii.ii...................................................................... 5200/9203
2019-10-21T14:14:55.0098072Z .................................................................................................... 5400/9203
2019-10-21T14:15:05.8561671Z ...........................................................................................i........ 5500/9203
2019-10-21T14:15:14.5879551Z .................................................................................................... 5600/9203
2019-10-21T14:15:19.6150485Z .................................................................................................... 5700/9203
2019-10-21T14:15:19.6150485Z .................................................................................................... 5700/9203
2019-10-21T14:15:30.9331821Z ........................................................................................ii...i..ii.. 5800/9203
2019-10-21T14:15:58.5690768Z .................................................................................................... 6000/9203
2019-10-21T14:16:08.2913187Z .................................................................................................... 6100/9203
2019-10-21T14:16:17.7607892Z .................................................................................................... 6200/9203
2019-10-21T14:16:17.7607892Z .................................................................................................... 6200/9203
2019-10-21T14:16:33.5166184Z ..........i..ii..................................................................................... 6300/9203
2019-10-21T14:16:54.3487547Z ......................................................................i............................. 6500/9203
2019-10-21T14:16:56.5677565Z .................................................................................................... 6600/9203
2019-10-21T14:16:59.1348023Z .............................................i...................................................... 6700/9203
2019-10-21T14:17:02.8614344Z .................................................................................................... 6800/9203
---
2019-10-21T14:21:47.6347838Z  finished in 5.965
2019-10-21T14:21:47.6531219Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:21:47.8432768Z 
2019-10-21T14:21:47.8432947Z running 153 tests
2019-10-21T14:21:51.1484179Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-21T14:21:53.1731604Z i..iiii..............i.........iii.i.........ii......
2019-10-21T14:21:53.1733762Z 
2019-10-21T14:21:53.1737886Z  finished in 5.520
2019-10-21T14:21:53.1933125Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:21:53.3707327Z 
---
2019-10-21T14:21:55.5965121Z  finished in 2.403
2019-10-21T14:21:55.6201947Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:21:55.7894961Z 
2019-10-21T14:21:55.7895260Z running 9 tests
2019-10-21T14:21:55.7896095Z iiiiiiiii
2019-10-21T14:21:55.7896485Z 
2019-10-21T14:21:55.7896533Z  finished in 0.171
2019-10-21T14:21:55.8095295Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:21:55.9964749Z 
---
2019-10-21T14:22:14.7611584Z  finished in 18.951
2019-10-21T14:22:14.7839097Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:22:14.9724088Z 
2019-10-21T14:22:14.9725385Z running 123 tests
2019-10-21T14:22:40.6979926Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-21T14:22:45.8155342Z i.i.i......iii.i.....ii
2019-10-21T14:22:45.8157364Z 
2019-10-21T14:22:45.8164108Z  finished in 31.032
2019-10-21T14:22:45.8178312Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:22:45.8180058Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-21T14:36:00.2931432Z 
2019-10-21T14:36:00.2932591Z    Doc-tests core
2019-10-21T14:36:05.5241924Z 
2019-10-21T14:36:05.5242356Z running 2406 tests
2019-10-21T14:36:17.1927697Z ......iiiii......................................................................................... 100/2406
2019-10-21T14:36:28.7215256Z ................................................................................ii.................. 200/2406
2019-10-21T14:36:55.8840663Z ..i................................................................................................. 400/2406
2019-10-21T14:36:55.8840663Z ..i................................................................................................. 400/2406
2019-10-21T14:37:07.0907479Z .................................................i..i.................iiii.......................... 500/2406
2019-10-21T14:37:28.9072811Z .................................................................................................... 700/2406
2019-10-21T14:37:39.9555280Z .................................................................................................... 800/2406
2019-10-21T14:37:50.9600278Z .................................................................................................... 900/2406
2019-10-21T14:38:02.0497955Z .................................................................................................... 1000/2406
---
2019-10-21T14:42:29.3512881Z 
2019-10-21T14:42:29.3514150Z running 994 tests
2019-10-21T14:42:52.1871656Z i................................................................................................... 100/994
2019-10-21T14:43:04.5441025Z .................................................................................................... 200/994
2019-10-21T14:43:13.4393330Z ...................iii......i......i...i......i..................................................... 300/994
2019-10-21T14:43:19.4721454Z .................................................................................................... 400/994
2019-10-21T14:43:27.7626430Z .....................................i..i.................................ii........................ 500/994
2019-10-21T14:43:43.5039153Z .................................................................................................... 700/994
2019-10-21T14:43:43.5039153Z .................................................................................................... 700/994
2019-10-21T14:43:52.2107916Z ....................iiii............................................................................ 800/994
2019-10-21T14:44:08.0779302Z .................................................................................................... 900/994
2019-10-21T14:44:16.2328295Z ..........................................iiii................................................
2019-10-21T14:44:16.2331004Z 
2019-10-21T14:44:16.2373443Z  finished in 205.832
2019-10-21T14:44:16.2392937Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T14:44:16.4608521Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-21T15:01:03.5248268Z  finished in 43.607
2019-10-21T15:01:03.5592141Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-21T15:01:03.7498504Z 
2019-10-21T15:01:03.7499986Z running 202 tests
2019-10-21T15:01:40.6585891Z ....................i...ii............................F.....................................i....... 100/202
2019-10-21T15:02:32.7275122Z ................................iiii.......i...........iiii.iii....................................i 200/202
2019-10-21T15:02:33.0721327Z failures:
2019-10-21T15:02:33.0759539Z 
2019-10-21T15:02:33.0763443Z ---- [run-make] run-make-fulldeps/foreign-exceptions stdout ----
2019-10-21T15:02:33.0763548Z 
2019-10-21T15:02:33.0763548Z 
2019-10-21T15:02:33.0763637Z error: make failed
2019-10-21T15:02:33.0768977Z status: exit code: 2
2019-10-21T15:02:33.0769055Z command: "make"
2019-10-21T15:02:33.0769190Z stdout:
2019-10-21T15:02:33.0770523Z ------------------------------------------
2019-10-21T15:02:33.0771004Z c++ -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o foo.cpp
2019-10-21T15:02:33.0776255Z Makefile:10: recipe for target '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o' failed
2019-10-21T15:02:33.0776641Z ------------------------------------------
2019-10-21T15:02:33.0776695Z stderr:
2019-10-21T15:02:33.0776911Z ------------------------------------------
2019-10-21T15:02:33.0776911Z ------------------------------------------
2019-10-21T15:02:33.0777586Z foo.cpp:6:16: warning: non-static data member initializers only available with -std=c++11 or -std=gnu++11
2019-10-21T15:02:33.0777690Z      bool* ok = nullptr;
2019-10-21T15:02:33.0777737Z                 ^
2019-10-21T15:02:33.0778141Z foo.cpp:6:16: error: 'nullptr' was not declared in this scope
2019-10-21T15:02:33.0778483Z make: *** [/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/foreign-exceptions/foreign-exceptions/libfoo.o] Error 1
2019-10-21T15:02:33.0778751Z ------------------------------------------
2019-10-21T15:02:33.0778786Z 
2019-10-21T15:02:33.0778831Z 
2019-10-21T15:02:33.0778995Z 
---
2019-10-21T15:02:33.0782209Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-21T15:02:33.0782323Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-21T15:02:33.0803527Z 
2019-10-21T15:02:33.0803620Z 
2019-10-21T15:02:33.0809318Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-21T15:02:33.0810361Z 
2019-10-21T15:02:33.0810396Z 
2019-10-21T15:02:33.0810456Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-21T15:02:33.0810530Z Build completed unsuccessfully in 1:50:57
2019-10-21T15:02:33.0810530Z Build completed unsuccessfully in 1:50:57
2019-10-21T15:02:33.0840875Z == clock drift check ==
2019-10-21T15:02:33.0859667Z   local time: Mon Oct 21 15:02:33 UTC 2019
2019-10-21T15:02:33.1436769Z   network time: Mon, 21 Oct 2019 15:02:33 GMT
2019-10-21T15:02:33.1439611Z == end clock drift check ==
2019-10-21T15:02:38.7241044Z 
2019-10-21T15:02:38.7353265Z ##[error]Bash exited with code '1'.
2019-10-21T15:02:38.7413518Z ##[section]Starting: Checkout
2019-10-21T15:02:38.7415407Z ==============================================================================
2019-10-21T15:02:38.7415488Z Task         : Get sources
2019-10-21T15:02:38.7415542Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
