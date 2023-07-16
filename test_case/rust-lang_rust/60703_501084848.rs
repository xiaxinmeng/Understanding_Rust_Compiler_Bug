plain
travis_time:end:00b0b6b0:start=1560301035931174978,finish=1560301123624139339,duration=87692964361
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:30:07] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:30:07] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:30:07] error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (signal: 6, SIGABRT: process abort signal)
[00:30:07] --- stderr
[00:30:07] *** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc': free(): invalid pointer: 0x00007fa62c989b20 ***
[00:30:07] ======= Backtrace: =========
[00:30:07] /lib/x86_64-linux-gnu/libc.so.6(+0x777e5)[0x7fa62c63c7e5]
[00:30:07] /lib/x86_64-linux-gnu/libc.so.6(+0x8037a)[0x7fa62c64537a]
[00:30:07] /lib/x86_64-linux-gnu/libc.so.6(cfree+0x4c)[0x7fa62c64953c]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-c844844bc956e13f.so(rust_metadata_std_417ad1778c4b3fb7d86aee3513cf2902+0x4f66e)[0x7fa62c9de66e]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-c844844bc956e13f.so(rust_metadata_std_417ad1778c4b3fb7d86aee3513cf2902+0x55809)[0x7fa62c9e4809]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-c844844bc956e13f.so(_ZN3std6thread6Thread3new17h0991791a49ff12feE+0x1a4)[0x7fa62c9e6794]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-c844844bc956e13f.so(_ZN3std2rt19lang_start_internal17ha50d02de22603038E+0x314)[0x7fa62ca063a4]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x962)[0x56037e7b5962]
[00:30:07] /lib/x86_64-linux-gnu/libc.so.6(__libc_start_main+0xf0)[0x7fa62c5e5830]
[00:30:07] /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(+0x829)[0x56037e7b5829]
[00:30:07] ======= Memory map: ========
[00:30:07] 56037e7b5000-56037e7b6000 r-xp 00000000 08:01 2321720                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
[00:30:07] 56037e9b5000-56037e9b6000 r--p 00000000 08:01 2321720                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
[00:30:07] 56037e9b6000-56037e9b7000 rw-p 00001000 08:01 2321720                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
[00:30:07] 56037ed71000-56037ed92000 rw-p 00000000 00:00 0                          [heap]
[00:30:07] 7fa61c000000-7fa61c021000 rw-p 00000000 00:00 0 
[00:30:07] 7fa61c021000-7fa620000000 ---p 00000000 00:00 0 
[00:30:07] 7fa623eda000-7fa623f05000 r-xp 00000000 08:01 2081447                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libterm-42a3bfb8cb8e322f.so
[00:30:07] 7fa623f05000-7fa624105000 ---p 0002b000 08:01 2081447                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libterm-42a3bfb8cb8e322f.so
[00:30:07] 7fa624105000-7fa62410b000 r--p 0002b000 08:01 2081447                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libterm-42a3bfb8cb8e322f.so
[00:30:07] 7fa62410b000-7fa62410c000 rw-p 00031000 08:01 2081447                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libterm-42a3bfb8cb8e322f.so
[00:30:07] 7fa62410c000-7fa624214000 r-xp 00000000 08:01 1292136                    /lib/x86_64-linux-gnu/libm-2.23.so
[00:30:07] 7fa624214000-7fa624413000 ---p 00108000 08:01 1292136                    /lib/x86_64-linux-gnu/libm-2.23.so
[00:30:07] 7fa624413000-7fa624414000 r--p 00107000 08:01 1292136                    /lib/x86_64-linux-gnu/libm-2.23.so
[00:30:07] 7fa624414000-7fa624415000 rw-p 00108000 08:01 1292136                    /lib/x86_64-linux-gnu/libm-2.23.so
[00:30:07] 7fa624415000-7fa624418000 r-xp 00000000 08:01 2321411                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_fs_util-5f0122864e858157.so
[00:30:07] 7fa624418000-7fa624617000 ---p 00003000 08:01 2321411                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_fs_util-5f0122864e858157.so
[00:30:07] 7fa624617000-7fa624618000 r--p 00002000 08:01 2321411                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_fs_util-5f0122864e858157.so
[00:30:07] 7fa624618000-7fa624619000 rw-p 00003000 08:01 2321411                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_fs_util-5f0122864e858157.so
[00:30:07] 7fa624619000-7fa624620000 r-xp 00000000 08:01 2321391                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libfmt_macros-a1c850595bc2a1b5.so
[00:30:07] 7fa624620000-7fa624820000 ---p 00007000 08:01 2321391                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libfmt_macros-a1c850595bc2a1b5.so
[00:30:07] 7fa624820000-7fa624821000 r--p 00007000 08:01 2321391                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libfmt_macros-a1c850595bc2a1b5.so
[00:30:07] 7fa624821000-7fa624822000 rw-p 00008000 08:01 2321391                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libfmt_macros-a1c850595bc2a1b5.so
[00:30:07] 7fa624822000-7fa62499d000 r-xp 00000000 08:01 2321703                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_ext-4a7deb09fe5690b5.so
[00:30:07] 7fa62499d000-7fa624b9d000 ---p 0017b000 08:01 2321703                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_ext-4a7deb09fe5690b5.so
[00:30:07] 7fa624b9d000-7fa624ba3000 r--p 0017b000 08:01 2321703                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_ext-4a7deb09fe5690b5.so
[00:30:07] 7fa624ba3000-7fa624ba4000 rw-p 00181000 08:01 2321703                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_ext-4a7deb09fe5690b5.so
[00:30:07] 7fa624ba4000-7fa624c22000 r-xp 00000000 08:01 2321711                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_allocator-aa18244e2dd8bdec.so
[00:30:07] 7fa624c22000-7fa624e21000 ---p 0007e000 08:01 2321711                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_allocator-aa18244e2dd8bdec.so
[00:30:07] 7fa624e21000-7fa624e23000 r--p 0007d000 08:01 2321711                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_allocator-aa18244e2dd8bdec.so
[00:30:07] 7fa624e23000-7fa624e24000 rw-p 0007f000 08:01 2321711                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_allocator-aa18244e2dd8bdec.so
[00:30:07] 7fa624e24000-7fa624f4f000 r-xp 00000000 08:01 2321713                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_incremental-81e652614e78dfc5.so
[00:30:07] 7fa624f4f000-7fa62514f000 ---p 0012b000 08:01 2321713                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_incremental-81e652614e78dfc5.so
[00:30:07] 7fa62514f000-7fa625157000 r--p 0012b000 08:01 2321713                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_incremental-81e652614e78dfc5.so
[00:30:07] 7fa625157000-7fa625158000 rw-p 00133000 08:01 2321713                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_incremental-81e652614e78dfc5.so
[00:30:07] 7fa625158000-7fa62535c000 r-xp 00000000 08:01 2321736                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_ssa-1ea33004e3d61cd6.so
[00:30:07] 7fa62535c000-7fa62555c000 ---p 00204000 08:01 2321736                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_ssa-1ea33004e3d61cd6.so
[00:30:07] 7fa62555c000-7fa625568000 r--p 00204000 08:01 2321736                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_ssa-1ea33004e3d61cd6.so
[00:30:07] 7fa625568000-7fa625569000 rw-p 00210000 08:01 2321736                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_ssa-1ea33004e3d61cd6.so
[00:30:07] 7fa625569000-7fa625694000 r-xp 00000000 08:01 2321744                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_passes-f55b0c388399c648.so
[00:30:07] 7fa625694000-7fa625893000 ---p 0012b000 08:01 2321744                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_passes-f55b0c388399c648.so
[00:30:07] 7fa625893000-7fa625899000 r--p 0012a000 08:01 2321744                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_passes-f55b0c388399c648.so
[00:30:07] 7fa625899000-7fa62589a000 rw-p 00130000 08:01 2321744                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_passes-f55b0c388399c648.so
[00:30:07] 7fa62589a000-7fa625900000 r-xp 00000000 08:01 2321740                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_plugin-c290d89822ea1ad8.so
[00:30:07] 7fa625900000-7fa625aff000 ---p 00066000 08:01 2321740                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_plugin-c290d89822ea1ad8.so
[00:30:07] 7fa625aff000-7fa625b02000 r--p 00065000 08:01 2321740                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_plugin-c290d89822ea1ad8.so
[00:30:07] 7fa625b02000-7fa625b03000 rw-p 00068000 08:01 2321740                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_plugin-c290d89822ea1ad8.so
[00:30:07] 7fa625b03000-7fa62607c000 r-xp 00000000 08:01 2321716                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-4983be871d2727e4.so
[00:30:07] 7fa62607c000-7fa62627b000 ---p 00579000 08:01 2321716                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-4983be871d2727e4.so
[00:30:07] 7fa62627b000-7fa626297000 r--p 00578000 08:01 2321716                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-4983be871d2727e4.so
[00:30:07] 7fa626297000-7fa626298000 rw-p 00594000 08:01 2321716                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_typeck-4983be871d2727e4.so
[00:30:07] 7fa626298000-7fa6263cd000 r-xp 00000000 08:01 2321734                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-7cc3bcaa39b354c5.so
[00:30:07] 7fa6263cd000-7fa6265cd000 ---p 00135000 08:01 2321734                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-7cc3bcaa39b354c5.so
[00:30:07] 7fa6265cd000-7fa6265d1000 r--p 00135000 08:01 2321734                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-7cc3bcaa39b354c5.so
[00:30:07] 7fa6265d1000-7fa6265d2000 rw-p 00139000 08:01 2321734                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_privacy-7cc3bcaa39b354c5.so
[00:30:07] 7fa6265d2000-7fa62675f000 r-xp 00000000 08:01 2321738                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-08a9e1c52eb01e95.so
[00:30:07] 7fa62675f000-7fa62695e000 ---p 0018d000 08:01 2321738                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-08a9e1c52eb01e95.so
[00:30:07] 7fa62695e000-7fa62696a000 r--p 0018c000 08:01 2321738                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-08a9e1c52eb01e95.so
[00:30:07] 7fa62696a000-7fa62696b000 rw-p 00198000 08:01 2321738                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_resolve-08a9e1c52eb01e95.so
[00:30:07] 7fa62696b000-7fa626b7d000 r-xp 00000000 08:01 2321732                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_traits-14f127a5e7933602.so
[00:30:07] 7fa626b7d000-7fa626d7c000 ---p 00212000 08:01 2321732                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_traits-14f127a5e7933602.so
[00:30:07] 7fa626d7c000-7fa626d87000 r--p 00211000 08:01 2321732                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_traits-14f127a5e7933602.so
[00:30:07] 7fa626d87000-7fa626d88000 rw-p 0021c000 08:01 2321732                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_traits-14f127a5e7933602.so
[00:30:07] 7fa626d88000-7fa626ed6000 r-xp 00000000 08:01 2321718                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-0417aecf75953604.so
[00:30:07] 7fa626ed6000-7fa6270d6000 ---p 0014e000 08:01 2321718                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-0417aecf75953604.so
[00:30:07] 7fa6270d6000-7fa6270e2000 r--p 0014e000 08:01 2321718                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-0417aecf75953604.so
[00:30:07] 7fa6270e2000-7fa6270e3000 rw-p 0015a000 08:01 2321718                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_lint-0417aecf75953604.so
[00:30:07] 7fa6270e3000-7fa6270ea000 r-xp 00000000 08:01 1292178                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:30:07] 7fa6270ea000-7fa6272e9000 ---p 00007000 08:01 1292178                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:30:07] 7fa6272e9000-7fa6272ea000 r--p 00006000 08:01 1292178                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:30:07] 7fa6272ea000-7fa6272eb000 rw-p 00007000 08:01 1292178                    /lib/x86_64-linux-gnu/librt-2.23.so
[00:30:07] 7fa6272eb000-7fa6272ee000 r-xp 00000000 08:01 1292117                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:30:07] 7fa6272ee000-7fa6274ed000 ---p 00003000 08:01 1292117                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:30:07] 7fa6274ed000-7fa6274ee000 r--p 00002000 08:01 1292117                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:30:07] 7fa6274ee000-7fa6274ef000 rw-p 00003000 08:01 1292117                    /lib/x86_64-linux-gnu/libdl-2.23.so
[00:30:07] 7fa6274ef000-7fa627505000 r-xp 00000000 08:01 1292125                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:30:07] 7fa627505000-7fa627704000 ---p 00016000 08:01 1292125                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:30:07] 7fa627704000-7fa627705000 rw-p 00015000 08:01 1292125                    /lib/x86_64-linux-gnu/libgcc_s.so.1
[00:30:07] 7fa627705000-7fa62771d000 r-xp 00000000 08:01 1292172                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:30:07] 7fa62771d000-7fa62791c000 ---p 00018000 08:01 1292172                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:30:07] 7fa62791c000-7fa62791d000 r--p 00017000 08:01 1292172                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:30:07] 7fa62791d000-7fa62791e000 rw-p 00018000 08:01 1292172                    /lib/x86_64-linux-gnu/libpthread-2.23.so
[00:30:07] 7fa62791e000-7fa627922000 rw-p 00000000 00:00 0 
[00:30:07] 7fa627922000-7fa627955000 r-xp 00000000 08:01 2321584                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libserialize-8259a4fbdb6377df.so
[00:30:07] 7fa627955000-7fa627b54000 ---p 00033000 08:01 2321584                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libserialize-8259a4fbdb6377df.so
[00:30:07] 7fa627b54000-7fa627b57000 r--p 00032000 08:01 2321584                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libserialize-8259a4fbdb6377df.so
[00:30:07] 7fa627b57000-7fa627b58000 rw-p 00035000 08:01 2321584                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libserialize-8259a4fbdb6377df.so
[00:30:07] 7fa627b58000-7fa627b9c000 r-xp 00000000 08:01 2321462                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_cratesio_shim-7a3fa46122ece1d2.so
[00:30:07] 7fa627b9c000-7fa627d9b000 ---p 00044000 08:01 2321462                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_cratesio_shim-7a3fa46122ece1d2.so
[00:30:07] 7fa627d9b000-7fa627d9d000 r--p 00043000 08:01 2321462                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_cratesio_shim-7a3fa46122ece1d2.so
[00:30:07] 7fa627d9d000-7fa627d9e000 rw-p 00045000 08:01 2321462                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_cratesio_shim-7a3fa46122ece1d2.so
[00:30:07] 7fa627d9e000-7fa627da4000 r-xp 00000000 08:01 2321396                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libgraphviz-5802bba1e3dd27f2.so
[00:30:07] 7fa627da4000-7fa627fa4000 ---p 00006000 08:01 2321396                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libgraphviz-5802bba1e3dd27f2.so
[00:30:07] 7fa627fa4000-7fa627fa5000 r--p 00006000 08:01 2321396                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libgraphviz-5802bba1e3dd27f2.so
[00:30:07] 7fa627fa5000-7fa627fa6000 rw-p 00007000 08:01 2321396                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libgraphviz-5802bba1e3dd27f2.so
[00:30:07] 7fa627fa6000-7fa627fcf000 r-xp 00000000 08:01 2321687                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_data_structures-4d186a5a2a9b2d12.so
[00:30:07] 7fa627fcf000-7fa6281cf000 ---p 00029000 08:01 2321687                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_data_structures-4d186a5a2a9b2d12.so
[00:30:07] 7fa6281cf000-7fa6281d2000 r--p 00029000 08:01 2321687                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_data_structures-4d186a5a2a9b2d12.so
[00:30:07] 7fa6281d2000-7fa6281d3000 rw-p 0002c000 08:01 2321687                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_data_structures-4d186a5a2a9b2d12.so
[00:30:07] 7fa6281d3000-7fa6281d5000 r-xp 00000000 08:01 2321686                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libarena-59b6aa170dd18238.so
[00:30:07] 7fa6281d5000-7fa6283d5000 ---p 00002000 08:01 2321686                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libarena-59b6aa170dd18238.so
[00:30:07] 7fa6283d5000-7fa6283d6000 r--p 00002000 08:01 2321686                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libarena-59b6aa170dd18238.so
[00:30:07] 7fa6283d6000-7fa6283d7000 rw-p 00003000 08:01 2321686                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libarena-59b6aa170dd18238.so
[00:30:07] 7fa6283d7000-7fa628412000 r-xp 00000000 08:01 2321692                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_pos-70ac6e9befe6e999.so
[00:30:07] 7fa628412000-7fa628612000 ---p 0003b000 08:01 2321692                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_pos-70ac6e9befe6e999.so
[00:30:07] 7fa628612000-7fa628618000 r--p 0003b000 08:01 2321692                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_pos-70ac6e9befe6e999.so
[00:30:07] 7fa628618000-7fa628619000 rw-p 00041000 08:01 2321692                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax_pos-70ac6e9befe6e999.so
[00:30:07] 7fa628619000-7fa6286a2000 r-xp 00000000 08:01 2321697                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_errors-5632f2539e9ae150.so
[00:30:07] 7fa6286a2000-7fa6288a2000 ---p 00089000 08:01 2321697                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_errors-5632f2539e9ae150.so
[00:30:07] 7fa6288a2000-7fa6288a9000 r--p 00089000 08:01 2321697                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_errors-5632f2539e9ae150.so
[00:30:07] 7fa6288a9000-7fa6288aa000 rw-p 00090000 08:01 2321697                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_errors-5632f2539e9ae150.so
[00:30:07] 7fa6288aa000-7fa628999000 r-xp 00000000 08:01 2321695                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_target-e2397e11a8e1e1e0.so
[00:30:07] 7fa628999000-7fa628b98000 ---p 000ef000 08:01 2321695                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_target-e2397e11a8e1e1e0.so
[00:30:07] 7fa628b98000-7fa628b9c000 r--p 000ee000 08:01 2321695                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_target-e2397e11a8e1e1e0.so
[00:30:07] 7fa628b9c000-7fa628b9d000 rw-p 000f2000 08:01 2321695                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_target-e2397e11a8e1e1e0.so
[00:30:07] 7fa628b9d000-7fa628f0e000 r-xp 00000000 08:01 2321700                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax-b63c0d6d31f30b86.so
[00:30:07] 7fa628f0e000-7fa62910d000 ---p 00371000 08:01 2321700                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax-b63c0d6d31f30b86.so
[00:30:07] 7fa62910d000-7fa62912b000 r--p 00370000 08:01 2321700                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax-b63c0d6d31f30b86.so
[00:30:07] 7fa62912b000-7fa62912c000 rw-p 0038e000 08:01 2321700                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libsyntax-b63c0d6d31f30b86.so
[00:30:07] 7fa62912c000-7fa629184000 r-xp 00000000 08:01 2081451                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-8612e47523425cd2.so
[00:30:07] 7fa629184000-7fa629384000 ---p 00058000 08:01 2081451                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-8612e47523425cd2.so
[00:30:07] 7fa629384000-7fa629389000 r--p 00058000 08:01 2081451                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-8612e47523425cd2.so
[00:30:07] 7fa629389000-7fa62938a000 rw-p 0005d000 08:01 2081451                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libtest-8612e47523425cd2.so
[00:30:07] 7fa62938a000-7fa62a3a9000 r-xp 00000000 08:01 2321705                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc-c3b86f1aefb9449e.so
[00:30:07] 7fa62a3a9000-7fa62a5a9000 ---p 0101f000 08:01 2321705                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc-c3b86f1aefb9449e.so
[00:30:07] 7fa62a5a9000-7fa62a5f0000 r--p 0101f000 08:01 2321705                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc-c3b86f1aefb9449e.so
[00:30:07] 7fa62a5f0000-7fa62a5f1000 rw-p 01066000 08:01 2321705                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc-c3b86f1aefb9449e.so
[00:30:07] 7fa62a5f1000-7fa62a9a0000 r-xp 00000000 08:01 2321719                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_metadata-cdae7847e75e1e2e.so
[00:30:07] 7fa62a9a0000-7fa62ab9f000 ---p 003af000 08:01 2321719                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_metadata-cdae7847e75e1e2e.so
[00:30:07] 7fa62ab9f000-7fa62abab000 r--p 003ae000 08:01 2321719                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_metadata-cdae7847e75e1e2e.so
[00:30:07] 7fa62abab000-7fa62abac000 rw-p 003ba000 08:01 2321719                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_metadata-cdae7847e75e1e2e.so
[00:30:07] 7fa62abac000-7fa62acfa000 r-xp 00000000 08:01 2321731                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_utils-4f0bb63db5f17f16.so
[00:30:07] 7fa62acfa000-7fa62aef9000 ---p 0014e000 08:01 2321731                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_utils-4f0bb63db5f17f16.so
[00:30:07] 7fa62aef9000-7fa62af00000 r--p 0014d000 08:01 2321731                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_utils-4f0bb63db5f17f16.so
[00:30:07] 7fa62af00000-7fa62af01000 rw-p 00154000 08:01 2321731                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_codegen_utils-4f0bb63db5f17f16.so
[00:30:07] 7fa62af01000-7fa62b0be000 r-xp 00000000 08:01 2321742                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_save_analysis-c90426a27b6dc38b.so
[00:30:07] 7fa62b0be000-7fa62b2bd000 ---p 001bd000 08:01 2321742                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_save_analysis-c90426a27b6dc38b.so
[00:30:07] 7fa62b2bd000-7fa62b2c8000 r--p 001bc000 08:01 2321742                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_save_analysis-c90426a27b6dc38b.so
[00:30:07] 7fa62b2c8000-7fa62b2c9000 rw-p 001c7000 08:01 2321742                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_save_analysis-c90426a27b6dc38b.so
[00:30:07] 7fa62b2c9000-7fa62bbd4000 r-xp 00000000 08:01 2321729                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-de4a50c08ad258b8.so
[00:30:07] 7fa62bbd4000-7fa62bdd3000 ---p 0090b000 08:01 2321729                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-de4a50c08ad258b8.so
[00:30:07] 7fa62bdd3000-7fa62be07000 r--p 0090a000 08:01 2321729                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-de4a50c08ad258b8.so
[00:30:07] 7fa62be07000-7fa62be08000 rw-p 0093e000 08:01 2321729                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_mir-de4a50c08ad258b8.so
[00:30:07] 7fa62be08000-7fa62bef6000 r-xp 00000000 08:01 2321730                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-92e1cdf71ea580e5.so
[00:30:07] 7fa62bef6000-7fa62c0f6000 ---p 000ee000 08:01 2321730                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-92e1cdf71ea580e5.so
[00:30:07] 7fa62c0f6000-7fa62c0fe000 r--p 000ee000 08:01 2321730                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-92e1cdf71ea580e5.so
[00:30:07] 7fa62c0fe000-7fa62c0ff000 rw-p 000f6000 08:01 2321730                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_borrowck-92e1cdf71ea580e5.so
[00:30:07] 7fa62c0ff000-7fa62c3ba000 r-xp 00000000 08:01 2321739                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_interface-632a879f82581358.so
[00:30:07] 7fa62c3ba000-7fa62c5b9000 ---p 002bb000 08:01 2321739                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_interface-632a879f82581358.so
[00:30:07] 7fa62c5b9000-7fa62c5c4000 r--p 002ba000 08:01 2321739                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_interface-632a879f82581358.so
[00:30:07] 7fa62c5c4000-7fa62c5c5000 rw-p 002c5000 08:01 2321739                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_interface-632a879f82581358.so
[00:30:07] 7fa62c5c5000-7fa62c785000 r-xp 00000000 08:01 1292104                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:30:07] 7fa62c785000-7fa62c985000 ---p 001c0000 08:01 1292104                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:30:07] 7fa62c985000-7fa62c989000 r--p 001c0000 08:01 1292104                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:30:07] 7fa62c989000-7fa62c98b000 rw-p 001c4000 08:01 1292104                    /lib/x86_64-linux-gnu/libc-2.23.so
[00:30:07] 7fa62c98b000-7fa62c98f000 rw-p 00000000 00:00 0 
[00:30:07] 7fa62c98f000-7fa62ca8a000 r-xp 00000000 08:01 1816845                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-c844844bc956e13f.so
[00:30:07] 7fa62ca8a000-7fa62cc89000 ---p 000fb000 08:01 1816845                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-c844844bc956e13f.so
[00:30:07] 7fa62cc89000-7fa62cc98000 r--p 000fa000 08:01 1816845                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-c844844bc956e13f.so
[00:30:07] 7fa62cc98000-7fa62cc99000 rw-p 00109000 08:01 1816845                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-c844844bc956e13f.so
[00:30:07] 7fa62cc99000-7fa62ce2c000 r-xp 00000000 08:01 2321743                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2539f3b09b1cdd91.so
[00:30:07] 7fa62ce2c000-7fa62d02c000 ---p 00193000 08:01 2321743                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2539f3b09b1cdd91.so
[00:30:07] 7fa62d02c000-7fa62d038000 r--p 00193000 08:01 2321743                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2539f3b09b1cdd91.so
[00:30:07] 7fa62d038000-7fa62d039000 rw-p 0019f000 08:01 2321743                    /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2539f3b09b1cdd91.so
[00:30:07] 7fa62d039000-7fa62d05f000 r-xp 00000000 08:01 1292084                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:30:07] 7fa62d241000-7fa62d258000 rw-p 00000000 00:00 0 
[00:30:07] 7fa62d25a000-7fa62d25e000 rw-p 00000000 00:00 0 
[00:30:07] 7fa62d25e000-7fa62d25f000 r--p 00025000 08:01 1292084                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:30:07] 7fa62d25f000-7fa62d260000 rw-p 00026000 08:01 1292084                    /lib/x86_64-linux-gnu/ld-2.23.so
[00:30:07] 7fa62d260000-7fa62d261000 rw-p 00000000 00:00 0 
[00:30:07] 7ffc662e4000-7ffc66305000 rw-p 00000000 00:00 0                          [stack]
[00:30:07] 7ffc6633d000-7ffc66340000 r--p 00000000 00:00 0                          [vvar]
[00:30:07] 7ffc66340000-7ffc66342000 r-xp 00000000 00:00 0                          [vdso]
[00:30:07] ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
[00:30:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:30:07] expected success, got: exit code: 101
[00:30:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:30:07] Build completed unsuccessfully in 0:23:25
