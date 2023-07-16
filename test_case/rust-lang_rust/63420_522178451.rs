plain
2019-08-16T22:41:25.1321379Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T22:41:25.1503624Z ##[command]git config gc.auto 0
2019-08-16T22:41:25.1581441Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T22:41:25.1634568Z ##[command]git config --get-all http.proxy
2019-08-16T22:41:25.1792091Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-16T22:42:01.1011222Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T22:42:01.1012334Z 
2019-08-16T22:42:01.1013382Z   git checkout -b <new-branch-name>
2019-08-16T22:42:01.1014166Z 
2019-08-16T22:42:01.1014867Z HEAD is now at 7e23ae3cb Merge bcd47c727c1f92ba36545d253b26792819e71441 into bdfd698f37184da42254a03ed466ab1f90e6fb6c
2019-08-16T22:42:01.1193809Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T22:42:01.1197562Z ==============================================================================
2019-08-16T22:42:01.1197628Z Task         : Bash
2019-08-16T22:42:01.1197680Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T23:14:35.5509448Z    Compiling semver v0.9.0
2019-08-16T23:14:37.9564399Z error: failed to run custom build command for `libc v0.2.60`
2019-08-16T23:14:37.9564736Z 
2019-08-16T23:14:37.9564793Z Caused by:
2019-08-16T23:14:37.9565312Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build` (signal: 6, SIGABRT: process abort signal)
2019-08-16T23:14:37.9565572Z --- stderr
2019-08-16T23:14:37.9565989Z *** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build': double free or corruption (fasttop): 0x00005561394de040 ***
2019-08-16T23:14:37.9566134Z ======= Backtrace: =========
2019-08-16T23:14:37.9566414Z /lib/x86_64-linux-gnu/libc.so.6(+0x777e5)[0x7f85dd3fa7e5]
2019-08-16T23:14:37.9566690Z /lib/x86_64-linux-gnu/libc.so.6(+0x8037a)[0x7f85dd40337a]
2019-08-16T23:14:37.9567003Z /lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7f85dd40753c]
2019-08-16T23:14:37.9567424Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std3env7_var_os17he43e01a1302ce367E+0x1ae)[0x55613927b4de]
2019-08-16T23:14:37.9567814Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x16cbf)[0x556139282cbf]
2019-08-16T23:14:37.9568273Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std9panicking20rust_panic_with_hook17hd2740c354aa5dd47E+0x190)[0x5561392839a0]
2019-08-16T23:14:37.9568649Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x17522)[0x556139283522]
2019-08-16T23:14:37.9569051Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(rust_begin_unwind+0x6)[0x556139283406]
2019-08-16T23:14:37.9569491Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN4core9panicking9panic_fmt17h72a7b8c354b02815E+0x3d)[0x556139298cad]
2019-08-16T23:14:37.9569935Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN4core6result13unwrap_failed17ha44dc08558b1b9e3E+0x87)[0x556139298da7]
2019-08-16T23:14:37.9570380Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std6thread6Thread3new17h92df7d02dd75ed4aE+0x2f1)[0x55613927a921]
2019-08-16T23:14:37.9570813Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std2rt19lang_start_internal17ha639f564efb1c10cE+0x2bf)[0x556139283e0f]
2019-08-16T23:14:37.9571207Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x9d55)[0x556139275d55]
2019-08-16T23:14:37.9571633Z /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7f85dd3a3830]
2019-08-16T23:14:37.9572043Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x8349)[0x556139274349]
2019-08-16T23:14:37.9572583Z ======= Memory map: ========
2019-08-16T23:14:37.9573093Z 55613926c000-5561392ad000 r-xp 00000000 08:01 12852489                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T23:14:37.9573542Z 5561394ac000-5561394af000 r--p 00040000 08:01 12852489                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T23:14:37.9573973Z 5561394af000-5561394b0000 rw-p 00043000 08:01 12852489                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T23:14:37.9574294Z 5561394de000-5561394ff000 rw-p 00000000 00:00 0                          [heap]
2019-08-16T23:14:37.9574588Z 7f85d8000000-7f85d8021000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9574858Z 7f85d8021000-7f85dc000000 ---p 00000000 00:00 0 
2019-08-16T23:14:37.9575351Z 7f85dd383000-7f85dd543000 r-xp 00000000 08:01 7962024                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T23:14:37.9575712Z 7f85dd543000-7f85dd743000 ---p 001c0000 08:01 7962024                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T23:14:37.9576054Z 7f85dd743000-7f85dd747000 r--p 001c0000 08:01 7962024                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T23:14:37.9576395Z 7f85dd747000-7f85dd749000 rw-p 001c4000 08:01 7962024                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T23:14:37.9576686Z 7f85dd749000-7f85dd74d000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9577019Z 7f85dd74d000-7f85dd763000 r-xp 00000000 08:01 7962045                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T23:14:37.9577366Z 7f85dd763000-7f85dd962000 ---p 00016000 08:01 7962045                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T23:14:37.9577727Z 7f85dd962000-7f85dd963000 rw-p 00015000 08:01 7962045                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T23:14:37.9578087Z 7f85dd963000-7f85dd97b000 r-xp 00000000 08:01 7962092                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T23:14:37.9578434Z 7f85dd97b000-7f85ddb7a000 ---p 00018000 08:01 7962092                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T23:14:37.9578799Z 7f85ddb7a000-7f85ddb7b000 r--p 00017000 08:01 7962092                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T23:14:37.9579146Z 7f85ddb7b000-7f85ddb7c000 rw-p 00018000 08:01 7962092                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T23:14:37.9579434Z 7f85ddb7c000-7f85ddb80000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9579771Z 7f85ddb80000-7f85ddb87000 r-xp 00000000 08:01 7962098                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T23:14:37.9580117Z 7f85ddb87000-7f85ddd86000 ---p 00007000 08:01 7962098                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T23:14:37.9580479Z 7f85ddd86000-7f85ddd87000 r--p 00006000 08:01 7962098                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T23:14:37.9580832Z 7f85ddd87000-7f85ddd88000 rw-p 00007000 08:01 7962098                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T23:14:37.9581169Z 7f85ddd88000-7f85ddd8b000 r-xp 00000000 08:01 7962037                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T23:14:37.9581527Z 7f85ddd8b000-7f85ddf8a000 ---p 00003000 08:01 7962037                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T23:14:37.9581865Z 7f85ddf8a000-7f85ddf8b000 r--p 00002000 08:01 7962037                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T23:14:37.9582641Z 7f85ddf8b000-7f85ddf8c000 rw-p 00003000 08:01 7962037                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T23:14:37.9583139Z 7f85ddf8c000-7f85ddfb2000 r-xp 00000000 08:01 7962004                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T23:14:37.9583453Z 7f85de1a7000-7f85de1ac000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9583720Z 7f85de1ae000-7f85de1b1000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9584090Z 7f85de1b1000-7f85de1b2000 r--p 00025000 08:01 7962004                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T23:14:37.9584425Z 7f85de1b2000-7f85de1b3000 rw-p 00026000 08:01 7962004                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T23:14:37.9584696Z 7f85de1b3000-7f85de1b4000 rw-p 00000000 00:00 0 
2019-08-16T23:14:37.9585010Z 7ffe5833a000-7ffe5835c000 rw-p 00000000 00:00 0                          [stack]
2019-08-16T23:14:37.9585311Z 7ffe5836a000-7ffe5836d000 r--p 00000000 00:00 0                          [vvar]
2019-08-16T23:14:37.9585610Z 7ffe5836d000-7ffe5836f000 r-xp 00000000 00:00 0                          [vdso]
2019-08-16T23:14:37.9585930Z ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
2019-08-16T23:14:37.9586271Z warning: build failed, waiting for other jobs to finish...
2019-08-16T23:14:40.9059790Z error: build failed
2019-08-16T23:14:40.9083537Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-16T23:14:40.9084159Z expected success, got: exit code: 101
2019-08-16T23:14:40.9084159Z expected success, got: exit code: 101
2019-08-16T23:14:40.9095421Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-16T23:14:40.9095907Z Build completed unsuccessfully in 0:26:02
2019-08-16T23:14:40.9155975Z == clock drift check ==
2019-08-16T23:14:40.9172076Z   local time: Fri Aug 16 23:14:40 UTC 2019
2019-08-16T23:14:41.1980210Z   network time: Fri, 16 Aug 2019 23:14:41 GMT
2019-08-16T23:14:41.1981189Z == end clock drift check ==
2019-08-16T23:14:42.3671457Z ##[error]Bash exited with code '1'.
2019-08-16T23:14:42.3718854Z ##[section]Starting: Checkout
2019-08-16T23:14:42.3720595Z ==============================================================================
2019-08-16T23:14:42.3720664Z Task         : Get sources
2019-08-16T23:14:42.3720711Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
