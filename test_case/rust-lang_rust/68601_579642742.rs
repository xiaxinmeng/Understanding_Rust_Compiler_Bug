plain
2020-01-29T06:31:25.4054587Z ========================== Starting Command Output ===========================
2020-01-29T06:31:25.4286229Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68499c36-e8eb-4c19-8fed-fccff8c0e8ee.sh
2020-01-29T06:31:25.4286450Z 
2020-01-29T06:31:25.4289892Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T06:31:25.4296157Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-29T06:31:25.4297647Z Task         : Get sources
2020-01-29T06:31:25.4297679Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T06:31:25.4297746Z Version      : 1.0.0
2020-01-29T06:31:25.4297777Z Author       : Microsoft
---
2020-01-29T06:31:26.4113614Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T06:31:26.4128618Z ##[command]git config gc.auto 0
2020-01-29T06:31:26.4131977Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T06:31:26.4136895Z ##[command]git config --get-all http.proxy
2020-01-29T06:31:26.4146987Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68601/merge:refs/remotes/pull/68601/merge
---
2020-01-29T07:23:34.5792996Z .................................................................................................... 1700/9557
2020-01-29T07:23:39.3667790Z .................................................................................................... 1800/9557
2020-01-29T07:23:51.4071482Z .........................i.......................................................................... 1900/9557
2020-01-29T07:23:58.1075195Z .................................................................................................... 2000/9557
2020-01-29T07:24:11.6824857Z ...............iiiii................................................................................ 2100/9557
2020-01-29T07:24:20.9081774Z .................................................................................................... 2300/9557
2020-01-29T07:24:23.1614483Z .................................................................................................... 2400/9557
2020-01-29T07:24:28.1449077Z .................................................................................................... 2500/9557
2020-01-29T07:24:47.5595933Z .................................................................................................... 2600/9557
---
2020-01-29T07:27:10.7703088Z .................................................................................................... 4800/9557
2020-01-29T07:27:15.2858884Z ...........................................................i...............i........................ 4900/9557
2020-01-29T07:27:22.3221713Z .................................................................................................... 5000/9557
2020-01-29T07:27:29.7124428Z .................................................................................................... 5100/9557
2020-01-29T07:27:34.1307800Z ..i................................................................................................. 5200/9557
2020-01-29T07:27:44.6822069Z ...........................................................................ii.ii........i...i....... 5300/9557
2020-01-29T07:27:52.9321513Z .............i...................................................................................... 5500/9557
2020-01-29T07:28:02.3382148Z .................................................................................................... 5600/9557
2020-01-29T07:28:08.4280357Z ..............................................................i..................................... 5700/9557
2020-01-29T07:28:15.2783429Z .................................................................................................... 5800/9557
2020-01-29T07:28:15.2783429Z .................................................................................................... 5800/9557
2020-01-29T07:28:22.8986770Z .................................................................................................... 5900/9557
2020-01-29T07:28:31.1779971Z .....................................................ii...i..ii...........i......................... 6000/9557
2020-01-29T07:28:51.5839466Z .................................................................................................... 6200/9557
2020-01-29T07:28:58.5444510Z .................................................................................................... 6300/9557
2020-01-29T07:28:58.5444510Z .................................................................................................... 6300/9557
2020-01-29T07:29:06.3501278Z .................................................................................i..ii.............. 6400/9557
2020-01-29T07:29:30.8311427Z .................................................................................................... 6600/9557
2020-01-29T07:29:35.6728758Z .........................................................i.......................................... 6700/9557
2020-01-29T07:29:37.6874654Z .................................................................................................... 6800/9557
2020-01-29T07:29:39.6829104Z ........................................................i........................................... 6900/9557
---
2020-01-29T07:31:14.2066916Z .................................................................................................... 7600/9557
2020-01-29T07:31:19.0855542Z .................................................................................................... 7700/9557
2020-01-29T07:31:25.2777782Z .................................................................................................... 7800/9557
2020-01-29T07:31:35.1401511Z .................................................................................................... 7900/9557
2020-01-29T07:31:40.8825281Z ............iiiiiii................................................................................. 8000/9557
2020-01-29T07:31:54.0624739Z .................................................................................................... 8200/9557
2020-01-29T07:32:03.8165717Z .................................................................................................... 8300/9557
2020-01-29T07:32:15.8663664Z .................................................................................................... 8400/9557
2020-01-29T07:32:22.0244655Z .................................................................................................... 8500/9557
---
2020-01-29T07:34:32.3495181Z  finished in 6.901
2020-01-29T07:34:32.3666077Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:34:33.0666364Z 
2020-01-29T07:34:33.0669999Z running 169 tests
2020-01-29T07:34:35.2300557Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-29T07:34:37.3625535Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-29T07:34:37.3626083Z 
2020-01-29T07:34:37.3628598Z  finished in 4.996
2020-01-29T07:34:37.3792408Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:34:37.5311920Z 
---
2020-01-29T07:34:39.4010101Z  finished in 2.021
2020-01-29T07:34:39.4192404Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:34:40.0677373Z 
2020-01-29T07:34:40.0677485Z running 9 tests
2020-01-29T07:34:40.0678244Z iiiiiiiii
2020-01-29T07:34:40.0678577Z 
2020-01-29T07:34:40.0678638Z  finished in 0.144
2020-01-29T07:34:40.0678937Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:34:40.0678973Z 
---
2020-01-29T07:34:58.5111092Z  finished in 18.929
2020-01-29T07:34:58.5291189Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:34:59.5711994Z 
2020-01-29T07:34:59.5712116Z running 116 tests
2020-01-29T07:35:12.6231973Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-01-29T07:35:14.5873649Z ....iiii.....ii.
2020-01-29T07:35:14.5874153Z 
2020-01-29T07:35:14.5878740Z  finished in 16.058
2020-01-29T07:35:14.5879657Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T07:35:14.5879997Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-29T07:47:41.2096228Z 
2020-01-29T07:47:41.2107108Z    Doc-tests core
2020-01-29T07:47:45.6376483Z 
2020-01-29T07:47:45.6378376Z running 2467 tests
2020-01-29T07:47:54.0286022Z ......iiiii......................................................................................... 100/2467
2020-01-29T07:48:02.2643318Z ..................................................................................ii................ 200/2467
2020-01-29T07:48:21.6459107Z .................i.................................................................................. 400/2467
2020-01-29T07:48:21.6459107Z .................i.................................................................................. 400/2467
2020-01-29T07:48:30.5762634Z ..................................................................i..i..................iiii........ 500/2467
2020-01-29T07:48:45.6857567Z .................................................................................................... 700/2467
2020-01-29T07:48:53.7122110Z .................................................................................................... 800/2467
2020-01-29T07:49:01.6012520Z .................................................................................................... 900/2467
2020-01-29T07:49:09.4485260Z .................................................................................................... 1000/2467
---
2020-01-29T07:52:19.6405418Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-01-29T07:52:19.6415211Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-01-29T07:52:19.6444334Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-01-29T07:52:19.6938248Z ................... 600/760
2020-01-29T07:52:21.7194027Z ...................thread '..<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-29T07:52:21.7201788Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-01-29T07:52:21.7203113Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
2020-01-29T07:52:21.7207890Z ....thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-01-29T07:52:21.7207998Z   left: `1`,
---
2020-01-29T07:52:30.6329810Z 
2020-01-29T07:52:30.6330068Z running 1003 tests
2020-01-29T07:52:47.4181082Z i................................................................................................... 100/1003
2020-01-29T07:52:56.5482205Z .................................................................................................... 200/1003
2020-01-29T07:53:03.0448101Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-29T07:53:07.6050770Z .................................................................................................... 400/1003
2020-01-29T07:53:13.8309022Z ..........................................i..i.....................................ii............... 500/1003
2020-01-29T07:53:25.6845052Z .................................................................................................... 700/1003
2020-01-29T07:53:25.6845052Z .................................................................................................... 700/1003
2020-01-29T07:53:31.6028600Z .............................iiii................................................................... 800/1003
2020-01-29T07:53:44.6011137Z .................................................................................................... 900/1003
2020-01-29T07:53:50.8760920Z ...................................................iiii............................................. 1000/1003
2020-01-29T07:53:50.9310242Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-29T07:53:50.9310815Z 
2020-01-29T07:53:50.9407157Z  finished in 156.044
2020-01-29T07:53:50.9418478Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-29T08:11:22.9989050Z  finished in 37.167
2020-01-29T08:11:23.0272667Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T08:11:23.2372437Z 
2020-01-29T08:11:23.2372953Z running 204 tests
2020-01-29T08:11:52.4383132Z .....................i...ii..................................F..................................i... 100/204
2020-01-29T08:12:27.5096806Z ....................................iiiiii......i............iii.................................... 200/204
2020-01-29T08:12:27.8812948Z .i..
2020-01-29T08:12:27.8822864Z thread '
2020-01-29T08:12:27.8823209Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-29T08:12:27.8823252Z 
2020-01-29T08:12:27.8823324Z error: make failed
2020-01-29T08:12:27.8823324Z error: make failed
2020-01-29T08:12:27.8823364Z status: exit code: 2
2020-01-29T08:12:27.8823402Z command: "make"
2020-01-29T08:12:27.8823458Z stdout:
2020-01-29T08:12:27.8823675Z ------------------------------------------
2020-01-29T08:12:27.8823725Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-29T08:12:27.8823982Z 
2020-01-29T08:12:27.8824896Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-29T08:12:27.8825263Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-29T08:12:27.8825469Z Makefile:4: recipe for target 'all' failed
2020-01-29T08:12:27.8825733Z ------------------------------------------
2020-01-29T08:12:27.8825776Z stderr:
2020-01-29T08:12:27.8825968Z ------------------------------------------
2020-01-29T08:12:27.8825968Z ------------------------------------------
2020-01-29T08:12:27.8826036Z error[E0407]: method `join_codegen_and_link` is not a member of trait `CodegenBackend`
2020-01-29T08:12:27.8826228Z   --> the_backend.rs:74:5
2020-01-29T08:12:27.8826492Z 74 | /     fn join_codegen_and_link(
2020-01-29T08:12:27.8826532Z 75 | |         &self,
2020-01-29T08:12:27.8826532Z 75 | |         &self,
2020-01-29T08:12:27.8826570Z 76 | |         ongoing_codegen: Box<dyn Any>,
2020-01-29T08:12:27.8826662Z ...  |
2020-01-29T08:12:27.8826697Z 95 | |         Ok(())
2020-01-29T08:12:27.8826732Z 96 | |     }
2020-01-29T08:12:27.8826787Z    | |_____^ not a member of trait `CodegenBackend`
2020-01-29T08:12:27.8826787Z    | |_____^ not a member of trait `CodegenBackend`
2020-01-29T08:12:27.8826815Z 
2020-01-29T08:12:27.8827049Z warning: ignoring --out-dir flag due to -o flag
2020-01-29T08:12:27.8827088Z 
2020-01-29T08:12:27.8827145Z error[E0046]: not all trait items implemented, missing: `join_codegen`, `link`
2020-01-29T08:12:27.8827335Z   --> the_backend.rs:44:1
2020-01-29T08:12:27.8827427Z 44 | impl CodegenBackend for TheBackend {
2020-01-29T08:12:27.8827427Z 44 | impl CodegenBackend for TheBackend {
2020-01-29T08:12:27.8827470Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `join_codegen`, `link` in implementation
2020-01-29T08:12:27.8827509Z    |
2020-01-29T08:12:27.8827973Z    = help: implement the missing item: `fn join_codegen(&self, _: std::boxed::Box<(dyn std::any::Any + 'static)>, _: &rustc::rustc_session::Session, _: &rustc::dep_graph::DepGraph) -> std::result::Result<std::boxed::Box<(dyn std::any::Any + 'static)>, rustc::util::common::ErrorReported> { unimplemented!() }`
2020-01-29T08:12:27.8828409Z    = help: implement the missing item: `fn link(&self, _: &rustc::rustc_session::Session, _: std::boxed::Box<(dyn std::any::Any + 'static)>, _: &rustc::rustc_session::config::OutputFilenames) -> std::result::Result<(), rustc::util::common::ErrorReported> { unimplemented!() }`
2020-01-29T08:12:27.8828518Z error: aborting due to 2 previous errors
2020-01-29T08:12:27.8828541Z 
2020-01-29T08:12:27.8828579Z Some errors have detailed explanations: E0046, E0407.
2020-01-29T08:12:27.8828814Z For more information about an error, try `rustc --explain E0046`.
2020-01-29T08:12:27.8828814Z For more information about an error, try `rustc --explain E0046`.
2020-01-29T08:12:27.8828859Z make: *** [all] Error 1
2020-01-29T08:12:27.8829166Z ------------------------------------------
2020-01-29T08:12:27.8829223Z 
2020-01-29T08:12:27.8829246Z 
2020-01-29T08:12:27.8829268Z 
---
2020-01-29T08:12:27.8834643Z main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-01-29T08:12:27.8835435Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-29T08:12:27.8843414Z 
2020-01-29T08:12:27.8843480Z 
2020-01-29T08:12:27.8847436Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-29T08:12:27.8848172Z 
2020-01-29T08:12:27.8848199Z 
2020-01-29T08:12:27.8900472Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-29T08:12:27.8900541Z Build completed unsuccessfully in 1:35:11
2020-01-29T08:12:27.8900541Z Build completed unsuccessfully in 1:35:11
2020-01-29T08:12:27.8903693Z == clock drift check ==
2020-01-29T08:12:27.8923437Z   local time: Wed Jan 29 08:12:27 UTC 2020
2020-01-29T08:12:28.0403251Z   network time: Wed, 29 Jan 2020 08:12:28 GMT
2020-01-29T08:12:28.0405535Z == end clock drift check ==
2020-01-29T08:12:28.7573036Z 
2020-01-29T08:12:28.7669093Z ##[error]Bash exited with code '1'.
2020-01-29T08:12:28.7681918Z ##[section]Finishing: Run build
2020-01-29T08:12:28.7711826Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-29T08:12:28.7713908Z Task         : Get sources
2020-01-29T08:12:28.7713954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T08:12:28.7714175Z Version      : 1.0.0
2020-01-29T08:12:28.7714215Z Author       : Microsoft
2020-01-29T08:12:28.7714215Z Author       : Microsoft
2020-01-29T08:12:28.7714419Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-29T08:12:28.7714482Z ==============================================================================
2020-01-29T08:12:29.1588968Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-29T08:12:29.1632986Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-29T08:12:29.1732881Z Cleaning up task key
2020-01-29T08:12:29.1733556Z Start cleaning up orphan processes.
2020-01-29T08:12:29.1866151Z Terminate orphan process: pid (7190) (python)
2020-01-29T08:12:29.2131229Z ##[section]Finishing: Finalize Job
