
test2.rs:14:73: 14:79 error: internal compiler error: Failed to unify `Obligation(predicate=ProjectionTy { trait_ref: <<T as Trait>::A as MultiDispatch<_>>, item_name: O(64) },depth=0)` and `ProjectionPredicate(ProjectionTy { trait_ref: <<T as Trait>::A as MultiDispatch<<T as Trait>::B>>, item_name: O(64) }, T)` in projection: expected associated type, found i32
test2.rs:14 fn test<T: Trait<B=i32>>(b: i32) -> T where T::A : MultiDispatch<i32> { T::new(b) }
                                                                                    ^~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:176

stack backtrace:
   1:        0x108accda0 - sys::backtrace::tracing::imp::write::he715e18a60aa49cbY7s
   2:        0x108aca149 - panicking::on_panic::hf85837ce31d02cf7wWw
   3:        0x108a936d2 - sys_common::unwind::begin_unwind_inner::h263a8f3a93eff05evas
   4:        0x1067a4d3a - sys_common::unwind::begin_unwind::h2734773420310895812
   5:        0x1067a4d02 - diagnostic::SpanHandler::span_bug::h2744963912bf7f266NA
   6:        0x105d33c98 - middle::traits::project::confirm_param_env_candidate::hf7d5c75e925102c3TzT
   7:        0x105d2ea6b - middle::traits::project::opt_normalize_projection_type::h4b8003c9d28f5606wNS
   8:        0x105d20870 - middle::traits::project::normalize_projection_type::h6ba4741429d69d6ckMS
   9:        0x105cc358a - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::hc3ec51c79dbc98331JS
  10:        0x105cc2f54 - middle::ty::fold::TypeFolder::fold_binder::h15745587145886015118
  11:        0x105cc37f4 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::hc3ec51c79dbc98331JS
  12:        0x1055eea24 - middle::traits::project::normalize::h8576955180410799035
  13:        0x1055ee26d - check::assoc::normalize_associated_types_in::h2297513543514642305
  14:        0x1055fcd9c - check::instantiate_path::h52b78dac89e9bd79Vzu
  15:        0x1056ee184 - check::check_expr_with_unifier::h16728426817096888307
  16:        0x105685f53 - check::callee::check_call::h579204e755e32a83qVm
  17:        0x1056d7f52 - check::check_expr_with_unifier::h838939668687904838
  18:        0x1056af604 - check::check_block_with_expected::h7c883143007fc089kau
  19:        0x105685720 - check::check_fn::h9cecf86a589f5df33Jp
  20:        0x1056a62fc - check::check_bare_fn::h0cc5198eac888ed5Czp
  21:        0x1056a3c86 - check::check_item_body::hbe1715c86b39f19aW0p
  22:        0x10575c761 - check_crate::h7049eea75dfbb8929CE
  23:        0x104f0bd27 - driver::phase_3_run_analysis_passes::closure.21575
  24:        0x104eef3fa - middle::ty::context::ctxt<'tcx>::create_and_enter::h13460673800781906500
  25:        0x104eeade3 - driver::phase_3_run_analysis_passes::h1322645905457543974
  26:        0x104ecb3b6 - driver::compile_input::hb59f474fec28f4580ba
  27:        0x105028170 - run_compiler::hdc2ad537dbabfa44qqc
  28:        0x1050259cb - boxed::F.FnBox<A>::call_box::h6015891024191261117
  29:        0x1050252d2 - sys_common::unwind::try::try_fn::h8012090198820404000
  30:        0x108ac9a28 - __rust_try
  31:        0x108abe460 - sys_common::unwind::try::inner_try::h93da745d827b7cb5T6r
  32:        0x105025482 - boxed::F.FnBox<A>::call_box::h6138197769578501793
  33:        0x108ad22cd - sys::thread::Thread::new::thread_start::h770a3938c0de9490puw
  34:     0x7fff93838059 - _pthread_body
  35:     0x7fff93837fd6 - _pthread_start
