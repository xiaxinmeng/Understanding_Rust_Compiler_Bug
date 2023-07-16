 sh
[pi@raspberrypi ~]$ gdb --args .multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/rustc main.rs
GNU gdb (Raspbian 7.7.1+dfsg-5) 7.7.1
Copyright (C) 2014 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "arm-linux-gnueabihf".

Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from .multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/rustc...(no debugging symbols found)...done.
(gdb) run
Starting program: /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/rustc main.rs
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
[New Thread 0x731ff260 (LWP 872)]

Program received signal SIGILL, Illegal instruction.
[Switching to Thread 0x731ff260 (LWP 872)]
0x769d7bb4 in rustc_trans::mir::constant::MirConstContext::new::he4e1ab069b9c99ae ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
(gdb) bt
#0  0x769d7bb4 in rustc_trans::mir::constant::MirConstContext::new::he4e1ab069b9c99ae ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#1  0x769df0d0 in rustc_trans::mir::constant::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$bcx$C$$u20$$u27$tcx$GT$$GT$::trans_constant::hd8197320dad23619 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#2  0x769e1164 in rustc_trans::mir::operand::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$bcx$C$$u20$$u27$tcx$GT$$GT$::trans_operand::h422464b4e23f1932 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#3  0x769e3e50 in rustc_trans::mir::rvalue::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$bcx$C$$u20$$u27$tcx$GT$$GT$::trans_rvalue_operand::h732e4a147649da60 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#4  0x769d0cbc in rustc_trans::mir::block::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$bcx$C$$u20$$u27$tcx$GT$$GT$::trans_block::hc88a54279f657f65 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#5  0x769cebdc in rustc_trans::mir::trans_mir::h543a14de51581634 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#6  0x769761e0 in rustc_trans::base::trans_closure::h9f2aad00e64599b1 ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#7  0x769eed08 in rustc_trans::trans_item::TransItem::define::h76471544c9971e4f ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#8  0x769793c0 in rustc_trans::base::trans_crate::h75a5a0a51f75dfcc ()
   from /home/pi/.multirust/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/../lib/librustc_trans-8be786fe9ad50a77.so
#9  0x00000000 in ?? ()
(gdb) 
