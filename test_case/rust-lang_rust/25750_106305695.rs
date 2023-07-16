
stack backtrace:
   1:     0x7f947750c183 - sys::backtrace::write::h615fdb4d732e4a93r6r
   2:     0x7f9477514049 - panicking::on_panic::h9a7e45e333684aa03Lw
   3:     0x7f94774d4e3a - rt::unwind::begin_unwind_inner::h995a60547c3f562eNrw
   4:     0x7f947482cbbd - rt::unwind::begin_unwind::h17375738179658217201
   5:     0x7f947482cb52 - diagnostic::SpanHandler::span_bug::h8482da51d213c57buqC
   6:     0x7f9475553cda - middle::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_subregion::hb699f35a3cfa39c6UPv
   7:     0x7f947556cd1f - middle::infer::mk_subr::h93c112efb784e579G5y
   8:     0x7f9476c3ba0f - check::regionck::type_must_outlive::h68e52339b49b7368Tte
   9:     0x7f9476c39baa - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  10:     0x7f9476c39d5c - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  11:     0x7f9476c39d5c - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  12:     0x7f9476c39f13 - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  13:     0x7f9476c39d5c - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  14:     0x7f9476c39d5c - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  15:     0x7f9476c39d5c - check::dropck::iterate_over_potentially_unsafe_regions_in_type::h80326f55a8cc05afZDa
  16:     0x7f9476c38738 - check::dropck::check_safety_of_destructor_if_necessary::h6d78b805219652e19xa
  17:     0x7f9476c7033d - check::regionck::check_safety_of_rvalue_destructor_if_necessary::ha65a719fd22d79bdjPd
  18:     0x7f9476c69b81 - check::regionck::visit_expr::h15cc0f7c6e849ab0fad
  19:     0x7f9476c67bb1 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h461f8d4e496574beHMc
  20:     0x7f9476c67090 - check::regionck::regionck_fn::hf51a587df53131353Ec
  21:     0x7f9476ce2dc8 - check::check_bare_fn::h56477091183bbe08gOn
  22:     0x7f9476ceed5c - check::check_method_body::h11b8e4f5d8c75e9crro
  23:     0x7f9476ce0913 - check::check_item_body::hc9468be7e10fb9049eo
  24:     0x7f9476ce0f02 - visit::walk_item::h10209465936042145449
  25:     0x7f9476ce278f - check::check_item_types::h22d39728d2ed74ffNLn
  26:     0x7f9476d9f026 - check_crate::h0c48e68f075d51d7g7C
  27:     0x7f9477a68d9f - driver::phase_3_run_analysis_passes::h34501209e3394a5cGGa
  28:     0x7f9477a4bf3c - driver::compile_input::h69c18a97a3bef03fQba
  29:     0x7f9477b01b31 - run_compiler::h737927eeae91ebd5n6b
  30:     0x7f9477aff382 - boxed::F.FnBox<A>::call_box::h1962494538046773488
  31:     0x7f9477afeb89 - rt::unwind::try::try_fn::h2931293082837064848
  32:     0x7f9477597138 - rust_try_inner
  33:     0x7f9477597125 - rust_try
  34:     0x7f94774ffc37 - rt::unwind::try::inner_try::h80226bb4deb2a8ebGnw
  35:     0x7f9477afeda9 - boxed::F.FnBox<A>::call_box::h2098047553192396313
  36:     0x7f9477512d01 - sys::thread::Thread::new::thread_start::h97730f674d383241Sxv
  37:     0x7f9471f720a3 - start_thread
  38:     0x7f947715c04c - clone
  39:                0x0 - <unknown>
