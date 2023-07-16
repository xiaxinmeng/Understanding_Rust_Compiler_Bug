
(gdb) r
Starting program: rust/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-94564/a 

Program received signal SIGSEGV, Segmentation fault.
0x00007ffff7f0c44a in __dcigettext ()
(gdb) bt
#0  0x00007ffff7f0c44a in __dcigettext ()
#1  0x00007ffff7f0b692 in __assert_fail ()
#2  0x00007ffff7f550ce in _dl_relocate_static_pie ()
#3  0x00007ffff7f0a438 in __libc_start_main_impl ()
#4  0x00007ffff7ecfc65 in _start () at ../sysdeps/x86_64/start.S:115
