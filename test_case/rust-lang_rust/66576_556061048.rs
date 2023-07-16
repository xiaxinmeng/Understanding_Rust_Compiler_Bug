
(gdb) b 'pretty-uninitialized-vec.rs':21
Breakpoint 1 at 0x5884: file ../src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
(gdb) r
Starting program: /home/pnkfelix/Dev/Mozilla/rust.git/objdir/pretty-uninitialized-vec
warning: Loadable section ".note.gnu.property" outside of ELF segments
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, pretty_uninitialized_vec::main () at ../src/test/debuginfo/pretty-uninitialized-vec.rs:21
21          zzz(); // #break
Missing separate debuginfos, use: dnf debuginfo-install libgcc-9.2.1-1.fc30.x86_64
(gdb) print vec
$1 = Vec<i32>(len: 93824992438960, cap: 140737478328320) = {inaccessible}
(gdb) 
