
[host-5-154:build eholk]$ gdb --args x86_64-apple-darwin/stage1/bin/rustc --cfg stage1  -O --target=x86_64-apple-darwin -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/libcore.dylib /Users/eholk/Documents/projects/mozilla/rust/src/libcore/core.rc
GNU gdb 6.3.50-20050815 (Apple version gdb-1705) (Fri Jul  1 10:50:06 UTC 2011)
Copyright 2004 Free Software Foundation, Inc.
GDB is free software, covered by the GNU General Public License, and you are
welcome to change it and/or distribute copies of it under certain conditions.
Type "show copying" to see the conditions.
There is absolutely no warranty for GDB.  Type "show warranty" for details.
This GDB was configured as "x86_64-apple-darwin"...Reading symbols for shared libraries .. done

(gdb) r
Starting program: /Users/eholk/Documents/projects/mozilla/rust/build/x86_64-apple-darwin/stage1/bin/rustc --cfg stage1 -O --target=x86_64-apple-darwin -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/lib/libcore.dylib /Users/eholk/Documents/projects/mozilla/rust/src/libcore/core.rc
Reading symbols for shared libraries .....+................................................................. done

Program received signal EXC_BAD_ACCESS, Could not access memory.
Reason: KERN_INVALID_ADDRESS at address: 0x00000000000000a8
[Switching to process 56322 thread 0x1e03]
0x0000000100cbd9c3 in middle::trans::base::bcx_icx::meth2293::insn_ctxt::_3e984ab9a2e9bf70::_02 ()
(gdb) bt
#0  0x0000000100cbd9c3 in middle::trans::base::bcx_icx::meth2293::insn_ctxt::_3e984ab9a2e9bf70::_02 ()
#1  0x000000010107a1ae in str2622 ()
#2  0x0000000100cdf5b2 in middle::trans::base::move_val::_dd8bbd759ed35a3d::_02 ()
#3  0x0000000100ce72b1 in middle::trans::base::trans_assign_op::_dcbae8ce5868daa8::_02 ()
#4  0x0000000100d1d1fb in middle::trans::base::trans_expr::unrooted::_e05f72d83f6e492c::_02 ()
#5  0x0000000100ce3e51 in middle::trans::base::trans_expr::_e05f72d83f6e492c::_02 ()
#6  0x0000000100d23b65 in middle::trans::base::trans_stmt::_17322c3f90f22da6::_02 ()
#7  0x0000000100cee351 in middle::trans::base::trans_block::_e3aa2aeff1a48c5c::_02 ()
#8  0x0000000100ceec75 in middle::trans::base::trans_while::_9effb9d55ffc7be5::_02 ()
#9  0x0000000100d1cda6 in middle::trans::base::trans_expr::unrooted::_e05f72d83f6e492c::_02 ()
#10 0x0000000101029804 in __morestack ()
Previous frame inner to this frame (gdb could not unwind past this frame)
(gdb) 
