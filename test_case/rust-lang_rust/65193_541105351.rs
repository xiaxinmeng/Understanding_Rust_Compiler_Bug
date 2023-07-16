plain
2019-10-11T13:13:49.0951941Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T13:13:49.1032016Z ##[command]git config gc.auto 0
2019-10-11T13:13:49.1037863Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T13:13:49.1046408Z ##[command]git config --get-all http.proxy
2019-10-11T13:13:49.1050224Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-11T14:18:04.6164916Z .................................................................................................... 1600/9146
2019-10-11T14:18:12.4236736Z .................................................................................................... 1700/9146
2019-10-11T14:18:24.6739417Z .................i...............i.................................................................. 1800/9146
2019-10-11T14:18:32.3809092Z .................................................................................................... 1900/9146
2019-10-11T14:18:48.3819772Z ........iiiii....................................................................................... 2000/9146
2019-10-11T14:18:58.8740391Z .................................................................................................... 2200/9146
2019-10-11T14:19:01.7351983Z .................................................................................................... 2300/9146
2019-10-11T14:19:07.8683862Z .................................................................................................... 2400/9146
2019-10-11T14:19:14.5954646Z .................................................................................................... 2500/9146
---
2019-10-11T14:22:18.3702764Z .................................................................................................... 4700/9146
2019-10-11T14:22:26.0497003Z .i...............i.................................................................................. 4800/9146
2019-10-11T14:22:38.1484475Z .................................................................................................... 4900/9146
2019-10-11T14:22:44.1936639Z .................................................................................................... 5000/9146
2019-10-11T14:22:56.0471564Z ...............................................................................................ii.ii 5100/9146
2019-10-11T14:23:07.3254387Z .................................................................................................... 5300/9146
2019-10-11T14:23:17.7744451Z .................................................................................................... 5400/9146
2019-10-11T14:23:25.1045576Z .............................................................i...................................... 5500/9146
2019-10-11T14:23:32.8748747Z .................................................................................................... 5600/9146
2019-10-11T14:23:32.8748747Z .................................................................................................... 5600/9146
2019-10-11T14:23:40.9573345Z .................................................................................................... 5700/9146
2019-10-11T14:23:52.1714364Z ..........................................................ii...i..ii...........i.................... 5800/9146
2019-10-11T14:24:19.1270638Z .................................................................................................... 6000/9146
2019-10-11T14:24:28.9110838Z .................................................................................................... 6100/9146
2019-10-11T14:24:28.9110838Z .................................................................................................... 6100/9146
2019-10-11T14:24:39.3477046Z ................................................................i..ii............................... 6200/9146
2019-10-11T14:25:10.0637549Z .................................................................................................... 6400/9146
2019-10-11T14:25:12.3702520Z ........................i........................................................................... 6500/9146
2019-10-11T14:25:14.7708520Z .................................................................................................i.. 6600/9146
2019-10-11T14:25:17.7066966Z .................................................................................................... 6700/9146
---
2019-10-11T14:30:14.0810199Z  finished in 6.167
2019-10-11T14:30:14.1014652Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:30:14.2859724Z 
2019-10-11T14:30:14.2859890Z running 153 tests
2019-10-11T14:30:17.8186842Z i....iii......iii..iiii....i.............................i..i..................i....i............ii. 100/153
2019-10-11T14:30:19.9837532Z i.i..iiii..............i.........iii.i.......ii......
2019-10-11T14:30:19.9840143Z 
2019-10-11T14:30:19.9842443Z  finished in 5.882
2019-10-11T14:30:20.0025983Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:30:20.1633168Z 
---
2019-10-11T14:30:22.4449128Z  finished in 2.442
2019-10-11T14:30:22.4651577Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:30:22.6329338Z 
2019-10-11T14:30:22.6330693Z running 9 tests
2019-10-11T14:30:22.6331902Z iiiiiiiii
2019-10-11T14:30:22.6333406Z 
2019-10-11T14:30:22.6333664Z  finished in 0.167
2019-10-11T14:30:22.6540762Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:30:22.8429501Z 
---
2019-10-11T14:30:42.8832183Z  finished in 19.696
2019-10-11T14:30:42.8832577Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:30:42.8832655Z 
2019-10-11T14:30:42.8832708Z running 123 tests
2019-10-11T14:31:08.7874787Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-11T14:31:13.8440028Z i.i.i......iii.i.....ii
2019-10-11T14:31:13.8441495Z 
2019-10-11T14:31:13.8444660Z  finished in 31.471
2019-10-11T14:31:13.8448988Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:31:13.8452765Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-11T14:45:17.5001725Z 
2019-10-11T14:45:17.5003010Z    Doc-tests core
2019-10-11T14:45:22.8520456Z 
2019-10-11T14:45:22.8529176Z running 2405 tests
2019-10-11T14:45:35.1923729Z ......iiiii......................................................................................... 100/2405
2019-10-11T14:45:47.1489264Z ...............................................................................ii................... 200/2405
2019-10-11T14:46:15.0930620Z .i.................................................................................................. 400/2405
2019-10-11T14:46:15.0930620Z .i.................................................................................................. 400/2405
2019-10-11T14:46:26.5995965Z ................................................i..i.................iiii........................... 500/2405
2019-10-11T14:46:48.8270535Z .................................................................................................... 700/2405
2019-10-11T14:47:00.1754113Z .................................................................................................... 800/2405
2019-10-11T14:47:11.8461087Z .................................................................................................... 900/2405
2019-10-11T14:47:23.3180169Z .................................................................................................... 1000/2405
---
2019-10-11T14:51:57.2438966Z 
2019-10-11T14:51:57.2440002Z running 994 tests
2019-10-11T14:52:19.9514604Z i................................................................................................... 100/994
2019-10-11T14:52:32.8484026Z .................................................................................................... 200/994
2019-10-11T14:52:42.1071440Z ...................iii......i......i...i......i..................................................... 300/994
2019-10-11T14:52:48.4375486Z .................................................................................................... 400/994
2019-10-11T14:52:57.0400003Z .....................................i..i.................................ii........................ 500/994
2019-10-11T14:53:13.3933587Z .................................................................................................... 700/994
2019-10-11T14:53:13.3933587Z .................................................................................................... 700/994
2019-10-11T14:53:22.2920802Z ....................iiii............................................................................ 800/994
2019-10-11T14:53:38.7889749Z .................................................................................................... 900/994
2019-10-11T14:53:47.2439312Z ..........................................iiii................................................
2019-10-11T14:53:47.2439810Z 
2019-10-11T14:53:47.2503097Z  finished in 211.346
2019-10-11T14:53:47.2521487Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T14:53:47.4666923Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-11T15:11:00.0152581Z  finished in 43.065
2019-10-11T15:11:00.0495312Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-11T15:11:00.2749553Z 
2019-10-11T15:11:00.2749864Z running 202 tests
2019-10-11T15:11:36.2259581Z ....................i...ii..........................................F......................i........ 100/202
2019-10-11T15:12:31.1338924Z ................................iiii.......i...........iiii.iii....................................i 200/202
2019-10-11T15:12:31.4876601Z failures:
2019-10-11T15:12:31.4887773Z 
2019-10-11T15:12:31.4887773Z 
2019-10-11T15:12:31.4888426Z ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
2019-10-11T15:12:31.4888544Z error: make failed
2019-10-11T15:12:31.4888588Z status: exit code: 2
2019-10-11T15:12:31.4888629Z command: "make"
2019-10-11T15:12:31.4888689Z stdout:
2019-10-11T15:12:31.4888689Z stdout:
2019-10-11T15:12:31.4889043Z ------------------------------------------
2019-10-11T15:12:31.4890226Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
2019-10-11T15:12:31.4892540Z Makefile:8: recipe for target 'all' failed
2019-10-11T15:12:31.4893264Z ------------------------------------------
2019-10-11T15:12:31.4893332Z stderr:
2019-10-11T15:12:31.4893560Z ------------------------------------------
2019-10-11T15:12:31.4894304Z error[E0063]: missing field `register_lints` in initializer of `rustc_interface::Config`
---
2019-10-11T15:12:31.4894976Z 
2019-10-11T15:12:31.4895019Z error: aborting due to previous error
2019-10-11T15:12:31.4895047Z 
2019-10-11T15:12:31.4895662Z For more information about this error, try `rustc --explain E0063`.
2019-10-11T15:12:31.4895730Z make: *** [all] Error 1
2019-10-11T15:12:31.4897228Z ------------------------------------------
2019-10-11T15:12:31.4897277Z 
2019-10-11T15:12:31.4897301Z 
2019-10-11T15:12:31.4897323Z 
---
2019-10-11T15:12:31.4899831Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-11T15:12:31.4899899Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T15:12:31.4903487Z 
2019-10-11T15:12:31.4909966Z 
2019-10-11T15:12:31.4915837Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-11T15:12:31.4916957Z 
2019-10-11T15:12:31.4916992Z 
2019-10-11T15:12:31.4921110Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-11T15:12:31.4922271Z Build completed unsuccessfully in 1:51:53
2019-10-11T15:12:31.4922271Z Build completed unsuccessfully in 1:51:53
2019-10-11T15:12:31.4979371Z == clock drift check ==
2019-10-11T15:12:31.5007811Z   local time: Fri Oct 11 15:12:31 UTC 2019
2019-10-11T15:12:31.5874113Z   network time: Fri, 11 Oct 2019 15:12:31 GMT
2019-10-11T15:12:31.5879150Z == end clock drift check ==
2019-10-11T15:12:32.1144996Z ##[error]Bash exited with code '1'.
2019-10-11T15:12:32.1198580Z ##[section]Starting: Checkout
2019-10-11T15:12:32.1201356Z ==============================================================================
2019-10-11T15:12:32.1201425Z Task         : Get sources
2019-10-11T15:12:32.1201477Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
