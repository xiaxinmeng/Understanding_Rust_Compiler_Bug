
$ llvm-nm `rustc --print sysroot`/lib/rustlib/x86_64-pc-windows-gnu/lib/libstd* | rg std_detect6detect5cache5CACHE
00000000 I __imp__ZN3std10std_detect6detect5cache5CACHE17hed91aa03ee9b39f3E
00000000 I __nm__ZN3std10std_detect6detect5cache5CACHE17hed91aa03ee9b39f3E
---------------- D _ZN3std10std_detect6detect5cache5CACHE17hed91aa03ee9b39f3E
