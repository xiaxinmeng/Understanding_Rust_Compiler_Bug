plain
2019-09-25T18:49:56.9604836Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T18:49:56.9858387Z ##[command]git config gc.auto 0
2019-09-25T18:49:56.9933930Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T18:49:56.9987748Z ##[command]git config --get-all http.proxy
2019-09-25T18:49:57.0132731Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64772/merge:refs/remotes/pull/64772/merge
---
2019-09-25T19:51:40.8496737Z .................................................................................................... 1500/9044
2019-09-25T19:51:46.7397748Z .................................................................................................... 1600/9044
2019-09-25T19:51:58.9402091Z ..........................................................................i...............i......... 1700/9044
2019-09-25T19:52:05.5280356Z .................................................................................................... 1800/9044
2019-09-25T19:52:13.9187386Z .................................................................iiiii.............................. 1900/9044
2019-09-25T19:52:33.1567803Z .................................................................................................... 2100/9044
2019-09-25T19:52:35.6580316Z .................................................................................................... 2200/9044
2019-09-25T19:52:38.7350910Z .................................................................................................... 2300/9044
2019-09-25T19:52:47.1093489Z .................................................................................................... 2400/9044
---
2019-09-25T19:55:41.3362113Z ........................................................i...............i........................... 4700/9044
2019-09-25T19:55:50.3184341Z .................................................................................................... 4800/9044
2019-09-25T19:55:58.8575109Z .................................................................................................... 4900/9044
2019-09-25T19:56:06.1915575Z .................................................................................................... 5000/9044
2019-09-25T19:56:15.6133326Z ...........................................ii.ii.................................................... 5100/9044
2019-09-25T19:56:25.5078200Z .................................................................................................... 5300/9044
2019-09-25T19:56:35.4970913Z .................................................................................................... 5400/9044
2019-09-25T19:56:42.6960749Z ........i........................................................................................... 5500/9044
2019-09-25T19:56:48.1875773Z .................................................................................................... 5600/9044
2019-09-25T19:56:48.1875773Z .................................................................................................... 5600/9044
2019-09-25T19:56:59.6898089Z .................................................................................................... 5700/9044
2019-09-25T19:57:12.3568236Z ...ii...i..ii...........i........................................................................... 5800/9044
2019-09-25T19:57:33.3903376Z .................................................................................................... 6000/9044
2019-09-25T19:57:42.3357655Z .................................................................................................... 6100/9044
2019-09-25T19:57:42.3357655Z .................................................................................................... 6100/9044
2019-09-25T19:57:56.1159859Z .....i..ii.......................................................................................... 6200/9044
2019-09-25T19:58:14.6790489Z .................................................................i.................................. 6400/9044
2019-09-25T19:58:16.8469963Z .................................................................................................... 6500/9044
2019-09-25T19:58:19.4029334Z .....................................i.............................................................. 6600/9044
2019-09-25T19:58:23.4010732Z .................................................................................................... 6700/9044
---
2019-09-25T19:59:16.7273238Z .................................................................................................... 7200/9044
2019-09-25T19:59:21.9635213Z .................................................................................................... 7300/9044
2019-09-25T19:59:29.1669307Z .................................................................................................... 7400/9044
2019-09-25T19:59:38.9891568Z .................................................................................................... 7500/9044
2019-09-25T19:59:48.9283953Z .........................................................................................ii......i.. 7600/9044
2019-09-25T19:59:59.9255422Z .................................................................................................... 7800/9044
2019-09-25T20:00:16.6275128Z .................................................................................................... 7900/9044
2019-09-25T20:00:24.6383099Z .................................................................................................... 8000/9044
2019-09-25T20:00:34.3796900Z .................................................................................................... 8100/9044
---
2019-09-25T20:02:46.2445269Z  finished in 5.257
2019-09-25T20:02:46.2657854Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:02:46.4188936Z 
2019-09-25T20:02:46.4190086Z running 150 tests
2019-09-25T20:02:49.6741148Z i....iii......iii..iiii....i..............................i.i..................i....i.........ii.i.i 100/150
2019-09-25T20:02:51.6032967Z ..iiii..............i.........iii.i.......ii......
2019-09-25T20:02:51.6034369Z 
2019-09-25T20:02:51.6040177Z  finished in 5.338
2019-09-25T20:02:51.6231737Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:02:51.7756035Z 
---
2019-09-25T20:02:53.8551397Z  finished in 2.232
2019-09-25T20:02:53.8733507Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:02:54.0216495Z 
2019-09-25T20:02:54.0217150Z running 9 tests
2019-09-25T20:02:54.0219658Z iiiiiiiii
2019-09-25T20:02:54.0221223Z 
2019-09-25T20:02:54.0221499Z  finished in 0.148
2019-09-25T20:02:54.0407620Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:02:54.1977268Z 
---
2019-09-25T20:03:11.9112164Z  finished in 17.870
2019-09-25T20:03:11.9322410Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:03:12.0928557Z 
2019-09-25T20:03:12.0929889Z running 123 tests
2019-09-25T20:03:35.8275573Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-25T20:03:40.3499991Z i.i.i......iii.i.....ii
2019-09-25T20:03:40.3501136Z 
2019-09-25T20:03:40.3503975Z  finished in 28.417
2019-09-25T20:03:40.3513943Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:03:40.3517505Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-25T20:16:30.9372968Z 
2019-09-25T20:16:30.9377814Z    Doc-tests core
2019-09-25T20:16:35.8604269Z 
2019-09-25T20:16:35.8605252Z running 2405 tests
2019-09-25T20:16:47.0765977Z ......iiiii......................................................................................... 100/2405
2019-09-25T20:16:57.8142981Z ...............................................................................ii................... 200/2405
2019-09-25T20:17:23.4355496Z .i.................................................................................................. 400/2405
2019-09-25T20:17:23.4355496Z .i.................................................................................................. 400/2405
2019-09-25T20:17:34.0773350Z ................................................i..i.................iiii........................... 500/2405
2019-09-25T20:17:54.4719404Z .................................................................................................... 700/2405
2019-09-25T20:18:05.1548569Z .................................................................................................... 800/2405
2019-09-25T20:18:15.6887141Z .................................................................................................... 900/2405
2019-09-25T20:18:26.1444656Z .................................................................................................... 1000/2405
---
2019-09-25T20:22:26.9399961Z ............thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
2019-09-25T20:22:26.9400262Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-09-25T20:22:26.9400317Z   left: `1`,
2019-09-25T20:22:26.9400554Z  right: `2`', src/libstd/sync/mutex.rs:653:13
2019-09-25T20:22:26.9477927Z ..........thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:791:13
2019-09-25T20:22:26.9494259Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2019-09-25T20:22:26.9526466Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-09-25T20:22:26.9526774Z .thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:635:13
2019-09-25T20:22:26.9527528Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:611:13
2019-09-25T20:22:26.9527875Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-09-25T20:22:28.9963298Z ..........................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-09-25T20:22:28.9995455Z ........................ 700/763
---
2019-09-25T20:22:36.3169636Z 
2019-09-25T20:22:36.3169966Z running 992 tests
2019-09-25T20:22:57.7351592Z i................................................................................................... 100/992
2019-09-25T20:23:09.4391657Z .................................................................................................... 200/992
2019-09-25T20:23:17.7663524Z .................iii......i......i...i......i....................................................... 300/992
2019-09-25T20:23:23.6160384Z .................................................................................................... 400/992
2019-09-25T20:23:31.5618113Z ...................................i..i.................................ii.......................... 500/992
2019-09-25T20:23:46.8378696Z .................................................................................................... 700/992
2019-09-25T20:23:46.8378696Z .................................................................................................... 700/992
2019-09-25T20:23:55.2449296Z ..................iiii.............................................................................. 800/992
2019-09-25T20:24:10.2886255Z .................................................................................................... 900/992
2019-09-25T20:24:17.9483958Z ........................................iiii................................................
2019-09-25T20:24:17.9487054Z 
2019-09-25T20:24:17.9519868Z  finished in 193.945
2019-09-25T20:24:17.9536545Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:24:18.1538763Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-25T20:40:04.9323902Z  finished in 37.509
2019-09-25T20:40:04.9649883Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-25T20:40:05.1204023Z 
2019-09-25T20:40:05.1206246Z running 202 tests
2019-09-25T20:40:39.6521908Z ....................i...ii................................F................................i........ 100/202
2019-09-25T20:41:25.2044879Z ................................iiii.......i...........iiii.iii....................................i 200/202
2019-09-25T20:41:26.6530758Z failures:
2019-09-25T20:41:26.6542607Z 
2019-09-25T20:41:26.6543549Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2019-09-25T20:41:26.6543789Z 
2019-09-25T20:41:26.6543789Z 
2019-09-25T20:41:26.6544019Z error: make failed
2019-09-25T20:41:26.6544176Z status: exit code: 2
2019-09-25T20:41:26.6544460Z command: "make"
2019-09-25T20:41:26.6544632Z stdout:
2019-09-25T20:41:26.6550458Z ------------------------------------------
2019-09-25T20:41:26.6550821Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2019-09-25T20:41:26.6550992Z 
2019-09-25T20:41:26.6552258Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2019-09-25T20:41:26.6552995Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2019-09-25T20:41:26.6553415Z Makefile:4: recipe for target 'all' failed
2019-09-25T20:41:26.6554301Z ------------------------------------------
2019-09-25T20:41:26.6554500Z stderr:
2019-09-25T20:41:26.6554883Z ------------------------------------------
2019-09-25T20:41:26.6554883Z ------------------------------------------
2019-09-25T20:41:26.6555283Z warning: ignoring --out-dir flag due to -o flag
2019-09-25T20:41:26.6555715Z warning: trait objects without an explicit `dyn` are deprecated
2019-09-25T20:41:26.6556055Z   --> the_backend.rs:44:38
2019-09-25T20:41:26.6556233Z    |
2019-09-25T20:41:26.6556233Z    |
2019-09-25T20:41:26.6556618Z 44 |     fn metadata_loader(&self) -> Box<MetadataLoader + Sync> {
2019-09-25T20:41:26.6557099Z    |                                      ^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn MetadataLoader + Sync`
2019-09-25T20:41:26.6557474Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-09-25T20:41:26.6557604Z 
2019-09-25T20:41:26.6557747Z warning: trait objects without an explicit `dyn` are deprecated
2019-09-25T20:41:26.6558640Z   --> the_backend.rs:67:33
2019-09-25T20:41:26.6558640Z   --> the_backend.rs:67:33
2019-09-25T20:41:26.6558914Z    |
2019-09-25T20:41:26.6559084Z 67 |         _rx: mpsc::Receiver<Box<Any + Send>>
2019-09-25T20:41:26.6559265Z    |                                 ^^^^^^^^^^ help: use `dyn`: `dyn Any + Send`
2019-09-25T20:41:26.6559545Z warning: trait objects without an explicit `dyn` are deprecated
2019-09-25T20:41:26.6559941Z   --> the_backend.rs:68:14
2019-09-25T20:41:26.6560135Z    |
2019-09-25T20:41:26.6560776Z 68 |     ) -> Box<Any> {
2019-09-25T20:41:26.6560776Z 68 |     ) -> Box<Any> {
2019-09-25T20:41:26.6562319Z    |              ^^^ help: use `dyn`: `dyn Any`
2019-09-25T20:41:26.6562541Z 
2019-09-25T20:41:26.6562979Z warning: trait objects without an explicit `dyn` are deprecated
2019-09-25T20:41:26.6563499Z   --> the_backend.rs:76:30
2019-09-25T20:41:26.6563701Z    |
2019-09-25T20:41:26.6563845Z 76 |         ongoing_codegen: Box<Any>,
2019-09-25T20:41:26.6564012Z    |                              ^^^ help: use `dyn`: `dyn Any`
2019-09-25T20:41:26.6564276Z warning: trait objects without an explicit `dyn` are deprecated
2019-09-25T20:41:26.6564670Z    --> the_backend.rs:101:41
2019-09-25T20:41:26.6564863Z     |
2019-09-25T20:41:26.6564863Z     |
2019-09-25T20:41:26.6565337Z 101 | pub fn __rustc_codegen_backend() -> Box<CodegenBackend> {
2019-09-25T20:41:26.6565551Z     |                                         ^^^^^^^^^^^^^^ help: use `dyn`: `dyn CodegenBackend`
2019-09-25T20:41:26.6565677Z 
2019-09-25T20:41:26.6565823Z error[E0050]: method `codegen_crate` has 5 parameters but the declaration in trait `rustc_codegen_utils::codegen_backend::CodegenBackend::codegen_crate` has 4
2019-09-25T20:41:26.6566314Z   --> the_backend.rs:63:9
2019-09-25T20:41:26.6566672Z 63 | /         &self,
2019-09-25T20:41:26.6567030Z 64 | |         tcx: TyCtxt<'tcx>,
2019-09-25T20:41:26.6567030Z 64 | |         tcx: TyCtxt<'tcx>,
2019-09-25T20:41:26.6567220Z 65 | |         _metadata: EncodedMetadata,
2019-09-25T20:41:26.6567384Z 66 | |         _need_metadata_module: bool,
2019-09-25T20:41:26.6567526Z 67 | |         _rx: mpsc::Receiver<Box<Any + Send>>
2019-09-25T20:41:26.6568347Z    |
2019-09-25T20:41:26.6568347Z    |
2019-09-25T20:41:26.6568922Z    = note: `codegen_crate` from trait: `fn(&Self, rustc::ty::TyCtxt<'tcx>, rustc::middle::cstore::EncodedMetadata, bool) -> std::boxed::Box<(dyn std::any::Any + 'static)>`
2019-09-25T20:41:26.6569289Z error: aborting due to previous error
2019-09-25T20:41:26.6569412Z 
2019-09-25T20:41:26.6569816Z For more information about this error, try `rustc --explain E0050`.
2019-09-25T20:41:26.6569816Z For more information about this error, try `rustc --explain E0050`.
2019-09-25T20:41:26.6570038Z make: *** [all] Error 1
2019-09-25T20:41:26.6570549Z ------------------------------------------
2019-09-25T20:41:26.6570743Z 
2019-09-25T20:41:26.6570891Z 
2019-09-25T20:41:26.6571018Z 
---
2019-09-25T20:41:26.6573632Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-25T20:41:26.6573687Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T20:41:26.6573717Z 
2019-09-25T20:41:26.6573755Z 
2019-09-25T20:41:26.6578444Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-25T20:41:26.6579390Z 
2019-09-25T20:41:26.6579421Z 
2019-09-25T20:41:26.6579481Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-25T20:41:26.6579531Z Build completed unsuccessfully in 1:44:27
2019-09-25T20:41:26.6579531Z Build completed unsuccessfully in 1:44:27
2019-09-25T20:41:26.6626797Z == clock drift check ==
2019-09-25T20:41:27.0548242Z   local time: Wed Sep 25 20:41:27 UTC 2019
2019-09-25T20:41:27.2698430Z   network time: Wed, 25 Sep 2019 20:41:27 GMT
2019-09-25T20:41:27.2702331Z == end clock drift check ==
2019-09-25T20:41:37.4368556Z ##[error]Bash exited with code '1'.
2019-09-25T20:41:37.4452045Z ##[section]Starting: Checkout
2019-09-25T20:41:37.4454013Z ==============================================================================
2019-09-25T20:41:37.4454279Z Task         : Get sources
2019-09-25T20:41:37.4454326Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
