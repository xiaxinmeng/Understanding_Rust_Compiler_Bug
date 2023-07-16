
gdb --args target/release/bfc sample_programs/hello_world.bf

Program received signal SIGSEGV, Segmentation fault.
0x7f642784 in thread_rng::h22bece718b8c83adkgf ()
(gdb) bt
#0  0x7f642784 in thread_rng::h22bece718b8c83adkgf ()
#1  0x7f6422dc in util::tmpname::h63004644db7838bePja ()
#2  0x7f641a00 in named::NamedTempFile::new::hfa7543c1e647f61ecqa ()
#3  0x7f61c308 in compile_file::h04704d1240bcd17a3Fc ()
#4  0x7f62077c in main::h68a433d56cae34167Pc ()
#5  0x7f65ab4c in panic::recover::h13621087687085827578 ()
#6  0x7f65a68c in rt::lang_start::h5fc8517878d759f3Dky ()
#7  0xb6d61632 in __libc_start_main (main=0x7f621dbc <main>, argc=2, argv=0xbeffef74, init=<optimized out>, fini=0x8004b0f9 <__libc_csu_fini>, 
    rtld_fini=0xb6fea4c5 <_dl_fini>, stack_end=0xbeffef74) at libc-start.c:287
#8  0x7f61a8d8 in _start ()
