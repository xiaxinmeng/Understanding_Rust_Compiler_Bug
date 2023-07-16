plain
2020-03-22T10:58:20.2780083Z ========================== Starting Command Output ===========================
2020-03-22T10:58:20.2782779Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3bd362fe-2644-4981-974a-382d8577e613.sh
2020-03-22T10:58:20.2783172Z 
2020-03-22T10:58:20.2787345Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T10:58:20.2805444Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-22T10:58:20.2810602Z Task         : Get sources
2020-03-22T10:58:20.2811380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T10:58:20.2811718Z Version      : 1.0.0
2020-03-22T10:58:20.2811877Z Author       : Microsoft
---
2020-03-22T10:58:23.1810175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T10:58:23.1818598Z ##[command]git config gc.auto 0
2020-03-22T10:58:23.1831618Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T10:58:23.1837832Z ##[command]git config --get-all http.proxy
2020-03-22T10:58:23.1845190Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70212/merge:refs/remotes/pull/70212/merge
---
2020-03-22T11:02:15.7988233Z configure: build.locked-deps    := True
2020-03-22T11:02:15.7988586Z configure: llvm.ccache          := sccache
2020-03-22T11:02:15.7989253Z configure: build.cargo-native-static := True
2020-03-22T11:02:15.7989804Z configure: dist.missing-tools   := True
2020-03-22T11:02:15.7990402Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T11:02:15.7991248Z configure: writing `config.toml` in current directory
2020-03-22T11:02:15.7991526Z configure: 
2020-03-22T11:02:15.7991988Z configure: run `python /checkout/x.py --help`
2020-03-22T11:02:15.7992285Z configure: 
---
2020-03-22T12:24:46.9450484Z     Finished release [optimized] target(s) in 1m 32s
2020-03-22T12:24:46.9690124Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> wasm32-unknown-emscripten)
2020-03-22T12:24:50.0285262Z 
2020-03-22T12:24:50.0286118Z running 9818 tests
2020-03-22T12:26:04.3858710Z ......ii..i.......i......i.......iiii.....................................ii................i......i 100/9818
2020-03-22T12:26:49.0293335Z i.............i.......................i....iiiiiii.iii.............................................. 200/9818
2020-03-22T12:27:41.6044066Z ........................................................................................ii.......... 400/9818
2020-03-22T12:27:41.6044066Z ........................................................................................ii.......... 400/9818
2020-03-22T12:28:08.8144062Z ...............................................................iii.................................. 500/9818
2020-03-22T12:28:39.2900784Z ........i........................................................iii................................ 600/9818
2020-03-22T12:30:02.2683503Z ...............................................i.................................................... 800/9818
2020-03-22T12:30:12.8685766Z .................................................................................................... 900/9818
2020-03-22T12:30:24.5037064Z .................................................................i.................................. 1000/9818
2020-03-22T12:31:03.4253601Z .................................................................................i........i.i....... 1100/9818
2020-03-22T12:31:03.4253601Z .................................................................................i........i.i....... 1100/9818
2020-03-22T12:31:24.4912991Z ......................................i............................................................. 1200/9818
2020-03-22T12:31:33.1825579Z .............................................................................iiiii.................. 1300/9818
2020-03-22T12:32:25.4232850Z ..............................................................................i..................... 1500/9818
2020-03-22T12:32:49.9809770Z .................................................................................................... 1600/9818
2020-03-22T12:33:24.4209722Z ..............................................................ii.i.................i................ 1700/9818
2020-03-22T12:33:45.6188474Z .................i..................i.i............................................................. 1800/9818
2020-03-22T12:33:45.6188474Z .................i..................i.i............................................................. 1800/9818
2020-03-22T12:34:23.9350192Z ...................................................i......i...................i...................ii 1900/9818
2020-03-22T12:34:46.5118500Z .................................................................................................... 2000/9818
2020-03-22T12:35:13.5996048Z ....................................................................iiiii.....................i..... 2100/9818
2020-03-22T12:35:57.3586572Z .................................................................................................... 2200/9818
2020-03-22T12:36:05.8735558Z ..............................iii...i............................................................... 2300/9818
2020-03-22T12:36:07.8219945Z .................................................................................................... 2400/9818
2020-03-22T12:36:10.5688212Z ............................................................................iiii.................... 2500/9818
2020-03-22T12:36:41.2396025Z i......................................................................................iiiiiiiiiiiii 2600/9818
2020-03-22T12:36:46.8072420Z ii.........i...........................i..........ii................................................ 2700/9818
2020-03-22T12:37:56.9357217Z .................................i....i............................................................. 2900/9818
2020-03-22T12:38:25.8792391Z ..............i............i..i..................................................................... 3000/9818
2020-03-22T12:38:49.1212870Z .....................................i.....................................................i........ 3100/9818
2020-03-22T12:39:11.7088517Z .................................................................................................... 3200/9818
2020-03-22T12:39:11.7088517Z .................................................................................................... 3200/9818
2020-03-22T12:39:33.1794366Z .................................................................................................... 3300/9818
2020-03-22T12:39:54.3302523Z ...............................................................................................ii... 3400/9818
2020-03-22T12:40:18.9255449Z .................ii.ii...................i...i................................i..................... 3500/9818
2020-03-22T12:41:21.6491449Z ...........................i...................i........................................i.......i... 3700/9818
2020-03-22T12:41:55.6232428Z .....i............................i................................................................. 3800/9818
2020-03-22T12:42:21.4931661Z .................................................i.................................................. 3900/9818
2020-03-22T12:42:46.1232770Z ...............i...........................................................i........................ 4000/9818
---
2020-03-22T12:53:51.2282401Z ..............i............. 5400/9818
2020-03-22T12:54:19.8645833Z ..................................................................................................i. 5500/9818
2020-03-22T12:54:35.9693008Z .........i..............................i........................................................... 5600/9818
2020-03-22T12:54:56.9353310Z ..............i.......................................................i...............i............. 5700/9818
2020-03-22T12:55:33.5276837Z ...............................ii...................................ii.............................. 5800/9818
2020-03-22T12:56:04.3462628Z ...........................................i...i.................................................... 5900/9818
2020-03-22T12:56:37.4989950Z .................................................................................................... 6000/9818
2020-03-22T12:57:07.5115900Z ........i.....................................................ii...i..ii...........i................ 6100/9818
2020-03-22T12:57:36.1325331Z .............i........i.i........................................................................... 6200/9818
2020-03-22T12:57:51.2974834Z .................................................................................................... 6400/9818
2020-03-22T12:57:51.2974834Z .................................................................................................... 6400/9818
2020-03-22T12:57:59.3218454Z ..........................................................................i..............i..i..ii... 6500/9818
2020-03-22T12:58:40.0495252Z .............................i.....ii............................................................... 6600/9818
2020-03-22T12:59:17.5646321Z .................................................................i..............................ii.. 6700/9818
2020-03-22T12:59:32.8594017Z ..................iii...Fii....i.iiiiii....................................................i........ 6800/9818
2020-03-22T12:59:37.6636760Z .................................................................................................... 7000/9818
2020-03-22T12:59:39.7232613Z ..........................i......................................................................... 7100/9818
2020-03-22T12:59:50.3359583Z ........................................................i........................................... 7200/9818
2020-03-22T13:00:03.8557025Z .................................................................i.................................. 7300/9818
2020-03-22T13:00:03.8557025Z .................................................................i.................................. 7300/9818
2020-03-22T13:01:15.8030783Z .............................................................i...................................... 7400/9818
2020-03-22T13:02:14.6321182Z ..........................i...i........i...................iiiiiiii................................. 7500/9818
2020-03-22T13:03:06.2529757Z .................................................................................................... 7700/9818
2020-03-22T13:03:29.8636714Z .................................................................................................... 7800/9818
2020-03-22T13:03:42.4466389Z .................................................................................................... 7900/9818
2020-03-22T13:04:00.3370379Z ................................................................................i................... 8000/9818
2020-03-22T13:04:00.3370379Z ................................................................................i................... 8000/9818
2020-03-22T13:04:43.9432530Z ......................................................................i............................. 8100/9818
2020-03-22T13:05:03.8954588Z .............................iiiiiiiiii.i........................................................... 8200/9818
2020-03-22T13:05:33.4326665Z .........................................i..............iii.....i......iiiiiiiiiii.i................ 8300/9818
2020-03-22T13:06:02.7793165Z .................................................................................................... 8500/9818
2020-03-22T13:07:04.6594473Z ............i.i.....................................................................i............... 8600/9818
2020-03-22T13:07:42.2970103Z ..................................................i................................................. 8700/9818
2020-03-22T13:07:53.0521633Z .................................................................................................... 8800/9818
2020-03-22T13:07:53.0521633Z .................................................................................................... 8800/9818
2020-03-22T13:08:52.1934072Z ............iii......i....................iiii....ii....i.i.....iiiiii.....iiiiiiii.ii...ii.iii..iii 8900/9818
2020-03-22T13:09:29.5945064Z i...............................................................i................................... 9000/9818
2020-03-22T13:10:36.8963031Z .................................................i.................................................. 9200/9818
2020-03-22T13:10:50.9296355Z ..........................i......................................................................... 9300/9818
2020-03-22T13:11:17.0602146Z .................................................................................................... 9400/9818
2020-03-22T13:11:49.6105680Z .................................................................................................... 9500/9818
---
2020-03-22T13:13:18.2906914Z ---- [ui] ui/panic-runtime/link-to-abort.rs stdout ----
2020-03-22T13:13:18.2907098Z 
2020-03-22T13:13:18.2907482Z error: test compilation failed although it shouldn't!
2020-03-22T13:13:18.2907718Z status: exit code: 1
2020-03-22T13:13:18.2909441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/link-to-abort.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary"
2020-03-22T13:13:18.2911178Z ------------------------------------------
2020-03-22T13:13:18.2911333Z 
2020-03-22T13:13:18.2911612Z ------------------------------------------
2020-03-22T13:13:18.2911771Z stderr:
2020-03-22T13:13:18.2911771Z stderr:
2020-03-22T13:13:18.2912054Z ------------------------------------------
2020-03-22T13:13:18.2912304Z error: linking with `emcc` failed: exit code: 1
2020-03-22T13:13:18.2912481Z    |
2020-03-22T13:13:18.2919001Z    = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.4hgw2zitnpvuzihs.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-649ead28735e66bb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-587216951de5aadc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-9f402acba710984b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-d4b94f9ef3267bc9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-a419154407e135b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-9bcd7ed55114abfd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-52b7539f915078e8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-c88276a1179c4990.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-a80fc12e123f7783.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-291d5652fb98867a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-74bc0e65f45846ac.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-bcc92ad1c6190631.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-eb3e1e6234b8fa89.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
2020-03-22T13:13:18.2924013Z    = note: cache:INFO: generating system library: libc++abi-noexcept.a... (this will be cached in "/root/.emscripten_cache/wasm-obj/libc++abi-noexcept.a" for subsequent builds)
2020-03-22T13:13:18.2924563Z            cache:INFO:  - ok
2020-03-22T13:13:18.2925546Z            wasm-ld: error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-649ead28735e66bb.rlib(std-649ead28735e66bb.std.9ruk0nu2-cgu.0.rcgu.o): undefined symbol: rust_eh_catch_typeinfo
2020-03-22T13:13:18.2926695Z            wasm-ld: error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-649ead28735e66bb.rlib(std-649ead28735e66bb.std.9ruk0nu2-cgu.0.rcgu.o): undefined symbol: rust_eh_catch_typeinfo
2020-03-22T13:13:18.2938618Z            shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp__R2z7i/a.wasm --allow-undefined --lto-O0 -L/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.4hgw2zitnpvuzihs.rcgu.o -L/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers -L/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary -L/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-649ead28735e66bb.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-587216951de5aadc.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-9f402acba710984b.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-d4b94f9ef3267bc9.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-a419154407e135b4.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-9bcd7ed55114abfd.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-52b7539f915078e8.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-c88276a1179c4990.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-a80fc12e123f7783.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-291d5652fb98867a.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-74bc0e65f45846ac.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-bcc92ad1c6190631.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-eb3e1e6234b8fa89.rlib -lc --fatal-warnings -L/emsdk-portable/upstream/emscripten/system/local/lib -L/emsdk-portable/upstream/emscripten/system/lib -L/root/.emscripten_cache/wasm-obj /root/.emscripten_cache/wasm-obj/libc.a /root/.emscripten_cache/wasm-obj/libcompiler_rt.a /root/.emscripten_cache/wasm-obj/libc-wasm.a /root/.emscripten_cache/wasm-obj/libc++abi-noexcept.a /root/.emscripten_cache/wasm-obj/libc-extras.a /root/.emscripten_cache/wasm-obj/libdlmalloc.a /root/.emscripten_cache/wasm-obj/libpthread_stub.a /root/.emscripten_cache/wasm-obj/libc_rt_wasm.a --import-memory --import-table -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ --export __cxa_is_pointer_type --export __cxa_can_catch -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (1)
2020-03-22T13:13:18.2944651Z 
2020-03-22T13:13:18.2944819Z error: aborting due to previous error
2020-03-22T13:13:18.2944954Z 
2020-03-22T13:13:18.2945035Z 
---
2020-03-22T13:13:18.2947763Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-22T13:13:18.2948106Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-22T13:13:18.2957271Z 
2020-03-22T13:13:18.2957446Z 
2020-03-22T13:13:18.2961790Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-22T13:13:18.2965003Z 
2020-03-22T13:13:18.2965082Z 
2020-03-22T13:13:18.3011281Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2020-03-22T13:13:18.3012190Z Build completed unsuccessfully in 2:09:24
2020-03-22T13:13:18.3012190Z Build completed unsuccessfully in 2:09:24
2020-03-22T13:13:18.3042069Z == clock drift check ==
2020-03-22T13:13:18.3060390Z   local time: Sun Mar 22 13:13:18 UTC 2020
2020-03-22T13:13:18.5915821Z   network time: Sun, 22 Mar 2020 13:13:18 GMT
2020-03-22T13:13:18.5916464Z == end clock drift check ==
2020-03-22T13:13:19.0629228Z 
2020-03-22T13:13:19.0692639Z ##[error]Bash exited with code '1'.
2020-03-22T13:13:19.0704865Z ##[section]Finishing: Run build
2020-03-22T13:13:19.0752259Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-22T13:13:19.0756460Z Task         : Get sources
2020-03-22T13:13:19.0756742Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T13:13:19.0756980Z Version      : 1.0.0
2020-03-22T13:13:19.0757144Z Author       : Microsoft
2020-03-22T13:13:19.0757144Z Author       : Microsoft
2020-03-22T13:13:19.0757432Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T13:13:19.0757741Z ==============================================================================
2020-03-22T13:13:19.3814263Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T13:13:19.3851548Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-22T13:13:19.3925606Z Cleaning up task key
2020-03-22T13:13:19.3926579Z Start cleaning up orphan processes.
2020-03-22T13:13:19.4082827Z Terminate orphan process: pid (4170) (python)
2020-03-22T13:13:19.4285096Z ##[section]Finishing: Finalize Job
