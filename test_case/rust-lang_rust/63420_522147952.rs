plain
2019-08-16T20:20:42.4044195Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T20:20:42.4212200Z ##[command]git config gc.auto 0
2019-08-16T20:20:42.4281616Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T20:20:42.4322599Z ##[command]git config --get-all http.proxy
2019-08-16T20:20:42.4464033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-16T20:21:18.4149190Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T20:21:18.4149910Z 
2019-08-16T20:21:18.4151087Z   git checkout -b <new-branch-name>
2019-08-16T20:21:18.4151526Z 
2019-08-16T20:21:18.4151937Z HEAD is now at 446844bd9 Merge fb527f0bf7d46822f31747a6461fec0f9dd4bdfd into 9a32ad0dd51f8451aa6e39d7e9ea89483cb8fcfa
2019-08-16T20:21:18.4317954Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T20:21:18.4321347Z ==============================================================================
2019-08-16T20:21:18.4321424Z Task         : Bash
2019-08-16T20:21:18.4321468Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T20:53:05.2926818Z    Compiling semver v0.9.0
2019-08-16T20:53:07.5661238Z error: failed to run custom build command for `libc v0.2.60`
2019-08-16T20:53:07.5662119Z 
2019-08-16T20:53:07.5662438Z Caused by:
2019-08-16T20:53:07.5663201Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build` (signal: 6, SIGABRT: process abort signal)
2019-08-16T20:53:07.5663728Z --- stderr
2019-08-16T20:53:07.5664552Z *** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build': double free or corruption (fasttop): 0x000055f13dc65040 ***
2019-08-16T20:53:07.5664807Z ======= Backtrace: =========
2019-08-16T20:53:07.5665211Z /lib/x86_64-linux-gnu/libc.so.6(+0x777e5)[0x7f1516fc47e5]
2019-08-16T20:53:07.5665701Z /lib/x86_64-linux-gnu/libc.so.6(+0x8037a)[0x7f1516fcd37a]
2019-08-16T20:53:07.5666139Z /lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7f1516fd153c]
2019-08-16T20:53:07.5666695Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std3env7_var_os17h933be281538e872fE+0x1ae)[0x55f13c3a14de]
2019-08-16T20:53:07.5667233Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x16cbf)[0x55f13c3a8cbf]
2019-08-16T20:53:07.5667800Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std9panicking20rust_panic_with_hook17hdb2ec2a272270ba3E+0x190)[0x55f13c3a99a0]
2019-08-16T20:53:07.5668341Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x17522)[0x55f13c3a9522]
2019-08-16T20:53:07.5668862Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(rust_begin_unwind+0x6)[0x55f13c3a9406]
2019-08-16T20:53:07.5669450Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN4core9panicking9panic_fmt17h72a7b8c354b02815E+0x3d)[0x55f13c3becad]
2019-08-16T20:53:07.5670181Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN4core6result13unwrap_failed17ha44dc08558b1b9e3E+0x87)[0x55f13c3beda7]
2019-08-16T20:53:07.5671469Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std6thread6Thread3new17h17b3868de0128787E+0x2f1)[0x55f13c3a0921]
2019-08-16T20:53:07.5672207Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(_ZN3std2rt19lang_start_internal17h39cb00454892d415E+0x2bf)[0x55f13c3a9e0f]
2019-08-16T20:53:07.5672856Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x9d45)[0x55f13c39bd45]
2019-08-16T20:53:07.5673447Z /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7f1516f6d830]
2019-08-16T20:53:07.5674064Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build(+0x8349)[0x55f13c39a349]
2019-08-16T20:53:07.5674383Z ======= Memory map: ========
2019-08-16T20:53:07.5675021Z 55f13c392000-55f13c3d3000 r-xp 00000000 08:01 12832758                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T20:53:07.5675764Z 55f13c5d2000-55f13c5d5000 r--p 00040000 08:01 12832758                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T20:53:07.5677297Z 55f13c5d5000-55f13c5d6000 rw-p 00043000 08:01 12832758                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-c9db8456b64370fd/build-script-build
2019-08-16T20:53:07.5678293Z 55f13dc65000-55f13dc86000 rw-p 00000000 00:00 0                          [heap]
2019-08-16T20:53:07.5679164Z 7f1510000000-7f1510021000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5679763Z 7f1510021000-7f1514000000 ---p 00000000 00:00 0 
2019-08-16T20:53:07.5680192Z 7f1516f4d000-7f151710d000 r-xp 00000000 08:01 7961938                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T20:53:07.5680586Z 7f151710d000-7f151730d000 ---p 001c0000 08:01 7961938                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T20:53:07.5681569Z 7f151730d000-7f1517311000 r--p 001c0000 08:01 7961938                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T20:53:07.5682068Z 7f1517311000-7f1517313000 rw-p 001c4000 08:01 7961938                    /lib/x86_64-linux-gnu/libc-2.23.so
2019-08-16T20:53:07.5682504Z 7f1517313000-7f1517317000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5682986Z 7f1517317000-7f151732d000 r-xp 00000000 08:01 7961959                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T20:53:07.5683486Z 7f151732d000-7f151752c000 ---p 00016000 08:01 7961959                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T20:53:07.5684090Z 7f151752c000-7f151752d000 rw-p 00015000 08:01 7961959                    /lib/x86_64-linux-gnu/libgcc_s.so.1
2019-08-16T20:53:07.5684712Z 7f151752d000-7f1517545000 r-xp 00000000 08:01 7962006                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T20:53:07.5685143Z 7f1517545000-7f1517744000 ---p 00018000 08:01 7962006                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T20:53:07.5685698Z 7f1517744000-7f1517745000 r--p 00017000 08:01 7962006                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T20:53:07.5686088Z 7f1517745000-7f1517746000 rw-p 00018000 08:01 7962006                    /lib/x86_64-linux-gnu/libpthread-2.23.so
2019-08-16T20:53:07.5686609Z 7f1517746000-7f151774a000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5686999Z 7f151774a000-7f1517751000 r-xp 00000000 08:01 7962012                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T20:53:07.5687397Z 7f1517751000-7f1517950000 ---p 00007000 08:01 7962012                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T20:53:07.5687778Z 7f1517950000-7f1517951000 r--p 00006000 08:01 7962012                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T20:53:07.5688270Z 7f1517951000-7f1517952000 rw-p 00007000 08:01 7962012                    /lib/x86_64-linux-gnu/librt-2.23.so
2019-08-16T20:53:07.5688919Z 7f1517952000-7f1517955000 r-xp 00000000 08:01 7961951                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T20:53:07.5689831Z 7f1517955000-7f1517b54000 ---p 00003000 08:01 7961951                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T20:53:07.5690305Z 7f1517b54000-7f1517b55000 r--p 00002000 08:01 7961951                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T20:53:07.5690905Z 7f1517b55000-7f1517b56000 rw-p 00003000 08:01 7961951                    /lib/x86_64-linux-gnu/libdl-2.23.so
2019-08-16T20:53:07.5691331Z 7f1517b56000-7f1517b7c000 r-xp 00000000 08:01 7961918                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T20:53:07.5691888Z 7f1517d71000-7f1517d76000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5692407Z 7f1517d78000-7f1517d7b000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5693201Z 7f1517d7b000-7f1517d7c000 r--p 00025000 08:01 7961918                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T20:53:07.5694403Z 7f1517d7c000-7f1517d7d000 rw-p 00026000 08:01 7961918                    /lib/x86_64-linux-gnu/ld-2.23.so
2019-08-16T20:53:07.5694833Z 7f1517d7d000-7f1517d7e000 rw-p 00000000 00:00 0 
2019-08-16T20:53:07.5695303Z 7ffc980dd000-7ffc980ff000 rw-p 00000000 00:00 0                          [stack]
2019-08-16T20:53:07.5695744Z 7ffc9818e000-7ffc98191000 r--p 00000000 00:00 0                          [vvar]
2019-08-16T20:53:07.5696203Z 7ffc98191000-7ffc98193000 r-xp 00000000 00:00 0                          [vdso]
2019-08-16T20:53:07.5696846Z ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
2019-08-16T20:53:07.5697608Z warning: build failed, waiting for other jobs to finish...
2019-08-16T20:53:10.9771389Z error: build failed
2019-08-16T20:53:10.9796432Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-16T20:53:10.9796627Z expected success, got: exit code: 101
2019-08-16T20:53:10.9796627Z expected success, got: exit code: 101
2019-08-16T20:53:10.9812465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-16T20:53:10.9812553Z Build completed unsuccessfully in 0:25:23
2019-08-16T20:53:10.9864024Z == clock drift check ==
2019-08-16T20:53:10.9879976Z   local time: Fri Aug 16 20:53:10 UTC 2019
2019-08-16T20:53:11.0325687Z   network time: Fri, 16 Aug 2019 20:53:11 GMT
2019-08-16T20:53:11.0328386Z == end clock drift check ==
2019-08-16T20:53:12.1703959Z ##[error]Bash exited with code '1'.
2019-08-16T20:53:12.1758160Z ##[section]Starting: Checkout
2019-08-16T20:53:12.1760068Z ==============================================================================
2019-08-16T20:53:12.1760124Z Task         : Get sources
2019-08-16T20:53:12.1760185Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
