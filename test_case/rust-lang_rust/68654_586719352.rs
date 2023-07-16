plain
2020-02-16T13:38:27.9675536Z ========================== Starting Command Output ===========================
2020-02-16T13:38:27.9677519Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/81214ddb-ecbe-4492-8825-950decb1041f.sh
2020-02-16T13:38:27.9677550Z 
2020-02-16T13:38:27.9682206Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T13:38:27.9689320Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T13:38:27.9690708Z Task         : Get sources
2020-02-16T13:38:27.9690735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T13:38:27.9690762Z Version      : 1.0.0
2020-02-16T13:38:27.9690828Z Author       : Microsoft
---
2020-02-16T13:38:31.6563857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T13:38:31.6580487Z ##[command]git config gc.auto 0
2020-02-16T13:38:31.6582929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T13:38:31.6585872Z ##[command]git config --get-all http.proxy
2020-02-16T13:38:31.6595688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68654/merge:refs/remotes/pull/68654/merge
---
2020-02-16T14:36:19.2307394Z .................................................................................................... 1700/9650
2020-02-16T14:36:23.9085371Z .................................................................................................... 1800/9650
2020-02-16T14:36:35.4852908Z ..................................i................................................................. 1900/9650
2020-02-16T14:36:42.8944259Z .................................................................................................... 2000/9650
2020-02-16T14:36:56.7921283Z ........................iiiii....................................................................... 2100/9650
2020-02-16T14:37:06.2763556Z .................................................................................................... 2300/9650
2020-02-16T14:37:08.7002126Z .................................................................................................... 2400/9650
2020-02-16T14:37:13.1318682Z .................................................................................................... 2500/9650
2020-02-16T14:37:33.3945284Z .................................................................................................... 2600/9650
---
2020-02-16T14:40:48.0643949Z .................................................................................................... 5600/9650
2020-02-16T14:40:58.0684976Z .......................................................................................i............ 5700/9650
2020-02-16T14:41:05.7333458Z .................................................................................................... 5800/9650
2020-02-16T14:41:10.7834585Z .....................................................................................i.............. 5900/9650
2020-02-16T14:41:20.2710406Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T14:41:32.1470034Z i................................................................................................... 6100/9650
2020-02-16T14:41:48.2566097Z .................................................................................................... 6300/9650
2020-02-16T14:41:55.8111345Z .................................................................................................... 6400/9650
2020-02-16T14:41:55.8111345Z .................................................................................................... 6400/9650
2020-02-16T14:42:11.4629949Z .......i..ii........................................................................................ 6500/9650
2020-02-16T14:42:30.4982113Z ...............................................................................................i.... 6700/9650
2020-02-16T14:42:32.6862976Z .................................................................................................... 6800/9650
2020-02-16T14:42:35.0145840Z .................................................................................................... 6900/9650
2020-02-16T14:42:37.6332432Z .....i.............................................................................................. 7000/9650
---
2020-02-16T14:44:12.3011376Z .................................................................................................... 7600/9650
2020-02-16T14:44:17.0934789Z .................................................................................................... 7700/9650
2020-02-16T14:44:23.1477539Z .................................................................................................... 7800/9650
2020-02-16T14:44:29.8711325Z .................................................................................................... 7900/9650
2020-02-16T14:44:39.3337730Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-16T14:44:54.9231829Z ...........................i......i................................................................. 8200/9650
2020-02-16T14:44:59.7193508Z .................................................................................................... 8300/9650
2020-02-16T14:45:10.8410125Z .................................................................................................... 8400/9650
2020-02-16T14:45:22.1839605Z .................................................................................................... 8500/9650
---
2020-02-16T14:47:41.1960125Z  finished in 6.972
2020-02-16T14:47:41.2150262Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:47:41.4065186Z 
2020-02-16T14:47:41.4065855Z running 178 tests
2020-02-16T14:47:44.1950272Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-16T14:47:46.3802638Z ...i..ii...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-16T14:47:46.3807604Z 
2020-02-16T14:47:46.3808701Z  finished in 5.165
2020-02-16T14:47:46.3996733Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:47:46.5591545Z 
---
2020-02-16T14:47:48.3595504Z  finished in 1.959
2020-02-16T14:47:48.3773493Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:47:48.5332255Z 
2020-02-16T14:47:48.5333969Z running 9 tests
2020-02-16T14:47:48.5335081Z iiiiiiiii
2020-02-16T14:47:48.5335539Z 
2020-02-16T14:47:48.5340198Z  finished in 0.156
2020-02-16T14:47:48.5521863Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:47:48.7343175Z 
---
2020-02-16T14:48:07.6052261Z  finished in 19.053
2020-02-16T14:48:07.6257998Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:48:07.8054775Z 
2020-02-16T14:48:07.8055456Z running 116 tests
2020-02-16T14:48:20.5880818Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-16T14:48:22.4478256Z ....iiii.....ii.
2020-02-16T14:48:22.4478920Z 
2020-02-16T14:48:22.4484884Z  finished in 14.822
2020-02-16T14:48:22.4488170Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T14:48:22.4491692Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-16T15:01:21.8070485Z .................................................................................................... 100/2471
2020-02-16T15:01:30.1456407Z ......................................................................ii............................ 200/2471
2020-02-16T15:01:40.1710531Z .................................................................................................... 300/2471
2020-02-16T15:01:50.4876596Z .....i.............................................................................................. 400/2471
2020-02-16T15:01:59.1756123Z ..........................................................i..i..................iiii................ 500/2471
2020-02-16T15:02:14.6578093Z .................................................................................................... 700/2471
2020-02-16T15:02:22.7744554Z .................................................................................................... 800/2471
2020-02-16T15:02:30.8379556Z .................................................................................................... 900/2471
2020-02-16T15:02:38.9227951Z .................................................................................................... 1000/2471
---
2020-02-16T15:03:58.7556729Z .................................................................................................... 2000/2471
2020-02-16T15:04:07.9219496Z .................................................................................................... 2100/2471
2020-02-16T15:04:16.1524655Z .................................................................................................... 2200/2471
2020-02-16T15:04:25.9967743Z .................................................................................................... 2300/2471
2020-02-16T15:04:36.7905690Z .................................................................i................................i. 2400/2471
2020-02-16T15:04:42.0371483Z .................................................................iiiii.
2020-02-16T15:04:42.0371700Z 
2020-02-16T15:04:42.0524747Z  finished in 268.971
2020-02-16T15:04:42.0538548Z Testing panic_abort stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T15:04:42.2125203Z     Finished release [optimized] target(s) in 0.15s
---
2020-02-16T15:06:04.2183148Z 
2020-02-16T15:06:04.2183350Z running 1009 tests
2020-02-16T15:06:21.8428649Z i................................................................................................... 100/1009
2020-02-16T15:06:31.7084061Z .................................................................................................... 200/1009
2020-02-16T15:06:38.6480438Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-16T15:06:43.4857403Z .................................................................................................... 400/1009
2020-02-16T15:06:49.9718189Z ............................................i..i.....................................ii............. 500/1009
2020-02-16T15:07:02.2512538Z .................................................................................................... 700/1009
2020-02-16T15:07:02.2512538Z .................................................................................................... 700/1009
2020-02-16T15:07:08.9040588Z ...................................iiii............................................................. 800/1009
2020-02-16T15:07:22.5530045Z .................................................................................................... 900/1009
2020-02-16T15:07:29.0623665Z .........................................................iiii....................................... 1000/1009
2020-02-16T15:07:29.4528919Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-16T15:07:29.4528962Z 
2020-02-16T15:07:29.4613416Z  finished in 163.690
2020-02-16T15:07:29.4627154Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-16T15:26:36.5191052Z  finished in 38.786
2020-02-16T15:26:36.5450355Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-16T15:26:36.7543570Z 
2020-02-16T15:26:36.7543772Z running 209 tests
2020-02-16T15:27:07.6161092Z ......................i...ii......................................................................i. 100/209
2020-02-16T15:27:43.6638384Z F......................................iiiiii......i..............iii............................... 200/209
2020-02-16T15:27:47.7016448Z failures:
2020-02-16T15:27:47.7025664Z 
2020-02-16T15:27:47.7026239Z ---- [run-make] run-make-fulldeps/libtest-json stdout ----
2020-02-16T15:27:47.7026283Z 
2020-02-16T15:27:47.7026283Z 
2020-02-16T15:27:47.7026356Z error: make failed
2020-02-16T15:27:47.7026402Z status: exit code: 2
2020-02-16T15:27:47.7026444Z command: "make"
2020-02-16T15:27:47.7026525Z stdout:
2020-02-16T15:27:47.7027231Z ------------------------------------------
2020-02-16T15:27:47.7028135Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
2020-02-16T15:27:47.7029269Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json || true
2020-02-16T15:27:47.7030372Z RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json --show-output > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json || true
2020-02-16T15:27:47.7030829Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json | "/usr/bin/python2.7" validate_json.py
2020-02-16T15:27:47.7031216Z cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-stdout-success.json | "/usr/bin/python2.7" validate_json.py
2020-02-16T15:27:47.7031299Z # Compare to output file
2020-02-16T15:27:47.7031655Z diff output-default.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output-default.json
2020-02-16T15:27:47.7031733Z 5c5
2020-02-16T15:27:47.7032140Z < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
2020-02-16T15:27:47.7032363Z ---
2020-02-16T15:27:47.7033181Z > { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', /checkout/src/test/run-make-fulldeps/libtest-json/f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
2020-02-16T15:27:47.7033452Z Makefile:9: recipe for target 'all' failed
2020-02-16T15:27:47.7033923Z ------------------------------------------
2020-02-16T15:27:47.7033972Z stderr:
2020-02-16T15:27:47.7034410Z ------------------------------------------
2020-02-16T15:27:47.7034410Z ------------------------------------------
2020-02-16T15:27:47.7034475Z make: *** [all] Error 1
2020-02-16T15:27:47.7034737Z ------------------------------------------
2020-02-16T15:27:47.7034768Z 
2020-02-16T15:27:47.7034815Z 
2020-02-16T15:27:47.7034840Z 
2020-02-16T15:27:47.7034840Z 
2020-02-16T15:27:47.7034881Z failures:
2020-02-16T15:27:47.7035119Z     [run-make] run-make-fulldeps/libtest-json
2020-02-16T15:27:47.7035169Z 
2020-02-16T15:27:47.7035468Z test result: FAILED. 193 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
2020-02-16T15:27:47.7035511Z 
2020-02-16T15:27:47.7039510Z 
2020-02-16T15:27:47.7039595Z 
2020-02-16T15:27:47.7046327Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T15:27:47.7047198Z 
2020-02-16T15:27:47.7047230Z 
2020-02-16T15:27:47.7047583Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T15:27:47.7047666Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T15:27:47.7047666Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T15:27:47.7100748Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T15:27:47.7100828Z Build completed unsuccessfully in 1:42:53
2020-02-16T15:27:47.7105196Z == clock drift check ==
2020-02-16T15:27:47.7127842Z   local time: Sun Feb 16 15:27:47 UTC 2020
2020-02-16T15:27:47.8538005Z   network time: Sun, 16 Feb 2020 15:27:47 GMT
2020-02-16T15:27:47.8541902Z == end clock drift check ==
2020-02-16T15:27:49.2086202Z 
2020-02-16T15:27:49.2182045Z ##[error]Bash exited with code '1'.
2020-02-16T15:27:49.2195805Z ##[section]Finishing: Run build
2020-02-16T15:27:49.2226326Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T15:27:49.2229284Z Task         : Get sources
2020-02-16T15:27:49.2229330Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T15:27:49.2229376Z Version      : 1.0.0
2020-02-16T15:27:49.2229434Z Author       : Microsoft
2020-02-16T15:27:49.2229434Z Author       : Microsoft
2020-02-16T15:27:49.2229480Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T15:27:49.2229680Z ==============================================================================
2020-02-16T15:27:49.6373798Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T15:27:49.6411915Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T15:27:49.6537632Z Cleaning up task key
2020-02-16T15:27:49.6538333Z Start cleaning up orphan processes.
2020-02-16T15:27:49.6648543Z Terminate orphan process: pid (13099) (python)
2020-02-16T15:27:49.7049927Z ##[section]Finishing: Finalize Job
