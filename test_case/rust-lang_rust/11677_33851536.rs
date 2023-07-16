
(gdb) bt
#0  0x00007ffff7761dc0 in rust_fail () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#1  0x00007ffff7761d9d in rt::unwind::Unwinder::begin_unwind::h0f27297e83395e62Myar::v0.10.pre () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#2  0x00007ffff76c84fd in rt::unwind::begin_unwind_inner::hee62edb139672150a4::v0.10.pre () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#3  0x00007ffff6193704 in rt::unwind::begin_unwind::h2b7d4cb5e9c4cb1caS::v0.10.pre () from /usr/local/bin/../lib/libsyntax-ebc61d75-0.10-pre.so
#4  0x00007ffff619434b in diagnostic::Handler::fatal::h91da524ac0d37804ZvaO::v0.10.pre () from /usr/local/bin/../lib/libsyntax-ebc61d75-0.10-pre.so
#5  0x00007ffff6194a31 in diagnostic::Handler::bug::h61c101b30cc0efcaZva3::v0.10.pre () from /usr/local/bin/../lib/libsyntax-ebc61d75-0.10-pre.so
#6  0x00007ffff440bb3c in metadata::tyencode::enc_sty::h9a5e62295185bad7aF::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#7  0x00007ffff440965c in metadata::tyencode::enc_ty::hea30b7c89375458daj::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#8  0x00007ffff432154f in metadata::tyencode::enc_substs::hccdb7226aa6e0aa7aa::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#9  0x00007ffff440ad20 in metadata::tyencode::enc_sty::h9a5e62295185bad7aF::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#10 0x00007ffff440965c in metadata::tyencode::enc_ty::hea30b7c89375458daj::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#11 0x00007ffff43f4e54 in metadata::encoder::encoded_ty::hb173727e268b8a19au::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#12 0x00007ffff43f4ab5 in back::link::symbol_hash::h24bb9f32bb911e6ea3::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#13 0x00007ffff43f50b1 in back::link::get_symbol_hash::h484d614864ff7bedae::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#14 0x00007ffff3e912b9 in back::link::mangle_internal_name_by_type_and_seq::hb771818da811002faW::v0.10.pre ()
   from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#15 0x00007ffff3e8ffb8 in middle::trans::glue::declare_tydesc::h726926024a67984daL::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#16 0x00007ffff3e85b5f in middle::trans::base::get_tydesc::h6c291847eb5a334dap::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#17 0x00007ffff3e8379c in middle::trans::glue::drop_ty::h76bdd32ec1c33ba4aA::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#18 0x00007ffff3fafbd3 in middle::trans::cleanup::CleanupHelperMethods$FunctionContext::trans_cleanups_to_exit_scope::hac2b4a799d12165b17at::v0.10.pre ()
   from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#19 0x00007ffff3fb26aa in middle::trans::cleanup::CleanupHelperMethods$FunctionContext::get_or_create_landing_pad::haff84fad79e91ef517ad::v0.10.pre ()
   from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#20 0x00007ffff3f0a848 in middle::trans::cleanup::CleanupMethods$FunctionContext::get_landing_pad::h7f26bac115e37944zGam::v0.10.pre ()
   from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#21 0x00007ffff3e8e0df in middle::trans::base::invoke::h8a4aeb7ea1893043aE::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#22 0x00007ffff3ea594c in middle::trans::callee::trans_call_inner::hd1e60f14453e368aay::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#23 0x00007ffff3e8167f in middle::trans::callee::trans_lang_call::h20ee8c3399bbb425ah::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#24 0x00007ffff3ec7a33 in middle::trans::base::malloc_raw_dyn::hbb860441522ea11fa0::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#25 0x00007ffff3ebb308 in middle::trans::expr::trans_boxed_expr::hae6116540b02fbd5aL::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#26 0x00007ffff3eb8ba9 in middle::trans::expr::trans_datum_unadjusted::he054d22b2c882015aY::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#27 0x00007ffff3ead851 in middle::trans::expr::trans_unadjusted::h885c810f27b9a708aQ::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#28 0x00007ffff3e73cf7 in middle::trans::expr::trans::h7c5760109eadd9caaL::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#29 0x00007ffff3e715e6 in middle::trans::expr::trans_into::hfe0700abc153a39eaM::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#30 0x00007ffff3ec2934 in middle::trans::expr::trans_adt::hac9fec24cb70939caK::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#31 0x00007ffff3ec6792 in middle::trans::expr::trans_rec_or_struct::anon::expr_fn::aW () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#32 0x00007ffff3eaf530 in middle::trans::expr::trans_rvalue_dps_unadjusted::h46e0ec9f248f5160a8::v0.10.pre ()
   from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#33 0x00007ffff3e718a0 in middle::trans::expr::trans_into::hfe0700abc153a39eaM::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#34 0x00007ffff3e70ad6 in middle::trans::controlflow::trans_stmt::h85f7d2d210bf4bc3aJ::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#35 0x00007ffff3e7267d in middle::trans::controlflow::trans_block::h2ae4af48ce0961f8aK::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#36 0x00007ffff3f4a820 in middle::trans::base::trans_closure::hdee6ddcf6417b26cak::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#37 0x00007ffff3e49bbc in middle::trans::base::trans_fn::h252a0d728ea6ed6fa6::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#38 0x00007ffff3e444c6 in middle::trans::base::trans_item::hc2f12d908815d445aX::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
---Type <return> to continue, or q <return> to quit---
#39 0x00007ffff3f4ea4f in middle::trans::base::trans_mod::h39c66ecf043ca5e4ay::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#40 0x00007ffff3f57583 in middle::trans::base::trans_crate::hb444800a0d928b31aO::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#41 0x00007ffff4462066 in driver::driver::phase_4_translate_to_llvm::hc0d9ecc122afaa97ab::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#42 0x00007ffff446516b in driver::driver::compile_input::h22970c803913ae85aL::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#43 0x00007ffff448ac53 in run_compiler::h1e814f97b6455ee1aR::v0.10.pre () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#44 0x00007ffff4494488 in main_args::anon::expr_fn::a3 () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#45 0x00007ffff4492831 in monitor::anon::expr_fn::aJ () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#46 0x00007ffff449018a in task::TaskBuilder::try::anon::expr_fn::FaaIag () from /usr/local/bin/../lib/librustc-0d83f02f-0.10-pre.so
#47 0x00007ffff741a2a8 in task::__extensions__::build_start_wrapper::anon::anon::expr_fn::aq () from /usr/local/bin/../lib/libgreen-80d9e76a-0.10-pre.so
#48 0x00007ffff775d473 in rt::task::__extensions__::run::anon::expr_fn::a9 () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#49 0x00007ffff77652fc in rust_try () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#50 0x00007ffff775d3e2 in rt::task::Task::run::hdbaa27a0ea8f3f3447a6::v0.10.pre () from /usr/local/bin/../lib/libstd-966edb7e-0.10-pre.so
#51 0x00007ffff7419f79 in task::__extensions__::build_start_wrapper::anon::expr_fn::ae () from /usr/local/bin/../lib/libgreen-80d9e76a-0.10-pre.so
#52 0x0000000000000000 in ?? ()
