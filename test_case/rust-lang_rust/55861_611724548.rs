
    #00  pc 0001c5a4  /system/lib/libc.so (__futex_syscall3+8)
    #01  pc 0000de88  /system/lib/libc.so
    #02  pc 0000f56d  /system/lib/libc.so (dlmalloc+48)
    #03  pc 000108a7  /system/lib/libc.so
    #04  pc 0000d9fb  /system/lib/libc.so (memalign+10)
    #05  pc 0006dcb0  /data/tmp/work/libstd-7d8e6950da20b76f.so (std::sys_common::alloc::realloc_fallback::h05f7cc89deafdb57+36)
    #06  pc 00073a50  /data/tmp/work/libstd-7d8e6950da20b76f.so (__rdl_realloc+40)
    #07  pc 0012a390  /data/tmp/work/test1/collectionstests-8c9066a5b2aa9a17 (alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_exact::h5da8ae41586c5215+104)
    #08  pc 0014c50c  /data/tmp/work/test1/collectionstests-8c9066a5b2aa9a17 (collectionstests::vec::overaligned_allocations::h298719e516d2db4a+112)
