plain
2020-01-10T00:20:43.8480735Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T00:20:43.8576122Z ##[command]git config gc.auto 0
2020-01-10T00:20:43.8646930Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T00:20:43.8688889Z ##[command]git config --get-all http.proxy
2020-01-10T00:20:43.8814324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67711/merge:refs/remotes/pull/67711/merge
---
2020-01-10T01:07:25.7983076Z .................................................................................................... 1600/9487
2020-01-10T01:07:30.7031544Z .................................................................................................... 1700/9487
2020-01-10T01:07:39.1451968Z .................................................................................................... 1800/9487
2020-01-10T01:07:46.9257611Z .....i.............................................................................................. 1900/9487
2020-01-10T01:07:52.9722074Z ...............................................................................................iiiii 2000/9487
2020-01-10T01:08:12.7608291Z .................................................................................................... 2200/9487
2020-01-10T01:08:14.9259414Z .................................................................................................... 2300/9487
2020-01-10T01:08:17.1519289Z .................................................................................................... 2400/9487
2020-01-10T01:08:22.4126929Z .................................................................................................... 2500/9487
---
2020-01-10T01:11:04.9173087Z ...........................i...............i........................................................ 4900/9487
2020-01-10T01:11:14.3335542Z .................................................................................................... 5000/9487
2020-01-10T01:11:19.8320261Z ........................................................................i........................... 5100/9487
2020-01-10T01:11:25.7590735Z .................................................................................................... 5200/9487
2020-01-10T01:11:34.3873876Z .......................................ii.ii...........i............................................ 5300/9487
2020-01-10T01:11:43.3056142Z .................................................................................................... 5500/9487
2020-01-10T01:11:51.9298221Z .................................................................................................... 5600/9487
2020-01-10T01:11:58.6585099Z .......................i............................................................................ 5700/9487
2020-01-10T01:12:04.4706570Z .................................................................................................... 5800/9487
2020-01-10T01:12:04.4706570Z .................................................................................................... 5800/9487
2020-01-10T01:12:14.7216727Z .................................................................................................... 5900/9487
2020-01-10T01:12:24.2650405Z ..............ii...i..ii...........i................................................................ 6000/9487
2020-01-10T01:12:40.6855815Z .................................................................................................... 6200/9487
2020-01-10T01:12:47.8842858Z .................................................................................................... 6300/9487
2020-01-10T01:12:47.8842858Z .................................................................................................... 6300/9487
2020-01-10T01:13:03.4774909Z .........................................i..ii...................................................... 6400/9487
2020-01-10T01:13:22.2573684Z .................................................................................................... 6600/9487
2020-01-10T01:13:24.1586841Z ................i................................................................................... 6700/9487
2020-01-10T01:13:26.2773792Z .................................................................................................... 6800/9487
2020-01-10T01:13:28.5689188Z ................i................................................................................... 6900/9487
---
2020-01-10T01:14:56.2017050Z .................................................................................................... 7500/9487
2020-01-10T01:14:59.9743607Z .................................................................................................... 7600/9487
2020-01-10T01:15:05.0722974Z .................................................................................................... 7700/9487
2020-01-10T01:15:14.2772184Z .................................................................................................... 7800/9487
2020-01-10T01:15:23.0155922Z .....................................................iiii........................................... 7900/9487
2020-01-10T01:15:36.8606262Z .................................................................................................... 8100/9487
2020-01-10T01:15:42.0063001Z .................................................................................................... 8200/9487
2020-01-10T01:15:56.7930601Z .................................................................................................... 8300/9487
2020-01-10T01:16:03.9163546Z .................................................................................................... 8400/9487
---
2020-01-10T01:18:11.2799490Z  finished in 6.315
2020-01-10T01:18:11.2991531Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:18:11.5512928Z 
2020-01-10T01:18:11.5513893Z running 166 tests
2020-01-10T01:18:14.2977025Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T01:18:16.2909563Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T01:18:16.2910029Z 
2020-01-10T01:18:16.2910076Z  finished in 4.991
2020-01-10T01:18:16.3114395Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:18:16.4713804Z 
---
2020-01-10T01:18:18.1958291Z  finished in 1.884
2020-01-10T01:18:18.2133709Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:18:18.3603306Z 
2020-01-10T01:18:18.3603536Z running 9 tests
2020-01-10T01:18:18.3604385Z iiiiiiiii
2020-01-10T01:18:18.3604803Z 
2020-01-10T01:18:18.3604850Z  finished in 0.146
2020-01-10T01:18:18.3781204Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:18:18.6286935Z 
---
2020-01-10T01:18:36.3960450Z  finished in 18.018
2020-01-10T01:18:36.4191831Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:18:36.6353904Z 
2020-01-10T01:18:36.6354154Z running 124 tests
2020-01-10T01:18:57.5754468Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T01:19:01.3674349Z .i.iii.....iiiiii.....ii
2020-01-10T01:19:01.3675016Z 
2020-01-10T01:19:01.3680238Z  finished in 24.949
2020-01-10T01:19:01.3686533Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:19:01.3687192Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-10T01:30:31.1843464Z 
2020-01-10T01:30:31.1844173Z    Doc-tests core
2020-01-10T01:30:35.3963077Z 
2020-01-10T01:30:35.3963743Z running 2441 tests
2020-01-10T01:30:44.7625635Z ......iiiii......................................................................................... 100/2441
2020-01-10T01:30:52.2660190Z ..................................................................................ii................ 200/2441
2020-01-10T01:31:12.2875019Z ................i................................................................................... 400/2441
2020-01-10T01:31:12.2875019Z ................i................................................................................... 400/2441
2020-01-10T01:31:21.3814939Z .................................................................i..i..................iiii......... 500/2441
2020-01-10T01:31:37.0646075Z .................................................................................................... 700/2441
2020-01-10T01:31:45.1580533Z .................................................................................................... 800/2441
2020-01-10T01:31:53.1438239Z .................................................................................................... 900/2441
2020-01-10T01:32:01.1990340Z .................................................................................................... 1000/2441
---
2020-01-10T01:35:09.6117940Z .......thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:606:13
2020-01-10T01:35:09.6118564Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:619:13
2020-01-10T01:35:11.6581547Z ......................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:229:13
2020-01-10T01:35:11.6627565Z ........................... 700/760
2020-01-10T01:35:11.6711791Z ................................thread '.<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1573:37
2020-01-10T01:35:12.2775267Z ...........thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1708:13
2020-01-10T01:35:12.2782409Z .thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1692:13
2020-01-10T01:35:12.2782775Z .thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1676:13
2020-01-10T01:35:12.2786597Z ..thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1727:37
2020-01-10T01:35:17.7863997Z test result: ok. 760 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-10T01:35:17.7864168Z 
2020-01-10T01:35:17.7873878Z      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/env-13ea053ab0dca3ec
2020-01-10T01:35:17.7904009Z 
---
2020-01-10T01:35:18.8293393Z 
2020-01-10T01:35:18.8300875Z running 1003 tests
2020-01-10T01:35:35.3848759Z i................................................................................................... 100/1003
2020-01-10T01:35:45.3419727Z .................................................................................................... 200/1003
2020-01-10T01:35:51.4098200Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-10T01:35:55.8735225Z .................................................................................................... 400/1003
2020-01-10T01:36:02.0586994Z ..........................................i..i.....................................ii............... 500/1003
2020-01-10T01:36:13.7583500Z .................................................................................................... 700/1003
2020-01-10T01:36:13.7583500Z .................................................................................................... 700/1003
2020-01-10T01:36:19.6184620Z .............................iiii................................................................... 800/1003
2020-01-10T01:36:32.3391853Z .................................................................................................... 900/1003
2020-01-10T01:36:38.6291720Z ...................................................iiii............................................. 1000/1003
2020-01-10T01:36:38.6992769Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-10T01:36:38.6993014Z 
2020-01-10T01:36:38.7104182Z  finished in 153.798
2020-01-10T01:36:38.7114294Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-10T01:52:46.7480193Z  finished in 35.479
2020-01-10T01:52:46.7796353Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T01:52:47.0821648Z 
2020-01-10T01:52:47.0821923Z running 206 tests
2020-01-10T01:53:15.7144749Z ...............F....i...ii....................................................................i..... 100/206
2020-01-10T01:53:50.5568801Z ..................................iiiiii......i............iiii.iii................................. 200/206
2020-01-10T01:53:50.9925169Z failures:
2020-01-10T01:53:50.9932845Z 
2020-01-10T01:53:50.9933337Z ---- [run-make] run-make-fulldeps/cdylib-fewer-symbols stdout ----
2020-01-10T01:53:50.9933397Z 
2020-01-10T01:53:50.9933397Z 
2020-01-10T01:53:50.9933434Z error: make failed
2020-01-10T01:53:50.9933467Z status: exit code: 2
2020-01-10T01:53:50.9933708Z command: "make" "make"
2020-01-10T01:53:50.9934109Z stdout:
2020-01-10T01:53:50.9934539Z ------------------------------------------
2020-01-10T01:53:50.9935418Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cdylib-fewer-symbols/cdylib-fewer-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cdylib-fewer-symbols/cdylib-fewer-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cdylib-fewer-symbols/cdylib-fewer-symbols  foo.rs
2020-01-10T01:53:50.9935971Z nm -g "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cdylib-fewer-symbols/cdylib-fewer-symbols/libfoo.so" | "/checkout/src/etc/cat-and-grep.sh" -v __rdl_ __rde_ __rg_ __rust_
2020-01-10T01:53:50.9936173Z [[[ begin stdout ]]]
2020-01-10T01:53:50.9936533Z                  w _ITM_deregisterTMCloneTable
2020-01-10T01:53:50.9936709Z                  w _ITM_registerTMCloneTable
2020-01-10T01:53:50.9936831Z                  U _Unwind_Backtrace@@GCC_3.3
2020-01-10T01:53:50.9936894Z                  U _Unwind_GetDataRelBase@@GCC_3.0
2020-01-10T01:53:50.9936945Z                  U _Unwind_GetIP@@GCC_3.0
2020-01-10T01:53:50.9937016Z                  U _Unwind_GetIPInfo@@GCC_4.2.0
2020-01-10T01:53:50.9937239Z                  U _Unwind_GetLanguageSpecificData@@GCC_3.0
2020-01-10T01:53:50.9937289Z                  U _Unwind_GetRegionStart@@GCC_3.0
2020-01-10T01:53:50.9937464Z                  U _Unwind_GetTextRelBase@@GCC_3.0
2020-01-10T01:53:50.9937604Z                  U _Unwind_RaiseException@@GCC_3.0
2020-01-10T01:53:50.9937674Z                  U _Unwind_Resume@@GCC_3.0
2020-01-10T01:53:50.9937725Z                  U _Unwind_SetGR@@GCC_3.0
2020-01-10T01:53:50.9937759Z                  U _Unwind_SetIP@@GCC_3.0
2020-01-10T01:53:50.9937830Z                  w __cxa_finalize@@GLIBC_2.2.5
2020-01-10T01:53:50.9937882Z                  w __cxa_thread_atexit_impl@@GLIBC_2.18
2020-01-10T01:53:50.9937932Z                  U __errno_location@@GLIBC_2.2.5
2020-01-10T01:53:50.9938196Z                  U __fxstat@@GLIBC_2.2.5
2020-01-10T01:53:50.9938272Z                  w __gmon_start__
2020-01-10T01:53:50.9938323Z 0000000000008400 T __rust_drop_panic
2020-01-10T01:53:50.9938400Z                  U __snprintf_chk@@GLIBC_2.3.4
2020-01-10T01:53:50.9938451Z                  U __stack_chk_fail@@GLIBC_2.4
2020-01-10T01:53:50.9938517Z                  U __tls_get_addr@@GLIBC_2.3
2020-01-10T01:53:50.9938569Z                  U __xpg_strerror_r@@GLIBC_2.3.4
2020-01-10T01:53:50.9938618Z                  U abort@@GLIBC_2.2.5
2020-01-10T01:53:50.9938688Z                  U close@@GLIBC_2.2.5
2020-01-10T01:53:50.9938738Z                  U dl_iterate_phdr@@GLIBC_2.2.5
2020-01-10T01:53:50.9938960Z                  U dladdr@@GLIBC_2.2.5
2020-01-10T01:53:50.9939042Z 00000000000039a0 T foo
2020-01-10T01:53:50.9939078Z                  U free@@GLIBC_2.2.5
2020-01-10T01:53:50.9939112Z                  U getcwd@@GLIBC_2.2.5
2020-01-10T01:53:50.9939164Z                  U getenv@@GLIBC_2.2.5
2020-01-10T01:53:50.9939205Z                  U getpid@@GLIBC_2.2.5
2020-01-10T01:53:50.9939239Z                  U lseek@@GLIBC_2.2.5
2020-01-10T01:53:50.9939289Z                  U malloc@@GLIBC_2.2.5
2020-01-10T01:53:50.9939329Z                  U memchr@@GLIBC_2.2.5
2020-01-10T01:53:50.9939362Z                  U memcmp@@GLIBC_2.2.5
2020-01-10T01:53:50.9939396Z                  U memcpy@@GLIBC_2.14
2020-01-10T01:53:50.9939446Z                  U memset@@GLIBC_2.2.5
2020-01-10T01:53:50.9939480Z                  U open@@GLIBC_2.2.5
2020-01-10T01:53:50.9939515Z                  U posix_memalign@@GLIBC_2.2.5
2020-01-10T01:53:50.9939704Z                  U pthread_cond_destroy@@GLIBC_2.3.2
2020-01-10T01:53:50.9939743Z                  U pthread_cond_init@@GLIBC_2.3.2
2020-01-10T01:53:50.9939833Z                  U pthread_condattr_destroy@@GLIBC_2.2.5
2020-01-10T01:53:50.9939907Z                  U pthread_condattr_init@@GLIBC_2.2.5
2020-01-10T01:53:50.9939964Z                  U pthread_condattr_setclock@@GLIBC_2.3.3
2020-01-10T01:53:50.9940042Z                  U pthread_getspecific@@GLIBC_2.2.5
2020-01-10T01:53:50.9940096Z                  U pthread_key_create@@GLIBC_2.2.5
2020-01-10T01:53:50.9940149Z                  U pthread_key_delete@@GLIBC_2.2.5
2020-01-10T01:53:50.9940227Z                  U pthread_mutex_destroy@@GLIBC_2.2.5
2020-01-10T01:53:50.9940281Z                  U pthread_mutex_init@@GLIBC_2.2.5
2020-01-10T01:53:50.9940334Z                  U pthread_mutex_lock@@GLIBC_2.2.5
2020-01-10T01:53:50.9940405Z                  U pthread_mutex_unlock@@GLIBC_2.2.5
2020-01-10T01:53:50.9940443Z                  U pthread_mutexattr_destroy@@GLIBC_2.2.5
2020-01-10T01:53:50.9940500Z                  U pthread_mutexattr_init@@GLIBC_2.2.5
2020-01-10T01:53:50.9940575Z                  U pthread_mutexattr_settype@@GLIBC_2.2.5
2020-01-10T01:53:50.9940639Z                  U pthread_rwlock_rdlock@@GLIBC_2.2.5
2020-01-10T01:53:50.9940679Z                  U pthread_rwlock_unlock@@GLIBC_2.2.5
2020-01-10T01:53:50.9940844Z                  U pthread_setspecific@@GLIBC_2.2.5
2020-01-10T01:53:50.9941123Z                  U read@@GLIBC_2.2.5
2020-01-10T01:53:50.9941163Z                  U realloc@@GLIBC_2.2.5
2020-01-10T01:53:50.9941324Z 0000000000009a00 T rust_eh_personality
2020-01-10T01:53:50.9941543Z                  U strcmp@@GLIBC_2.2.5
2020-01-10T01:53:50.9941609Z                  U strlen@@GLIBC_2.2.5
2020-01-10T01:53:50.9941766Z                  U strncmp@@GLIBC_2.2.5
2020-01-10T01:53:50.9941828Z                  U strrchr@@GLIBC_2.2.5
2020-01-10T01:53:50.9941885Z                  U write@@GLIBC_2.2.5
2020-01-10T01:53:50.9941944Z                  U writev@@GLIBC_2.2.5
2020-01-10T01:53:50.9942387Z 
2020-01-10T01:53:50.9942444Z [[[ end stdout ]]]
2020-01-10T01:53:50.9942794Z Error: should not match: __rust_
2020-01-10T01:53:50.9943133Z Makefile:11: recipe for target 'all' failed
2020-01-10T01:53:50.9943541Z ------------------------------------------
2020-01-10T01:53:50.9943695Z stderr:
2020-01-10T01:53:50.9944080Z ------------------------------------------
2020-01-10T01:53:50.9944080Z ------------------------------------------
2020-01-10T01:53:50.9944121Z make: *** [all] Error 1
2020-01-10T01:53:50.9944486Z ------------------------------------------
2020-01-10T01:53:50.9944658Z 
2020-01-10T01:53:50.9944734Z 
2020-01-10T01:53:50.9944777Z 
---
2020-01-10T01:53:50.9950110Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T01:53:50.9950194Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T01:53:50.9956614Z 
2020-01-10T01:53:50.9956806Z 
2020-01-10T01:53:50.9961949Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-7/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-7/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T01:53:50.9963238Z 
2020-01-10T01:53:50.9963327Z 
2020-01-10T01:53:50.9968152Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T01:53:50.9968230Z Build completed unsuccessfully in 1:28:08
2020-01-10T01:53:50.9968230Z Build completed unsuccessfully in 1:28:08
2020-01-10T01:53:51.0022329Z == clock drift check ==
2020-01-10T01:53:51.0041486Z   local time: Fri Jan 10 01:53:51 UTC 2020
2020-01-10T01:53:51.1047877Z   network time: Fri, 10 Jan 2020 01:53:51 GMT
2020-01-10T01:53:51.1051149Z == end clock drift check ==
2020-01-10T01:53:52.1838151Z 
2020-01-10T01:53:52.1955867Z ##[error]Bash exited with code '1'.
2020-01-10T01:53:52.1998567Z ##[section]Starting: Checkout
2020-01-10T01:53:52.2000000Z ==============================================================================
2020-01-10T01:53:52.2000041Z Task         : Get sources
2020-01-10T01:53:52.2000093Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
