 rust
   Compiling regex v0.1.73                                                                                                                                                                                   [36/902]
     Running `rustc /home/bheesham/.cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.1.73/src/lib.rs --crate-name regex --crate-type lib -C opt-level=3 -C metadata=a1c323daba09617d -C extra-filename=-a1c323d
aba09617d --out-dir /tmp/cargo-install.LvzOMyvmwYQf/release/deps --emit=dep-info,link -L dependency=/tmp/cargo-install.LvzOMyvmwYQf/release/deps -L dependency=/tmp/cargo-install.LvzOMyvmwYQf/release/deps --extern 
aho_corasick=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libaho_corasick-e528bf4fdf3954ff.rlib --extern regex_syntax=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libregex_syntax-b24da02611351433.rlib --extern mem
chr=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libmemchr-c555f740a543880f.rlib --extern utf8_ranges=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libutf8_ranges-5c6a6dacba3be7ce.rlib --extern thread_local=/tmp/ca
rgo-install.LvzOMyvmwYQf/release/deps/libthread_local-e5ce0d44bcaf00e6.rlib --cap-lints allow`
error: internal compiler error: ../src/librustc/infer/region_inference/mod.rs:734: cannot relate bound region: '_#0r <= ReEarlyBound(TypeSpace, 0, 'r)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:580
stack backtrace:
   1:     0x7f39dba1512f - std::sys::backtrace::tracing::imp::write::h29f5fdb9fc0a7395
   2:     0x7f39dba2c7fb - std::panicking::default_hook::_{{closure}}::h2cc84f0378700526
   3:     0x7f39dba25377 - std::panicking::default_hook::hbbe7fa36a995aca0
   4:     0x7f39dba25a9c - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   5:     0x7f39daf532f8 - std::panicking::begin_panic::hccc513334ab977d2
   6:     0x7f39dafb88a5 - rustc::session::opt_span_bug_fmt::_{{closure}}::h62b0957667555cfe
   7:     0x7f39dafb867d - rustc::ty::context::tls::with_opt::_{{closure}}::h9c829922aa02dbc6
   8:     0x7f39dafb6efd - rustc::ty::context::tls::with::_{{closure}}::hbf1d3065f68fcd1e
   9:     0x7f39d7f62454 - rustc::session::opt_span_bug_fmt::hb71219f119a31511
  10:     0x7f39d7f62374 - rustc::session::span_bug_fmt::hcdfbe9cf944f6fc3
  11:     0x7f39d7ea622e - rustc::infer::region_inference::RegionVarBindings::make_subregion::h317992193ee4c5a0
  12:     0x7f39d7ea5dbb - rustc::infer::region_inference::RegionVarBindings::make_eqregion::h0e220ba3bafe2b05
  13:     0x7f39d7e826f9 - _<rustc..infer..equate..Equate<'a, 'gcx, 'tcx> as rustc..ty..relate..TypeRelation<'a, 'gcx, 'tcx>>::regions::hfeaa1987b2434170
  14:     0x7f39d7dd3192 - _<&'a mut I as core..iter..iterator..Iterator>::next::h4e9e3cae09381cb3
  15:     0x7f39d7fc2b20 - rustc::ty::relate::relate_substs::hfca1bbf0a4c87b72
  16:     0x7f39d7fbca56 - rustc::ty::relate::relate_item_substs::h2ca943a9161309fa
  17:     0x7f39d7e809eb - _<rustc..infer..equate..Equate<'a, 'gcx, 'tcx> as rustc..ty..relate..TypeRelation<'a, 'gcx, 'tcx>>::tys::h071cff434ebc2062
  18:     0x7f39d802a10f - rustc::ty::relate::relate_type_params::_{{closure}}::h8ac4475a7c689931
  19:     0x7f39d7fc239a - rustc::ty::relate::relate_substs::hfca1bbf0a4c87b72
  20:     0x7f39d7fbca56 - rustc::ty::relate::relate_item_substs::h2ca943a9161309fa
  21:     0x7f39d7fc6f19 - _<rustc..ty..sty..TraitRef<'tcx> as rustc..ty..relate..Relate<'tcx>>::relate::ha893f4c3644ad0ba
  22:     0x7f39d805c811 - rustc::infer::InferCtxt::eq_trait_refs::_{{closure}}::ha2acc0c117c4abfe
  23:     0x7f39d7ec15c2 - rustc::infer::InferCtxt::eq_trait_refs::ha953de9a6de4df6f
  24:     0x7f39d7f9004d - rustc::traits::select::SelectionContext::match_impl::h5f197969667ff0a7
  25:     0x7f39d8071435 - rustc::traits::select::SelectionContext::assemble_candidates_from_impls::_{{closure}}::_{{closure}}::h99d7024cc5b33d57
  26:     0x7f39d7fcf0b0 - rustc::ty::trait_def::TraitDef::for_each_relevant_impl::h3e2606f0bfb24ea5
  27:     0x7f39d7f880b6 - rustc::traits::select::SelectionContext::assemble_candidates::h320e58c9e1989fd0
  28:     0x7f39d7f85dea - rustc::traits::select::SelectionContext::candidate_from_obligation::h1953bec99c7b3463
  29:     0x7f39d7f83df6 - rustc::traits::select::SelectionContext::select::h78564e26e8d35d15
  30:     0x7f39d805f8c2 - rustc::traits::project::assemble_candidates_from_impls::_{{closure}}::hfcfd6050f6426acd
  31:     0x7f39d7f7c304 - rustc::traits::project::opt_normalize_projection_type::hac8607daffaabac5
  32:     0x7f39d7f7afd1 - rustc::traits::project::normalize_projection_type::hc8d921334194224d
  33:     0x7f39d7f7a77c - _<rustc..traits..project..AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hf3bb60f68196fec9
  34:     0x7f39d7f7aa04 - _<rustc..traits..project..AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc..ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::hf3bb60f68196fec9
  35:     0x7f39d7f78af3 - rustc::traits::project::normalize_with_depth::h6465df03df9d3bb9
  36:     0x7f39d7fb37be - rustc::ty::layout::normalize_associated_type::h12ef1a63a80457ae
  37:     0x7f39d7fb4776 - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  38:     0x7f39d7fd8907 - rustc::ty::util::_<impl rustc..ty..TyS<'tcx>>::layout::h6e715527ac894584
  39:     0x7f39d7fb478e - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  40:     0x7f39d7fd8907 - rustc::ty::util::_<impl rustc..ty..TyS<'tcx>>::layout::h6e715527ac894584
  41:     0x7f39d7fb6828 - rustc::ty::layout::Layout::compute_uncached::h52016a57380b87fc
  42:     0x7f39da378956 - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::_{{closure}}::h6b7ff37c120a8611
  43:     0x7f39da3766d3 - rustc::infer::InferCtxtBuilder::enter::_{{closure}}::h10eb5ff826c2a812
  44:     0x7f39da371c6b - _<rustc_lint..types..VariantSizeDifferences as rustc..lint..LateLintPass>::check_item::h4ad35c8e1975958a
  45:     0x7f39d80619df - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  46:     0x7f39d7edefdc - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  47:     0x7f39d7ee11ab - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  48:     0x7f39d8061c54 - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::_{{closure}}::h51b3d4cdb6cc3f1c
  49:     0x7f39d7edefdc - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h1fbfa027bcad3e54
  50:     0x7f39d7ee11ab - _<rustc..lint..context..LateContext<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_mod::h8bbbe7b918370796
  51:     0x7f39d80614ce - rustc::lint::context::check_crate::_{{closure}}::hfe9ec516ca63b199
  52:     0x7f39d7ee864d - rustc::lint::context::check_crate::hb1b51642e0e5e526
  53:     0x7f39dc03c6c8 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h94bde61ccfd9ab2d
  54:     0x7f39dc0359d2 - rustc::ty::context::tls::enter_global::_{{closure}}::hf534c52a5122a522
  55:     0x7f39dbf94a91 - rustc::ty::context::TyCtxt::create_and_enter::h7a94c61bd28cb0b9
  56:     0x7f39dbff20c3 - rustc_driver::driver::compile_input::hb4cc34cf85dc1edf
  57:     0x7f39dc0185dc - rustc_driver::run_compiler::h50f95674bd902ab5
  58:     0x7f39dc08f207 - rustc_driver::run::_{{closure}}::h578f58fb80a79b9c
  59:     0x7f39dc08a0ab - rustc_driver::monitor::_{{closure}}::he296944eac047e81
  60:     0x7f39dbf6d48d - std::panicking::try::call::h4577500a5284c6ff
  61:     0x7f39dba3a87b - __rust_try
  62:     0x7f39dba3a75e - __rust_maybe_catch_panic
  63:     0x7f39dc0344ed - std::thread::Builder::spawn::_{{closure}}::haaf8628155e4305c
  64:     0x7f39dbf872ee - _<F as alloc..boxed..FnBox<A>>::call_box::h24f3eb0b42327962
  65:     0x7f39dba232c4 - std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea
  66:     0x7f39d2dd5619 - start_thread
  67:     0x7f39db65859c - clone
  68:                0x0 - <unknown>

