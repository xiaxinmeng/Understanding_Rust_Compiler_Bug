
#0  upcall_fail (expr=0x102b0ef50 "option::get none", file=0x101668170 "/Users/nmatsakis/versioned/rust-green/src/rustc/rustc.rc", line=1) at rust_upcall.cpp:94
#1  0x00000001001120f7 in rt::rustrt::rust_upcall_fail::_74a64c4360d2dcec::_04 ()
#2  0x000000010003c5c7 in rt::rt_fail_::_74a64c4360d2dcec::_04 ()
#3  0x0000000100b62952 in option::get_22339::_89c15131464a9573::_04 ()
#4  0x0000000100d3b2a5 in middle::trans::meth::method_from_methods::_6b67243aebfac69::_04 ()
#5  0x0000000100d38f98 in middle::trans::meth::method_with_name::_19b276bbd665a346::_04 ()
#6  0x0000000100bd2592 in middle::trans::meth::trans_static_method_callee::_f9ec6ca09ccac979::_04 ()
#7  0x0000000100bcb9bd in middle::trans::callee::trans::trans_def::_67363582be12cd36::_04 ()
#8  0x0000000100bcb4fe in middle::trans::callee::trans::_f48ed4270a0c3c0::_04 ()
#9  0x00000001015b78fa in __morestack ()
