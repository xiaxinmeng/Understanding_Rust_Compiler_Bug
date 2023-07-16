
stack backtrace:
   1:     0x7f458a441e69 - sys::backtrace::write::h928094d59f02e42euYr
   2:     0x7f458a449c66 - panicking::on_panic::h63ff4dcb77cfcfd8iow
   3:     0x7f458a40bbe2 - rt::unwind::begin_unwind_inner::hdb381b8543316733s3v
   4:     0x7f458a40c9a7 - rt::unwind::begin_unwind_fmt::hcf17bc68f9d21adby2v
   5:     0x7f458a449846 - rust_begin_unwind
   6:     0x7f458a495e84 - panicking::panic_fmt::hfe7d5ac25990b130yKy
   7:     0x7f458a4a020c - panicking::panic_bounds_check::h215216d6195ef019EJy
   8:     0x7f4588512d2e - middle::infer::freshen::TypeFreshener<'a, 'tcx>.TypeFolder<'tcx>::fold_ty::h5b268b4226c40e95sQw
   9:     0x7f45885140d9 - middle::ty_fold::TypeFolder::fold_substs::h11516006423260476848
  10:     0x7f458858653c - middle::traits::select::SelectionContext<'cx, 'tcx>::select::hcb3cbfe0f063ffbcLYP
  11:     0x7f4588583cf5 - middle::traits::fulfill::FulfillmentContext<'tcx>::select::h212762fc4b72bb85WQN
  12:     0x7f458858333b - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h90e9b9b285e1bde0aQN
  13:     0x7f4589b9ee48 - check::vtable::select_fcx_obligations_where_possible::h8046b67f72f03a23I0b
  14:     0x7f4589b9ea2a - check::vtable::select_all_fcx_obligations_and_apply_defaults::h91d8c9ca8d40e7ab7Wb
  15:     0x7f4589c2bdd4 - check::check_bare_fn::h56950a2b577630famUn
  16:     0x7f4589c29f22 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::h6380ca56bad0b447pRn
  17:     0x7f4589ceadaa - check_crate::closure.38501
  18:     0x7f4589ce6140 - check_crate::h3d2be1a2f2b833642EC
  19:     0x7f458a984798 - driver::phase_3_run_analysis_passes::hb4a5c5398e7ee860tGa
  20:     0x7f458a965e90 - driver::compile_input::h3635371ed416578cQba
  21:     0x7f458aa235d1 - run_compiler::hc7ca7ffdda3bac71F4b
  22:     0x7f458aa20e22 - boxed::F.FnBox<A>::call_box::h3375003542065736186
  23:     0x7f458aa203d9 - rt::unwind::try::try_fn::h12589008938305638456
  24:     0x7f458a4bd588 - rust_try_inner
  25:     0x7f458a4bd575 - rust_try
  26:     0x7f458aa20680 - boxed::F.FnBox<A>::call_box::h9305475706708510069
  27:     0x7f458a448a01 - sys::thread::Thread::new::thread_start::hde1d5c4c7d7f250fT9u
  28:     0x7f4584537373 - start_thread
  29:     0x7f458a0a027c - clone
  30:                0x0 - <unknown>
