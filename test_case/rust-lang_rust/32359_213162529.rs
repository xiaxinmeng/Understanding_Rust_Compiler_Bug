 rust
Program received signal SIGSEGV, Segmentation fault.
__memcpy_neon () at ../ports/sysdeps/arm/armv7/multiarch/memcpy_impl.S:749
749     ../ports/sysdeps/arm/armv7/multiarch/memcpy_impl.S: No such file or directory.
(gdb) bt
#0  __memcpy_neon () at ../ports/sysdeps/arm/armv7/multiarch/memcpy_impl.S:749
#1  0x7f57ddbc in exa::options::view::{{impl}}::deduce (
    matches=<optimized out>, filter=..., dir_action=...)
    at src/options/view.rs:173
#2  0x7f5840bc in exa::options::{{impl}}::deduce (matches=0xbeffd568)
    at src/options/mod.rs:140
#3  0x7f55fe4c in exa::options::{{impl}}::getopts<collections::string::String>
    (args=...) at src/options/mod.rs:120
#4  0x7f55b23c in exa::{{impl}}::new<std::io::stdio::Stdout,collections::string::String> (
    args=<error reading variable: access outside bounds of object referenced via synthetic pointer>, writer=<optimized out>) at src/exa.rs:56
#5  exa::main () at src/bin/main.rs:12
#6  0x7f607334 in std::sys_common::unwind::try::try_fn::h9c7a855073631f82 ()
#7  0x7f603824 in __rust_try ()
#8  0x7f606d54 in std::rt::lang_start::h801b666f82634252 ()
#9  0xb6d17632 in __libc_start_main (main=0x7f5789d4 <main>, argc=2, 
    argv=0xbeffee34, init=<optimized out>, fini=0x7f61dbf5 <__libc_csu_fini>, 
    rtld_fini=0xb6fea4c5 <_dl_fini>, stack_end=0xbeffee34) at libc-start.c:287
#10 0x7f55af6c in _start ()
