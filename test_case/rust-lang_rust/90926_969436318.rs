plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-10-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
*** Error in `/android/sdk/emulator/emulator64-arm': malloc(): smallbin double linked list corrupted: 0x00007fcdd0438270 ***
======= Backtrace: =========
/lib/x86_64-linux-gnu/libc.so.6(+0x777f5)[0x7fcde0cc87f5]
/lib/x86_64-linux-gnu/libc.so.6(+0x82679)[0x7fcde0cd3679]
/lib/x86_64-linux-gnu/libc.so.6(__libc_malloc+0x54)[0x7fcde0cd51d4]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x264e48)[0x7fcdd4deee48]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x92fa4)[0x7fcdd4c1cfa4]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0xadd9d)[0x7fcdd4c37d9d]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x8dfca)[0x7fcdd4c17fca]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x835d0)[0x7fcdd4c0d5d0]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x850fa)[0x7fcdd4c0f0fa]
/android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so(+0x9f374)[0x7fcdd4c29374]
/android/sdk/emulator/lib64/lib64GLES_CM_translator.so(+0xe0aab)[0x7fcdd6bbbaab]
/android/sdk/emulator/lib64/lib64OpenglRender.so(+0x4c59e)[0x7fcde346b59e]
/android/sdk/emulator/lib64/lib64OpenglRender.so(+0x267c6)[0x7fcde34457c6]
/android/sdk/emulator/lib64/lib64OpenglRender.so(+0x67ec2)[0x7fcde3486ec2]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x76ba)[0x7fcde12386ba]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x7fcde0d5851d]
======= Memory map: ========
00400000-021f3000 r-xp 00000000 00:35 14196284                           /android/sdk/emulator/emulator64-arm
021f3000-02252000 r--p 01df2000 00:35 14196284                           /android/sdk/emulator/emulator64-arm
02252000-022a0000 rw-p 01e51000 00:35 14196284                           /android/sdk/emulator/emulator64-arm
022a0000-02e88000 rw-p 00000000 00:00 0 
02fbc000-0641b000 rw-p 00000000 00:00 0                                  [heap]
417d4000-46cd4000 rwxp 00000000 00:00 0 
7fcd6c000000-7fcd6c021000 rw-p 00000000 00:00 0 
7fcd6c021000-7fcd70000000 ---p 00000000 00:00 0 
7fcd74000000-7fcd7414e000 rw-p 00000000 00:00 0 
7fcd7414e000-7fcd78000000 ---p 00000000 00:00 0 
7fcd7aff3000-7fcd7aff4000 ---p 00000000 00:00 0 
7fcd7aff4000-7fcd7bff4000 rw-p 00000000 00:00 0 
7fcd7bff4000-7fcd7bff5000 ---p 00000000 00:00 0 
7fcd7bff5000-7fcd7cff5000 rw-p 00000000 00:00 0 
7fcd7cff5000-7fcd7cff6000 ---p 00000000 00:00 0 
7fcd7cff6000-7fcd7dff6000 rw-p 00000000 00:00 0 
7fcd7dff6000-7fcd7dff7000 ---p 00000000 00:00 0 
7fcd7dff7000-7fcd7eff7000 rw-p 00000000 00:00 0 
7fcd7eff7000-7fcd7eff8000 ---p 00000000 00:00 0 
7fcd7eff8000-7fcd7fff8000 rw-p 00000000 00:00 0 
7fcd7fff8000-7fcd7fff9000 ---p 00000000 00:00 0 
7fcd7fff9000-7fcd80ff9000 rw-p 00000000 00:00 0 
7fcd80ff9000-7fcd80ffa000 ---p 00000000 00:00 0 
7fcd80ffa000-7fcd81ffa000 rw-p 00000000 00:00 0 
7fcd81ffa000-7fcd81ffb000 ---p 00000000 00:00 0 
7fcd81ffb000-7fcd82ffb000 rw-p 00000000 00:00 0 
7fcd82ffb000-7fcd82ffc000 ---p 00000000 00:00 0 
7fcd82ffc000-7fcd83ffc000 rw-p 00000000 00:00 0 
7fcd83ffc000-7fcd83ffd000 ---p 00000000 00:00 0 
7fcd83ffd000-7fcd84ffd000 rw-p 00000000 00:00 0 
7fcd84ffd000-7fcd84ffe000 ---p 00000000 00:00 0 
7fcd84ffe000-7fcd85ffe000 rw-p 00000000 00:00 0 
7fcd85ffe000-7fcd85fff000 ---p 00000000 00:00 0 
7fcd85fff000-7fcd86fff000 rw-p 00000000 00:00 0 
7fcd86fff000-7fcd87000000 ---p 00000000 00:00 0 
7fcd87000000-7fcd88000000 rw-p 00000000 00:00 0 
7fcd88000000-7fcd8acda000 rw-p 00000000 00:00 0 
7fcd8acda000-7fcd8c000000 ---p 00000000 00:00 0 
7fcd8cffd000-7fcd8cffe000 ---p 00000000 00:00 0 
7fcd8cffe000-7fcd8dffe000 rw-p 00000000 00:00 0 
7fcd8dffe000-7fcd8dfff000 ---p 00000000 00:00 0 
7fcd8dfff000-7fcd8efff000 rw-p 00000000 00:00 0 
7fcd8efff000-7fcd8f000000 ---p 00000000 00:00 0 
7fcd8f000000-7fcd90000000 rw-p 00000000 00:00 0 
7fcd90000000-7fcd9027a000 rw-p 00000000 00:00 0 
7fcd9027a000-7fcd94000000 ---p 00000000 00:00 0 
7fcd94ff5000-7fcd94ff6000 ---p 00000000 00:00 0 
7fcd94ff6000-7fcd95ff6000 rw-p 00000000 00:00 0 
7fcd95ff6000-7fcd95ff7000 ---p 00000000 00:00 0 
7fcd95ff7000-7fcd96ff7000 rw-p 00000000 00:00 0 
7fcd96ff7000-7fcd96ff8000 ---p 00000000 00:00 0 
7fcd96ff8000-7fcd97ff8000 rw-p 00000000 00:00 0 
7fcd97ff8000-7fcd97ff9000 ---p 00000000 00:00 0 
7fcd97ff9000-7fcd98ff9000 rw-p 00000000 00:00 0 
7fcd98ff9000-7fcd98ffa000 ---p 00000000 00:00 0 
7fcd98ffa000-7fcd99ffa000 rw-p 00000000 00:00 0 
7fcd99ffa000-7fcd99ffb000 ---p 00000000 00:00 0 
7fcd99ffb000-7fcd9affb000 rw-p 00000000 00:00 0 
7fcd9affb000-7fcd9affc000 ---p 00000000 00:00 0 
7fcd9affc000-7fcd9bffc000 rw-p 00000000 00:00 0 
7fcd9bffc000-7fcd9bffd000 ---p 00000000 00:00 0 
7fcd9bffd000-7fcd9cffd000 rw-p 00000000 00:00 0 
7fcd9cffd000-7fcd9cffe000 ---p 00000000 00:00 0 
7fcd9cffe000-7fcd9dffe000 rw-p 00000000 00:00 0 
7fcd9dffe000-7fcd9dfff000 ---p 00000000 00:00 0 
7fcd9dfff000-7fcd9efff000 rw-p 00000000 00:00 0 
7fcd9efff000-7fcd9f000000 ---p 00000000 00:00 0 
7fcd9f000000-7fcda0000000 rw-p 00000000 00:00 0 
7fcda0000000-7fcda4000000 rw-p 00000000 00:00 0 
7fcda4541000-7fcda4542000 ---p 00000000 00:00 0 
7fcda4542000-7fcda4742000 rw-p 00000000 00:00 0 
7fcda4742000-7fcda4743000 ---p 00000000 00:00 0 
7fcda4743000-7fcda4943000 rw-p 00000000 00:00 0 
7fcda4943000-7fcda4944000 ---p 00000000 00:00 0 
7fcda4944000-7fcda5944000 rw-p 00000000 00:00 0 
7fcda5944000-7fcda5945000 ---p 00000000 00:00 0 
7fcda5945000-7fcda6945000 rw-p 00000000 00:00 0 
7fcda6945000-7fcda6946000 ---p 00000000 00:00 0 
7fcda6946000-7fcda7946000 rw-p 00000000 00:00 0 
7fcda7946000-7fcda7947000 ---p 00000000 00:00 0 
7fcda7947000-7fcda7b47000 rw-p 00000000 00:00 0 
7fcda7b47000-7fcda7b48000 ---p 00000000 00:00 0 
7fcda7b48000-7fcda8b48000 rw-p 00000000 00:00 0 
7fcda8b48000-7fcdacb49000 rw-s 00000000 00:3c 3                          /dev/shm/pulse-shm-2326517884
7fcdacb49000-7fcdacb73000 r-xp 00000000 00:35 14193357                   /usr/lib/x86_64-linux-gnu/libvorbis.so.0.4.8
7fcdacb73000-7fcdacd72000 ---p 0002a000 00:35 14193357                   /usr/lib/x86_64-linux-gnu/libvorbis.so.0.4.8
7fcdacd72000-7fcdacd73000 r--p 00029000 00:35 14193357                   /usr/lib/x86_64-linux-gnu/libvorbis.so.0.4.8
7fcdacd73000-7fcdacd74000 rw-p 0002a000 00:35 14193357                   /usr/lib/x86_64-linux-gnu/libvorbis.so.0.4.8
7fcdacd74000-7fcdacd7b000 r-xp 00000000 00:35 14193340                   /usr/lib/x86_64-linux-gnu/libogg.so.0.8.2
7fcdacd7b000-7fcdacf7b000 ---p 00007000 00:35 14193340                   /usr/lib/x86_64-linux-gnu/libogg.so.0.8.2
7fcdacf7b000-7fcdacf7c000 r--p 00007000 00:35 14193340                   /usr/lib/x86_64-linux-gnu/libogg.so.0.8.2
7fcdacf7c000-7fcdacf7d000 rw-p 00008000 00:35 14193340                   /usr/lib/x86_64-linux-gnu/libogg.so.0.8.2
7fcdacf7d000-7fcdacf8f000 r-xp 00000000 00:35 4128934                    /lib/x86_64-linux-gnu/libgpg-error.so.0.17.0
7fcdacf8f000-7fcdad18f000 ---p 00012000 00:35 4128934                    /lib/x86_64-linux-gnu/libgpg-error.so.0.17.0
7fcdad18f000-7fcdad190000 r--p 00012000 00:35 4128934                    /lib/x86_64-linux-gnu/libgpg-error.so.0.17.0
7fcdad190000-7fcdad191000 rw-p 00013000 00:35 4128934                    /lib/x86_64-linux-gnu/libgpg-error.so.0.17.0
7fcdad191000-7fcdad1ff000 r-xp 00000000 00:35 4128974                    /lib/x86_64-linux-gnu/libpcre.so.3.13.2
7fcdad1ff000-7fcdad3ff000 ---p 0006e000 00:35 4128974                    /lib/x86_64-linux-gnu/libpcre.so.3.13.2
7fcdad3ff000-7fcdad400000 r--p 0006e000 00:35 4128974                    /lib/x86_64-linux-gnu/libpcre.so.3.13.2
7fcdad400000-7fcdad401000 rw-p 0006f000 00:35 4128974                    /lib/x86_64-linux-gnu/libpcre.so.3.13.2
7fcdad401000-7fcdad48e000 r-xp 00000000 00:35 14193359                   /usr/lib/x86_64-linux-gnu/libvorbisenc.so.2.0.11
7fcdad48e000-7fcdad68d000 ---p 0008d000 00:35 14193359                   /usr/lib/x86_64-linux-gnu/libvorbisenc.so.2.0.11
7fcdad68d000-7fcdad6a9000 r--p 0008c000 00:35 14193359                   /usr/lib/x86_64-linux-gnu/libvorbisenc.so.2.0.11
7fcdad6a9000-7fcdad6aa000 rw-p 000a8000 00:35 14193359                   /usr/lib/x86_64-linux-gnu/libvorbisenc.so.2.0.11
7fcdad6aa000-7fcdad71d000 r-xp 00000000 00:35 14193282                   /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0
7fcdad71d000-7fcdad91d000 ---p 00073000 00:35 14193282                   /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0
7fcdad91d000-7fcdad91e000 r--p 00073000 00:35 14193282                   /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0
7fcdad91e000-7fcdad91f000 rw-p 00074000 00:35 14193282                   /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0
7fcdad91f000-7fcdad9f6000 r-xp 00000000 00:35 4128932                    /lib/x86_64-linux-gnu/libgcrypt.so.20.0.5
7fcdad9f6000-7fcdadbf6000 ---p 000d7000 00:35 4128932                    /lib/x86_64-linux-gnu/libgcrypt.so.20.0.5
7fcdadbf6000-7fcdadbf7000 r--p 000d7000 00:35 4128932                    /lib/x86_64-linux-gnu/libgcrypt.so.20.0.5
7fcdadbf7000-7fcdadbff000 rw-p 000d8000 00:35 4128932                    /lib/x86_64-linux-gnu/libgcrypt.so.20.0.5
7fcdadbff000-7fcdadc00000 rw-p 00000000 00:00 0 
7fcdadc00000-7fcdc3000000 rw-p 00000000 00:00 0 
7fcdc3003000-7fcdc3030000 r-xp 00000000 00:00 0 
7fcdc3030000-7fcdc8000000 rw-p 00000000 00:00 0 
7fcdc8000000-7fcdc8b8a000 rw-p 00000000 00:00 0 
7fcdc8b8a000-7fcdcc000000 ---p 00000000 00:00 0 
7fcdcc01d000-7fcdcc04a000 r-xp 00000000 00:00 0 
7fcdcc04a000-7fcdcc06a000 rw-p 00000000 00:00 0 
7fcdcc091000-7fcdcc0a7000 r-xp 00000000 00:35 4128952                    /lib/x86_64-linux-gnu/libnsl-2.23.so
7fcdcc0a7000-7fcdcc2a6000 ---p 00016000 00:35 4128952                    /lib/x86_64-linux-gnu/libnsl-2.23.so
7fcdcc2a6000-7fcdcc2a7000 r--p 00015000 00:35 4128952                    /lib/x86_64-linux-gnu/libnsl-2.23.so
7fcdcc2a7000-7fcdcc2a8000 rw-p 00016000 00:35 4128952                    /lib/x86_64-linux-gnu/libnsl-2.23.so
7fcdcc2a8000-7fcdcc2aa000 rw-p 00000000 00:00 0 
7fcdcc2aa000-7fcdcc2cb000 r-xp 00000000 00:35 4128940                    /lib/x86_64-linux-gnu/liblzma.so.5.0.0
7fcdcc2cb000-7fcdcc4ca000 ---p 00021000 00:35 4128940                    /lib/x86_64-linux-gnu/liblzma.so.5.0.0
7fcdcc4ca000-7fcdcc4cb000 r--p 00020000 00:35 4128940                    /lib/x86_64-linux-gnu/liblzma.so.5.0.0
7fcdcc4cb000-7fcdcc4cc000 rw-p 00021000 00:35 4128940                    /lib/x86_64-linux-gnu/liblzma.so.5.0.0
7fcdcc4cc000-7fcdcc4eb000 r-xp 00000000 00:35 4128987                    /lib/x86_64-linux-gnu/libselinux.so.1
7fcdcc4eb000-7fcdcc6ea000 ---p 0001f000 00:35 4128987                    /lib/x86_64-linux-gnu/libselinux.so.1
7fcdcc6ea000-7fcdcc6eb000 r--p 0001e000 00:35 4128987                    /lib/x86_64-linux-gnu/libselinux.so.1
7fcdcc6eb000-7fcdcc6ec000 rw-p 0001f000 00:35 4128987                    /lib/x86_64-linux-gnu/libselinux.so.1
7fcdcc6ec000-7fcdcc6ee000 rw-p 00000000 00:00 0 
7fcdcc6ee000-7fcdcc6fd000 r-xp 00000000 00:35 4128899                    /lib/x86_64-linux-gnu/libapparmor.so.1.4.0
7fcdcc6fd000-7fcdcc8fc000 ---p 0000f000 00:35 4128899                    /lib/x86_64-linux-gnu/libapparmor.so.1.4.0
7fcdcc8fc000-7fcdcc8fd000 r--p 0000e000 00:35 4128899                    /lib/x86_64-linux-gnu/libapparmor.so.1.4.0
7fcdcc8fd000-7fcdcc8fe000 rw-p 0000f000 00:35 4128899                    /lib/x86_64-linux-gnu/libapparmor.so.1.4.0
7fcdcc8fe000-7fcdcc903000 r-xp 00000000 00:35 14193308                   /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1
7fcdcc903000-7fcdccb02000 ---p 00005000 00:35 14193308                   /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1
7fcdccb02000-7fcdccb03000 r--p 00004000 00:35 14193308                   /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1
7fcdccb03000-7fcdccb04000 rw-p 00005000 00:35 14193308                   /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1
7fcdccb04000-7fcdccb67000 r-xp 00000000 00:35 14193354                   /usr/lib/x86_64-linux-gnu/libsndfile.so.1.0.25
7fcdccb67000-7fcdccd67000 ---p 00063000 00:35 14193354                   /usr/lib/x86_64-linux-gnu/libsndfile.so.1.0.25
7fcdccd67000-7fcdccd69000 r--p 00063000 00:35 14193354                   /usr/lib/x86_64-linux-gnu/libsndfile.so.1.0.25
7fcdccd69000-7fcdccd6a000 rw-p 00065000 00:35 14193354                   /usr/lib/x86_64-linux-gnu/libsndfile.so.1.0.25
7fcdccd6a000-7fcdccd6e000 rw-p 00000000 00:00 0 
7fcdccd6e000-7fcdccd76000 r-xp 00000000 00:35 14192800                   /lib/x86_64-linux-gnu/libwrap.so.0.7.6
7fcdccd76000-7fcdccf75000 ---p 00008000 00:35 14192800                   /lib/x86_64-linux-gnu/libwrap.so.0.7.6
7fcdccf75000-7fcdccf76000 r--p 00007000 00:35 14192800                   /lib/x86_64-linux-gnu/libwrap.so.0.7.6
7fcdccf76000-7fcdccf77000 rw-p 00008000 00:35 14192800                   /lib/x86_64-linux-gnu/libwrap.so.0.7.6
7fcdccf77000-7fcdccf78000 rw-p 00000000 00:00 0 
7fcdccf78000-7fcdccff8000 r-xp 00000000 00:35 4128994                    /lib/x86_64-linux-gnu/libsystemd.so.0.14.0
7fcdccff8000-7fcdccffb000 r--p 0007f000 00:35 4128994                    /lib/x86_64-linux-gnu/libsystemd.so.0.14.0
7fcdccffb000-7fcdccffc000 rw-p 00082000 00:35 4128994                    /lib/x86_64-linux-gnu/libsystemd.so.0.14.0
7fcdccffc000-7fcdccffd000 rw-p 00000000 00:00 0 
7fcdccffd000-7fcdccffe000 ---p 00000000 00:00 0 
7fcdccffe000-7fcdcdffe000 rw-p 00000000 00:00 0 
7fcdcdffe000-7fcdcdfff000 ---p 00000000 00:00 0 
7fcdcdfff000-7fcdcefff000 rw-p 00000000 00:00 0 
7fcdcefff000-7fcdcf000000 ---p 00000000 00:00 0 
7fcdcf000000-7fcdd0000000 rw-p 00000000 00:00 0 
7fcdd0000000-7fcdd3d5b000 rw-p 00000000 00:00 0 
7fcdd3d5b000-7fcdd4000000 ---p 00000000 00:00 0 
7fcdd4021000-7fcdd4028000 r-xp 00000000 00:00 0 
7fcdd4028000-7fcdd402b000 rw-p 00000000 00:00 0 
7fcdd402b000-7fcdd402e000 r-xp 00000000 00:00 0 
7fcdd402e000-7fcdd402f000 rw-p 00000000 00:00 0 
7fcdd402f000-7fcdd4034000 r-xp 00000000 00:00 0 
7fcdd4034000-7fcdd4035000 rw-p 00000000 00:00 0 
7fcdd4035000-7fcdd4038000 r-xp 00000000 00:00 0 
7fcdd4038000-7fcdd4039000 rw-p 00000000 00:00 0 
7fcdd4039000-7fcdd403e000 r-xp 00000000 00:00 0 
7fcdd403e000-7fcdd4041000 rw-p 00000000 00:00 0 
7fcdd4041000-7fcdd408c000 r-xp 00000000 00:35 14192794                   /lib/x86_64-linux-gnu/libdbus-1.so.3.14.6
7fcdd408c000-7fcdd428b000 ---p 0004b000 00:35 14192794                   /lib/x86_64-linux-gnu/libdbus-1.so.3.14.6
7fcdd428b000-7fcdd428c000 r--p 0004a000 00:35 14192794                   /lib/x86_64-linux-gnu/libdbus-1.so.3.14.6
7fcdd428c000-7fcdd428d000 rw-p 0004b000 00:35 14192794                   /lib/x86_64-linux-gnu/libdbus-1.so.3.14.6
7fcdd428d000-7fcdd4308000 r-xp 00000000 00:35 14193390                   /usr/lib/x86_64-linux-gnu/pulseaudio/libpulsecommon-8.0.so
7fcdd4308000-7fcdd4508000 ---p 0007b000 00:35 14193390                   /usr/lib/x86_64-linux-gnu/pulseaudio/libpulsecommon-8.0.so
7fcdd4508000-7fcdd4509000 r--p 0007b000 00:35 14193390                   /usr/lib/x86_64-linux-gnu/pulseaudio/libpulsecommon-8.0.so
7fcdd4509000-7fcdd450a000 rw-p 0007c000 00:35 14193390                   /usr/lib/x86_64-linux-gnu/pulseaudio/libpulsecommon-8.0.so
7fcdd450a000-7fcdd4558000 r-xp 00000000 00:35 14193349                   /usr/lib/x86_64-linux-gnu/libpulse.so.0.19.0
7fcdd4558000-7fcdd4758000 ---p 0004e000 00:35 14193349                   /usr/lib/x86_64-linux-gnu/libpulse.so.0.19.0
7fcdd4758000-7fcdd4759000 r--p 0004e000 00:35 14193349                   /usr/lib/x86_64-linux-gnu/libpulse.so.0.19.0
7fcdd4759000-7fcdd475a000 rw-p 0004f000 00:35 14193349                   /usr/lib/x86_64-linux-gnu/libpulse.so.0.19.0
7fcdd475a000-7fcdd475d000 r-xp 00000000 00:35 4129773                    /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
7fcdd475d000-7fcdd495c000 ---p 00003000 00:35 4129773                    /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
7fcdd495c000-7fcdd495d000 r--p 00002000 00:35 4129773                    /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
7fcdd495d000-7fcdd495e000 rw-p 00003000 00:35 4129773                    /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
7fcdd495e000-7fcdd495f000 ---p 00000000 00:00 0 
7fcdd495f000-7fcdd4b5f000 rw-p 00000000 00:00 0 
7fcdd4b5f000-7fcdd4b86000 r-xp 00000000 00:35 14196344                   /android/sdk/emulator/lib64/gles_swiftshader/libEGL.so
7fcdd4b86000-7fcdd4b87000 ---p 00027000 00:35 14196344                   /android/sdk/emulator/lib64/gles_swiftshader/libEGL.so
7fcdd4b87000-7fcdd4b89000 r--p 00027000 00:35 14196344                   /android/sdk/emulator/lib64/gles_swiftshader/libEGL.so
7fcdd4b89000-7fcdd4b8a000 rw-p 00029000 00:35 14196344                   /android/sdk/emulator/lib64/gles_swiftshader/libEGL.so
7fcdd4b8a000-7fcdd4ef1000 r-xp 00000000 00:35 14196346                   /android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so
7fcdd4ef1000-7fcdd4f0f000 r--p 00366000 00:35 14196346                   /android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so
7fcdd4f0f000-7fcdd4f10000 rw-p 00384000 00:35 14196346                   /android/sdk/emulator/lib64/gles_swiftshader/libGLESv2.so
7fcdd4f10000-7fcdd4f62000 rw-p 00000000 00:00 0 
7fcdd4f62000-7fcdd4f63000 ---p 00000000 00:00 0 
7fcdd4f63000-7fcdd5f63000 rw-p 00000000 00:00 0 
7fcdd5f63000-7fcdd66a2000 r-xp 00000000 00:35 14196352                   /android/sdk/emulator/lib64/lib64GLES_V2_translator.so
7fcdd66a2000-7fcdd66a3000 ---p 0073f000 00:35 14196352                   /android/sdk/emulator/lib64/lib64GLES_V2_translator.so
7fcdd66a3000-7fcdd66b5000 r--p 0073f000 00:35 14196352                   /android/sdk/emulator/lib64/lib64GLES_V2_translator.so
7fcdd66b5000-7fcdd66b6000 rw-p 00751000 00:35 14196352                   /android/sdk/emulator/lib64/lib64GLES_V2_translator.so
7fcdd66b6000-7fcdd66b9000 rw-p 00000000 00:00 0 
7fcdd66b9000-7fcdd66d0000 r-xp 00000000 00:35 4128981                    /lib/x86_64-linux-gnu/libresolv-2.23.so
7fcdd66d0000-7fcdd68d0000 ---p 00017000 00:35 4128981                    /lib/x86_64-linux-gnu/libresolv-2.23.so
7fcdd68d0000-7fcdd68d1000 r--p 00017000 00:35 4128981                    /lib/x86_64-linux-gnu/libresolv-2.23.so
7fcdd68d1000-7fcdd68d2000 rw-p 00018000 00:35 4128981                    /lib/x86_64-linux-gnu/libresolv-2.23.so
7fcdd68d2000-7fcdd68d4000 rw-p 00000000 00:00 0 
7fcdd68d4000-7fcdd68d9000 r-xp 00000000 00:35 4128956                    /lib/x86_64-linux-gnu/libnss_dns-2.23.so
7fcdd68d9000-7fcdd6ad9000 ---p 00005000 00:35 4128956                    /lib/x86_64-linux-gnu/libnss_dns-2.23.so
7fcdd6ad9000-7fcdd6ada000 r--p 00005000 00:35 4128956                    /lib/x86_64-linux-gnu/libnss_dns-2.23.so
7fcdd6ada000-7fcdd6adb000 rw-p 00006000 00:35 4128956                    /lib/x86_64-linux-gnu/libnss_dns-2.23.so
7fcdd6adb000-7fcdd6fee000 r-xp 00000000 00:35 14196351                   /android/sdk/emulator/lib64/lib64GLES_CM_translator.so
7fcdd6fee000-7fcdd6ffc000 r--p 00513000 00:35 14196351                   /android/sdk/emulator/lib64/lib64GLES_CM_translator.so
7fcdd6ffc000-7fcdd6ffd000 rw-p 00521000 00:35 14196351                   /android/sdk/emulator/lib64/lib64GLES_CM_translator.so
7fcdd6ffd000-7fcdd6fff000 rw-p 00000000 00:00 0 
7fcdd6fff000-7fcdd7000000 ---p 00000000 00:00 0 
7fcdd7000000-7fcdd8000000 rw-p 00000000 00:00 0 
7fcdd8000000-7fcddbca9000 rw-p 00000000 00:00 0 
7fcddbca9000-7fcddc000000 ---p 00000000 00:00 0 
7fcddc000000-7fcddc002000 r-xp 00000000 00:00 0 
7fcddc002000-7fcddc003000 rw-p 00000000 00:00 0 
7fcddc003000-7fcddc006000 r-xp 00000000 00:00 0 
7fcddc006000-7fcddc007000 rw-p 00000000 00:00 0 
7fcddc007000-7fcddc012000 r-xp 00000000 00:35 4128958                    /lib/x86_64-linux-gnu/libnss_files-2.23.so
7fcddc012000-7fcddc211000 ---p 0000b000 00:35 4128958                    /lib/x86_64-linux-gnu/libnss_files-2.23.so
7fcddc211000-7fcddc212000 r--p 0000a000 00:35 4128958                    /lib/x86_64-linux-gnu/libnss_files-2.23.so
7fcddc212000-7fcddc213000 rw-p 0000b000 00:35 4128958                    /lib/x86_64-linux-gnu/libnss_files-2.23.so
7fcddc213000-7fcddc219000 rw-p 00000000 00:00 0 
7fcddc219000-7fcddc62d000 r-xp 00000000 00:35 14196349                   /android/sdk/emulator/lib64/lib64EGL_translator.so
7fcddc62d000-7fcddc62e000 ---p 00414000 00:35 14196349                   /android/sdk/emulator/lib64/lib64EGL_translator.so
7fcddc62e000-7fcddc636000 r--p 00414000 00:35 14196349                   /android/sdk/emulator/lib64/lib64EGL_translator.so
7fcddc636000-7fcddc637000 rw-p 0041c000 00:35 14196349                   /android/sdk/emulator/lib64/lib64EGL_translator.so
7fcddc637000-7fcddc639000 rw-p 00000000 00:00 0 
7fcddc639000-7fcddc63a000 ---p 00000000 00:00 0 
7fcddc63a000-7fcddd63a000 rw-p 00000000 00:00 0 
7fcddd63a000-7fcddd63b000 ---p 00000000 00:00 0 
7fcddd63b000-7fcdde63b000 rw-p 00000000 00:00 0 
7fcdde63b000-7fcdde640000 r-xp 00000000 00:35 14193294                   /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
7fcdde640000-7fcdde83f000 ---p 00005000 00:35 14193294                   /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
7fcdde83f000-7fcdde840000 r--p 00004000 00:35 14193294                   /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
7fcdde840000-7fcdde841000 rw-p 00005000 00:35 14193294                   /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
7fcdde841000-7fcdde843000 r-xp 00000000 00:35 14193290                   /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
7fcdde843000-7fcddea43000 ---p 00002000 00:35 14193290                   /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
7fcddea43000-7fcddea44000 r--p 00002000 00:35 14193290                   /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
7fcddea44000-7fcddea45000 rw-p 00003000 00:35 14193290                   /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
7fcddea45000-7fcddeb4b000 r-xp 00000000 00:35 14196424                   /android/sdk/emulator/lib64/qt/lib/libc++.so
7fcddeb4b000-7fcddeb4c000 ---p 00106000 00:35 14196424                   /android/sdk/emulator/lib64/qt/lib/libc++.so
7fcddeb4c000-7fcddeb54000 r--p 00106000 00:35 14196424                   /android/sdk/emulator/lib64/qt/lib/libc++.so
7fcddeb54000-7fcddeb55000 rw-p 0010e000 00:35 14196424                   /android/sdk/emulator/lib64/qt/lib/libc++.so
7fcddeb55000-7fcddeb58000 rw-p 00000000 00:00 0 
7fcddeb58000-7fcddeb68000 r-xp 00000000 00:35 14193315                   /usr/lib/x86_64-linux-gnu/libdrm.so.2.4.0
7fcddeb68000-7fcdded68000 ---p 00010000 00:35 14193315                   /usr/lib/x86_64-linux-gnu/libdrm.so.2.4.0
7fcdded68000-7fcdded69000 r--p 00010000 00:35 14193315                   /usr/lib/x86_64-linux-gnu/libdrm.so.2.4.0
7fcdded69000-7fcdded6a000 rw-p 00011000 00:35 14193315                   /usr/lib/x86_64-linux-gnu/libdrm.so.2.4.0
7fcdded6a000-7fcdded6f000 r-xp 00000000 00:35 14193306                   /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0
7fcdded6f000-7fcddef6e000 ---p 00005000 00:35 14193306                   /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0
7fcddef6e000-7fcddef6f000 r--p 00004000 00:35 14193306                   /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0
7fcddef6f000-7fcddef70000 rw-p 00005000 00:35 14193306                   /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0
7fcddef70000-7fcddef74000 r-xp 00000000 00:35 14193361                   /usr/lib/x86_64-linux-gnu/libxcb-dri2.so.0.0.0
7fcddef74000-7fcddf173000 ---p 00004000 00:35 14193361                   /usr/lib/x86_64-linux-gnu/libxcb-dri2.so.0.0.0
7fcddf173000-7fcddf174000 r--p 00003000 00:35 14193361                   /usr/lib/x86_64-linux-gnu/libxcb-dri2.so.0.0.0
7fcddf174000-7fcddf175000 rw-p 00004000 00:35 14193361                   /usr/lib/x86_64-linux-gnu/libxcb-dri2.so.0.0.0
7fcddf175000-7fcddf18c000 r-xp 00000000 00:35 14193365                   /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0
7fcddf18c000-7fcddf38b000 ---p 00017000 00:35 14193365                   /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0
7fcddf38b000-7fcddf38d000 r--p 00016000 00:35 14193365                   /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0
7fcddf38d000-7fcddf38e000 rw-p 00018000 00:35 14193365                   /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0
7fcddf38e000-7fcddf38f000 r-xp 00000000 00:35 14193286                   /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0
7fcddf38f000-7fcddf58e000 ---p 00001000 00:35 14193286                   /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0
7fcddf58e000-7fcddf58f000 r--p 00000000 00:35 14193286                   /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0
7fcddf58f000-7fcddf590000 rw-p 00001000 00:35 14193286                   /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0
7fcddf590000-7fcddf595000 r-xp 00000000 00:35 14193298                   /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
7fcddf595000-7fcddf794000 ---p 00005000 00:35 14193298                   /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
7fcddf794000-7fcddf795000 r--p 00004000 00:35 14193298                   /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
7fcddf795000-7fcddf796000 rw-p 00005000 00:35 14193298                   /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
7fcddf796000-7fcddf798000 r-xp 00000000 00:35 14193292                   /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
7fcddf798000-7fcddf997000 ---p 00002000 00:35 14193292                   /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
7fcddf997000-7fcddf998000 r--p 00001000 00:35 14193292                   /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
7fcddf998000-7fcddf999000 rw-p 00002000 00:35 14193292                   /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
7fcddf999000-7fcddf9aa000 r-xp 00000000 00:35 14193296                   /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
7fcddf9aa000-7fcddfba9000 ---p 00011000 00:35 14193296                   /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
7fcddfba9000-7fcddfbaa000 r--p 00010000 00:35 14193296                   /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
7fcddfbaa000-7fcddfbab000 rw-p 00011000 00:35 14193296                   /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
7fcddfbab000-7fcddfbd7000 r-xp 00000000 00:35 14193331                   /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0
7fcddfbd7000-7fcddfdd6000 ---p 0002c000 00:35 14193331                   /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0
7fcddfdd6000-7fcddfdda000 r--p 0002b000 00:35 14193331                   /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0
7fcddfdda000-7fcddfddb000 rw-p 0002f000 00:35 14193331                   /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0
7fcddfddb000-7fcddfddc000 rw-p 00000000 00:00 0 
7fcddfddc000-7fcddfddd000 r-xp 00000000 00:35 14193373                   /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0
7fcddfddd000-7fcddffdd000 ---p 00001000 00:35 14193373                   /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0
7fcddffdd000-7fcddffde000 r--p 00001000 00:35 14193373                   /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0
7fcddffde000-7fcddffdf000 rw-p 00002000 00:35 14193373                   /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0
7fcddffdf000-7fcddffe4000 r-xp 00000000 00:35 14193369                   /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0
7fcddffe4000-7fcde01e4000 ---p 00005000 00:35 14193369                   /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0
7fcde01e4000-7fcde01e5000 r--p 00005000 00:35 14193369                   /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0
7fcde01e5000-7fcde01e6000 rw-p 00006000 00:35 14193369                   /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0
7fcde01e6000-7fcde01e8000 r-xp 00000000 00:35 14193367                   /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0
7fcde01e8000-7fcde03e7000 ---p 00002000 00:35 14193367                   /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0
7fcde03e7000-7fcde03e8000 r--p 00001000 00:35 14193367                   /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0
7fcde03e8000-7fcde03e9000 rw-p 00002000 00:35 14193367                   /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0
7fcde03e9000-7fcde03eb000 r-xp 00000000 00:35 14193363                   /usr/lib/x86_64-linux-gnu/libxcb-dri3.so.0.0.0
7fcde03eb000-7fcde05ea000 ---p 00002000 00:35 14193363                   /usr/lib/x86_64-linux-gnu/libxcb-dri3.so.0.0.0
7fcde05ea000-7fcde05eb000 r--p 00001000 00:35 14193363                   /usr/lib/x86_64-linux-gnu/libxcb-dri3.so.0.0.0
7fcde05eb000-7fcde05ec000 rw-p 00002000 00:35 14193363                   /usr/lib/x86_64-linux-gnu/libxcb-dri3.so.0.0.0
7fcde05ec000-7fcde0612000 r-xp 00000000 00:35 11619126                   /lib/x86_64-linux-gnu/libexpat.so.1.6.0
7fcde0612000-7fcde0812000 ---p 00026000 00:35 11619126                   /lib/x86_64-linux-gnu/libexpat.so.1.6.0
7fcde0812000-7fcde0814000 r--p 00026000 00:35 11619126                   /lib/x86_64-linux-gnu/libexpat.so.1.6.0
7fcde0814000-7fcde0815000 rw-p 00028000 00:35 11619126                   /lib/x86_64-linux-gnu/libexpat.so.1.6.0
7fcde0815000-7fcde082e000 r-xp 00000000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7fcde082e000-7fcde0a2d000 ---p 00019000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7fcde0a2d000-7fcde0a2e000 r--p 00018000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7fcde0a2e000-7fcde0a2f000 rw-p 00019000 00:35 4129008                    /lib/x86_64-linux-gnu/libz.so.1.2.8
7fcde0a2f000-7fcde0a50000 r-xp 00000000 00:35 14193371                   /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
7fcde0a50000-7fcde0c4f000 ---p 00021000 00:35 14193371                   /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
7fcde0c4f000-7fcde0c50000 r--p 00020000 00:35 14193371                   /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
7fcde0c50000-7fcde0c51000 rw-p 00021000 00:35 14193371                   /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
7fcde0c51000-7fcde0e11000 r-xp 00000000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fcde0e11000-7fcde1011000 ---p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fcde1011000-7fcde1015000 r--p 001c0000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fcde1015000-7fcde1017000 rw-p 001c4000 00:35 4128909                    /lib/x86_64-linux-gnu/libc-2.23.so
7fcde1017000-7fcde101b000 rw-p 00000000 00:00 0 
7fcde101b000-7fcde1031000 r-xp 00000000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fcde1031000-7fcde1230000 ---p 00016000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fcde1230000-7fcde1231000 rw-p 00015000 00:35 4128930                    /lib/x86_64-linux-gnu/libgcc_s.so.1
7fcde1231000-7fcde1249000 r-xp 00000000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fcde1249000-7fcde1448000 ---p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fcde1448000-7fcde1449000 r--p 00017000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fcde1449000-7fcde144a000 rw-p 00018000 00:35 4128977                    /lib/x86_64-linux-gnu/libpthread-2.23.so
7fcde144a000-7fcde144e000 rw-p 00000000 00:00 0 
7fcde144e000-7fcde1450000 r-xp 00000000 00:35 4129003                    /lib/x86_64-linux-gnu/libutil-2.23.so
7fcde1450000-7fcde164f000 ---p 00002000 00:35 4129003                    /lib/x86_64-linux-gnu/libutil-2.23.so
7fcde164f000-7fcde1650000 r--p 00001000 00:35 4129003                    /lib/x86_64-linux-gnu/libutil-2.23.so
7fcde1650000-7fcde1651000 rw-p 00002000 00:35 4129003                    /lib/x86_64-linux-gnu/libutil-2.23.so
7fcde1651000-7fcde1759000 r-xp 00000000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fcde1759000-7fcde1958000 ---p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fcde1958000-7fcde1959000 r--p 00107000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fcde1959000-7fcde195a000 rw-p 00108000 00:35 4128941                    /lib/x86_64-linux-gnu/libm-2.23.so
7fcde195a000-7fcde1eb5000 r-xp 00000000 00:35 14196377                   /android/sdk/emulator/lib64/qt/lib/libQt5Core.so.5
7fcde1eb5000-7fcde1ec3000 r--p 0055a000 00:35 14196377                   /android/sdk/emulator/lib64/qt/lib/libQt5Core.so.5
7fcde1ec3000-7fcde1ee0000 rw-p 00568000 00:35 14196377                   /android/sdk/emulator/lib64/qt/lib/libQt5Core.so.5
7fcde1ee0000-7fcde1ee3000 rw-p 00000000 00:00 0 
7fcde1ee3000-7fcde23e2000 r-xp 00000000 00:35 14196385                   /android/sdk/emulator/lib64/qt/lib/libQt5Gui.so.5
7fcde23e2000-7fcde23f9000 r--p 004fe000 00:35 14196385                   /android/sdk/emulator/lib64/qt/lib/libQt5Gui.so.5
7fcde23f9000-7fcde240d000 rw-p 00515000 00:35 14196385                   /android/sdk/emulator/lib64/qt/lib/libQt5Gui.so.5
7fcde240d000-7fcde2412000 rw-p 00000000 00:00 0 
7fcde2412000-7fcde29df000 r-xp 00000000 00:35 14196413                   /android/sdk/emulator/lib64/qt/lib/libQt5Widgets.so.5
7fcde29df000-7fcde2a0d000 r--p 005cc000 00:35 14196413                   /android/sdk/emulator/lib64/qt/lib/libQt5Widgets.so.5
7fcde2a0d000-7fcde2a34000 rw-p 005fa000 00:35 14196413                   /android/sdk/emulator/lib64/qt/lib/libQt5Widgets.so.5
7fcde2a34000-7fcde2a35000 rw-p 00000000 00:00 0 
7fcde2a35000-7fcde2a38000 r-xp 00000000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fcde2a38000-7fcde2c37000 ---p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fcde2c37000-7fcde2c38000 r--p 00002000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fcde2c38000-7fcde2c39000 rw-p 00003000 00:35 4128922                    /lib/x86_64-linux-gnu/libdl-2.23.so
7fcde2c39000-7fcde2ca9000 r-xp 00000000 00:35 14193377                   /usr/lib/x86_64-linux-gnu/mesa/libGL.so.1.2.0
7fcde2ca9000-7fcde2ea8000 ---p 00070000 00:35 14193377                   /usr/lib/x86_64-linux-gnu/mesa/libGL.so.1.2.0
7fcde2ea8000-7fcde2eab000 r--p 0006f000 00:35 14193377                   /usr/lib/x86_64-linux-gnu/mesa/libGL.so.1.2.0
7fcde2eab000-7fcde2eac000 rw-p 00072000 00:35 14193377                   /usr/lib/x86_64-linux-gnu/mesa/libGL.so.1.2.0
7fcde2eac000-7fcde2ead000 rw-p 00000000 00:00 0 
7fcde2ead000-7fcde2fe2000 r-xp 00000000 00:35 14193288                   /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
7fcde2fe2000-7fcde31e2000 ---p 00135000 00:35 14193288                   /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
7fcde31e2000-7fcde31e3000 r--p 00135000 00:35 14193288                   /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
7fcde31e3000-7fcde31e7000 rw-p 00136000 00:35 14193288                   /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
7fcde31e7000-7fcde31ee000 r-xp 00000000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fcde31ee000-7fcde33ed000 ---p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fcde33ed000-7fcde33ee000 r--p 00006000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fcde33ee000-7fcde33ef000 rw-p 00007000 00:35 4128983                    /lib/x86_64-linux-gnu/librt-2.23.so
7fcde33ef000-7fcde3415000 r-xp 00000000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fcde3416000-7fcde3418000 r-xp 00000000 00:00 0 
7fcde3418000-7fcde341f000 r--s 00000000 00:35 4129778                    /usr/lib/x86_64-linux-gnu/gconv/gconv-modules.cache
7fcde341f000-7fcde34bd000 r-xp 00000000 00:35 14196353                   /android/sdk/emulator/lib64/lib64OpenglRender.so
7fcde34bd000-7fcde34be000 ---p 0009e000 00:35 14196353                   /android/sdk/emulator/lib64/lib64OpenglRender.so
7fcde34be000-7fcde34c0000 r--p 0009e000 00:35 14196353                   /android/sdk/emulator/lib64/lib64OpenglRender.so
7fcde34c0000-7fcde34c1000 rw-p 000a0000 00:35 14196353                   /android/sdk/emulator/lib64/lib64OpenglRender.so
7fcde34c1000-7fcde34d3000 rw-p 00000000 00:00 0 
7fcde34d3000-7fcde35ab000 r-xp 00000000 00:35 14196364                   /android/sdk/emulator/lib64/libc++.so.1
7fcde35ab000-7fcde35b3000 r--p 000d7000 00:35 14196364                   /android/sdk/emulator/lib64/libc++.so.1
7fcde35b3000-7fcde35b6000 rw-p 000df000 00:35 14196364                   /android/sdk/emulator/lib64/libc++.so.1
7fcde35b6000-7fcde35ba000 rw-p 00000000 00:00 0 
7fcde35ba000-7fcde3607000 r-xp 00000000 00:35 14196405                   /android/sdk/emulator/lib64/qt/lib/libQt5Svg.so.5
7fcde3607000-7fcde3609000 r--p 0004c000 00:35 14196405                   /android/sdk/emulator/lib64/qt/lib/libQt5Svg.so.5
7fcde3609000-7fcde360b000 rw-p 0004e000 00:35 14196405                   /android/sdk/emulator/lib64/qt/lib/libQt5Svg.so.5
7fcde360b000-7fcde360d000 rw-p 00000000 00:00 0 
7fcde360d000-7fcde360f000 r-xp 00000000 00:00 0 
7fcde360f000-7fcde3610000 rw-p 00000000 00:00 0 
7fcde3610000-7fcde3612000 r-xp 00000000 00:00 0 
7fcde3612000-7fcde3614000 rw-p 00000000 00:00 0 
7fcde3614000-7fcde3615000 r--p 00025000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fcde3615000-7fcde3616000 rw-p 00026000 00:35 4128889                    /lib/x86_64-linux-gnu/ld-2.23.so
7fcde3616000-7fcde3617000 rw-p 00000000 00:00 0 
7ffe3a5c5000-7ffe3a5e6000 rw-p 00000000 00:00 0                          [stack]
7ffe3a5ed000-7ffe3a5f0000 r--p 00000000 00:00 0                          [vvar]
7ffe3a5f0000-7ffe3a5f1000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]
##[error]Process completed with exit code 134.
