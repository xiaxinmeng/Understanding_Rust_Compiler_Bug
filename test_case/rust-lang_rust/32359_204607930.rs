 shell
$ gdb --args exa -l ~/
Program received signal SIGILL, Illegal instruction.
0xb6e84268 in ?? () from /lib/arm-linux-gnueabihf/libcrypto.so.1.0.0
(gdb) c
Continuing.

Program received signal SIGBUS, Bus error.
0x7f5dbd94 in exa::options::Options::getopts::h06436b42770af21e ()
(gdb) bt
#0  0x7f5dbd94 in exa::options::Options::getopts::h06436b42770af21e ()
#1  0x7f5b953c in exa::main::h4671dcdc83e99fa9 ()
#2  0x7f658930 in std::sys_common::unwind::try::try_fn::h0f540c0db5f980f7 ()
#3  0x7f661f48 in std::sys_common::unwind::inner_try::h74a189ca1fbe8e07 ()
#4  0x7f65844c in std::rt::lang_start::h7bd5d5e5d238827f ()
#5  0xb6d17632 in __libc_start_main (main=0x7f5babe4 <main>, argc=3, argv=0xbeffee54, init=<optimized out>, fini=0x7f6749bd <__libc_csu_fini>, 
    rtld_fini=0xb6fea4c5 <_dl_fini>, stack_end=0xbeffee54) at libc-start.c:287
#6  0x7f5b7114 in _start ()