Build failed, waiting for other jobs to finish...
error: failed to compile `rustfmt v0.5.0`, intermediate artifacts can be found at `/tmp/cargo-install.LvzOMyvmwYQf`

Caused by:
  Could not compile `regex`.

Caused by:
  Process didn't exit successfully: `rustc /home/bheesham/.cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.1.73/src/lib.rs --crate-name regex --crate-type lib -C opt-level=3 -C metadata=a1c323daba09617d -C 
extra-filename=-a1c323daba09617d --out-dir /tmp/cargo-install.LvzOMyvmwYQf/release/deps --emit=dep-info,link -L dependency=/tmp/cargo-install.LvzOMyvmwYQf/release/deps -L dependency=/tmp/cargo-install.LvzOMyvmwYQf
/release/deps --extern aho_corasick=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libaho_corasick-e528bf4fdf3954ff.rlib --extern regex_syntax=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libregex_syntax-b24da026113
51433.rlib --extern memchr=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libmemchr-c555f740a543880f.rlib --extern utf8_ranges=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libutf8_ranges-5c6a6dacba3be7ce.rlib --exte
rn thread_local=/tmp/cargo-install.LvzOMyvmwYQf/release/deps/libthread_local-e5ce0d44bcaf00e6.rlib --cap-lints allow` (exit code: 101)
