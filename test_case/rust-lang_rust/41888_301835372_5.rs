
#0  je_rtree_get (dependent=true, key=key@entry=3, rtree=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/rtree.h:325
#1  je_chunk_lookup (dependent=true, ptr=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/chunk.h:89
#2  huge_node_get (ptr=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/src/huge.c:11
#3  je_huge_salloc (tsdn=<optimized out>, ptr=ptr@entry=0x0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/huge.c:455
#4  0x0000555555573953 in je_arena_salloc (demote=false, ptr=0x0, tsdn=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/arena.h:1417
#5  je_isalloc (demote=false, ptr=0x0, tsdn=<optimized out>) at include/jemalloc/internal/jemalloc_internal.h:1054
#6  rallocx (ptr=0x0, size=9007189332333568, flags=<optimized out>) at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:2401
#7  0x000055555555b1ff in _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::double::h51d74fc503deb7ce ()
#8  0x000055555556e44b in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:98
#9  0x000055555555b612 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hc72d94f6aec109ae ()
#10 0x0000555555565786 in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:658
#11 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:21
#12 std::sys::imp::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:84
#13 0x00007ffff77b56ba in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#14 0x00007ffff72d582d in clone () from /lib/x86_64-linux-gnu/libc.so.6
