
   Compiling xml-rs v0.3.4
error: internal compiler error: ../src/librustc/infer/combine.rs:348: Encountered early bound region when generalizing: ReEarlyBound(TypeSpace, 0, 'a)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:580
stack backtrace:
   1:        0x10679d53b - std::sys::backtrace::tracing::imp::write::h29f5fdb9fc0a7395
   2:        0x1067b3c8a - std::panicking::default_hook::_{{closure}}::h2cc84f0378700526
   3:        0x1067ac219 - std::panicking::default_hook::hbbe7fa36a995aca0
   4:        0x1067ac958 - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   5:        0x101b6eeeb - std::panicking::begin_panic::hccc513334ab977d2
   6:        0x101c9ff9d - rustc::session::opt_span_bug_fmt::_{{closure}}::h62b0957667555cfe
   7:        0x101c974da - rustc::ty::context::tls::with_opt::_{{closure}}::h9c829922aa02dbc6
   8:        0x101c8d15e - rustc::ty::context::tls::with::_{{closure}}::hbf1d3065f68fcd1e
   9:        0x102f4cfb0 - rustc::session::opt_span_bug_fmt::hb71219f119a31511
  10:        0x102f4cea7 - rustc::session::span_bug_fmt::hcdfbe9cf944f6fc3
  11:        0x102f953ad - rustc::ty::fold::TypeFolder::fold_substs::hc69b98ee2dbfb317
  12:        0x102e65347 - _<rustc..infer..combine..Generalizer<'cx, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hff97da309fb356f8
  13:        0x102e65316 - _<rustc..infer..combine..Generalizer<'cx, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hff97da309fb356f8
  14:        0x102e64a18 - rustc::infer::combine::CombineFields::instantiate::h1684211fdd70f7fb
  15:        0x102e65c21 - _<rustc..infer..equate..Equate<'a, 'gcx, 'tcx> as rustc..ty..relate..TypeRelation<'a, 'gcx, 'tcx>>::tys::h071cff434ebc2062
  16:        0x10301b774 - rustc::ty::relate::relate_type_params::_{{closure}}::h8ac4475a7c689931
  17:        0x102fafd6f - rustc::ty::relate::relate_substs::hfca1bbf0a4c87b72
  18:        0x102faa4bf - rustc::ty::relate::relate_item_substs::h2ca943a9161309fa
  19:        0x102fb43c5 - _<rustc..ty..sty..TraitRef<'tcx> as rustc..ty..relate..Relate<'tcx>>::relate::ha893f4c3644ad0ba
  20:        0x1030509b6 - rustc::infer::InferCtxt::eq_trait_refs::_{{closure}}::ha2acc0c117c4abfe
  21:        0x102ea73a6 - rustc::infer::InferCtxt::eq_trait_refs::ha953de9a6de4df6f
  22:        0x102f7b9b3 - rustc::traits::select::SelectionContext::match_impl::h5f197969667ff0a7
  23:        0x1030664ab - rustc::traits::select::SelectionContext::assemble_candidates_from_impls::_{{closure}}::_{{closure}}::h99d7024cc5b33d57
  24:        0x102fbc262 - rustc::ty::trait_def::TraitDef::for_each_relevant_impl::h3e2606f0bfb24ea5
  25:        0x102f737e8 - rustc::traits::select::SelectionContext::assemble_candidates::h320e58c9e1989fd0
  26:        0x102f713f3 - rustc::traits::select::SelectionContext::candidate_from_obligation::h1953bec99c7b3463
  27:        0x102f6f396 - rustc::traits::select::SelectionContext::select::h78564e26e8d35d15
  28:        0x103053834 - rustc::traits::project::assemble_candidates_from_impls::_{{closure}}::hfcfd6050f6426acd
  29:        0x102f6731a - rustc::traits::project::opt_normalize_projection_type::hac8607daffaabac5
  30:        0x102f65f4a - rustc::traits::project::normalize_projection_type::hc8d921334194224d
  31:        0x102f6561d - _<rustc..traits..project..AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hf3bb60f68196fec9
  32:        0x102f6352a - rustc::traits::project::normalize_with_depth::h6465df03df9d3bb9
  33:        0x102fa12bf - rustc::ty::layout::normalize_associated_type::h12ef1a63a80457ae
  34:        0x1030270d2 - rustc::ty::layout::Layout::compute_uncached::_{{closure}}::hd50a310e56175a45
  35:        0x102fa404c - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  36:        0x102fc60a7 - rustc::ty::util::_<impl rustc..ty..TyS<'tcx>>::layout::h6e715527ac894584
  37:        0x103025d28 - rustc::ty::layout::Layout::compute_uncached::_{{closure}}::h08e14e2c0e8eddad
  38:        0x102d32b36 - _<core..option..Option<T>>::map::hbc7b31596bff422b
  39:        0x102db439b - _<&'a mut I as core..iter..iterator..Iterator>::next::h7900471b21c85cc0
  40:        0x102d33543 - _<collections..vec..Vec<T>>::extend_desugared::hd43be45a514f90ff
  41:        0x102d2f0e1 - _<core..result..Result<V, E> as core..iter..traits..FromIterator<core..result..Result<A, E>>>::from_iter::h244cdc9f241c9a66
  42:        0x102fa4b16 - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  43:        0x102b638d6 - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::_{{closure}}::h6b7ff37c120a8611
  44:        0x102b61635 - rustc::infer::InferCtxtBuilder::enter::_{{closure}}::h10eb5ff826c2a812
  45:        0x102b5c867 - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::h4ad35c8e1975958a
  46:        0x103055b8f - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  47:        0x102ec5581 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  48:        0x102ec79ac - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  49:        0x103055dd1 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  50:        0x102ec5581 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  51:        0x102ec79ac - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  52:        0x103055dd1 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  53:        0x102ec5581 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  54:        0x102ec79ac - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  55:        0x103055672 - rustc::lint::context::check_crate::_{{closure}}::hfe9ec516ca63b199
  56:        0x102ecf77e - rustc::lint::context::check_crate::hb1b51642e0e5e526
  57:        0x1018b99c4 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h94bde61ccfd9ab2d
  58:        0x1018b30bd - rustc::ty::context::tls::enter_global::_{{closure}}::hf534c52a5122a522
  59:        0x10181378c - rustc::ty::context::TyCtxt::create_and_enter::h7a94c61bd28cb0b9
  60:        0x10186f099 - rustc_driver::driver::compile_input::hb4cc34cf85dc1edf
  61:        0x101894815 - rustc_driver::run_compiler::h50f95674bd902ab5
  62:        0x10190d98f - rustc_driver::run::_{{closure}}::h578f58fb80a79b9c
  63:        0x101908610 - rustc_driver::monitor::_{{closure}}::he296944eac047e81
  64:        0x1017eaba1 - std::panicking::try::call::h4577500a5284c6ff
  65:        0x1067ba5fb - __rust_try
  66:        0x1067ba4b5 - __rust_maybe_catch_panic
  67:        0x1018b1b06 - std::thread::Builder::spawn::_{{closure}}::haaf8628155e4305c
  68:        0x1018056f8 - _<F as alloc..boxed..FnBox<A>>::call_box::h24f3eb0b42327962
  69:        0x1067aac98 - std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea
  70:     0x7fff83f5799c - _pthread_body
  71:     0x7fff83f57919 - _pthread_start

error: Could not compile `xml-rs`.

