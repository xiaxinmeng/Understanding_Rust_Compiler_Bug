
jurily@jurily ~> ldd /usr/bin/rustc | grep libstd-
    libstd-aad93cea-0.11-pre.so => /usr/bin/../lib/libstd-aad93cea-0.11-pre.so (0x00007ff49174f000)

jurily@jurily ~> ldd /usr/lib64/libsplay-c8a1a9db-0.0.so | grep libstd-
    libstd-aad93cea-0.11-pre.so => /usr/lib64/../../../../../../../usr/lib64/rustlib/x86_64-unknown-linux-gnu/lib/libstd-aad93cea-0.11-pre.so (0x00007f94d1fad000)
