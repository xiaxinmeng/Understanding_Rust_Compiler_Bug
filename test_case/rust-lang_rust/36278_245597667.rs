
[...]

Breakpoint 4, alloc_jemalloc::__rust_allocate (size=128, align=8)
    at /home/fklock/Dev/Mozilla/rust.git/src/liballoc_jemalloc/lib.rs:98
(rr) finish
Run till exit from #0  alloc_jemalloc::__rust_allocate (size=128, align=8)
    at /home/fklock/Dev/Mozilla/rust.git/src/liballoc_jemalloc/lib.rs:98
0x000055d8f114849d in alloc::heap::allocate::h4983eab5bdfc0c37 ()
Value returned is $18 = (u8 *) 0x7fd2faa1b000 '\245' <repeats 128 times>
(rr) c
Continuing.

[...]

size: 24 val: 24
size: 32 val: 32
Dropping y

Breakpoint 3, alloc_jemalloc::__rust_deallocate (ptr=0x7fd2faa1b000 "", old_size=136,
    align=8) at /home/fklock/Dev/Mozilla/rust.git/src/liballoc_jemalloc/lib.rs:124
(rr)
