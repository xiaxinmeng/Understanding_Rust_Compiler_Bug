
--------------------------------------------------------------------------------
Ir          I1mr   ILmr   Dr          D1mr      DLmr    Dw          D1mw    DLmw    
--------------------------------------------------------------------------------
-16,399,587 48,614 -1,909 -15,203,345 1,677,474 -18,799 -14,933,944 939,273 145,091  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          I1mr   ILmr  Dr          D1mr      DLmr    Dw          D1mw    DLmw     file:function
--------------------------------------------------------------------------------
-12,941,229    644    12 -14,697,113    22,667  33,296 -14,697,093 196,198 103,505  /usr/src/debug/glibc-2.30-34-g994e529a37/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_avx_unaligned_erms
  5,834,640    309   116   1,361,416    22,558   8,013   1,944,880       0       0  ???:_ZN4core3ptr13drop_in_place17h2d334451ff604121E.llvm.3047232575757559832
 -5,834,640   -309   -93  -1,361,416   -22,305  -8,147  -1,944,880       0       0  ???:_ZN4core3ptr13drop_in_place17h2d334451ff604121E.llvm.14951046493754649665
 -4,704,001 -6,786  -275  -1,031,214   -29,888   6,301    -802,849  13,182   4,175  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:_int_malloc
  2,660,698  2,227     0     958,237        -6       0     481,776      20       0  ???:_ZN5cargo4util8progress5State5print17h25c4eb5dec857e24E.llvm.17714790172995471728
   -826,762  1,803   -15    -153,015   -82,049 -46,742    -153,358   1,001     647  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:malloc_consolidate
   -806,184   -651   -24    -201,566   -64,090       9      52,412  -6,989     -10  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:malloc
   -591,222  5,241   -46    -148,617    -4,060     753      84,645  10,875    -157  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:_int_free
    403,708     40    -2         510         0       0         255       0       0  ???:core::intrinsics::is_nonoverlapping
   -301,845   -275   -26    -115,970   -43,771 -44,619     -33,584       0       0  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:unlink_chunk.isra.0
    196,931    677   108      54,384    -2,842      -2      43,974   2,948     -19  ???:alloc::raw_vec::RawVec<T,A>::reserve
    186,666    802   -34      40,289    -4,337    -187      14,859     -53      -6  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:realloc
    146,690 -1,963   -44      31,542     7,476    -668      22,101     -61      -4  /usr/src/debug/glibc-2.30-34-g994e529a37/malloc/malloc.c:_int_realloc
    131,682    342     0      33,170        13       0      32,318      -1       0  ???:_ZN5cargo4util8progress5State4tick17h24f78f2baccd5adbE.llvm.17714790172995471728
    -58,890   -820   203     -14,193    58,285  -2,498      -3,741  -3,269      -2  ???:hashbrown::map::RawEntryBuilderMut<K,V,S>::from_hash
    -31,850    777   -10      -7,350      -721     -38           0       0       0  ???:hashbrown::raw::RawTable<T>::bucket
     25,476      0     0      10,615         0       0           0       0       0  /rustc/7760cd0fbbbf2c59a625e075a5bdfa88b8e30f8a//src/libstd/sys/unix/alloc.rs:__rdl_realloc
     16,984      1     0           0         0       0      10,615       0       0  /rustc/7760cd0fbbbf2c59a625e075a5bdfa88b8e30f8a//src/libstd/alloc.rs:__rdl_realloc
