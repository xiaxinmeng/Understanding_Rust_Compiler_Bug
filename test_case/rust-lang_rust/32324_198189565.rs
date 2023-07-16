
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:330
stack backtrace:
   1:     0x7f413757f240 - sys::backtrace::tracing::imp::write::h3bbb320d569781f92cv
   2:     0x7f413758943f - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44520
   3:     0x7f4137588fb3 - panicking::default_handler::hf1209fec3f963ae7H0z
   4:     0x7f413755142c - sys_common::unwind::begin_unwind_inner::h2ff9ae045f111c6dg2t
   5:     0x7f4137552e18 - sys_common::unwind::begin_unwind_fmt::hf803f209d02109a7m1t
   6:     0x7f413757cb41 - rust_begin_unwind
   7:     0x7f41375d2dcf - panicking::panic_fmt::hd76b622b6f659cc5FRL
   8:     0x7f41375d38c8 - panicking::panic::h129376b8b435084ecQL
   9:     0x7f413493456f - middle::traits::specialize::specialization_graph::Graph::parent::h886c310fe93a88f1Ti0
  10:     0x7f4134934897 - middle::traits::specialize::specialization_graph::Ancestors<'a, 'tcx>.Iterator::next::h5f1f8c8250ab34dbOo0
  11:     0x7f4134934aa5 - iter::FlatMap<I, U, F>.Iterator::next::h12062584353781924757
  12:     0x7f41349095ae - middle::traits::project::assoc_ty_def::h5d120741f44a11c06LV
  13:     0x7f41349050e1 - middle::traits::project::opt_normalize_projection_type::h36d3a11130067195ORU
  14:     0x7f41348f1566 - middle::traits::project::normalize_projection_type::h92bf646187f6dba6DQU
  15:     0x7f413482f56b - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::h3e59de177ca5ef75kOU
  16:     0x7f41348db5b4 - middle::ty::fold::TypeFolder::fold_substs::h17488734294738903057
  17:     0x7f41348db164 - middle::ty::structural_impls::ty..Predicate<'tcx>.TypeFoldable<'tcx>::super_fold_with::h7573436449953994461
  18:     0x7f413492d853 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>::fold::h13602687418497751053
  19:     0x7f413492d473 - middle::traits::select::SelectionContext<'cx, 'tcx>::impl_or_trait_obligations::_$u7b$$u7b$closure$u7d$$u7d$::closure.94585
  20:     0x7f413492c51e - iter::FlatMap<I, U, F>.Iterator::next::h325336949154668876
  21:     0x7f413492b613 - vec::Vec<T>.FromIterator<T>::from_iter::h5234805169717341186
  22:     0x7f4134926156 - middle::traits::select::SelectionContext<'cx, 'tcx>::impl_or_trait_obligations::h066c3f9f1f5e12a2lYZ
  23:     0x7f4134926a75 - middle::traits::select::SelectionContext<'cx, 'tcx>::vtable_impl::h0dd41e2ec59f0b116iZ
  24:     0x7f41349266f5 - middle::infer::InferCtxt<'a, 'tcx>::commit_if_ok::h16395859049726102179
  25:     0x7f41349150a1 - middle::traits::select::SelectionContext<'cx, 'tcx>::confirm_candidate::h64b3b64f12dac6e0LQY
  26:     0x7f413491abbe - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_candidate::_$u7b$$u7b$closure$u7d$$u7d$::closure.94162
  27:     0x7f413491a81e - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_stack::h525d267d7df64b2b5bX
  28:     0x7f4134919a8b - middle::traits::select::SelectionContext<'cx, 'tcx>::evaluate_predicate_recursively::hc18edfbb929b1024i3W
  29:     0x7f413482a1a8 - middle::traits::type_known_to_meet_builtin_bound::h559f2f377672bde3sf2
  30:     0x7f413495db01 - middle::ty::util::ty..TyS<'tcx>::impls_bound::hea825de3b49d4af69v8
  31:     0x7f413482a58f - middle::ty::util::ty..TyS<'tcx>::moves_by_default::h7508f4e507ba4fabqy8
  32:     0x7f4134781abc - middle::infer::InferCtxt<'a, 'tcx>::type_moves_by_default::h46ffa30aa912310fu3D
  33:     0x7f413495af84 - middle::ty::util::ParameterEnvironment<'a, 'tcx>::can_type_implement_copy::h368bfd5da63957a9147
  34:     0x7f413697a222 - coherence::CoherenceChecker<'a, 'tcx>::check_implementations_of_copy::_$u7b$$u7b$closure$u7d$$u7d$::closure.47300
  35:     0x7f413687a8ec - coherence::check_coherence::h4b13e71c9c2e25ffqeB
  36:     0x7f4136873798 - check_crate::h77c69415d924c1d56KC
  37:     0x7f4137aaf5ec - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.30392
  38:     0x7f4137aadbe4 - middle::ty::context::TyCtxt<'tcx>::create_and_enter::h11491607379472695545
  39:     0x7f4137aaa7c2 - driver::phase_3_run_analysis_passes::h6358585460292857359
  40:     0x7f4137a7cfbf - driver::compile_input::haf40356f083c88b6Pca
  41:     0x7f4137a6a88c - run_compiler::hb6d95af7384b3b47JPc
  42:     0x7f4137a67f11 - sys_common::unwind::try::try_fn::h18012394097158299251
  43:     0x7f413757cacb - __rust_try
  44:     0x7f413757ca5d - sys_common::unwind::inner_try::h87a6865a03ff0831iZt
  45:     0x7f4137a6875a - boxed::F.FnBox<A>::call_box::h17291146252477097894
  46:     0x7f4137587549 - sys::thread::Thread::new::thread_start::haca995caeb3b74699Xy
  47:     0x7f412fc62181 - start_thread
  48:     0x7f41371f347c - __clone
  49:                0x0 - <unknown>
