
stack backtrace:
   1:        0x108bb553b - std::sys::backtrace::tracing::imp::write::h29f5fdb9fc0a7395
   2:        0x108bcbc8a - std::panicking::default_hook::_{{closure}}::h2cc84f0378700526
   3:        0x108bc4219 - std::panicking::default_hook::hbbe7fa36a995aca0
   4:        0x108bc4958 - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   5:        0x103f7feeb - std::panicking::begin_panic::hccc513334ab977d2
   6:        0x1040b0f9d - rustc::session::opt_span_bug_fmt::_{{closure}}::h62b0957667555cfe
   7:        0x1040a84da - rustc::ty::context::tls::with_opt::_{{closure}}::h9c829922aa02dbc6
   8:        0x10409e15e - rustc::ty::context::tls::with::_{{closure}}::hbf1d3065f68fcd1e
   9:        0x105366fb0 - rustc::session::opt_span_bug_fmt::hb71219f119a31511
  10:        0x105366ea7 - rustc::session::span_bug_fmt::hcdfbe9cf944f6fc3
  11:        0x1052a52a9 - rustc::infer::region_inference::RegionVarBindings::make_subregion::h317992193ee4c5a0
  12:        0x1052a4dc9 - rustc::infer::region_inference::RegionVarBindings::make_eqregion::h0e220ba3bafe2b05
  13:        0x105281bda - _<rustc..infer..equate..Equate<'a, 'gcx, 'tcx> as rustc..ty..relate..TypeRelation<'a, 'gcx, 'tcx>>::regions::hfeaa1987b2434170
  14:        0x1051ce24c - _<&'a mut I as core..iter..iterator..Iterator>::next::h4e9e3cae09381cb3
  15:        0x1053ca3f1 - rustc::ty::relate::relate_substs::hfca1bbf0a4c87b72
  16:        0x1053c44bf - rustc::ty::relate::relate_item_substs::h2ca943a9161309fa
  17:        0x1052803cc - _<rustc..infer..equate..Equate<'a, 'gcx, 'tcx> as rustc..ty..relate..TypeRelation<'a, 'gcx, 'tcx>>::tys::h071cff434ebc2062
  18:        0x105435774 - rustc::ty::relate::relate_type_params::_{{closure}}::h8ac4475a7c689931
  19:        0x1053c9d6f - rustc::ty::relate::relate_substs::hfca1bbf0a4c87b72
  20:        0x1053c44bf - rustc::ty::relate::relate_item_substs::h2ca943a9161309fa
  21:        0x1053ce3c5 - _<rustc..ty..sty..TraitRef<'tcx> as rustc..ty..relate..Relate<'tcx>>::relate::ha893f4c3644ad0ba
  22:        0x10546a9b6 - rustc::infer::InferCtxt::eq_trait_refs::_{{closure}}::ha2acc0c117c4abfe
  23:        0x1052c13a6 - rustc::infer::InferCtxt::eq_trait_refs::ha953de9a6de4df6f
  24:        0x1053959b3 - rustc::traits::select::SelectionContext::match_impl::h5f197969667ff0a7
  25:        0x1054804ab - rustc::traits::select::SelectionContext::assemble_candidates_from_impls::_{{closure}}::_{{closure}}::h99d7024cc5b33d57
  26:        0x1053d645b - rustc::ty::trait_def::TraitDef::for_each_relevant_impl::h3e2606f0bfb24ea5
  27:        0x10538d7e8 - rustc::traits::select::SelectionContext::assemble_candidates::h320e58c9e1989fd0
  28:        0x10538b3f3 - rustc::traits::select::SelectionContext::candidate_from_obligation::h1953bec99c7b3463
  29:        0x105389396 - rustc::traits::select::SelectionContext::select::h78564e26e8d35d15
  30:        0x10546d834 - rustc::traits::project::assemble_candidates_from_impls::_{{closure}}::hfcfd6050f6426acd
  31:        0x10538131a - rustc::traits::project::opt_normalize_projection_type::hac8607daffaabac5
  32:        0x10537ff4a - rustc::traits::project::normalize_projection_type::hc8d921334194224d
  33:        0x10537f61d - _<rustc..traits..project..AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hf3bb60f68196fec9
  34:        0x10537f90a - _<rustc..traits..project..AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hf3bb60f68196fec9
  35:        0x10537d52a - rustc::traits::project::normalize_with_depth::h6465df03df9d3bb9
  36:        0x1053bb2bf - rustc::ty::layout::normalize_associated_type::h12ef1a63a80457ae
  37:        0x1053bc3a6 - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  38:        0x1053e00a7 - rustc::ty::util::_<impl rustc..ty..TyS<'tcx>>::layout::h6e715527ac894584
  39:        0x1053bc3bc - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  40:        0x1053e00a7 - rustc::ty::util::_<impl rustc..ty..TyS<'tcx>>::layout::h6e715527ac894584
  41:        0x1053be6bf - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  42:        0x104f7a8d6 - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::_{{closure}}::h6b7ff37c120a8611
  43:        0x104f78635 - rustc::infer::InferCtxtBuilder::enter::_{{closure}}::h10eb5ff826c2a812
  44:        0x104f73867 - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::h4ad35c8e1975958a
  45:        0x10546fb8f - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  46:        0x1052df581 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  47:        0x1052e19ac - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  48:        0x10546fdd1 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  49:        0x1052df581 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  50:        0x1052e19ac - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  51:        0x10546f672 - rustc::lint::context::check_crate::_{{closure}}::hfe9ec516ca63b199
  52:        0x1052e977e - rustc::lint::context::check_crate::hb1b51642e0e5e526
  53:        0x103cc89c4 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h94bde61ccfd9ab2d
  54:        0x103cc20bd - rustc::ty::context::tls::enter_global::_{{closure}}::hf534c52a5122a522
  55:        0x103c2278c - rustc::ty::context::TyCtxt::create_and_enter::h7a94c61bd28cb0b9
  56:        0x103c7e099 - rustc_driver::driver::compile_input::hb4cc34cf85dc1edf
  57:        0x103ca3815 - rustc_driver::run_compiler::h50f95674bd902ab5
  58:        0x103d1c98f - rustc_driver::run::_{{closure}}::h578f58fb80a79b9c
  59:        0x103d17610 - rustc_driver::monitor::_{{closure}}::he296944eac047e81
  60:        0x103bf9ba1 - std::panicking::try::call::h4577500a5284c6ff
  61:        0x108bd25fb - __rust_try
  62:        0x108bd24b5 - __rust_maybe_catch_panic
  63:        0x103cc0b06 - std::thread::Builder::spawn::_{{closure}}::haaf8628155e4305c
  64:        0x103c146f8 - _<F as alloc..boxed..FnBox<A>>::call_box::h24f3eb0b42327962
  65:        0x108bc2c98 - std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea
  66:     0x7fff84f3f99c - _pthread_body
  67:     0x7fff84f3f919 - _pthread_start
