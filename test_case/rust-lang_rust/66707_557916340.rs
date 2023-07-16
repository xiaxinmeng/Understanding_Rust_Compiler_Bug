plain
2019-11-24T17:02:23.8354337Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T17:02:23.8604758Z ##[command]git config gc.auto 0
2019-11-24T17:02:23.8657379Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T17:02:23.8708866Z ##[command]git config --get-all http.proxy
2019-11-24T17:02:23.8851577Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66707/merge:refs/remotes/pull/66707/merge
---
2019-11-24T17:58:25.3831239Z .................................................................................................... 1600/9282
2019-11-24T17:58:30.0425807Z .................................................................................................... 1700/9282
2019-11-24T17:58:42.8056495Z .............................i...................................................................... 1800/9282
2019-11-24T17:58:49.3288798Z .................................................................................................... 1900/9282
2019-11-24T17:59:02.7373593Z ..............iiiii................................................................................. 2000/9282
2019-11-24T17:59:12.1760942Z .................................................................................................... 2200/9282
2019-11-24T17:59:14.7455867Z .................................................................................................... 2300/9282
2019-11-24T17:59:20.0683392Z .................................................................................................... 2400/9282
2019-11-24T17:59:39.9755884Z .................................................................................................... 2500/9282
---
2019-11-24T18:02:16.2473727Z ..............i...............i..................................................................... 4800/9282
2019-11-24T18:02:26.1053759Z .................................................................................................... 4900/9282
2019-11-24T18:02:31.7149221Z .................................................................................................... 5000/9282
2019-11-24T18:02:41.1632854Z .................................................................................................... 5100/9282
2019-11-24T18:02:47.1795811Z ...................ii.ii...........i................................................................ 5200/9282
2019-11-24T18:02:56.3751922Z .................................................................................................... 5400/9282
2019-11-24T18:03:06.8875253Z .................................................................................................... 5500/9282
2019-11-24T18:03:14.5460777Z .i.................................................................................................. 5600/9282
2019-11-24T18:03:19.8266131Z .................................................................................................... 5700/9282
2019-11-24T18:03:19.8266131Z .................................................................................................... 5700/9282
2019-11-24T18:03:29.9777918Z .......................................................................................ii...i..ii... 5800/9282
2019-11-24T18:03:51.6556126Z .................................................................................................... 6000/9282
2019-11-24T18:03:59.3764596Z .................................................................................................... 6100/9282
2019-11-24T18:04:05.0747737Z .................................................................................................... 6200/9282
2019-11-24T18:04:05.0747737Z .................................................................................................... 6200/9282
2019-11-24T18:04:18.1994905Z ..........i..ii..................................................................................... 6300/9282
2019-11-24T18:04:36.0140201Z ..............................................................................i..................... 6500/9282
2019-11-24T18:04:38.3140685Z .................................................................................................... 6600/9282
2019-11-24T18:04:40.4871450Z .....................................................................i.............................. 6700/9282
2019-11-24T18:04:43.3083200Z .................................................................................................... 6800/9282
---
2019-11-24T18:09:46.3779538Z  finished in 5.627
2019-11-24T18:09:46.3944612Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:09:46.5619150Z 
2019-11-24T18:09:46.5619334Z running 157 tests
2019-11-24T18:09:49.2970829Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/157
2019-11-24T18:09:51.1050289Z .i.i..iiii..............i.........iii.i..........ii......
2019-11-24T18:09:51.1051672Z 
2019-11-24T18:09:51.1055992Z  finished in 4.711
2019-11-24T18:09:51.1234048Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:09:51.2744239Z 
---
2019-11-24T18:09:53.1972426Z  finished in 2.073
2019-11-24T18:09:53.2144731Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:09:53.3604977Z 
2019-11-24T18:09:53.3605257Z running 9 tests
2019-11-24T18:09:53.3606604Z iiiiiiiii
2019-11-24T18:09:53.3607300Z 
2019-11-24T18:09:53.3608625Z  finished in 0.146
2019-11-24T18:09:53.3802863Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:09:54.3932480Z 
---
2019-11-24T18:10:12.4912034Z  finished in 19.111
2019-11-24T18:10:12.6096819Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:10:12.7784033Z 
2019-11-24T18:10:12.7784691Z running 124 tests
2019-11-24T18:10:37.2099220Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-11-24T18:10:42.2955944Z .i.i.i......iii.i.....ii
2019-11-24T18:10:42.2957370Z 
2019-11-24T18:10:42.2960815Z  finished in 29.686
2019-11-24T18:10:42.2973171Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:10:42.2973637Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-24T18:22:11.2080961Z 
2019-11-24T18:22:11.2088131Z    Doc-tests core
2019-11-24T18:22:15.8304280Z 
2019-11-24T18:22:15.8305182Z running 2421 tests
2019-11-24T18:22:25.7885240Z ......iiiii......................................................................................... 100/2421
2019-11-24T18:22:46.4927238Z .................................................................................................... 300/2421
2019-11-24T18:22:58.1233056Z ..i................................................................................................. 400/2421
2019-11-24T18:22:58.1233056Z ..i................................................................................................. 400/2421
2019-11-24T18:23:07.8039903Z ..................................................i..i..................iiii........................ 500/2421
2019-11-24T18:23:26.1805672Z .................................................................................................... 700/2421
2019-11-24T18:23:35.6392450Z .................................................................................................... 800/2421
2019-11-24T18:23:45.0034033Z .................................................................................................... 900/2421
2019-11-24T18:23:55.1222754Z .................................................................................................... 1000/2421
---
2019-11-24T18:27:54.1638508Z 
2019-11-24T18:27:54.1639442Z running 999 tests
2019-11-24T18:28:12.2392744Z i................................................................................................... 100/999
2019-11-24T18:28:22.7408141Z .................................................................................................... 200/999
2019-11-24T18:28:30.0638781Z ..................iii......i......i...i......i...................................................... 300/999
2019-11-24T18:28:35.0679013Z .................................................................................................... 400/999
2019-11-24T18:28:42.0947869Z ..........................................i..i.................................ii................... 500/999
2019-11-24T18:28:55.2697285Z .................................................................................................... 700/999
2019-11-24T18:28:55.2697285Z .................................................................................................... 700/999
2019-11-24T18:29:02.1515397Z .........................iiii....................................................................... 800/999
2019-11-24T18:29:16.0911468Z .................................................................................................... 900/999
2019-11-24T18:29:22.7227604Z ...............................................iiii................................................
2019-11-24T18:29:22.7229563Z 
2019-11-24T18:29:22.7276356Z  finished in 176.694
2019-11-24T18:29:22.7289020Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:29:22.9213171Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-24T18:46:18.4477490Z  finished in 39.736
2019-11-24T18:46:18.4843298Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T18:46:19.1670815Z 
2019-11-24T18:46:19.1670942Z running 203 tests
2019-11-24T18:46:46.2160309Z ....................i...ii............................................F......................i...... 100/203
2019-11-24T18:47:21.6825205Z .................................iiii......i............iiii.iii.................................... 200/203
2019-11-24T18:47:22.0478470Z i..
2019-11-24T18:47:22.0486065Z 
2019-11-24T18:47:22.0486664Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2019-11-24T18:47:22.0487406Z 
2019-11-24T18:47:22.0487683Z error: make failed
2019-11-24T18:47:22.0487683Z error: make failed
2019-11-24T18:47:22.0487760Z status: exit code: 2
2019-11-24T18:47:22.0487849Z command: "make"
2019-11-24T18:47:22.0487921Z stdout:
2019-11-24T18:47:22.0488235Z ------------------------------------------
2019-11-24T18:47:22.0489653Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2019-11-24T18:47:22.0490277Z Makefile:8: recipe for target 'all' failed
2019-11-24T18:47:22.0490852Z ------------------------------------------
2019-11-24T18:47:22.0491065Z stderr:
2019-11-24T18:47:22.0491599Z ------------------------------------------
2019-11-24T18:47:22.0491823Z error[E0599]: no method named `link` found for type `&rustc_interface::interface::Compiler` in the current scope
2019-11-24T18:47:22.0491823Z error[E0599]: no method named `link` found for type `&rustc_interface::interface::Compiler` in the current scope
2019-11-24T18:47:22.0492159Z   --> foo.rs:69:18
2019-11-24T18:47:22.0492216Z    |
2019-11-24T18:47:22.0492415Z 69 |         compiler.link().ok();
2019-11-24T18:47:22.0492600Z 
2019-11-24T18:47:22.0492671Z error: aborting due to previous error
2019-11-24T18:47:22.0492704Z 
2019-11-24T18:47:22.0493086Z For more information about this error, try `rustc --explain E0599`.
2019-11-24T18:47:22.0493086Z For more information about this error, try `rustc --explain E0599`.
2019-11-24T18:47:22.0493289Z make: *** [all] Error 1
2019-11-24T18:47:22.0493686Z ------------------------------------------
2019-11-24T18:47:22.0493723Z 
2019-11-24T18:47:22.0493884Z 
2019-11-24T18:47:22.0493945Z 
---
2019-11-24T18:47:22.0495559Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-24T18:47:22.0495662Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-24T18:47:22.0507561Z 
2019-11-24T18:47:22.0507649Z 
2019-11-24T18:47:22.0513209Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-24T18:47:22.0514584Z 
2019-11-24T18:47:22.0514623Z 
2019-11-24T18:47:22.0519087Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-24T18:47:22.0519405Z Build completed unsuccessfully in 1:38:53
2019-11-24T18:47:22.0519405Z Build completed unsuccessfully in 1:38:53
2019-11-24T18:47:22.0578580Z == clock drift check ==
2019-11-24T18:47:22.0605354Z   local time: Sun Nov 24 18:47:22 UTC 2019
2019-11-24T18:47:22.6815089Z   network time: Sun, 24 Nov 2019 18:47:22 GMT
2019-11-24T18:47:22.6817351Z == end clock drift check ==
2019-11-24T18:47:25.8087988Z 
2019-11-24T18:47:25.8222373Z ##[error]Bash exited with code '1'.
2019-11-24T18:47:25.8277005Z ##[section]Starting: Checkout
2019-11-24T18:47:25.8278918Z ==============================================================================
2019-11-24T18:47:25.8278980Z Task         : Get sources
2019-11-24T18:47:25.8279034Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
