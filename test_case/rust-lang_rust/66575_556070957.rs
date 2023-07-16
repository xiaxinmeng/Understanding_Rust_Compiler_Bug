plain
2019-11-20T14:05:37.0896965Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T14:05:37.1103089Z ##[command]git config gc.auto 0
2019-11-20T14:05:37.1184709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T14:05:37.1245141Z ##[command]git config --get-all http.proxy
2019-11-20T14:05:37.1384467Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66575/merge:refs/remotes/pull/66575/merge
---
2019-11-20T15:04:59.7154288Z .................................................................................................... 1500/9257
2019-11-20T15:05:05.6599014Z .................................................................................................... 1600/9257
2019-11-20T15:05:14.3515148Z .................................................................................................... 1700/9257
2019-11-20T15:05:23.3892107Z ........i........................................................................................... 1800/9257
2019-11-20T15:05:29.9636702Z .............................................................................................iiiii.. 1900/9257
2019-11-20T15:05:51.4242626Z .................................................................................................... 2100/9257
2019-11-20T15:05:53.8155726Z .................................................................................................... 2200/9257
2019-11-20T15:05:56.3378425Z .................................................................................................... 2300/9257
2019-11-20T15:06:02.4669425Z .................................................................................................... 2400/9257
---
2019-11-20T15:08:57.7955964Z ..............................................................................................i..... 4700/9257
2019-11-20T15:09:04.2931204Z ..........i......................................................................................... 4800/9257
2019-11-20T15:09:14.1738230Z .................................................................................................... 4900/9257
2019-11-20T15:09:19.2540848Z .................................................................................................... 5000/9257
2019-11-20T15:09:29.7289296Z ...................................................................................................i 5100/9257
2019-11-20T15:09:35.3407148Z i.ii...........i.................................................................................... 5200/9257
2019-11-20T15:09:45.9411148Z .................................................................................................... 5400/9257
2019-11-20T15:09:56.5203851Z .................................................................................i.................. 5500/9257
2019-11-20T15:10:04.3599195Z .................................................................................................... 5600/9257
2019-11-20T15:10:10.6188316Z .................................................................................................... 5700/9257
2019-11-20T15:10:10.6188316Z .................................................................................................... 5700/9257
2019-11-20T15:10:20.8416526Z ...................................................................ii...i..ii...........i........... 5800/9257
2019-11-20T15:10:43.1592354Z .................................................................................................... 6000/9257
2019-11-20T15:10:51.5641241Z .................................................................................................... 6100/9257
2019-11-20T15:10:51.5641241Z .................................................................................................... 6100/9257
2019-11-20T15:10:55.8427499Z ........................................................................................i..ii....... 6200/9257
2019-11-20T15:11:19.9127841Z .................................................................................................... 6400/9257
2019-11-20T15:11:28.2222116Z ........................................................i........................................... 6500/9257
2019-11-20T15:11:30.5067332Z .................................................................................................... 6600/9257
2019-11-20T15:11:32.9559322Z .............................................i...................................................... 6700/9257
---
2019-11-20T15:16:48.0611354Z  finished in 5.777
2019-11-20T15:16:48.0789039Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:16:48.2644700Z 
2019-11-20T15:16:48.2644934Z running 156 tests
2019-11-20T15:16:51.1503525Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-20T15:16:53.1421671Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-20T15:16:53.1422977Z 
2019-11-20T15:16:53.1426824Z  finished in 5.063
2019-11-20T15:16:53.1618390Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:16:53.3408273Z 
---
2019-11-20T15:16:55.3479203Z  finished in 2.186
2019-11-20T15:16:55.3791133Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:16:55.5323685Z 
2019-11-20T15:16:55.5323934Z running 9 tests
2019-11-20T15:16:55.5325301Z iiiiiiiii
2019-11-20T15:16:55.5325822Z 
2019-11-20T15:16:55.5326437Z  finished in 0.153
2019-11-20T15:16:55.5506272Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:16:55.7217264Z 
---
2019-11-20T15:17:14.8247261Z  finished in 19.274
2019-11-20T15:17:14.8472173Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:17:15.0321125Z 
2019-11-20T15:17:15.0321359Z running 123 tests
2019-11-20T15:17:39.6561938Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-20T15:17:44.3182284Z i.i.i......iii.i.....ii
2019-11-20T15:17:44.3183279Z 
2019-11-20T15:17:44.3186232Z  finished in 29.471
2019-11-20T15:17:44.3197624Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:17:44.3199489Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-20T15:30:01.8381398Z 
2019-11-20T15:30:01.8386429Z    Doc-tests core
2019-11-20T15:30:07.0843991Z 
2019-11-20T15:30:07.0845072Z running 2421 tests
2019-11-20T15:30:17.6663769Z ......iiiii......................................................................................... 100/2421
2019-11-20T15:30:28.0221226Z .................................................................................ii................. 200/2421
2019-11-20T15:30:52.3675776Z ...i................................................................................................ 400/2421
2019-11-20T15:30:52.3675776Z ...i................................................................................................ 400/2421
2019-11-20T15:31:02.5697593Z ...................................................i..i..................iiii....................... 500/2421
2019-11-20T15:31:21.5900206Z .................................................................................................... 700/2421
2019-11-20T15:31:31.7643218Z .................................................................................................... 800/2421
2019-11-20T15:31:41.7597100Z .................................................................................................... 900/2421
2019-11-20T15:31:51.8385983Z .................................................................................................... 1000/2421
---
2019-11-20T15:35:46.4722683Z ............................................... 300/763
2019-11-20T15:35:46.4730779Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-11-20T15:35:46.5640039Z .................................................................................................... 400/763
2019-11-20T15:35:48.6426748Z .................................................................................................... 500/763
2019-11-20T15:35:48.6706720Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-20T15:35:48.6735151Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1187:5
2019-11-20T15:35:48.6767242Z .<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs.:.1187:5...
2019-11-20T15:35:48.6785859Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-20T15:35:48.8953997Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-20T15:35:48.8977197Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-20T15:35:48.9006966Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1187:5
2019-11-20T15:35:50.9954483Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-11-20T15:35:50.9957156Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-11-20T15:35:50.9959795Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-11-20T15:35:50.9971013Z .........thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-11-20T15:36:00.2333728Z 
2019-11-20T15:36:00.2333976Z running 999 tests
2019-11-20T15:36:19.7414162Z i................................................................................................... 100/999
2019-11-20T15:36:30.7849010Z .................................................................................................... 200/999
2019-11-20T15:36:38.6101503Z ..................iii......i......i...i......i...................................................... 300/999
2019-11-20T15:36:43.8718673Z .................................................................................................... 400/999
2019-11-20T15:36:51.3856670Z ..........................................i..i.................................ii................... 500/999
2019-11-20T15:37:05.4038941Z .................................................................................................... 700/999
2019-11-20T15:37:05.4038941Z .................................................................................................... 700/999
2019-11-20T15:37:12.7509717Z .........................iiii....................................................................... 800/999
2019-11-20T15:37:27.5274027Z .................................................................................................... 900/999
2019-11-20T15:37:34.7366348Z ...............................................iiii................................................
2019-11-20T15:37:34.7369883Z 
2019-11-20T15:37:34.7466598Z  finished in 190.410
2019-11-20T15:37:34.7481946Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:37:34.9712656Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-20T15:55:50.4413180Z  finished in 41.807
2019-11-20T15:55:50.4824111Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T15:55:50.6910249Z 
2019-11-20T15:55:50.6910425Z running 204 tests
2019-11-20T15:56:20.9080041Z ....................i...ii...................................................................i...... 100/204
2019-11-20T15:56:54.0947782Z .................................iiii...F...i............iiii.iii................................... 200/204
2019-11-20T15:56:55.0638085Z .i..
2019-11-20T15:56:55.0646502Z 
2019-11-20T15:56:55.0646993Z ---- [run-make] run-make-fulldeps/pretty-print-path-suffix stdout ----
2019-11-20T15:56:55.0647032Z 
2019-11-20T15:56:55.0647072Z error: make failed
2019-11-20T15:56:55.0647072Z error: make failed
2019-11-20T15:56:55.0647137Z status: exit code: 2
2019-11-20T15:56:55.0647174Z command: "make"
2019-11-20T15:56:55.0647209Z stdout:
2019-11-20T15:56:55.0647436Z ------------------------------------------
2019-11-20T15:56:55.0648353Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-print-path-suffix/pretty-print-path-suffix/foo.out -Z unpretty=hir=foo input.rs
2019-11-20T15:56:55.0648646Z Makefile:4: recipe for target 'all' failed
2019-11-20T15:56:55.0648887Z ------------------------------------------
2019-11-20T15:56:55.0649072Z stderr:
2019-11-20T15:56:55.0649307Z ------------------------------------------
2019-11-20T15:56:55.0649307Z ------------------------------------------
2019-11-20T15:56:55.0649681Z error: argument to `unpretty` must be one of `normal`, `expanded`, `identified`, `expanded,identified`, `expanded,hygiene`, `everybody_loops`, `hir`, `hir,identified`, `hir,typed`, `hir-tree`, `mir` or `mir-cfg`; got hir=foo
2019-11-20T15:56:55.0649727Z 
2019-11-20T15:56:55.0649767Z make: *** [all] Error 1
2019-11-20T15:56:55.0650232Z ------------------------------------------
2019-11-20T15:56:55.0650266Z 
2019-11-20T15:56:55.0650289Z 
2019-11-20T15:56:55.0650322Z 
---
2019-11-20T15:56:55.0656592Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-20T15:56:55.0656702Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-20T15:56:55.0662373Z 
2019-11-20T15:56:55.0662447Z 
2019-11-20T15:56:55.0673786Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-20T15:56:55.0674901Z 
2019-11-20T15:56:55.0674929Z 
2019-11-20T15:56:55.0678235Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-20T15:56:55.0678319Z Build completed unsuccessfully in 1:45:15
2019-11-20T15:56:55.0678319Z Build completed unsuccessfully in 1:45:15
2019-11-20T15:56:55.0730646Z == clock drift check ==
2019-11-20T15:56:55.0905445Z   local time: Wed Nov 20 15:56:55 UTC 2019
2019-11-20T15:56:55.5365407Z   network time: Wed, 20 Nov 2019 15:56:55 GMT
2019-11-20T15:56:55.5367641Z == end clock drift check ==
2019-11-20T15:57:03.2628437Z 
2019-11-20T15:57:03.2719929Z ##[error]Bash exited with code '1'.
2019-11-20T15:57:03.2764477Z ##[section]Starting: Checkout
2019-11-20T15:57:03.2766260Z ==============================================================================
2019-11-20T15:57:03.2766313Z Task         : Get sources
2019-11-20T15:57:03.2766370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
