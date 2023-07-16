
#0  upcall_fail (expr=0x7ffff686d820 <str44907> "struct variant kinds unimpl in enum_variants",
    file=0x7ffff686bba0 <str41803> "/home/cmr/hacking/rust/src/librustc/middle/ty.rs", line=3848) at ../src/rt/rust_upcall.cpp:127
#1  0x00007ffff78c1eac in sys::begin_unwind_::_89e154cd0915671::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#2  0x00007ffff78c2032 in sys::__extensions__::fail_with::anon::anon::expr_fn_21923 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#3  0x00007ffff78c1a01 in str::as_buf_21914::_f8c6f4a6f9cb738::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#4  0x00007ffff78c1fec in sys::__extensions__::fail_with::anon::expr_fn_21921 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#5  0x00007ffff78c1a01 in str::as_buf_21914::_f8c6f4a6f9cb738::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#6  0x00007ffff7840d64 in sys::__extensions__::meth_11647::fail_with::_db4c44d01ce4116::_07 () from /home/cmr/.local/bin/../lib/libstd-6c65cf4b443341b1-0.7.so
#7  0x00007ffff6396fb5 in middle::ty::enum_variants::anon::expr_fn_44902 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#8  0x00007ffff6396cd6 in vec::__extensions__::from_iterator_44886::_b0951ab2c2e0bf21::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#9  0x00007ffff61a584c in middle::ty::enum_variants::_e3f61f3e575b85e0::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#10 0x00007ffff6382411 in middle::ty::is_instantiable::subtypes_require::_3b1720274220e5c8::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#11 0x00007ffff6381e1f in middle::ty::is_instantiable::type_requires::_3b1720274220e5c8::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#12 0x00007ffff6382a34 in middle::ty::is_instantiable::subtypes_require::_3b1720274220e5c8::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
---Type <return> to continue, or q <return> to quit---
#13 0x00007ffff6381ba5 in middle::ty::is_instantiable::_cbea27c897118d::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#14 0x00007ffff64a4d6f in middle::typeck::check::check_instantiable::_8a6eebd8f8955129::_07 ()
   from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#15 0x00007ffff64a4bc7 in middle::typeck::check::check_struct::_e45827b3213eb::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#16 0x00007ffff649cc36 in middle::typeck::check::check_item::_cf67bf2e5288ae23::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#17 0x00007ffff649c5a1 in middle::typeck::check::check_item_types::anon::expr_fn_54648 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#18 0x00007ffff6e08d1c in visit::mk_simple_visitor::v_item::_95d273f23709279::_07 () from /home/cmr/.local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.7.so
#19 0x00007ffff6e0b208 in visit::mk_simple_visitor::anon::expr_fn_20738 () from /home/cmr/.local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.7.so
#20 0x00007ffff6ddeb45 in visit::visit_mod_17780::_421b74dd948ddfb::_07 () from /home/cmr/.local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.7.so
#21 0x00007ffff6e08851 in visit::mk_simple_visitor::v_mod::_f6d0e148e4624814::_07 () from /home/cmr/.local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.7.so
#22 0x00007ffff6e0aec4 in visit::mk_simple_visitor::anon::expr_fn_20727 () from /home/cmr/.local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.7.so
#23 0x00007ffff62f247f in visit::visit_crate_36133::_996a32a2e1b9d1b1::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#24 0x00007ffff649c3da in middle::typeck::check::check_item_types::_1743fb48bc7807::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#25 0x00007ffff65ace20 in util::common::time_62384::_c4d0513e54dc658e::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#26 0x00007ffff65ac921 in middle::typeck::check_crate::_6db1a4f04c458a29::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#27 0x00007ffff682c696 in driver::driver::compile_rest::_e58f61c4387ba8b7::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#28 0x00007ffff6861854 in __morestack () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
#29 0x00007ffff682ffbc in driver::driver::compile_upto::_af4a64d82884294::_07 () from /home/cmr/.local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.7.so
