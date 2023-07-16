
stack backtrace:
   1:     0x7f168dd7d240 - sys::backtrace::write::h2d5b774b42a611774sy
   2:     0x7f168dda0940 - failure::on_fail::h0eb07a9d41d28278YWF
   3:     0x7f168dd007c0 - rt::unwind::begin_unwind_inner::ha069250519e26ab8vBF
   4:     0x7f168dd012d0 - rt::unwind::begin_unwind_fmt::h9f78c298dbf494751zF
   5:     0x7f168ca1d1d0 - trans::glue::trans_struct_drop::h9466bdc3f9700c551Oe
   6:     0x7f168ca1a260 - trans::glue::trans_struct_drop_flag::h53f57a0615513e0d2Me
   7:     0x7f168ca11dd0 - trans::glue::get_drop_glue::hdbf820817aebde49RHe
   8:     0x7f168ca11ad0 - trans::glue::drop_ty::hd6a4f4d0316c3ae28De
   9:     0x7f168cb48740 - trans::cleanup::DropValue<'tcx>.Cleanup<'tcx>::trans::h3d0e4bca022963da59L
  10:     0x7f168ca25e50 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::h21d0ccfe42e4e080tbL
  11:     0x7f168cad4260 - trans::base::trans_closure::hd51332b4b14ebe7dW5t
  12:     0x7f168c9f61f0 - trans::base::trans_fn::h746e9efbc2b14f5b6gu
  13:     0x7f168c9f6750 - trans::monomorphize::monomorphic_fn::h823f1e98b4e6e83aQwd
  14:     0x7f168ca36df0 - trans::callee::trans_fn_ref_with_substs::hbe01e94c86fab1c8Hyg
  15:     0x7f168ca354d0 - trans::meth::trans_static_method_callee::h09f1ec79e37540b3y0y
  16:     0x7f168ca32260 - trans::callee::trans::hbc42ee6c50b5706dvbg
  17:     0x7f168ca49bd0 - trans::callee::trans_call_inner::h9042099031439240329
  18:     0x7f168ca52290 - trans::expr::trans_rvalue_dps_unadjusted::h13e0f4c2f7eed4fbrUi
  19:     0x7f168ca516a0 - trans::expr::trans_unadjusted::hebcf330aa76000bd3li
  20:     0x7f168ca0a890 - trans::expr::trans::hd2f453f3aad5f5202Dh
  21:     0x7f168caf13c0 - trans::_match::trans_match_inner::h25d04caf88e64d1bStx
  22:     0x7f168ca52290 - trans::expr::trans_rvalue_dps_unadjusted::h13e0f4c2f7eed4fbrUi
  23:     0x7f168ca09700 - trans::expr::trans_into::hded554efed2359ffyAh
  24:     0x7f168ca08ba0 - trans::controlflow::trans_stmt_semi::hdedd5d5900214185T9d
  25:     0x7f168ca09d10 - trans::controlflow::trans_block::h7c59eb740e387008Kae
  26:     0x7f168cad4260 - trans::base::trans_closure::hd51332b4b14ebe7dW5t
  27:     0x7f168c9f61f0 - trans::base::trans_fn::h746e9efbc2b14f5b6gu
  28:     0x7f168c9f12d0 - trans::base::trans_item::h8616676269d63cd8xFu
  29:     0x7f168cadb430 - trans::base::trans_crate::he650edc239d6aefahCv
  30:     0x7f168e37cbd0 - driver::phase_4_translate_to_llvm::h7647c8286ab427432Oa
  31:     0x7f168e355a80 - driver::compile_input::hc24e252c6a9debbbCba
  32:     0x7f168e427c00 - run_compiler::h07afdfea03e574126ac
  33:     0x7f168e426290 - thunk::F.Invoke<A, R>::invoke::h7167023940119686520
  34:     0x7f168e4251c0 - rt::unwind::try::try_fn::h7099284992991867290
  35:     0x7f168de0ca80 - rust_try_inner
  36:     0x7f168de0ca70 - rust_try
  37:     0x7f168e425470 - thunk::F.Invoke<A, R>::invoke::h7565834387748213469
  38:     0x7f168dd8ce30 - sys::thread::thread_start::h7df8ddcb97714d51VCB
  39:     0x7f1687dd90c0 - start_thread
  40:     0x7f168d97cfd9 - __clone
  41:                0x0 - <unknown>
