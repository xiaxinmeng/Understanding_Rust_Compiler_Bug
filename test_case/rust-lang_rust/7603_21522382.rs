
(gdb) bt
#0  0x00007ffff6896037 in raise () from /lib/x86_64-linux-gnu/libc.so.6
#1  0x00007ffff6899698 in abort () from /lib/x86_64-linux-gnu/libc.so.6
#2  0x00007ffff6e68939 in __morestack ()
   from /home/chris/vc/rust-dev/rusthttpserver/src/examples/../../../../../opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so
#3  0x00007ffff6e58e9c in call_on_c_stack (fn_ptr=0x7ffff7847880 <abort__c_stack_shim>, args=0x7ffff5c2c6e8, 
    this=0x633ff0) at /home/chris/vc/rust/src/rt/rust_task.h:481
#4  upcall_call_shim_on_c_stack (args=0x7ffff5c2c6e8, fn_ptr=0x7ffff7847880 <abort__c_stack_shim>)
    at /home/chris/vc/rust/src/rt/rust_upcall.cpp:70
/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Quit this debugging session? (y or n) n

/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Create a core file of GDB? (y or n) n

#5  0x00007ffff7810601 in libc::funcs::c95::stdlib::abort::_3462191b9dd9e70::_0$x2e8$x2dpre ()
   from /home/chris/vc/rust-dev/rusthttpserver/src/examples/../../../../../opt/rust/lib/rustc/x86_64-unknown-linux-gnu/lib/libstd-6c65cf4b443341b1-0.8-pre.so
/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Quit this debugging session? (y or n) n

/build/buildd/gdb-7.6~20130417/gdb/dwarf2read.c:10350: internal-error: dwarf2_record_block_ranges: Assertion `dwarf2_per_objfile->ranges.readin' failed.
A problem internal to GDB has been detected,
further debugging may prove unreliable.
Create a core file of GDB? (y or n) n
