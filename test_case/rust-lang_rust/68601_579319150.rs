plain
2020-01-28T14:13:37.8437248Z ========================== Starting Command Output ===========================
2020-01-28T14:13:37.8439177Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/56d564dc-97d2-4f78-a812-1dc60f9a5226.sh
2020-01-28T14:13:37.8439216Z 
2020-01-28T14:13:37.8442395Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T14:13:37.8449642Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-28T14:13:37.8451393Z Task         : Get sources
2020-01-28T14:13:37.8451432Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T14:13:37.8451468Z Version      : 1.0.0
2020-01-28T14:13:37.8451505Z Author       : Microsoft
---
2020-01-28T14:13:39.6336685Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T14:13:39.6347329Z ##[command]git config gc.auto 0
2020-01-28T14:13:39.6349699Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T14:13:39.6351646Z ##[command]git config --get-all http.proxy
2020-01-28T14:13:39.6357980Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68601/merge:refs/remotes/pull/68601/merge
---
2020-01-28T15:08:20.4199221Z .................................................................................................... 1700/9557
2020-01-28T15:08:25.4788391Z .................................................................................................... 1800/9557
2020-01-28T15:08:38.1765233Z .........................i.......................................................................... 1900/9557
2020-01-28T15:08:45.1049684Z .................................................................................................... 2000/9557
2020-01-28T15:08:59.4396102Z ...............iiiii................................................................................ 2100/9557
2020-01-28T15:09:09.8214047Z .................................................................................................... 2300/9557
2020-01-28T15:09:12.2988085Z .................................................................................................... 2400/9557
2020-01-28T15:09:17.3277887Z .................................................................................................... 2500/9557
2020-01-28T15:09:37.8839091Z .................................................................................................... 2600/9557
---
2020-01-28T15:12:13.6942361Z .................................................................................................... 4800/9557
2020-01-28T15:12:18.7475868Z ...........................................................i...............i........................ 4900/9557
2020-01-28T15:12:26.5034628Z .................................................................................................... 5000/9557
2020-01-28T15:12:34.8359462Z .................................................................................................... 5100/9557
2020-01-28T15:12:39.6411332Z ..i................................................................................................. 5200/9557
2020-01-28T15:12:50.4531581Z ...........................................................................ii.ii........i...i....... 5300/9557
2020-01-28T15:12:58.9544869Z .............i...................................................................................... 5500/9557
2020-01-28T15:13:08.8031928Z .................................................................................................... 5600/9557
2020-01-28T15:13:15.7234186Z ..............................................................i..................................... 5700/9557
2020-01-28T15:13:22.1905452Z .................................................................................................... 5800/9557
2020-01-28T15:13:22.1905452Z .................................................................................................... 5800/9557
2020-01-28T15:13:30.0370394Z .................................................................................................... 5900/9557
2020-01-28T15:13:38.7284221Z .....................................................ii...i..ii...........i......................... 6000/9557
2020-01-28T15:14:00.5538870Z .................................................................................................... 6200/9557
2020-01-28T15:14:05.3296144Z .................................................................................................... 6300/9557
2020-01-28T15:14:05.3296144Z .................................................................................................... 6300/9557
2020-01-28T15:14:09.7803729Z .................................................................................i..ii.............. 6400/9557
2020-01-28T15:14:35.8734178Z .................................................................................................... 6600/9557
2020-01-28T15:14:41.3211884Z .........................................................i.......................................... 6700/9557
2020-01-28T15:14:43.5096806Z .................................................................................................... 6800/9557
2020-01-28T15:14:45.7527910Z ........................................................i........................................... 6900/9557
---
2020-01-28T15:16:28.7831664Z .................................................................................................... 7600/9557
2020-01-28T15:16:34.1068268Z .................................................................................................... 7700/9557
2020-01-28T15:16:40.6671582Z .................................................................................................... 7800/9557
2020-01-28T15:16:51.7525940Z .................................................................................................... 7900/9557
2020-01-28T15:16:58.0579071Z ............iiiiiii................................................................................. 8000/9557
2020-01-28T15:17:12.7654731Z .................................................................................................... 8200/9557
2020-01-28T15:17:23.7023880Z .................................................................................................... 8300/9557
2020-01-28T15:17:37.1568513Z .................................................................................................... 8400/9557
2020-01-28T15:17:43.9101506Z .................................................................................................... 8500/9557
---
2020-01-28T15:20:01.7954169Z  finished in 7.026
2020-01-28T15:20:01.8153611Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:02.0087522Z 
2020-01-28T15:20:02.0087766Z running 169 tests
2020-01-28T15:20:04.9710888Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-28T15:20:07.1881684Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-28T15:20:07.1882280Z 
2020-01-28T15:20:07.1887817Z  finished in 5.373
2020-01-28T15:20:07.2097714Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:07.3927510Z 
---
2020-01-28T15:20:09.3645699Z  finished in 2.155
2020-01-28T15:20:09.3859358Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:09.5459350Z 
2020-01-28T15:20:09.5460280Z running 9 tests
2020-01-28T15:20:09.5461844Z iiiiiiiii
2020-01-28T15:20:09.5462293Z 
2020-01-28T15:20:09.5465222Z  finished in 0.160
2020-01-28T15:20:09.5683289Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:09.7693165Z 
---
2020-01-28T15:20:29.3560996Z  finished in 19.787
2020-01-28T15:20:29.3803659Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:29.5736258Z 
2020-01-28T15:20:29.5737675Z running 116 tests
2020-01-28T15:20:43.3871296Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-01-28T15:20:45.1403112Z ....iiii.....ii.
2020-01-28T15:20:45.1405394Z 
2020-01-28T15:20:45.1407708Z  finished in 15.760
2020-01-28T15:20:45.1416148Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:20:45.1416526Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-28T15:33:33.3053287Z 
2020-01-28T15:33:33.3054154Z    Doc-tests core
2020-01-28T15:33:37.9021428Z 
2020-01-28T15:33:37.9022374Z running 2467 tests
2020-01-28T15:33:46.9523721Z ......iiiii......................................................................................... 100/2467
2020-01-28T15:33:55.4867144Z ..................................................................................ii................ 200/2467
2020-01-28T15:34:15.5503929Z .................i.................................................................................. 400/2467
2020-01-28T15:34:15.5503929Z .................i.................................................................................. 400/2467
2020-01-28T15:34:24.6166060Z ..................................................................i..i..................iiii........ 500/2467
2020-01-28T15:34:40.7080752Z .................................................................................................... 700/2467
2020-01-28T15:34:48.7266979Z .................................................................................................... 800/2467
2020-01-28T15:34:57.0506206Z .................................................................................................... 900/2467
2020-01-28T15:35:05.4543642Z .................................................................................................... 1000/2467
---
2020-01-28T15:38:25.0593833Z .................................................................................................... 500/760
2020-01-28T15:38:25.1080428Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-01-28T15:38:25.1095594Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-01-28T15:38:25.1097736Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-01-28T15:38:25.1118879Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-01-28T15:38:25.5653196Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997..:22
2020-01-28T15:38:25.5660758Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-01-28T15:38:25.5682441Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-01-28T15:38:25.5788038Z .................. 600/760
2020-01-28T15:38:27.6082005Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-28T15:38:27.6088725Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-01-28T15:38:27.6105860Z ........thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
---
2020-01-28T15:38:36.6160823Z 
2020-01-28T15:38:36.6163000Z running 1003 tests
2020-01-28T15:38:54.0658296Z i................................................................................................... 100/1003
2020-01-28T15:39:04.3588838Z .................................................................................................... 200/1003
2020-01-28T15:39:11.3124118Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-28T15:39:16.1524463Z .................................................................................................... 400/1003
2020-01-28T15:39:22.7499225Z ..........................................i..i.....................................ii............... 500/1003
2020-01-28T15:39:34.8980734Z .................................................................................................... 700/1003
2020-01-28T15:39:34.8980734Z .................................................................................................... 700/1003
2020-01-28T15:39:41.2675006Z .............................iiii................................................................... 800/1003
2020-01-28T15:39:54.6269443Z .................................................................................................... 900/1003
2020-01-28T15:40:01.2503552Z ...................................................iiii............................................. 1000/1003
2020-01-28T15:40:01.3473338Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-28T15:40:01.3473754Z 
2020-01-28T15:40:01.3606834Z  finished in 164.979
2020-01-28T15:40:01.3616848Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-28T15:57:50.5104807Z  finished in 37.094
2020-01-28T15:57:50.5470471Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-28T15:57:50.7573105Z 
2020-01-28T15:57:50.7573799Z running 204 tests
2020-01-28T15:58:19.7222993Z .....................i...ii...................................F.................................i... 100/204
2020-01-28T15:58:53.9665302Z ....................................iiiiii......i............iii.................................... 200/204
2020-01-28T15:58:54.9020980Z .i..
2020-01-28T15:58:54.9031381Z 
2020-01-28T15:58:54.9031980Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-28T15:58:54.9032020Z 
2020-01-28T15:58:54.9035676Z error: make failed
2020-01-28T15:58:54.9035676Z error: make failed
2020-01-28T15:58:54.9036041Z status: exit code: 2
2020-01-28T15:58:54.9036128Z command: "make"
2020-01-28T15:58:54.9036216Z stdout:
2020-01-28T15:58:54.9036774Z ------------------------------------------
2020-01-28T15:58:54.9038332Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-28T15:58:54.9038430Z 
2020-01-28T15:58:54.9039626Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-28T15:58:54.9040063Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-28T15:58:54.9040315Z Makefile:4: recipe for target 'all' failed
2020-01-28T15:58:54.9040590Z ------------------------------------------
2020-01-28T15:58:54.9040658Z stderr:
2020-01-28T15:58:54.9040886Z ------------------------------------------
2020-01-28T15:58:54.9040886Z ------------------------------------------
2020-01-28T15:58:54.9040954Z error[E0407]: method `join_codegen_and_link` is not a member of trait `CodegenBackend`
2020-01-28T15:58:54.9041193Z   --> the_backend.rs:74:5
2020-01-28T15:58:54.9041291Z 74 | /     fn join_codegen_and_link(
2020-01-28T15:58:54.9041356Z 75 | |         &self,
2020-01-28T15:58:54.9041356Z 75 | |         &self,
2020-01-28T15:58:54.9041406Z 76 | |         ongoing_codegen: Box<dyn Any>,
2020-01-28T15:58:54.9041516Z ...  |
2020-01-28T15:58:54.9041562Z 95 | |         Ok(())
2020-01-28T15:58:54.9041608Z 96 | |     }
2020-01-28T15:58:54.9041659Z    | |_____^ not a member of trait `CodegenBackend`
2020-01-28T15:58:54.9041659Z    | |_____^ not a member of trait `CodegenBackend`
2020-01-28T15:58:54.9041709Z 
2020-01-28T15:58:54.9041945Z warning: ignoring --out-dir flag due to -o flag
2020-01-28T15:58:54.9041988Z 
2020-01-28T15:58:54.9042040Z error[E0046]: not all trait items implemented, missing: `join_codegen`, `link`
2020-01-28T15:58:54.9042274Z   --> the_backend.rs:44:1
2020-01-28T15:58:54.9042573Z 44 | impl CodegenBackend for TheBackend {
2020-01-28T15:58:54.9042573Z 44 | impl CodegenBackend for TheBackend {
2020-01-28T15:58:54.9042652Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `join_codegen`, `link` in implementation
2020-01-28T15:58:54.9042701Z    |
2020-01-28T15:58:54.9043218Z    = help: implement the missing item: `fn join_codegen(&self, _: std::boxed::Box<(dyn std::any::Any + 'static)>, _: &rustc::rustc_session::Session, _: &rustc::dep_graph::DepGraph) -> std::result::Result<std::boxed::Box<(dyn std::any::Any + 'static)>, rustc::util::common::ErrorReported> { unimplemented!() }`
2020-01-28T15:58:54.9043734Z    = help: implement the missing item: `fn link(&self, _: &rustc::rustc_session::Session, _: std::boxed::Box<(dyn std::any::Any + 'static)>, _: &rustc::rustc_session::config::OutputFilenames) -> std::result::Result<(), rustc::util::common::ErrorReported> { unimplemented!() }`
2020-01-28T15:58:54.9043941Z error: aborting due to 2 previous errors
2020-01-28T15:58:54.9043974Z 
2020-01-28T15:58:54.9044023Z Some errors have detailed explanations: E0046, E0407.
2020-01-28T15:58:54.9044305Z For more information about an error, try `rustc --explain E0046`.
2020-01-28T15:58:54.9044305Z For more information about an error, try `rustc --explain E0046`.
2020-01-28T15:58:54.9044377Z make: *** [all] Error 1
2020-01-28T15:58:54.9044638Z ------------------------------------------
2020-01-28T15:58:54.9044671Z 
2020-01-28T15:58:54.9044697Z 
2020-01-28T15:58:54.9044741Z 
---
2020-01-28T15:58:54.9045678Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-28T15:58:54.9045764Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-28T15:58:54.9045830Z 
2020-01-28T15:58:54.9045859Z 
2020-01-28T15:58:54.9064355Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-28T15:58:54.9065413Z 
2020-01-28T15:58:54.9065447Z 
2020-01-28T15:58:54.9068315Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-28T15:58:54.9068390Z Build completed unsuccessfully in 1:39:08
2020-01-28T15:58:54.9068390Z Build completed unsuccessfully in 1:39:08
2020-01-28T15:58:54.9124096Z == clock drift check ==
2020-01-28T15:58:54.9141554Z   local time: Tue Jan 28 15:58:54 UTC 2020
2020-01-28T15:58:55.2084299Z   network time: Tue, 28 Jan 2020 15:58:55 GMT
2020-01-28T15:58:55.2090779Z == end clock drift check ==
2020-01-28T15:58:56.5452472Z 
2020-01-28T15:58:56.5544308Z ##[error]Bash exited with code '1'.
2020-01-28T15:58:56.5557017Z ##[section]Finishing: Run build
2020-01-28T15:58:56.5576465Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-28T15:58:56.5578407Z Task         : Get sources
2020-01-28T15:58:56.5578460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T15:58:56.5578529Z Version      : 1.0.0
2020-01-28T15:58:56.5578576Z Author       : Microsoft
2020-01-28T15:58:56.5578576Z Author       : Microsoft
2020-01-28T15:58:56.5578628Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T15:58:56.5578700Z ==============================================================================
2020-01-28T15:58:56.9809720Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T15:58:56.9849370Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-28T15:58:56.9956264Z Cleaning up task key
2020-01-28T15:58:56.9957192Z Start cleaning up orphan processes.
2020-01-28T15:58:57.0057713Z Terminate orphan process: pid (3515) (python)
2020-01-28T15:58:57.0854626Z ##[section]Finishing: Finalize Job
