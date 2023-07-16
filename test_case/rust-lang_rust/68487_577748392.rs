plain
2020-01-23T14:14:27.0632455Z ========================== Starting Command Output ===========================
2020-01-23T14:14:27.0634047Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/713e2c3c-ce55-4512-929e-a355086ce0a2.sh
2020-01-23T14:14:27.0634529Z 
2020-01-23T14:14:27.0638976Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T14:14:27.0645300Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68487/merge to s
2020-01-23T14:14:27.0647504Z Task         : Get sources
2020-01-23T14:14:27.0647537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T14:14:27.0647613Z Version      : 1.0.0
2020-01-23T14:14:27.0648621Z Author       : Microsoft
---
2020-01-23T14:14:28.1079764Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T14:14:28.1095373Z ##[command]git config gc.auto 0
2020-01-23T14:14:28.1098630Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T14:14:28.1100840Z ##[command]git config --get-all http.proxy
2020-01-23T14:14:28.1107353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68487/merge:refs/remotes/pull/68487/merge
---
2020-01-23T15:11:22.6287905Z .................................................................................................... 1700/9546
2020-01-23T15:11:28.8202868Z .................................................................................................... 1800/9546
2020-01-23T15:11:40.3025839Z .....................i.............................................................................. 1900/9546
2020-01-23T15:11:47.3004559Z .................................................................................................... 2000/9546
2020-01-23T15:12:02.4740351Z ...........iiiii.................................................................................... 2100/9546
2020-01-23T15:12:12.0952143Z .................................................................................................... 2300/9546
2020-01-23T15:12:14.5836774Z .................................................................................................... 2400/9546
2020-01-23T15:12:19.9629917Z .................................................................................................... 2500/9546
2020-01-23T15:12:40.9808630Z .................................................................................................... 2600/9546
---
2020-01-23T15:15:20.9869629Z .......................................................i...............i............................ 4900/9546
2020-01-23T15:15:29.1012898Z .................................................................................................... 5000/9546
2020-01-23T15:15:37.1659528Z ..................................................................................................i. 5100/9546
2020-01-23T15:15:42.2855469Z .................................................................................................... 5200/9546
2020-01-23T15:15:53.0536101Z ......................................................................ii.ii...........i............. 5300/9546
2020-01-23T15:16:02.2067335Z .......i............................................................................................ 5500/9546
2020-01-23T15:16:12.2913017Z .................................................................................................... 5600/9546
2020-01-23T15:16:18.9338686Z ........................................................i........................................... 5700/9546
2020-01-23T15:16:26.0179703Z .................................................................................................... 5800/9546
2020-01-23T15:16:26.0179703Z .................................................................................................... 5800/9546
2020-01-23T15:16:36.5857130Z .................................................................................................... 5900/9546
2020-01-23T15:16:42.9100376Z ...............................................ii...i..ii...........i............................... 6000/9546
2020-01-23T15:17:05.4652770Z .................................................................................................... 6200/9546
2020-01-23T15:17:13.7716784Z .................................................................................................... 6300/9546
2020-01-23T15:17:13.7716784Z .................................................................................................... 6300/9546
2020-01-23T15:17:23.6975012Z ...........................................................................i..ii.................... 6400/9546
2020-01-23T15:17:52.3849709Z .................................................................................................... 6600/9546
2020-01-23T15:17:57.0306784Z ...................................................i................................................ 6700/9546
2020-01-23T15:17:59.2895035Z .................................................................................................... 6800/9546
2020-01-23T15:18:01.5967958Z ..................................................i................................................. 6900/9546
---
2020-01-23T15:19:45.0028266Z .................................................................................................... 7600/9546
2020-01-23T15:19:50.8336215Z .................................................................................................... 7700/9546
2020-01-23T15:19:57.3489737Z .................................................................................................... 7800/9546
2020-01-23T15:20:08.0350529Z .................................................................................................... 7900/9546
2020-01-23T15:20:14.0294565Z ......iiiiiii....................................................................................... 8000/9546
2020-01-23T15:20:28.8351912Z .................................................................................................... 8200/9546
2020-01-23T15:20:40.5211126Z .................................................................................................... 8300/9546
2020-01-23T15:20:53.2901391Z .................................................................................................... 8400/9546
2020-01-23T15:20:59.6042628Z .................................................................................................... 8500/9546
---
2020-01-23T15:23:22.8370796Z  finished in 7.504
2020-01-23T15:23:22.8568287Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:23:23.1079712Z 
2020-01-23T15:23:23.1080405Z running 169 tests
2020-01-23T15:23:26.2048508Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-23T15:23:28.4682221Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-23T15:23:28.4683415Z 
2020-01-23T15:23:28.4687600Z  finished in 5.612
2020-01-23T15:23:28.4872704Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:23:28.6615429Z 
---
2020-01-23T15:23:30.6553341Z  finished in 2.168
2020-01-23T15:23:30.6749116Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:23:30.8414851Z 
2020-01-23T15:23:30.8415128Z running 9 tests
2020-01-23T15:23:30.8415896Z iiiiiiiii
2020-01-23T15:23:30.8416279Z 
2020-01-23T15:23:30.8416325Z  finished in 0.166
2020-01-23T15:23:30.8613368Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:23:31.0725742Z 
---
2020-01-23T15:23:51.1257681Z  finished in 20.264
2020-01-23T15:23:51.3832124Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:23:52.3367740Z 
2020-01-23T15:23:52.3367960Z running 116 tests
2020-01-23T15:24:17.5601146Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-23T15:24:21.1127496Z .....iiii.....ii
2020-01-23T15:24:21.1129897Z 
2020-01-23T15:24:21.1130507Z  finished in 29.730
2020-01-23T15:24:21.1135822Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T15:24:21.1137400Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-23T15:37:51.8399747Z 
2020-01-23T15:37:51.8400636Z    Doc-tests core
2020-01-23T15:37:56.8342100Z 
2020-01-23T15:37:56.8343167Z running 2443 tests
2020-01-23T15:38:06.0750496Z ......iiiii......................................................................................... 100/2443
2020-01-23T15:38:15.1211230Z ..................................................................................ii................ 200/2443
2020-01-23T15:38:36.6509177Z .................i.................................................................................. 400/2443
2020-01-23T15:38:36.6509177Z .................i.................................................................................. 400/2443
2020-01-23T15:38:46.3845859Z ..................................................................i..i..................iiii........ 500/2443
2020-01-23T15:39:02.9434870Z .................................................................................................... 700/2443
2020-01-23T15:39:12.1478065Z .................................................................................................... 800/2443
2020-01-23T15:39:20.8192844Z .................................................................................................... 900/2443
2020-01-23T15:39:29.4519011Z .................................................................................................... 1000/2443
---
2020-01-23T15:42:58.3583713Z .................................................................................................... 500/760
2020-01-23T15:42:58.3919142Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-01-23T15:42:58.3932818Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-01-23T15:42:58.3938611Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-01-23T15:42:58.3959970Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-01-23T15:42:58.6729459Z ..........................................thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-01-23T15:42:58.6750297Z ........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-01-23T15:42:58.6764291Z ..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-01-23T15:43:00.7168226Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-23T15:43:00.7169151Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-01-23T15:43:00.7178946Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-01-23T15:43:00.7187613Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-01-23T15:43:09.9362631Z 
2020-01-23T15:43:09.9371327Z running 1003 tests
2020-01-23T15:43:28.6267785Z i................................................................................................... 100/1003
2020-01-23T15:43:38.8209822Z .................................................................................................... 200/1003
2020-01-23T15:43:45.9435282Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-23T15:43:51.0406317Z .................................................................................................... 400/1003
2020-01-23T15:43:57.9593510Z ..........................................i..i.....................................ii............... 500/1003
2020-01-23T15:44:10.8448027Z .................................................................................................... 700/1003
2020-01-23T15:44:10.8448027Z .................................................................................................... 700/1003
2020-01-23T15:44:17.3711816Z .............................iiii................................................................... 800/1003
2020-01-23T15:44:31.6392122Z .................................................................................................... 900/1003
2020-01-23T15:44:38.5548077Z ...................................................iiii............................................. 1000/1003
2020-01-23T15:44:38.6497928Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-23T15:44:38.6498233Z 
2020-01-23T15:44:38.6620355Z  finished in 173.198
2020-01-23T15:44:38.6634019Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-23T16:03:44.4385520Z  finished in 40.190
2020-01-23T16:03:44.4744294Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-23T16:03:44.7485122Z 
2020-01-23T16:03:44.7485310Z running 203 tests
2020-01-23T16:04:18.4039750Z ....................i...ii...................................F.................................i.... 100/203
2020-01-23T16:04:54.8633838Z ...................................iiiiii......i............iii..................................... 200/203
2020-01-23T16:04:55.5624854Z i..
2020-01-23T16:04:55.5634026Z 
2020-01-23T16:04:55.5636607Z ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
2020-01-23T16:04:55.5637229Z 
2020-01-23T16:04:55.5640581Z error: make failed
2020-01-23T16:04:55.5640581Z error: make failed
2020-01-23T16:04:55.5641986Z status: exit code: 2
2020-01-23T16:04:55.5642156Z command: "make"
2020-01-23T16:04:55.5642306Z stdout:
2020-01-23T16:04:55.5642846Z ------------------------------------------
2020-01-23T16:04:55.5643083Z /bin/echo || exit 0 # This test requires /bin/echo to exist
2020-01-23T16:04:55.5643222Z 
2020-01-23T16:04:55.5644368Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
2020-01-23T16:04:55.5644999Z  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
2020-01-23T16:04:55.5645434Z Makefile:4: recipe for target 'all' failed
2020-01-23T16:04:55.5646560Z ------------------------------------------
2020-01-23T16:04:55.5646770Z stderr:
2020-01-23T16:04:55.5647148Z ------------------------------------------
2020-01-23T16:04:55.5647148Z ------------------------------------------
2020-01-23T16:04:55.5647567Z warning: ignoring --out-dir flag due to -o flag
2020-01-23T16:04:55.5647887Z error[E0046]: not all trait items implemented, missing: `link`
2020-01-23T16:04:55.5648258Z   --> the_backend.rs:44:1
2020-01-23T16:04:55.5648442Z    |
2020-01-23T16:04:55.5648600Z 44 | impl CodegenBackend for TheBackend {
2020-01-23T16:04:55.5648600Z 44 | impl CodegenBackend for TheBackend {
2020-01-23T16:04:55.5648747Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `link` in implementation
2020-01-23T16:04:55.5649099Z    |
2020-01-23T16:04:55.5649646Z    = help: implement the missing item: `fn link(&self, _: &rustc::rustc_session::Session, _: &rustc::rustc_session::config::Input, _: &rustc::rustc_session::config::OutputFilenames) -> std::result::Result<(), rustc::util::common::ErrorReported> { unimplemented!() }`
2020-01-23T16:04:55.5649954Z error: aborting due to previous error
2020-01-23T16:04:55.5650091Z 
2020-01-23T16:04:55.5650449Z For more information about this error, try `rustc --explain E0046`.
2020-01-23T16:04:55.5650449Z For more information about this error, try `rustc --explain E0046`.
2020-01-23T16:04:55.5650631Z make: *** [all] Error 1
2020-01-23T16:04:55.5651830Z ------------------------------------------
2020-01-23T16:04:55.5653021Z 
2020-01-23T16:04:55.5653215Z 
2020-01-23T16:04:55.5653556Z 
---
2020-01-23T16:04:55.5655630Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-23T16:04:55.5655837Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-23T16:04:55.5656069Z 
2020-01-23T16:04:55.5656300Z 
2020-01-23T16:04:55.5662227Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-23T16:04:55.5663713Z 
2020-01-23T16:04:55.5663854Z 
2020-01-23T16:04:55.5688414Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T16:04:55.5688528Z Build completed unsuccessfully in 1:44:45
2020-01-23T16:04:55.5688528Z Build completed unsuccessfully in 1:44:45
2020-01-23T16:04:55.5709288Z == clock drift check ==
2020-01-23T16:04:55.5736555Z   local time: Thu Jan 23 16:04:55 UTC 2020
2020-01-23T16:04:55.8662211Z   network time: Thu, 23 Jan 2020 16:04:55 GMT
2020-01-23T16:04:55.8662319Z == end clock drift check ==
2020-01-23T16:04:57.1805068Z 
2020-01-23T16:04:57.1973197Z ##[error]Bash exited with code '1'.
2020-01-23T16:04:57.1985130Z ##[section]Finishing: Run build
2020-01-23T16:04:57.2008426Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68487/merge to s
2020-01-23T16:04:57.2010643Z Task         : Get sources
2020-01-23T16:04:57.2010898Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T16:04:57.2010957Z Version      : 1.0.0
2020-01-23T16:04:57.2011011Z Author       : Microsoft
2020-01-23T16:04:57.2011011Z Author       : Microsoft
2020-01-23T16:04:57.2011609Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T16:04:57.2011670Z ==============================================================================
2020-01-23T16:04:57.6718120Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T16:04:57.6762208Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68487/merge to s
2020-01-23T16:04:57.6885572Z Cleaning up task key
2020-01-23T16:04:57.6886865Z Start cleaning up orphan processes.
2020-01-23T16:04:57.7003633Z Terminate orphan process: pid (3671) (python)
2020-01-23T16:04:57.7307509Z ##[section]Finishing: Finalize Job
