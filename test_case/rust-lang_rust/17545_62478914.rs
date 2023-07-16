
$ RUST_BACKTRACE=1 cargo test
   Compiling multipart v0.0.1 (file:///home/austin/Rust/multipart)
/home/austin/Rust/multipart/src/server.rs:90:46: 90:51 error: internal compiler error: cannot relate bound region: '_#3r <= ReLateBound(1268, BrNamed(DefId { krate: 0, node: 1267 }, 'a))
/home/austin/Rust/multipart/src/server.rs:90                 Ok((name, field)) => f(name, field),
                                                                                          ^~~~~
/home/austin/Rust/multipart/src/server.rs:90:46: 90:51 error: internal compiler error: cannot relate bound region: '_#3r <= ReLateBound(1269, BrNamed(DefId { krate: 0, node: 1268 }, 'a))
/home/austin/Rust/multipart/src/server.rs:90                 Ok((name, field)) => f(name, field),
                                                                                          ^~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:116

stack backtrace:
   1:     0x7fcbb5ec3080 - rt::backtrace::imp::write::h939301e75513bc72Pdt
   2:     0x7fcbb5ec6100 - failure::on_fail::hdd4bf65d63a3996bqzt
   3:     0x7fcbba453d90 - unwind::begin_unwind_inner::h96243874d9cdbfadE9c
   4:     0x7fcbb6a30e90 - unwind::begin_unwind::h2280031143806423275
   5:     0x7fcbb6a30e10 - diagnostic::SpanHandler::span_bug::h909bcfe987334462oZF
   6:     0x7fcbbb041bf0 - middle::typeck::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_subregion::heecdefbb83ebd877Mvc
   7:     0x7fcbbb002980 - middle::typeck::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_eqregion::hec1abaf8657c5de6bvc
   8:     0x7fcbbb002600 - middle::typeck::infer::equate::Equate<'f, 'tcx>.Combine<'tcx>::regions::hfcbcc1514e9dd075Sc8
   9:     0x7fcbbb052eb0 - middle::typeck::infer::combine::Combine::substs_variances::h12618975266850111103
  10:     0x7fcbbaff57a0 - middle::typeck::infer::sub::Sub<'f, 'tcx>.Combine<'tcx>::tys::h49732cfdf439ed2eZ9e
  11:     0x7fcbbaff5630 - middle::typeck::infer::coercion::Coerce<'f, 'tcx>::subtype::h76eec3cf0fbe6132b95
  12:     0x7fcbbaff5560 - middle::typeck::infer::coercion::Coerce<'f, 'tcx>::tys::closure.134456
  13:     0x7fcbbaff2ed0 - middle::typeck::infer::coercion::Coerce<'f, 'tcx>::unpack_actual_value::h17225693816979566150
  14:     0x7fcbbaff2630 - middle::typeck::infer::coercion::Coerce<'f, 'tcx>::tys::h9c832c5f684ff3ecC15
  15:     0x7fcbbb05e820 - middle::typeck::infer::mk_coercety::closure.136288
  16:     0x7fcbbb05e580 - middle::typeck::infer::InferCtxt<'a, 'tcx>::commit_if_ok::closure.136280
  17:     0x7fcbbb05e410 - middle::typeck::infer::InferCtxt<'a, 'tcx>::commit_unconditionally::h5666366677428435825
  18:     0x7fcbbad85300 - middle::typeck::infer::mk_coercety::h6a60305071835d6bJei
  19:     0x7fcbbaf31140 - middle::typeck::check::FnCtxt<'a, 'tcx>::mk_assignty::h8a150c42e3060d04qlY
  20:     0x7fcbbaf30db0 - middle::typeck::check::demand::coerce::h21461a79c8ef4384FbS
  21:     0x7fcbbaf9f2f0 - middle::typeck::check::check_expr_coercable_to_type::closure.133462
  22:     0x7fcbbaf95830 - middle::typeck::check::check_expr_with_unifier::ha8a701ce54eb86308GZ
  23:     0x7fcbbaf93e20 - middle::typeck::check::check_argument_types::h9f4c2f0dccbd555627Y
  24:     0x7fcbbaf91130 - middle::typeck::check::check_method_argument_types::hd796294ab62231c8d6Y
  25:     0x7fcbbaf90480 - middle::typeck::check::try_overloaded_call::he07eaaf93c96c66etFY
  26:     0x7fcbbaf95830 - middle::typeck::check::check_expr_with_unifier::ha8a701ce54eb86308GZ
  27:     0x7fcbbaef0ec0 - middle::typeck::check::_match::check_match::h9811d9d3da484cf5BpN
  28:     0x7fcbbaf95830 - middle::typeck::check::check_expr_with_unifier::ha8a701ce54eb86308GZ
  29:     0x7fcbbaf5a280 - middle::typeck::check::check_block_with_expected::h24847bbc3646b17eeS1
  30:     0x7fcbbafa0b30 - middle::typeck::check::check_block_no_value::h3fa80357d9a3da45jR1
  31:     0x7fcbbaf95830 - middle::typeck::check::check_expr_with_unifier::ha8a701ce54eb86308GZ
  32:     0x7fcbbaf5a280 - middle::typeck::check::check_block_with_expected::h24847bbc3646b17eeS1
  33:     0x7fcbbaf55b80 - middle::typeck::check::check_fn::h5d26e9dbe23051f4yjW
  34:     0x7fcbbaf558c0 - middle::typeck::check::check_bare_fn::h2ebb8057bcbc86d4N8V
  35:     0x7fcbbaf5cdb0 - middle::typeck::check::check_method_body::h0cd80d47cea5b434GDW
  36:     0x7fcbbaf519c0 - middle::typeck::check::check_item::h2d34e2049f88b98aWsW
  37:     0x7fcbbaf55320 - visit::walk_mod::h14778329676043754467
  38:     0x7fcbbaf55670 - middle::typeck::check::check_item_types::h09fc778fd4368a2dX7V
  39:     0x7fcbbaa45980 - util::common::time::h5887625128919507912
  40:     0x7fcbbb2793b0 - middle::typeck::check_crate::hc836e7120e6c8418Uup
  41:     0x7fcbbb2e2340 - driver::driver::phase_3_run_analysis_passes::h2ba914a4e9f02ba2leC
  42:     0x7fcbbb2dd1d0 - driver::driver::compile_input::h4cd135e1510b41346UB
  43:     0x7fcbbb360470 - driver::run_compiler::hefe828ab7d56e443nLF
  44:     0x7fcbbb360360 - driver::run::closure.146371
  45:     0x7fcbbaa5e8b0 - task::TaskBuilder<S>::try_future::closure.104805
  46:     0x7fcbbaa5e6a0 - task::TaskBuilder<S>::spawn_internal::closure.104776
  47:     0x7fcbba751b40 - task::NativeSpawner.Spawner::spawn::closure.2581
  48:     0x7fcbba4a8ca0 - rust_try_inner
  49:     0x7fcbba4a8c90 - rust_try
  50:     0x7fcbba451710 - unwind::try::h523801dc9878f329mYc
  51:     0x7fcbba4515a0 - task::Task::run::hfb2d586240eaedeeu4b
  52:     0x7fcbba751880 - task::NativeSpawner.Spawner::spawn::closure.2507
  53:     0x7fcbba452db0 - thread::thread_start::hb97ea5d68265d21frpc
  54:     0x7fcbb52580c0 - start_thread
  55:     0x7fcbba11df89 - __clone
  56:                0x0 - <unknown>
