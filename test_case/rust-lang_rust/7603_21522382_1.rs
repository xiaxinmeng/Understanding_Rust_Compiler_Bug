
(gdb) bt
#0  0x00007ffff6443218 in pthread_join () from /lib/x86_64-linux-gnu/libpthread.so.0
#1  0x00007ffff6e50bc7 in rust_thread::join (this=this@entry=0x64c5e0)
    at /home/chris/vc/rust/src/rt/sync/rust_thread.cpp:65
#2  0x00007ffff6e52a16 in rust_raw_thread_join_delete (
/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Quit this debugging session? (y or n) n

/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Create a core file of GDB? (y or n) n
thread=0x64c5e0) at /home/chris/vc/rust/src/rt/rust_builtin.cpp:756
#3  0x00007ffff6e58f2c in upcall_call_shim_on_c_stack (
/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Quit this debugging session? (y or n) n

/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Create a core file of GDB? (y or n) n
[1]    14224 segmentation fault (core dumped)  RUST_NEWRT=1 gdb src/examples/info
