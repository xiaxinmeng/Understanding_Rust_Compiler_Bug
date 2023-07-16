
[root@li1424-173 work-derive]# nm i686-unknown-linux-gnu/stage1/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-9026086f.so | grep lang_start
000db210 T _ZN2rt10lang_start20hbfee00d943992dc10UzE
[root@li1424-173 work-derive]# nm i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-9026086f.so | grep lang_start
000db280 T _ZN2rt10lang_start20hdfbcc23f8b0cf0d20UzE
