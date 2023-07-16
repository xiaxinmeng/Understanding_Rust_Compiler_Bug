plain
2020-03-15T17:22:24.2650395Z ========================== Starting Command Output ===========================
2020-03-15T17:22:24.2654138Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f9fa2559-c06d-4301-9620-513297732410.sh
2020-03-15T17:22:24.2654447Z 
2020-03-15T17:22:24.2659999Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-15T17:22:24.2681012Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70028/merge to s
2020-03-15T17:22:24.2684642Z Task         : Get sources
2020-03-15T17:22:24.2684968Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T17:22:24.2685350Z Version      : 1.0.0
2020-03-15T17:22:24.2685588Z Author       : Microsoft
---
2020-03-15T17:22:27.1344690Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-15T17:22:27.1361474Z ##[command]git config gc.auto 0
2020-03-15T17:22:27.1367223Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-15T17:22:27.1373927Z ##[command]git config --get-all http.proxy
2020-03-15T17:22:27.1385204Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70028/merge:refs/remotes/pull/70028/merge
---
2020-03-15T18:27:08.5058196Z .................................................................................................... 1700/9771
2020-03-15T18:27:13.2012918Z .................................................................................................... 1800/9771
2020-03-15T18:27:25.3319980Z ...................................................................i................................ 1900/9771
2020-03-15T18:27:32.4999212Z .................................................................................................... 2000/9771
2020-03-15T18:27:47.4812193Z .........................................................iiiii...................................... 2100/9771
2020-03-15T18:27:58.4877035Z .................................................................................................... 2300/9771
2020-03-15T18:28:00.7646977Z .................................................................................................... 2400/9771
2020-03-15T18:28:03.9871045Z .................................................................................................... 2500/9771
2020-03-15T18:28:26.6712837Z .................................................................................................... 2600/9771
---
2020-03-15T18:31:07.9482917Z .............................i...............i...................................................... 5000/9771
2020-03-15T18:31:17.8295529Z .................................................................................................... 5100/9771
2020-03-15T18:31:23.7594698Z ........................................................................i........................... 5200/9771
2020-03-15T18:31:29.3417380Z .................................................................................................... 5300/9771
2020-03-15T18:31:39.2477731Z .....................................................ii.ii........i...i............................. 5400/9771
2020-03-15T18:31:47.4142380Z .................................................................................................... 5600/9771
2020-03-15T18:31:57.2083896Z .................................................................................................... 5700/9771
2020-03-15T18:32:03.3031031Z .............................................i...................................................... 5800/9771
2020-03-15T18:32:09.8220597Z .................................................................................................... 5900/9771
2020-03-15T18:32:09.8220597Z .................................................................................................... 5900/9771
2020-03-15T18:32:19.9955154Z .................................................................................................... 6000/9771
2020-03-15T18:32:25.9619890Z .......................................ii...i..ii...........i....................................... 6100/9771
2020-03-15T18:32:46.5493133Z .................................................................................................... 6300/9771
2020-03-15T18:32:53.4924822Z .................................................................................................... 6400/9771
2020-03-15T18:32:53.4924822Z .................................................................................................... 6400/9771
2020-03-15T18:33:03.0328889Z .....................................................................i..ii.......................... 6500/9771
2020-03-15T18:33:28.2434536Z .................................................................................................... 6700/9771
2020-03-15T18:33:36.8614213Z ...................................................................i................................ 6800/9771
2020-03-15T18:33:38.9810350Z .................................................................................................... 6900/9771
2020-03-15T18:33:41.2059516Z .................................................................................................... 7000/9771
---
2020-03-15T18:35:26.2576500Z .................................................................................................... 7800/9771
2020-03-15T18:35:32.3109160Z .................................................................................................... 7900/9771
2020-03-15T18:35:38.2564185Z ....................................................i............................................... 8000/9771
2020-03-15T18:35:48.9070182Z .................................................................................................... 8100/9771
2020-03-15T18:35:54.6597012Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-15T18:36:09.2753356Z .................................................................................................... 8400/9771
2020-03-15T18:36:20.6656017Z .................................................................................................... 8500/9771
2020-03-15T18:36:33.9965258Z .................................................................................................... 8600/9771
2020-03-15T18:36:39.9620259Z .................................................................................................... 8700/9771
---
2020-03-15T18:39:01.3170158Z  finished in 8.018
2020-03-15T18:39:01.3349149Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:01.5017062Z 
2020-03-15T18:39:01.5018643Z running 183 tests
2020-03-15T18:39:04.3121574Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-15T18:39:06.9713847Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-15T18:39:06.9717228Z 
2020-03-15T18:39:06.9721655Z  finished in 5.637
2020-03-15T18:39:06.9901171Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:07.1340244Z 
---
2020-03-15T18:39:09.0424447Z  finished in 2.052
2020-03-15T18:39:09.0602640Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:09.2162140Z 
2020-03-15T18:39:09.2163462Z running 9 tests
2020-03-15T18:39:09.2164583Z iiiiiiiii
2020-03-15T18:39:09.2165555Z 
2020-03-15T18:39:09.2165916Z  finished in 0.156
2020-03-15T18:39:09.2365886Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:09.3841569Z 
---
2020-03-15T18:39:29.4269916Z  finished in 20.190
2020-03-15T18:39:29.4462877Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:29.5962550Z 
2020-03-15T18:39:29.5962898Z running 115 tests
2020-03-15T18:39:43.8674124Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-15T18:39:45.5861115Z ...iiii.....ii.
2020-03-15T18:39:45.5867772Z 
2020-03-15T18:39:45.5871954Z  finished in 16.141
2020-03-15T18:39:45.5877063Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T18:39:45.5878493Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-15T18:53:12.7622534Z 
2020-03-15T18:53:12.7623462Z    Doc-tests core
2020-03-15T18:53:17.7190686Z 
2020-03-15T18:53:17.7191595Z running 2480 tests
2020-03-15T18:53:26.9618353Z ......iiiii......................................................................................... 100/2480
2020-03-15T18:53:36.2371895Z ....................................................................................ii.............. 200/2480
2020-03-15T18:53:57.2888960Z ...................i................................................................................ 400/2480
2020-03-15T18:53:57.2888960Z ...................i................................................................................ 400/2480
2020-03-15T18:54:07.1763291Z ........................................................................i..i..................iiii.. 500/2480
2020-03-15T18:54:24.0131034Z .................................................................................................... 700/2480
2020-03-15T18:54:32.6976530Z .................................................................................................... 800/2480
2020-03-15T18:54:41.3883195Z .................................................................................................... 900/2480
2020-03-15T18:54:50.0768000Z .................................................................................................... 1000/2480
---
2020-03-15T18:58:32.4461290Z 
2020-03-15T18:58:32.4461743Z running 1010 tests
2020-03-15T18:58:50.6613003Z i................................................................................................... 100/1010
2020-03-15T18:59:00.8539625Z .................................................................................................... 200/1010
2020-03-15T18:59:08.0017588Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-15T18:59:13.2402199Z .................................................................................................... 400/1010
2020-03-15T18:59:20.6897463Z ............................................i..i......................................ii............ 500/1010
2020-03-15T18:59:33.9118234Z .................................................................................................... 700/1010
2020-03-15T18:59:33.9118234Z .................................................................................................... 700/1010
2020-03-15T18:59:41.1521669Z ....................................iiii............................................................ 800/1010
2020-03-15T18:59:55.5799750Z .................................................................................................... 900/1010
2020-03-15T19:00:02.7267411Z ..........................................................iiii...................................... 1000/1010
2020-03-15T19:00:03.1560913Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-15T19:00:03.1561171Z 
2020-03-15T19:00:03.1676387Z  finished in 171.487
2020-03-15T19:00:03.1689755Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-15T19:18:53.0962824Z  finished in 43.569
2020-03-15T19:18:53.1236509Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-15T19:18:53.3468997Z 
2020-03-15T19:18:53.3470065Z running 210 tests
2020-03-15T19:19:21.2413485Z ......................i...ii...................................F.........F.........................i 100/210
2020-03-15T19:20:01.5571919Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-15T19:20:06.1631976Z failures:
2020-03-15T19:20:06.1637008Z 
2020-03-15T19:20:06.1639238Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-03-15T19:20:06.1639474Z 
2020-03-15T19:20:06.1639474Z 
2020-03-15T19:20:06.1639627Z error: make failed
2020-03-15T19:20:06.1639843Z status: exit code: 2
2020-03-15T19:20:06.1640032Z command: "make"
2020-03-15T19:20:06.1640188Z stdout:
2020-03-15T19:20:06.1640591Z ------------------------------------------
2020-03-15T19:20:06.1640892Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-03-15T19:20:06.1641092Z 
2020-03-15T19:20:06.1643459Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-03-15T19:20:06.1645167Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-03-15T19:20:06.1645729Z Makefile:4: recipe for target 'all' failed
2020-03-15T19:20:06.1646205Z ------------------------------------------
2020-03-15T19:20:06.1646403Z stderr:
2020-03-15T19:20:06.1646729Z ------------------------------------------
2020-03-15T19:20:06.1647028Z error[E0433]: failed to resolve: maybe a missing crate `rustc_session`?
---
2020-03-15T19:20:06.1652109Z 
2020-03-15T19:20:06.1652493Z error[E0433]: failed to resolve: use of undeclared type or module `CrateType`
2020-03-15T19:20:06.1653067Z   --> the_backend.rs:97:30
2020-03-15T19:20:06.1653229Z    |
2020-03-15T19:20:06.1653463Z 97 |             if crate_type != CrateType::Rlib {
2020-03-15T19:20:06.1654029Z 
2020-03-15T19:20:06.1654252Z error[E0412]: cannot find type `OutputFilenames` in this scope
2020-03-15T19:20:06.1654651Z   --> the_backend.rs:89:19
2020-03-15T19:20:06.1654813Z    |
2020-03-15T19:20:06.1654813Z    |
2020-03-15T19:20:06.1654994Z 89 |         outputs: &OutputFilenames,
2020-03-15T19:20:06.1655282Z    |                   ^^^^^^^^^^^^^^^ not found in this scope
2020-03-15T19:20:06.1655466Z 
2020-03-15T19:20:06.1655804Z warning: ignoring --out-dir flag due to -o flag
2020-03-15T19:20:06.1656157Z error: aborting due to 5 previous errors
2020-03-15T19:20:06.1656310Z 
2020-03-15T19:20:06.1656522Z Some errors have detailed explanations: E0412, E0432, E0433.
2020-03-15T19:20:06.1657030Z For more information about an error, try `rustc --explain E0412`.
2020-03-15T19:20:06.1657030Z For more information about an error, try `rustc --explain E0412`.
2020-03-15T19:20:06.1657276Z make: *** [all] Error 1
2020-03-15T19:20:06.1657724Z ------------------------------------------
2020-03-15T19:20:06.1657895Z 
2020-03-15T19:20:06.1657980Z 
2020-03-15T19:20:06.1658331Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2020-03-15T19:20:06.1658331Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2020-03-15T19:20:06.1658626Z 
2020-03-15T19:20:06.1659015Z error: make failed
2020-03-15T19:20:06.1659214Z status: exit code: 2
2020-03-15T19:20:06.1659411Z command: "make"
2020-03-15T19:20:06.1659589Z stdout:
2020-03-15T19:20:06.1659966Z ------------------------------------------
2020-03-15T19:20:06.1661938Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2020-03-15T19:20:06.1663855Z Makefile:8: recipe for target 'all' failed
2020-03-15T19:20:06.1664403Z ------------------------------------------
2020-03-15T19:20:06.1664586Z stderr:
2020-03-15T19:20:06.1664910Z ------------------------------------------
2020-03-15T19:20:06.1665225Z error[E0433]: failed to resolve: maybe a missing crate `rustc_session`?
2020-03-15T19:20:06.1665225Z error[E0433]: failed to resolve: maybe a missing crate `rustc_session`?
2020-03-15T19:20:06.1665611Z  --> foo.rs:9:5
2020-03-15T19:20:06.1665753Z   |
2020-03-15T19:20:06.1665958Z 9 | use rustc_session::config::{Input, Options,
2020-03-15T19:20:06.1666261Z   |     ^^^^^^^^^^^^^ maybe a missing crate `rustc_session`?
2020-03-15T19:20:06.1666621Z error[E0432]: unresolved import `rustc_session`
2020-03-15T19:20:06.1666981Z  --> foo.rs:8:5
2020-03-15T19:20:06.1667121Z   |
2020-03-15T19:20:06.1667302Z 8 | use rustc_session::DiagnosticOutput;
---
2020-03-15T19:20:06.1669762Z 
2020-03-15T19:20:06.1670028Z error[E0433]: failed to resolve: use of undeclared type or module `OutputTypes`
2020-03-15T19:20:06.1670565Z   --> foo.rs:40:25
2020-03-15T19:20:06.1670756Z    |
2020-03-15T19:20:06.1671039Z 40 |     opts.output_types = OutputTypes::new(&[(OutputType::Exe, None)]);
2020-03-15T19:20:06.1671467Z    |                         ^^^^^^^^^^^ use of undeclared type or module `OutputTypes`
2020-03-15T19:20:06.1671996Z error[E0433]: failed to resolve: use of undeclared type or module `OutputType`
2020-03-15T19:20:06.1672676Z   --> foo.rs:40:45
2020-03-15T19:20:06.1672824Z    |
2020-03-15T19:20:06.1672824Z    |
2020-03-15T19:20:06.1673095Z 40 |     opts.output_types = OutputTypes::new(&[(OutputType::Exe, None)]);
2020-03-15T19:20:06.1673508Z    |                                             ^^^^^^^^^^ use of undeclared type or module `OutputType`
2020-03-15T19:20:06.1674011Z error[E0433]: failed to resolve: use of undeclared type or module `Input`
2020-03-15T19:20:06.1674412Z   --> foo.rs:48:17
2020-03-15T19:20:06.1674560Z    |
2020-03-15T19:20:06.1674560Z    |
2020-03-15T19:20:06.1674795Z 48 |     let input = Input::Str { name, input: code };
2020-03-15T19:20:06.1675110Z    |                 ^^^^^ use of undeclared type or module `Input`
2020-03-15T19:20:06.1675468Z error: aborting due to 6 previous errors
2020-03-15T19:20:06.1675638Z 
2020-03-15T19:20:06.1675840Z Some errors have detailed explanations: E0432, E0433.
2020-03-15T19:20:06.1676320Z For more information about an error, try `rustc --explain E0432`.
2020-03-15T19:20:06.1676320Z For more information about an error, try `rustc --explain E0432`.
2020-03-15T19:20:06.1676589Z make: *** [all] Error 1
2020-03-15T19:20:06.1677031Z ------------------------------------------
2020-03-15T19:20:06.1677186Z 
2020-03-15T19:20:06.1677271Z 
2020-03-15T19:20:06.1677374Z 
---
2020-03-15T19:20:06.1688408Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-15T19:20:06.1689080Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-15T19:20:06.1689316Z 
2020-03-15T19:20:06.1689413Z 
2020-03-15T19:20:06.1701178Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-15T19:20:06.1707805Z 
2020-03-15T19:20:06.1707915Z 
2020-03-15T19:20:06.1710592Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-15T19:20:06.1710940Z Build completed unsuccessfully in 1:50:22
2020-03-15T19:20:06.1710940Z Build completed unsuccessfully in 1:50:22
2020-03-15T19:20:06.1804969Z == clock drift check ==
2020-03-15T19:20:06.1837704Z   local time: Sun Mar 15 19:20:06 UTC 2020
2020-03-15T19:20:06.2847523Z   network time: Sun, 15 Mar 2020 19:20:06 GMT
2020-03-15T19:20:06.2850543Z == end clock drift check ==
2020-03-15T19:20:08.6611291Z 
2020-03-15T19:20:08.6717555Z ##[error]Bash exited with code '1'.
2020-03-15T19:20:08.6734576Z ##[section]Finishing: Run build
2020-03-15T19:20:08.6791047Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70028/merge to s
2020-03-15T19:20:08.6797003Z Task         : Get sources
2020-03-15T19:20:08.6797318Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T19:20:08.6797610Z Version      : 1.0.0
2020-03-15T19:20:08.6797839Z Author       : Microsoft
2020-03-15T19:20:08.6797839Z Author       : Microsoft
2020-03-15T19:20:08.6798158Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-15T19:20:08.6798530Z ==============================================================================
2020-03-15T19:20:09.0295402Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-15T19:20:09.0345531Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70028/merge to s
2020-03-15T19:20:09.0454682Z Cleaning up task key
2020-03-15T19:20:09.0456084Z Start cleaning up orphan processes.
2020-03-15T19:20:09.0668385Z Terminate orphan process: pid (3857) (python)
2020-03-15T19:20:09.0999900Z ##[section]Finishing: Finalize Job
