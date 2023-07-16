
[localhost:build] /opt/ndk/bin/arm-linux-androideabi-nm arm-linux-androideabi/rt/rust_android_dummy.o | grep log2f
00000018 T log2f
[localhost:build] /opt/ndk/bin/arm-linux-androideabi-nm arm-linux-androideabi/rt/librust_builtin.a | grep log2f
00000018 T log2f
[localhost:build] /opt/ndk/bin/arm-linux-androideabi-nm x86_64-apple-darwin/stage2/lib/rustlib/arm-linux-androideabi/lib/librustrt-4e7c5e5c.so | grep log2f
         U log2f
[localhost:build] /opt/ndk/bin/arm-linux-androideabi-nm x86_64-apple-darwin/stage2/lib/rustlib/arm-linux-androideabi/lib/libstd-4e7c5e5c.so | grep log2f
000848e8 T log2f
