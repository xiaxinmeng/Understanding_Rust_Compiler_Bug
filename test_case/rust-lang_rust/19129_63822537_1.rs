
foo.rs:1:1: 1:1 error: internal compiler error: Type parameter `B/AssocSpace.1` (B/AssocSpace/1) out of range when substituting (root type=B) substs=Substs[types=[[A];[Self];[B];[]], regions=[[];[];[];[]]]
foo.rs:1 #![feature(associated_types)]
         ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /home/tom/src/rust/rust/src/libsyntax/diagnostic.rs:121

stack backtrace:
   1:     0x7f42b15e5750 - rt::backtrace::imp::write::hd4dc9c5b7adc70deB9s
   2:     0x7f42b15e88f0 - failure::on_fail::h601ae62da37820fduwt
   3:     0x7f42b1d8b5b0 - unwind::begin_unwind_inner::hd5e2a13fc766a9ednbd
   4:     0x7f42af488b30 - unwind::begin_unwind::h1840779095798302194
   5:     0x7f42af488ab0 - diagnostic::SpanHandler::span_bug::haca8008c8efcaccdl7F
   6:     0x7f42b05766a0 - middle::subst::SubstFolder<'a, 'tcx>.TypeFolder<'tcx>::fold_ty::h6dfb50fd9b2a318dKTP
   7:     0x7f42b0617f80 - middle::ty_fold::VecPerParamSpace<T>.TypeFoldable<'tcx>::fold_with::closure.96163
   8:     0x7f42b0617b30 - middle::subst::VecPerParamSpace<T>::map_enumerated::h1858619917310266163
   9:     0x7f42b0617270 - middle::ty_fold::TypeFolder::fold_substs::h10218276810570421665
  10:     0x7f42b0650d50 - middle::ty_fold::Vec<T>.TypeFoldable<'tcx>::fold_with::closure.98606
  11:     0x7f42b0650bb0 - vec::Vec<T>.FromIterator<T>::from_iter::h15054265504360828229
  12:     0x7f42b06506f0 - middle::ty_fold::ty..ParamBounds<'tcx>.TypeFoldable<'tcx>::fold_with::h5912314470836935956
  13:     0x7f42b063db90 - middle::ty::Generics<'tcx>::to_bounds::hbca748b7f43f8696vo1
  14:     0x7f42b0652470 - middle::ty::construct_parameter_environment::he8cb628e2da26785Gy6
  15:     0x7f42b0651dd0 - middle::ty::ParameterEnvironment<'tcx>::for_item::haae90d8e2fdf61c2ju1
  16:     0x7f42b071ef30 - middle::typeck::check::check_method_body::he4cebab5fe2fb8f8nwi
  17:     0x7f42b07134a0 - middle::typeck::check::check_item::h2db4e12f763145a7vli
  18:     0x7f42b0717350 - middle::typeck::check::check_item_types::h6c8b3f561b623f38x0h
  19:     0x7f42b0b03fa0 - util::common::time::h16216271059762254987
  20:     0x7f42b0b031b0 - middle::typeck::check_crate::hd1db30e85ca4d877jVL
  21:     0x7f42b2298aa0 - driver::driver::phase_3_run_analysis_passes::h27f0c337e17ae602bfS
  22:     0x7f42b228d8c0 - driver::driver::compile_input::h144d59d4a02adda6VVR
  23:     0x7f42b2312840 - driver::run_compiler::h0f9dd69b80cf00c4pUT
  24:     0x7f42b2312730 - driver::run::closure.59734
  25:     0x7f42b211ef60 - task::TaskBuilder<S>::try_future::closure.39040
  26:     0x7f42b211ed50 - task::TaskBuilder<S>::spawn_internal::closure.39011
  27:     0x7f42b2632a40 - task::NativeSpawner.Spawner::spawn::closure.2471
  28:     0x7f42b1dedc60 - rust_try_inner
  29:     0x7f42b1dedc50 - rust_try
  30:     0x7f42b1d88f10 - unwind::try::hbf9691835cc163a4IZc
  31:     0x7f42b1d88da0 - task::Task::run::h309f69a6701379ady5b
  32:     0x7f42b2632780 - task::NativeSpawner.Spawner::spawn::closure.2397
  33:     0x7f42b1d8a5c0 - thread::thread_start::h721d15c10ff33d0aEqc
  34:     0x7f42ac7f9250 - start_thread
  35:     0x7f42b1a613b9 - clone
  36:                0x0 - <unknown>

