
composite.rs:1:1: 1:10 warning: code is never used: `A`, #[warn(dead_code)] on by default
composite.rs:1 struct A;
               ^~~~~~~~~
composite.rs:4:5: 4:32 warning: code is never used: `test`, #[warn(dead_code)] on by default
composite.rs:4     fn test(v: Vec<&'a int>) {}
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~
composite.rs:4:13: 4:14 warning: unused variable: `v`, #[warn(unused_variable)] on by default
composite.rs:4     fn test(v: Vec<&'a int>) {}
                           ^
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'OwnedSlice: index out of bounds', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libstd/option.rs:167
stack backtrace:
   1:     0x7f5dd43c4ed0 - rt::backtrace::imp::write::h829de4821815e280PUy::v0.11.0.pre
   2:     0x7f5dd4341c50 - rt::unwind::begin_unwind_inner::h3c95a01106a60df1Xuy::v0.11.0.pre
   3:     0x7f5dd4e35250 - rt::unwind::begin_unwind::h8363202580871814802::v0.11.0.pre
   4:     0x7f5dd5026520 - middle::ty_fold::super_fold_sty::h15172236762007780758::v0.11.0.pre
   5:     0x7f5dd4eb1d90 - middle::subst::SubstFolder<'a>.TypeFolder::fold_ty::hcd91430b94d291ceqsR::v0.11.0.pre
   6:     0x7f5dd5026520 - middle::ty_fold::super_fold_sty::h15172236762007780758::v0.11.0.pre
   7:     0x7f5dd4eb1d90 - middle::subst::SubstFolder<'a>.TypeFolder::fold_ty::hcd91430b94d291ceqsR::v0.11.0.pre
   8:     0x7f5dd4eb2600 - iter::Iterator::collect::h14410322869207911680::v0.11.0.pre
   9:     0x7f5dd4eb1550 - middle::ty_fold::TypeFolder::fold_substs::h1021501308133928567::v0.11.0.pre
  10:     0x7f5dd5026520 - middle::ty_fold::super_fold_sty::h15172236762007780758::v0.11.0.pre
  11:     0x7f5dd4eb1d90 - middle::subst::SubstFolder<'a>.TypeFolder::fold_ty::hcd91430b94d291ceqsR::v0.11.0.pre
  12:     0x7f5dd4f382f0 - middle::subst::T.Subst::subst_spanned::h10156260079228704847::v0.11.0.pre
  13:     0x7f5dd4e9f640 - middle::trans::base::new_fn_ctxt::h85dd97abb8178a15q3o::v0.11.0.pre
  14:     0x7f5dd4f3de40 - middle::trans::base::trans_enum_variant_or_tuple_like_struct::h5f94827cc9bdcd5e7up::v0.11.0.pre
  15:     0x7f5dd4e48e60 - middle::trans::monomorphize::monomorphic_fn::h29e7539c7e7490a1Lka::v0.11.0.pre
  16:     0x7f5dd4ea9b10 - middle::trans::callee::trans_fn_ref_with_vtables::h739a9d33fff1d6b8Kjd::v0.11.0.pre
  17:     0x7f5dd4ea52f0 - middle::trans::callee::trans_fn_ref::hf912bb2e1b25d34aScd::v0.11.0.pre
  18:     0x7f5dd4eb59f0 - middle::trans::callee::trans_call::closure.58847
  19:     0x7f5dd4eb40e0 - middle::trans::callee::trans_call_inner::h6ec9e97b2f0cf3a9xId::v0.11.0.pre
  20:     0x7f5dd4eb3eb0 - middle::trans::callee::trans_call::h16885ad67fa12e03FCd::v0.11.0.pre
  21:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  22:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  23:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  24:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  25:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  26:     0x7f5dd4e7fe00 - middle::trans::controlflow::trans_if::h01c9aa3d8c5a3a6fabb::v0.11.0.pre
  27:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  28:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  29:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  30:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  31:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  32:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  33:     0x7f5dd4f3ca70 - middle::trans::base::trans_closure::h247c5303be3eb095Php::v0.11.0.pre
  34:     0x7f5dd4e48960 - middle::trans::base::trans_fn::hac7f31459ea86cc45pp::v0.11.0.pre
  35:     0x7f5dd4e48e60 - middle::trans::monomorphize::monomorphic_fn::h29e7539c7e7490a1Lka::v0.11.0.pre
  36:     0x7f5dd4ea9b10 - middle::trans::callee::trans_fn_ref_with_vtables::h739a9d33fff1d6b8Kjd::v0.11.0.pre
  37:     0x7f5dd4ea52f0 - middle::trans::callee::trans_fn_ref::hf912bb2e1b25d34aScd::v0.11.0.pre
  38:     0x7f5dd4eb6610 - middle::trans::meth::trans_method_callee::he0d18bd58a692f20t8u::v0.11.0.pre
  39:     0x7f5dd4eb65a0 - middle::trans::callee::trans_method_call::closure.58859
  40:     0x7f5dd4eb40e0 - middle::trans::callee::trans_call_inner::h6ec9e97b2f0cf3a9xId::v0.11.0.pre
  41:     0x7f5dd4eb5ef0 - middle::trans::callee::trans_method_call::hb058dc63facf7471NDd::v0.11.0.pre
  42:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  43:     0x7f5dd4ebdb00 - middle::trans::expr::trans_unadjusted::ha9f4b115887b34acmPe::v0.11.0.pre
  44:     0x7f5dd4e81720 - middle::trans::expr::trans::h31b1d37217a231c6cwe::v0.11.0.pre
  45:     0x7f5dd4e7f2d0 - middle::trans::expr::trans_to_lvalue::hb984fceab2a5c659pOe::v0.11.0.pre
  46:     0x7f5dd4ed4390 - middle::trans::_match::trans_match::h46f5f2aadf9d3482Y3s::v0.11.0.pre
  47:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  48:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  49:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  50:     0x7f5dd4e85eb0 - middle::trans::controlflow::trans_loop::hedc49c108e35ee4e1lb::v0.11.0.pre
  51:     0x7f5dd4ec1810 - middle::trans::expr::trans_rvalue_stmt_unadjusted::h780b112cf72e72d0sdf::v0.11.0.pre
  52:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  53:     0x7f5dd4ed4390 - middle::trans::_match::trans_match::h46f5f2aadf9d3482Y3s::v0.11.0.pre
  54:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  55:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  56:     0x7f5dd4e7e710 - middle::trans::controlflow::trans_stmt_semi::heb6a618a213b5a21P6a::v0.11.0.pre
  57:     0x7f5dd4e7dc10 - middle::trans::controlflow::trans_stmt::h774968e17f6d8b2aB2a::v0.11.0.pre
  58:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  59:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  60:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  61:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  62:     0x7f5dd4e7fe00 - middle::trans::controlflow::trans_if::h01c9aa3d8c5a3a6fabb::v0.11.0.pre
  63:     0x7f5dd4ebec30 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea9eb494db080d538hf::v0.11.0.pre
  64:     0x7f5dd4e7f430 - middle::trans::expr::trans_into::h15e286d35b829d1fgse::v0.11.0.pre
  65:     0x7f5dd4e7f9e0 - middle::trans::controlflow::trans_block::h7d3f8bec55eca940I7a::v0.11.0.pre
  66:     0x7f5dd4f3ca70 - middle::trans::base::trans_closure::h247c5303be3eb095Php::v0.11.0.pre
  67:     0x7f5dd4e48960 - middle::trans::base::trans_fn::hac7f31459ea86cc45pp::v0.11.0.pre
  68:     0x7f5dd4e48e60 - middle::trans::monomorphize::monomorphic_fn::h29e7539c7e7490a1Lka::v0.11.0.pre
  69:     0x7f5dd4e9a2c0 - middle::trans::base::get_res_dtor::hc8e22370ecd1ac66S2n::v0.11.0.pre
  70:     0x7f5dd4e99c50 - middle::trans::glue::trans_struct_drop::h5e384d30618e6dea8Pb::v0.11.0.pre
  71:     0x7f5dd4e92130 - middle::trans::glue::make_drop_glue::as_closure.58433
  72:     0x7f5dd4e91790 - middle::trans::glue::make_generic_glue::hd31ab36dd27f9301Bic::v0.11.0.pre
  73:     0x7f5dd4e8da80 - middle::trans::glue::get_drop_glue::he7b14323023aaf3fnFb::v0.11.0.pre
  74:     0x7f5dd4e8d910 - middle::trans::glue::drop_ty::h96a422be07664c893Cb::v0.11.0.pre
  75:     0x7f5dd4e9bff0 - middle::trans::cleanup::FunctionContext<'a>.CleanupMethods<'a>::pop_and_trans_custom_cleanup_scope::hcc5f7f1a1a504564RWF::v0.11.0.pre
  76:     0x7f5dd4f3ca70 - middle::trans::base::trans_closure::h247c5303be3eb095Php::v0.11.0.pre
  77:     0x7f5dd4e48960 - middle::trans::base::trans_fn::hac7f31459ea86cc45pp::v0.11.0.pre
  78:     0x7f5dd4f40c40 - middle::trans::meth::trans_impl::hcad0b0d6fab6bc8cX3u::v0.11.0.pre
  79:     0x7f5dd4e431a0 - middle::trans::base::trans_item::h62a895b39c0cb8ccCBp::v0.11.0.pre
  80:     0x7f5dd4f4b1f0 - middle::trans::base::trans_crate::h3026657c475a7720poq::v0.11.0.pre
  81:     0x7f5dd5706d30 - driver::driver::phase_4_translate_to_llvm::h639e7dc77f10a518H1j::v0.11.0.pre
  82:     0x7f5dd56fc000 - driver::driver::compile_input::hc7e76cb3959dc3aeiJj::v0.11.0.pre
  83:     0x7f5dd57c7820 - driver::run_compiler::ha4687608fe641c15bmm::v0.11.0.pre
  84:     0x7f5dd57c7740 - driver::main_args::closure.93513
  85:     0x7f5dd57dca20 - driver::monitor::closure.94585
  86:     0x7f5dd57d7d10 - task::TaskBuilder::try::closure.94348
  87:     0x7f5dd48e35a0 - task::spawn_opts::closure.7809
  88:     0x7f5dd43bf050 - rt::task::Task::run::closure.25264
  89:     0x7f5dd4423b10 - rust_try
  90:     0x7f5dd43befa0 - rt::task::Task::run::h82b2a3f233cfc4a9Ajw::v0.11.0.pre
  91:     0x7f5dd48e3350 - task::spawn_opts::closure.7782
  92:     0x7f5dd43c32d0 - rt::thread::thread_start::he85cc53b3abeddd182w::v0.11.0.pre
  93:     0x7f5dd1e61060 - start_thread
  94:     0x7f5dd4009489 - __clone
  95:                0x0 - <unknown>
