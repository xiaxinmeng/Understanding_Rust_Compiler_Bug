 rust
Program received signal SIGSEGV, Segmentation fault.
exa::options::{{impl}}::deduce::{{closure}} () at src/options.rs:181
181                         recurse: dir_action.recurse_options(),
(gdb) bt
#0  exa::options::{{impl}}::deduce::{{closure}} () at src/options.rs:181
#1  exa::options::{{impl}}::deduce (matches=<optimized out>, dir_action=..., 
    filter=...) at src/options.rs:291
#2  exa::options::{{impl}}::deduce (matches=<optimized out>)
    at src/options.rs:133
#3  exa::options::{{impl}}::getopts (args=...) at src/options.rs:113
#4  0x7f55ed8c in exa::main () at src/main.rs:145
#5  0x7f6051d0 in std::sys_common::unwind::try::try_fn::h76efc8ed010787e5 ()
#6  0x7f60167c in __rust_try ()
#7  0x7f604bfc in std::rt::lang_start::h73c48c6a9af036ed ()
#8  0xb6d09632 in __libc_start_main (main=0x7f560314 <main>, argc=2, 
    argv=0xbeffee44, init=<optimized out>, fini=0x7f7fa96d <__libc_csu_fini>, 
    rtld_fini=0xb6fea4c5 <_dl_fini>, stack_end=0xbeffee44) at libc-start.c:287
#9  0x7f55be70 in _start ()
