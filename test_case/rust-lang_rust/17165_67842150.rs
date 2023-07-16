
unused manifest key: profile.build.debug
       Fresh traverse v0.0.2
   Compiling collect v0.0.6 (file:///C:/users/alexis/documents/github/collect-rs)
     Running `rustc C:\users\alexis\documents\github\collect-rs\src\lib.rs --crate-name collect --crate-type lib --cfg ndebug --test -C metadata=507ed9788c885314 -C extra-filename=-507ed9788c885314 --out-dir C:\users\alexis\documents\github\collect-rs\target --emit=dep-info,link -L C:\users\alexis\documents\github\collect-rs\target -L C:\users\alexis\documents\github\collect-rs\target\deps --extern traverse=C:\users\alexis\documents\github\collect-rs\target\deps/libtraverse-c0a1cc9151073bab.rlib`
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1036:5: 1042:6 error: internal compiler error: cannot relate bound region: ReLateBound(DebruijnIndex { depth: 1 }, BrAnon(0)) <= ReLateBound(DebruijnIndex { depth: 2 }, BrAnon(0))
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1036     impl<'a, 'b> FnMut(&int) -> bool for Counter<'a, 'b> {
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1037         extern "rust-call" fn call_mut(&mut self, (&x,): (&int,)) -> bool {
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1038             assert_eq!(x, self.expected[*self.i]);
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1039             *self.i += 1;
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1040             true
C:\users\alexis\documents\github\collect-rs\src\tree\set.rs:1041         }
                                                                 ...
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', C:\bot\slave\nightly-win-64\build\src\libsyntax\diagnostic.rs:123

stack backtrace:
   1:         0x69bef7f8 - sys::backtrace::write::h5b0ba28ad71857afeQt
   2:         0x69c021e9 - rt::unwind::register::h91aefd9b472076aecGz
   3:         0x69b82fcb - rt::unwind::begin_unwind_inner::h04b772eae7a7fc7bJDz
   4:         0x6f8acd56 - diagnostic::SpanHandler::span_bug::h90408fcd1499494fwZF
   5:         0x6f8acd00 - diagnostic::SpanHandler::span_bug::h90408fcd1499494fwZF
   6:           0x972a31 - middle::infer::region_inference::RegionVarBindings<$u{27}a$C$$u{20}$u{27}tcx$GT$::make_subregion::h2d032b24a1882941bAw
   7:           0x90f621 - middle::infer::region_inference::RegionVarBindings<$u{27}a$C$$u{20}$u{27}tcx$GT$::make_eqregion::h2d032b24a1882941zzw
   8:           0x90f4a3 - middle::infer::equate::Equate<$u{27}f$C$$u{20}$u{27}tcx$GT$.Combine$LT$$u{27}tcx$GT$::regions::hc467de7375d80198Akr
   9:           0x908f8f - middle::infer::equate::Equate<$u{27}f$C$$u{20}$u{27}tcx$GT$.Combine$LT$$u{27}tcx$GT$::tys::h912a5dd2b206dbe1Frr
  10:           0x921c09 - middle::ty::mk_unboxed_closure::hcc6ac3bf2616fcf0sv2
  11:           0x9095c5 - middle::infer::equate::Equate<$u{27}f$C$$u{20}$u{27}tcx$GT$.Combine$LT$$u{27}tcx$GT$::tys::h912a5dd2b206dbe1Frr
  12:           0x91f43a - util::ppaux::ty..BoundRegion.Repr<$u{27}tcx$GT$::repr::hbe9f644adb44adbbPbt
  13:           0x91efd3 - util::ppaux::ty..BoundRegion.Repr<$u{27}tcx$GT$::repr::hbe9f644adb44adbbPbt
  14:           0x91eca2 - util::ppaux::ty..BoundRegion.Repr<$u{27}tcx$GT$::repr::hbe9f644adb44adbbPbt
  15:           0x99113e - middle::infer::InferCtxt<$u{27}a$C$$u{20}$u{27}tcx$GT$::sub_trait_refs::h390adbc09a3b494bgyB
  16:           0xa00c4e - middle::ty::TraitRef<$u{27}tcx$GT$...std..cmp..PartialEq::eq::h543edb6d2ba7d3b3ruZ
  17:           0xa0ca51 - middle::ty::GenericBounds<$u{27}tcx$GT$::is_empty::hc31f5ae763726d73PQ1
  18:           0xa07660 - middle::traits::select::Candidate<$u{27}tcx$GT$.Repr$LT$$u{27}tcx$GT$::repr::h48b108f0ecd54c0dbPS
  19:           0x9f8055 - middle::ty::TraitRef<$u{27}tcx$GT$.RegionEscape::has_regions_escaping_depth::he874f51e3eb2d51fZQ7
  20:           0x9f4b4c - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::select::hf1da19fa1e7dadc9ZoQ
  21:           0x9ee4cd - middle::traits::select::SelectionContext<$u{27}cx$C$$u{20}$u{27}tcx$GT$::new::hd2f65bc2e1895228nnQ
  22:           0x8c7a83 - middle::traits::fulfill::FulfillmentContext<$u{27}tcx$GT$::select_all_or_error::hcc757e1ded501152KKP
  23:         0x6d4db274
  24:         0x6d54dbe0 - check::method::CandidateSource...std..cmp..PartialOrd::ge::he78261c7d2364bd3edi
  25:         0x6d552623 - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  26:         0x6d553f9f - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  27:         0x6d5526f6 - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  28:         0x6d553f9f - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  29:         0x6d5526f6 - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  30:         0x6d553f9f - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  31:         0x6d5526f6 - check::wf::CheckTypeWellFormedVisitor<$u{27}ccx$C$$u{20}$u{27}tcx$GT$.Visitor$LT$$u{27}v$GT$::visit_item::h83ad74d1a29d2d8c58i
  32:         0x6d8273db - check_crate::hd1761d8b5c95808dQZy
  33:         0x6d822422 - check_crate::hd1761d8b5c95808dQZy
  34:         0x70b227ae - driver::phase_3_run_analysis_passes::h50b942faf81eb47bIta
  35:         0x70b03695 - driver::compile_input::hd01226bf772688afvba
  36:         0x70ca3e95 - run::h9cdc49dc8c011aeedYb
  37:         0x70ca05b9 - run::h9cdc49dc8c011aeedYb
  38:         0x69c6648c - rust_try
  39:         0x69c66469 - rust_try
  40:         0x70ca0d0f - run::h9cdc49dc8c011aeedYb
  41:         0x69bf4ee7 - sys::tcp::TcpListener::bind::hb1b42652ff0a60d9Fqw
  42:     0x7ffcd28616ad - BaseThreadInitThunk

Could not compile `collect`.

Caused by:
  Process didn't exit successfully: `rustc C:\users\alexis\documents\github\collect-rs\src\lib.rs --crate-name collect --crate-type lib --cfg ndebug --test -C metadata=507ed9788c885314 -C extra-filename=-507ed9788c885314 --out-dir C:\users\alexis\documents\github\collect-rs\target --emit=dep-info,link -L C:\users\alexis\documents\github\collect-rs\target -L C:\users\alexis\documents\github\collect-rs\target\deps --extern traverse=C:\users\alexis\documents\github\collect-rs\target\deps/libtraverse-c0a1cc9151073bab.rlib` (status=101)
