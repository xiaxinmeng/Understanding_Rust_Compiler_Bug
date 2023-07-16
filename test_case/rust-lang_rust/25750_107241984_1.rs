
src/lib.rs:6:5: 6:9 error: internal compiler error: cannot relate bound region: ReScope(DestructionScope(18)) <= ReLateBound(DebruijnIndex { depth: 1 }, BrAnon(0))
src/lib.rs:6     None
                 ^~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:176

stack backtrace:
   1:        0x1089077b5 - sys::backtrace::write::h1561c90743d5c179M9r
   2:        0x1089103f0 - panicking::on_panic::h8a5d860d5a124e0a2uw
   3:        0x1088ca948 - rt::unwind::begin_unwind_inner::hb33d13fc310a98afkdw
   4:        0x10807987e - rt::unwind::begin_unwind::h9699143272701214950
   5:        0x10807980a - diagnostic::SpanHandler::span_bug::h4a339f86ecbb72a1uqC
   6:        0x105ae38b4 - middle::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_subregion::h30bc770a31311672yRv
   7:        0x105afdf1b - middle::infer::mk_subr::h9a5b48e4c252ec91k7y
   8:        0x105606b2a - check::regionck::type_must_outlive::h8d529e58bca1ccb0Tte
   9:        0x105604ac5 - check::dropck::iterate_over_potentially_unsafe_regions_in_type::ha9d169bb6b0bbca3ZDa
  10:        0x105604e1e - check::dropck::iterate_over_potentially_unsafe_regions_in_type::ha9d169bb6b0bbca3ZDa
  11:        0x105603701 - check::dropck::check_safety_of_destructor_if_necessary::h3060ac778c5992289xa
  12:        0x10563d93e - check::regionck::check_safety_of_rvalue_destructor_if_necessary::hcf8a4293ce5c60bfjPd
  13:        0x105636eb3 - check::regionck::visit_expr::h79aea6e9fda81e97fad
  14:        0x105634cde - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h09869d1c412f3624HMc
  15:        0x1056340a3 - check::regionck::regionck_fn::hfdd6d8acf07f17563Ec
  16:        0x1056b133d - check::check_bare_fn::hc9c7a17ca1e4729egOn
  17:        0x1056aef37 - check::check_item_body::h103df4808fcb6e559eo
  18:        0x1056b0c73 - check::check_item_types::h7c6eaf781d12ace8NLn
  19:        0x105770f10 - check_crate::h8fe4e687b73e5015g7C
  20:        0x104f9b68f - driver::phase_3_run_analysis_passes::hef736cb124de6565GGa
  21:        0x104f7f403 - driver::compile_input::h7a3800e0d9c4bbe3Qba
  22:        0x105039e04 - run_compiler::h3864143d185b8964n6b
  23:        0x10503756a - boxed::F.FnBox<A>::call_box::h4644100161726726905
  24:        0x105036c87 - rt::unwind::try::try_fn::h5144344175023612934
  25:        0x108998bf8 - rust_try_inner
  26:        0x108998be5 - rust_try
  27:        0x1088fa315 - rt::unwind::try::inner_try::hbe91c83f4d153e36d9v
  28:        0x105036ee2 - boxed::F.FnBox<A>::call_box::h10871128639736673174
  29:        0x10890eeed - sys::thread::Thread::new::thread_start::h89e4611b0ac51fcbMxv
  30:     0x7fff8852a267 - _pthread_body
  31:     0x7fff8852a1e4 - _pthread_start
