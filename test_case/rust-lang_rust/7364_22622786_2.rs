
#0  rust_begin_unwind (token=839147) at /home/huon/rust/src/rt/rust_builtin.cpp:561
#1  0x00007ffff785d058 in rt::task::__extensions__::meth_24421::begin_unwind::_7c12263797ed078::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#2  0x00007ffff785c364 in sys::begin_unwind_::_89e154cd0915671::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#3  0x00007ffff785c552 in sys::__extensions__::fail_with::anon::anon::expr_fn_24404 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#4  0x00007ffff785be1b in c_str::__extensions__::with_ref_24395::_c6798931b183a7::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#5  0x00007ffff785c4e2 in sys::__extensions__::fail_with::anon::expr_fn_24402 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#6  0x00007ffff785be1b in c_str::__extensions__::with_ref_24395::_c6798931b183a7::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#7  0x00007ffff77e16ba in sys::__extensions__::meth_13539::fail_with::_db4c44d01ce4116::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#8  0x00007ffff6d87583 in diagnostic::__extensions__::meth_11052::span_fatal::_17fbc7a98adb45a9::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.8-pre.so
#9  0x00007ffff6d88b35 in diagnostic::__extensions__::meth_11185::span_bug::_17fbc7a98adb45a9::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libsyntax-64629f7f0c6a9bc-0.8-pre.so
#10 0x00007ffff611eb47 in driver::session::__extensions__::meth_27583::span_bug::_5ca9b114762e4285::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#11 0x00007ffff61d2395 in middle::trans::consts::const_expr::_671312514195361::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#12 0x00007ffff61d1732 in vec::__extensions__::map_35449::_a852b87e80994fe1::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#13 0x00007ffff61d5744 in middle::trans::consts::const_expr_unadjusted::_13ad30435862da2::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#14 0x00007ffff61d189d in middle::trans::consts::const_expr::_671312514195361::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#15 0x00007ffff60d68f8 in middle::trans::base::get_item_val::_e7eda3f16af37a6d::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#16 0x00007ffff61d3ec0 in middle::trans::consts::trans_const::_347eb4f8ccd2eef3::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#17 0x00007ffff60d4a4a in middle::trans::base::trans_item::_017a54bba8f16e::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#18 0x00007ffff6203e1f in middle::trans::base::trans_mod::_73877ea894e8fec2::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#19 0x00007ffff6211ee6 in middle::trans::base::trans_crate::_187127adc635a799::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#20 0x00007ffff67f575a in driver::driver::phase_4_translate_to_llvm::_6de7deb5a62c5935::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#21 0x00007ffff67f65c7 in driver::driver::compile_input::_dafa6bd69c9c3d6f::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#22 0x00007ffff68169e6 in run_compiler::_ff6d1ce49ab09970::_0$x2e8$x2dpre () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#23 0x00007ffff68244be in main::anon::expr_fn_101251 () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#24 0x00007ffff6822b45 in monitor::anon::expr_fn_101130 () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#25 0x00007ffff68206fe in task::__extensions__::try_100612::anon::expr_fn_100812 () from /usr/local/bin/../lib/librustc-d3cb8c2ccd84a7a7-0.8-pre.so
#26 0x00007ffff78303d0 in task::spawn::spawn_raw_newsched::anon::expr_fn_20731 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#27 0x00007ffff787648e in rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn_27643 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#28 0x00007ffff7874c0d in rt::task::__extensions__::run::anon::expr_fn_27562 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#29 0x00007ffff78767ad in rt::task::__extensions__::try_fn::_c0d285c244b7d16::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#30 0x00007ffff5d77964 in rust_try (f=<optimized out>, fptr=<optimized out>, env=<optimized out>) at /home/huon/rust/src/rt/rust_builtin.cpp:552
#31 0x00007ffff7874ac4 in rt::task::__extensions__::meth_27560::try::_199ab8d6eb226980::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#32 0x00007ffff787499a in rt::task::__extensions__::meth_27557::run::_199ab8d6eb226980::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#33 0x00007ffff787616c in rt::task::__extensions__::build_start_wrapper::anon::expr_fn_27627 () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
#34 0x00007ffff78acb49 in rt::context::__extensions__::task_start_wrapper::_d625afdc49afb93::_0$x2e8$x2dpre () from /usr/local/bin/../lib/libstd-6c65cf4b443341b1-0.8-pre.so
