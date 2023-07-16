
test.rs:14:73: 14:79 error: internal compiler error: Failed to unify `Obligation(predicate=<<T as Trait>::A as MultiDispatch<_>>::O,depth=0)` and `ProjectionPredicate(<<T as Trait>::A as MultiDispatch<<T as Trait>::B>>::O, T)` in projection: expected associated type, found i32
test.rs:14 fn test<T: Trait<B=i32>>(b: i32) -> T where T::A : MultiDispatch<i32> { T::new(b) } 
                                                                                   ^~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:130

stack backtrace:
   1:        0x104a09f77 - sys::backtrace::write::h799ab135045ca9ca1LC
   2:        0x104a37e4d - panicking::on_panic::h9502e488a92fc8bcHAI
   3:        0x10495c2de - rt::unwind::begin_unwind_inner::hf0537788ee16eadbiiI
   4:        0x1041335ae - rt::unwind::begin_unwind::h447731439410619827
   5:        0x10413355b - diagnostic::SpanHandler::span_bug::h3f51c1d819f22bcfjnB
   6:        0x101c2ba43 - middle::traits::project::confirm_param_env_candidate::hf852dc0f5b320b18N1P
   7:        0x101c23962 - middle::traits::project::project_type::h7ca468ea33bc79f1AwP
   8:        0x101c1f367 - middle::traits::project::opt_normalize_projection_type::h766518f8b1310230cpP
   9:        0x101c083bf - middle::traits::project::normalize_projection_type::h734766f7a9915c990nP
  10:        0x101bf59f0 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h3729e5ef2601270eElP
  11:        0x101bf509f - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h3729e5ef2601270eElP
  12:        0x100f953a8 - check::FnCtxt<'a, 'tcx>::normalize_associated_types_in::h16934591160211678581
  13:        0x100f94d2e - check::FnCtxt<'a, 'tcx>::instantiate_type_scheme::h5994741691476743977
  14:        0x100fa3b96 - check::instantiate_path::h72ec382ec88529f7HQs
  15:        0x1010cb36a - check::check_expr_with_unifier::h6698900367440735538
  16:        0x101053279 - check::callee::check_call::hf15968ef2ca0f38apcm
  17:        0x1010ab233 - check::check_expr_with_unifier::h7970691808293538245
  18:        0x1010759db - check::check_block_with_expected::h60bff5b1f9d55793fps
  19:        0x101051e05 - check::check_fn::h410d0ffafb47f675l1n
  20:        0x10107032e - check::check_bare_fn::h05c77be534ada2cfUQn
  21:        0x101069ee2 - check::check_item::h9d89bedf29e4fe82F9n
  22:        0x101146676 - check_crate::closure.36302
  23:        0x101140ae7 - check_crate::h09aa7856a322fd44bwC
  24:        0x100e61355 - driver::phase_3_run_analysis_passes::h1af60e7c926ca39dgGa
  25:        0x100e45a85 - driver::compile_input::h27cf4320fd56e5cbQba
  26:        0x100f03a15 - run_compiler::h93c68a2a3c26052bV4b
  27:        0x100f011e2 - boxed::F.FnBox<A>::call_box::h4498816167135712709
  28:        0x100f006d7 - rt::unwind::try::try_fn::h2387014819904265058
  29:        0x104ac1ea8 - rust_try_inner
  30:        0x104ac1e95 - rust_try
  31:        0x100f009ca - boxed::F.FnBox<A>::call_box::h4850138824497903476
  32:        0x104a2137d - sys::thread::create::thread_start::h42d6d904fce5365aihH
  33:     0x7fff91f75898 - _pthread_body
  34:     0x7fff91f75729 - _pthread_start
