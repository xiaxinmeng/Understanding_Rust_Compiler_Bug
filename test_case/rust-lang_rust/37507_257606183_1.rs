
(gdb) r
Starting program: /root/hello_world 
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.

Program received signal SIGILL, Illegal instruction.
je_arena_mapbits_binind_get () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:801
801     /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h: No such file or directory.
(gdb) bt
#0  je_arena_mapbits_binind_get () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:801
#1  je_arena_salloc () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:1317
#2  je_isalloc () at include/jemalloc/internal/jemalloc_internal.h:924
#3  je_iallocztm () at include/jemalloc/internal/jemalloc_internal.h:937
#4  a0ialloc () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/jemalloc.c:321
#5  je_a0malloc () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/jemalloc.c:336
#6  je_arena_tdata_get_hard () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/jemalloc.c:519
#7  0x5558f7dc in je_arena_tdata_get () at include/jemalloc/internal/jemalloc_internal.h:818
#8  je_decay_ticker_get () at include/jemalloc/internal/jemalloc_internal.h:842
#9  je_arena_decay_ticks () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:1253
#10 je_arena_decay_tick () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:1264
#11 je_arena_malloc_large () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/arena.c:2476
#12 0x55590ad4 in je_arena_malloc () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/arena.h:1287
#13 je_arena_palloc () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/arena.c:2601
#14 0x555b2420 in je_ipallocztm () at include/jemalloc/internal/jemalloc_internal.h:982
#15 je_tcache_create () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/tcache.c:328
#16 0x555849b0 in je_tcache_get () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/include/jemalloc/internal/tcache.h:236
#17 je_imalloc () at include/jemalloc/internal/jemalloc_internal.h:954
#18 imallocx_no_prof () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/jemalloc.c:2135
#19 mallocx () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/src/jemalloc/src/jemalloc.c:2164
#20 0x5557bb30 in alloc_jemalloc::imp::__rust_allocate () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/liballoc_jemalloc/lib.rs:106
#21 0x5557aaa4 in alloc::heap::allocate () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/liballoc/heap.rs:59
#22 alloc::raw_vec::{{impl}}::with_capacity<u8> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/liballoc/raw_vec.rs:96
#23 collections::vec::{{impl}}::with_capacity<u8> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libcollections/vec.rs:356
#24 collections::slice::hack::to_vec<u8> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libcollections/slice.rs:160
#25 collections::slice::{{impl}}::to_vec<u8> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libcollections/slice.rs:1161
#26 collections::slice::{{impl}}::to_owned<u8> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libcollections/slice.rs:1281
#27 collections::str::{{impl}}::to_owned () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libcollections/str.rs:167
#28 0x5556c934 in std::rt::lang_start () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-cross-linux/build/obj/../src/libstd/rt.rs:50
#29 0x55561cd8 in main ()
warning: GDB can't find the start of the function at 0x77f78acf.

    GDB is unable to find the start of the function at 0x77f78acf
and thus can't determine the size of that function's stack frame.
This means that GDB may be unable to access that stack frame, or
the frames below it.
    This problem is most likely caused by an invalid program counter or
stack pointer.
    However, if you think GDB should simply search farther back
from 0x77f78acf for code which looks like the beginning of a
function, you can increase the range of the search using the `set
heuristic-fence-post' command.
