plain
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
[RUSTC-TIMING] rustc_ty_utils test:false 28.544
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
*** Error in `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc': double free or corruption (out): 0x00007ff6fd307330 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7ff7622267f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x8038a)[0x7ff76222f38a]
/lib/x86_64-linux-gnu/libc.so.6(+0x83380)[0x7ff762232380]
/lib/x86_64-linux-gnu/libc.so.6(realloc+0x199)[0x7ff7622338a9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x33c07bd)[0x7ff765ce77bd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x31ae9b1)[0x7ff765ad59b1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x31b7bee)[0x7ff765adebee]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x31b0128)[0x7ff765ad7128]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x3534992)[0x7ff765e5b992]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x317392b)[0x7ff765a9a92b]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x3179bf3)[0x7ff765aa0bf3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x2563c2d)[0x7ff764e8ac2d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x1578a04)[0x7ff763e9fa04]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x2564e55)[0x7ff764e8be55]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x22a13ce)[0x7ff764bc83ce]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x152a611)[0x7ff763e51611]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x1394e3c)[0x7ff763cbbe3c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x139bac6)[0x7ff763cc2ac6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x1388ae0)[0x7ff763cafae0]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x1381960)[0x7ff763ca8960]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x144b995)[0x7ff763d72995]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-dd5bcd4cdc2c6057.so(+0x14e6b5a)[0x7ff763e0db5a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-9e3036d1402b84ee.so(rust_metadata_std_836a811975e52724+0xc3733)[0x7ff76263c733]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x76ba)[0x7ff761b8d6ba]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x7ff7622b651d]
======= Memory map: ========
560518a00000-560518a01000 r-xp 00000000 08:01 14715264                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
560518c00000-560518c01000 r--p 00000000 08:01 14715264                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
560518c01000-560518c02000 rw-p 00001000 08:01 14715264                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
56051913e000-56051b7bf000 rw-p 00000000 00:00 0                          [heap]
7ff6fc000000-7ff6ffffc000 rw-p 00000000 00:00 0 
7ff6ffffc000-7ff700000000 ---p 00000000 00:00 0 
7ff704000000-7ff70788c000 rw-p 00000000 00:00 0 
7ff70788c000-7ff708000000 ---p 00000000 00:00 0 
7ff708000000-7ff70b9f4000 rw-p 00000000 00:00 0 
7ff70b9f4000-7ff70c000000 ---p 00000000 00:00 0 
7ff70c000000-7ff710000000 rw-p 00000000 00:00 0 
7ff710000000-7ff713f99000 rw-p 00000000 00:00 0 
7ff713f99000-7ff714000000 ---p 00000000 00:00 0 
7ff714000000-7ff714021000 rw-p 00000000 00:00 0 
7ff714021000-7ff718000000 ---p 00000000 00:00 0 
7ff718000000-7ff718b6e000 rw-p 00000000 00:00 0 
7ff718b6e000-7ff71c000000 ---p 00000000 00:00 0 
7ff71c000000-7ff71fd8b000 rw-p 00000000 00:00 0 
7ff71fd8b000-7ff720000000 ---p 00000000 00:00 0 
7ff720000000-7ff724000000 rw-p 00000000 00:00 0 
7ff724000000-7ff727fff000 rw-p 00000000 00:00 0 
7ff727fff000-7ff728000000 ---p 00000000 00:00 0 
7ff728000000-7ff72bc71000 rw-p 00000000 00:00 0 
7ff72bc71000-7ff72c000000 ---p 00000000 00:00 0 
7ff72c000000-7ff72ffac000 rw-p 00000000 00:00 0 
7ff72ffac000-7ff730000000 ---p 00000000 00:00 0 
7ff734000000-7ff737e7a000 rw-p 00000000 00:00 0 
7ff737e7a000-7ff738000000 ---p 00000000 00:00 0 
7ff738000000-7ff73bda2000 rw-p 00000000 00:00 0 
7ff73bda2000-7ff73c000000 ---p 00000000 00:00 0 
7ff73c000000-7ff740000000 rw-p 00000000 00:00 0 
7ff740000000-7ff744000000 rw-p 00000000 00:00 0 
7ff7456c7000-7ff7476c8000 rw-p 00000000 00:00 0 
7ff7476c8000-7ff747dc5000 r-xp 00000000 08:01 14968904                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libchalk_derive-7c986050a4304ad1.so
7ff747dc5000-7ff747fc4000 ---p 006fd000 08:01 14968904                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libchalk_derive-7c986050a4304ad1.so
7ff747fc4000-7ff747fff000 r--p 006fc000 08:01 14968904                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libchalk_derive-7c986050a4304ad1.so
7ff747fff000-7ff748000000 rw-p 00737000 08:01 14968904                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libchalk_derive-7c986050a4304ad1.so
7ff748000000-7ff74c000000 rw-p 00000000 00:00 0 
7ff74cc7c000-7ff74cc7d000 ---p 00000000 00:00 0 
7ff74cc7d000-7ff74ce7d000 rw-p 00000000 00:00 0 
7ff74ce7d000-7ff74ce7e000 ---p 00000000 00:00 0 
7ff74ce7e000-7ff74d07e000 rw-p 00000000 00:00 0 
7ff74d07e000-7ff74d07f000 ---p 00000000 00:00 0 
7ff74d07f000-7ff74d27f000 rw-p 00000000 00:00 0 
7ff74d69e000-7ff74d8df000 rw-p 00000000 00:00 0 
7ff74dc7f000-7ff74dc80000 ---p 00000000 00:00 0 
7ff74dc80000-7ff74df61000 rw-p 00000000 00:00 0 
7ff74e3a5000-7ff74e3a6000 ---p 00000000 00:00 0 
7ff74e3a6000-7ff74e5a6000 rw-p 00000000 00:00 0 
7ff74ebc9000-7ff74ec4a000 rw-p 00000000 00:00 0 
7ff74f2f2000-7ff74faa2000 r-xp 00000000 08:01 14969085                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libserde_derive-6eac6d6816e82c0c.so
7ff74faa2000-7ff74fca2000 ---p 007b0000 08:01 14969085                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libserde_derive-6eac6d6816e82c0c.so
7ff74fca2000-7ff74fcdb000 r--p 007b0000 08:01 14969085                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libserde_derive-6eac6d6816e82c0c.so
7ff74fcdb000-7ff74fcdc000 rw-p 007e9000 08:01 14969085                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libserde_derive-6eac6d6816e82c0c.so
7ff751287000-7ff751931000 r-xp 00000000 08:01 14968913                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libunic_langid_macros_impl-f0a62b738d1ce3af.so
7ff751931000-7ff751b31000 ---p 006aa000 08:01 14968913                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libunic_langid_macros_impl-f0a62b738d1ce3af.so
7ff751b31000-7ff751b6a000 r--p 006aa000 08:01 14968913                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libunic_langid_macros_impl-f0a62b738d1ce3af.so
7ff751b6a000-7ff751b6b000 rw-p 006e3000 08:01 14968913                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libunic_langid_macros_impl-f0a62b738d1ce3af.so
7ff751b6b000-7ff751d8b000 r-xp 00000000 08:01 14968453                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro_hack-fd0fddafec950a99.so
7ff751d8b000-7ff751f8a000 ---p 00220000 08:01 14968453                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro_hack-fd0fddafec950a99.so
7ff751f8a000-7ff751f9d000 r--p 0021f000 08:01 14968453                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro_hack-fd0fddafec950a99.so
7ff751f9d000-7ff751f9e000 rw-p 00232000 08:01 14968453                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro_hack-fd0fddafec950a99.so
7ff75200d000-7ff7526f9000 r-xp 00000000 08:01 14969011                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libthiserror_impl-8382b828622c6de3.so
7ff7526f9000-7ff7528f8000 ---p 006ec000 08:01 14969011                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libthiserror_impl-8382b828622c6de3.so
7ff7528f8000-7ff752931000 r--p 006eb000 08:01 14969011                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libthiserror_impl-8382b828622c6de3.so
7ff752931000-7ff752932000 rw-p 00724000 08:01 14969011                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libthiserror_impl-8382b828622c6de3.so
7ff754c8b000-7ff75534e000 r-xp 00000000 08:01 14968986                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libtracing_attributes-21b93e6fd5e85f7f.so
7ff75534e000-7ff75554d000 ---p 006c3000 08:01 14968986                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libtracing_attributes-21b93e6fd5e85f7f.so
7ff75554d000-7ff755585000 r--p 006c2000 08:01 14968986                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libtracing_attributes-21b93e6fd5e85f7f.so
7ff755585000-7ff755586000 rw-p 006fa000 08:01 14968986                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libtracing_attributes-21b93e6fd5e85f7f.so
7ff755990000-7ff75631d000 r-xp 00000000 08:01 14969102                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-423d3bca2d735392.so
7ff75631d000-7ff75651d000 ---p 0098d000 08:01 14969102                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-423d3bca2d735392.so
7ff75651d000-7ff756570000 r--p 0098d000 08:01 14969102                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-423d3bca2d735392.so
7ff756570000-7ff756571000 rw-p 009e0000 08:01 14969102                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-423d3bca2d735392.so
7ff75c000000-7ff760000000 rw-p 00000000 00:00 0 
7ff7609c0000-7ff760b39000 rw-p 00000000 00:00 0 
7ff760c0b000-7ff760c4c000 rw-p 00000000 00:00 0 
7ff760c4c000-7ff760c4d000 ---p 00000000 00:00 0 
7ff760c4d000-7ff76144d000 rw-p 00000000 00:00 0 
7ff76144d000-7ff761463000 r-xp 00000000 08:01 4388593                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff761463000-7ff761662000 ---p 00016000 08:01 4388593                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff761662000-7ff761663000 rw-p 00015000 08:01 4388593                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7ff761663000-7ff76167c000 r-xp 00000000 08:01 4388671                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7ff76167c000-7ff76187b000 ---p 00019000 08:01 4388671                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7ff76187b000-7ff76187c000 r--p 00018000 08:01 4388671                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7ff76187c000-7ff76187d000 rw-p 00019000 08:01 4388671                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7ff76187d000-7ff761985000 r-xp 00000000 08:01 4388604                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff761985000-7ff761b84000 ---p 00108000 08:01 4388604                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff761b84000-7ff761b85000 r--p 00107000 08:01 4388604                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff761b85000-7ff761b86000 rw-p 00108000 08:01 4388604                    /lib/x86_64-linux-gnu/libm-2.23.so
7ff761b86000-7ff761b9e000 r-xp 00000000 08:01 4388640                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff761b9e000-7ff761d9d000 ---p 00018000 08:01 4388640                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff761d9d000-7ff761d9e000 r--p 00017000 08:01 4388640                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff761d9e000-7ff761d9f000 rw-p 00018000 08:01 4388640                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7ff761d9f000-7ff761da3000 rw-p 00000000 00:00 0 
7ff761da3000-7ff761da6000 r-xp 00000000 08:01 4388585                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff761da6000-7ff761fa5000 ---p 00003000 08:01 4388585                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff761fa5000-7ff761fa6000 r--p 00002000 08:01 4388585                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff761fa6000-7ff761fa7000 rw-p 00003000 08:01 4388585                    /lib/x86_64-linux-gnu/libdl-2.23.so
7ff761fa7000-7ff761fae000 r-xp 00000000 08:01 4388646                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff761fae000-7ff7621ad000 ---p 00007000 08:01 4388646                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff7621ad000-7ff7621ae000 r--p 00006000 08:01 4388646                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff7621ae000-7ff7621af000 rw-p 00007000 08:01 4388646                    /lib/x86_64-linux-gnu/librt-2.23.so
7ff7621af000-7ff76236f000 r-xp 00000000 08:01 4388572                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff76236f000-7ff76256f000 ---p 001c0000 08:01 4388572                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff76256f000-7ff762573000 r--p 001c0000 08:01 4388572                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff762573000-7ff762575000 rw-p 001c4000 08:01 4388572                    /lib/x86_64-linux-gnu/libc-2.23.so
7ff762575000-7ff762579000 rw-p 00000000 00:00 0 
7ff762579000-7ff762715000 r-xp 00000000 08:01 14454595                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-9e3036d1402b84ee.so
7ff762715000-7ff762915000 ---p 0019c000 08:01 14454595                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-9e3036d1402b84ee.so
7ff762915000-7ff762926000 r--p 0019c000 08:01 14454595                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-9e3036d1402b84ee.so
7ff762926000-7ff762927000 rw-p 001ad000 08:01 14454595                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-9e3036d1402b84ee.so
7ff762927000-7ff76d774000 r-xp 00000000 08:01 14715272                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-dd5bcd4cdc2c6057.so
7ff76d774000-7ff76d973000 ---p 0ae4d000 08:01 14715272                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-dd5bcd4cdc2c6057.so
7ff76d973000-7ff76e227000 r--p 0ae4c000 08:01 14715272                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-dd5bcd4cdc2c6057.so
7ff76e227000-7ff76e255000 rw-p 0b700000 08:01 14715272                   /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-dd5bcd4cdc2c6057.so
7ff76e255000-7ff76e2cd000 rw-p 00000000 00:00 0 
7ff76e2cd000-7ff76e2f3000 r-xp 00000000 08:01 4388552                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff76e443000-7ff76e487000 rw-p 00000000 00:00 0 
7ff76e4da000-7ff76e4db000 ---p 00000000 00:00 0 
7ff76e4db000-7ff76e4dd000 rw-p 00000000 00:00 0 
7ff76e4dd000-7ff76e4de000 ---p 00000000 00:00 0 
7ff76e4de000-7ff76e4e0000 rw-p 00000000 00:00 0 
7ff76e4e0000-7ff76e4e1000 ---p 00000000 00:00 0 
7ff76e4e1000-7ff76e4e9000 rw-p 00000000 00:00 0 
7ff76e4ea000-7ff76e4eb000 rw-p 00000000 00:00 0 
7ff76e4eb000-7ff76e4ec000 ---p 00000000 00:00 0 
7ff76e4ec000-7ff76e4ee000 rw-p 00000000 00:00 0 
7ff76e4ee000-7ff76e4ef000 ---p 00000000 00:00 0 
7ff76e4ef000-7ff76e4f2000 rw-p 00000000 00:00 0 
7ff76e4f2000-7ff76e4f3000 r--p 00025000 08:01 4388552                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff76e4f3000-7ff76e4f4000 rw-p 00026000 08:01 4388552                    /lib/x86_64-linux-gnu/ld-2.23.so
7ff76e4f4000-7ff76e4f5000 rw-p 00000000 00:00 0 
7ffdbd2de000-7ffdbd301000 rw-p 00000000 00:00 0                          [stack]
7ffdbd397000-7ffdbd39b000 r--p 00000000 00:00 0                          [vvar]
7ffdbd39b000-7ffdbd39d000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
[RUSTC-TIMING] rustc_middle test:false 126.726
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2021 compiler/rustc_middle/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'values(feature, "rustc-rayon", "rustc-rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=8ca6d1bcacfa74c9 -C extra-filename=-8ca6d1bcacfa74c9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-a0f9ad9e790380e4.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-c4038f8d28adc265.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-924fd47eda5a462d.rmeta --extern gsgdt=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgsgdt-51b5ee1770562e83.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-9a6ef456fce62eaa.rmeta --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librand-936e149112727ea0.rmeta --extern rand_xoshiro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librand_xoshiro-0006584bc4389c20.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-0203a9a89213091e.rmeta --extern rustc_arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_arena-f866031a36736f3e.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-218de71c30f772bb.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-962cd8aeb3f008d5.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b5816097eb3814b.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-383812afd966c626.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-0f85b6e8f96c3cf5.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-8307ff5e6e91afd5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-a84ff33154c4aa21.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-f98053a1fb176437.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-423d3bca2d735392.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-f83d419dbbdb58c0.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-5e340c329fd5bf40.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-441d705622a4bcc3.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-90dce1a288f6cb81.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a468455e141dc6a8.rmeta --extern rustc_type_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_type_ir-808e848445d4b650.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-7a7afa36e12edff4.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-840743c069ba125d.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-c5d4bf986938deb6/out` (exit status: 254)
[RUSTC-TIMING] rustc_infer test:false 106.724
[RUSTC-TIMING] rustc_codegen_ssa test:false 92.220
[RUSTC-TIMING] rustc_lint test:false 75.081
[RUSTC-TIMING] rustc_metadata test:false 137.382
