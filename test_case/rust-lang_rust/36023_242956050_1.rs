 rust
#0  je_rtree_get (rtree=<optimized out>, dependent=true, key=0) at /home/eddy/Projects/rust-2/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/rtree.h:253
#1  je_chunk_lookup (dependent=true, ptr=0x0) at /home/eddy/Projects/rust-2/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/chunk.h:91
#2  huge_node_get (ptr=0x0) at /home/eddy/Projects/rust-2/src/liballoc_jemalloc/../jemalloc/src/huge.c:11
#3  je_huge_dalloc (tsd=0x7ffff7ff1770, ptr=0x0, tcache=0x7ffff6a0c000) at /home/eddy/Projects/rust-2/src/liballoc_jemalloc/../jemalloc/src/huge.c:375
#4  0x000055555555b262 in alloc::heap::deallocate (align=1, ptr=<optimized out>, old_size=<optimized out>) at /home/eddy/Projects/rust-2/src/liballoc/heap.rs:113
#5  alloc::raw_vec::{{impl}}::drop<u8> (self=<optimized out>) at /home/eddy/Projects/rust-2/src/liballoc/raw_vec.rs:561
#6  test::main () at /home/eddy/Projects/rust-2/test.rs:21
#7  0x0000555555569997 in __rust_maybe_catch_panic ()
#8  0x000055555556241f in std::rt::lang_start::h5381d9388ae2d3b7 ()
#9  0x00007ffff7219770 in __libc_start_main () from /nix/store/qc1jm0rplpgkwcacnc03hs9w4c8v2m31-glibc-multi-2.23/lib/libc.so.6
#10 0x000055555555b059 in _start () at ../sysdeps/x86_64/start.S:108
