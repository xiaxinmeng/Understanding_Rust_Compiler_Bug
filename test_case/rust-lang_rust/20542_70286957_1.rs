
stack backtrace:
   1:     0x7fe03b886ed0 - sys::backtrace::write::hf251c2373d8c39658Rt
   2:     0x7fe03b8a88b0 - failure::on_fail::h2ff2bf5f19ce9580S5z
   3:     0x7fe03b8177b0 - rt::unwind::begin_unwind_inner::h639cf27e708a803eKKz
   4:     0x7fe0352e1490 - rt::unwind::begin_unwind::h17544861623042600436
   5:     0x7fe0352e0230 - reader::get_doc::hc8977291e5d4282fZKa
   6:     0x7fe039de2710 - metadata::decoder::item_type::h9de63b0b6048c689Xpj
   7:     0x7fe039df3120 - metadata::decoder::get_type::h0e444e6b974068c1KBj
   8:     0x7fe039da4dc0 - metadata::csearch::get_type::h860647b7be886ccd2sm
   9:     0x7fe03b0cddc0 - collect::CollectCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::hc2534e432eeb53efcNt
  10:     0x7fe03b0b90a0 - astconv::ast_path_to_ty::h7dc2fdd21d7bc5aafss
  11:     0x7fe03b0bab40 - astconv::ast_ty_to_ty::unboxed_closure.29407
  12:     0x7fe03b06d0e0 - astconv::ast_ty_to_ty::hc931bbae55b3495bzSs
  13:     0x7fe03b0e6f30 - collect::ty_generics::hdeb07cf2e292b5fa3Vu
  14:     0x7fe03b0e2640 - collect::ty_generics_for_fn_or_method::h61776e35acfd2992GSu
  15:     0x7fe03b0e28d0 - collect::convert_methods::ty_of_method::hfb2b2df960a5e773hbu
  16:     0x7fe03b0d1e90 - collect::convert::h263630c2880766eboeu
  17:     0x7fe03b116680 - check_crate::unboxed_closure.30640
  18:     0x7fe03b1146c0 - check_crate::h6601a13b429d0a57ney
  19:     0x7fe03bdd8ff0 - driver::phase_3_run_analysis_passes::h039ad5f8102225f2EEa
  20:     0x7fe03bdc6dd0 - driver::compile_input::haa1e0a1302dab243Aba
  21:     0x7fe03be886c0 - run_compiler::h385e1ee7d7e284bee5b
  22:     0x7fe03be86e30 - thunk::F.Invoke<A, R>::invoke::h17064898529926817575
  23:     0x7fe03be85d90 - rt::unwind::try::try_fn::h13784017587596973898
  24:     0x7fe03b90fa60 - rust_try_inner
  25:     0x7fe03b90fa50 - rust_try
  26:     0x7fe03be86040 - thunk::F.Invoke<A, R>::invoke::h10896099676476614463
  27:     0x7fe03b896550 - sys::thread::thread_start::h9cd6def278b6034eVJw
  28:     0x7fe035e40250 - start_thread
  29:     0x7fe03b4c8219 - clone
  30:                0x0 - <unknown>
