
Compiling dsp-chain v0.0.0 (file:///Users/Mitch/Programming/Rust/dsp-chain)
src/node.rs:69:35: 69:47 error: mismatched types:
 expected `&mut <node::Node as node::Node>::Buffer`,
    found `&mut <Self as node::Node>::Buffer`
(expected trait node::Node,
    found Self) [E0308]
src/node.rs:69             input.audio_requested(&mut working, settings);
                                                 ^~~~~~~~~~~~
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librbml/lib.rs:266

stack backtrace:
   1:        0x109d07cd7 - sys::backtrace::write::h4ee5c5ad264b65f5mbu
   2:        0x109d2de9c - failure::on_fail::h55e5ff39107f11fcHbB
   3:        0x109c8f618 - rt::unwind::begin_unwind_inner::habba3c5d37f59591DTA
   4:        0x1092c1454 - rt::unwind::begin_unwind::h15939399727117421951
   5:        0x1092bffe4 - reader::get_doc::h0276305dd2ee2970nLa
   6:        0x1071088e8 - metadata::decoder::item_type::heedfd52684953eeaTqh
   7:        0x107119d51 - metadata::decoder::get_type::h2931af7f8c7c3e42uDh
   8:        0x106f6871d - middle::ty::lookup_item_type::ha73f50b132016f83P46
   9:        0x10691fcdb - check::FnCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h12679af05f759596hTm
  10:        0x1069acac7 - astconv::ast_path_to_ty::h57df746a8fa80a7b3It
  11:        0x1069b0cd9 - astconv::ast_ty_to_ty::closure.32669
  12:        0x106948550 - astconv::ast_ty_to_ty::h8d7cfb197a62e3f6N9t
  13:        0x1069424fd - check::check_cast::h78db49c38d52a23edOm
  14:        0x1069752f8 - check::check_expr_with_unifier::h16292217873934053862
  15:        0x10698c62c - check::check_expr_with_unifier::h11019213537881022167
  16:        0x10693cd26 - check::check_block_with_expected::h951febd74f526f5c7Mq
  17:        0x10696459e - check::check_block_no_value::h03e1e2535bed2e50bMq
  18:        0x10697399e - check::check_expr_with_unifier::h16292217873934053862
  19:        0x10693cfdf - check::check_block_with_expected::h951febd74f526f5c7Mq
  20:        0x10696459e - check::check_block_no_value::h03e1e2535bed2e50bMq
  21:        0x10697399e - check::check_expr_with_unifier::h16292217873934053862
  22:        0x10693cfdf - check::check_block_with_expected::h951febd74f526f5c7Mq
  23:        0x10696459e - check::check_block_no_value::h03e1e2535bed2e50bMq
  24:        0x10696d6fb - check::check_expr_with_unifier::h16914743892577524087
  25:        0x10693cc54 - check::check_block_with_expected::h951febd74f526f5c7Mq
  26:        0x106921508 - check::check_fn::h676b72eb902b2f23e1l
  27:        0x10693998f - check::check_bare_fn::h3a34bba2748b9e14GQl
  28:        0x10693d83c - check::check_method_body::h2d16a6bb30bdd699Nmm
  29:        0x10693469a - check::check_item::h2a002d682f8542d2j9l
  30:        0x1069375b2 - visit::walk_item::h521929741097464564
  31:        0x106a00ede - check_crate::closure.33893
  32:        0x1069fc42f - check_crate::hafc86ed9389e97e5IEz
  33:        0x1063b33b5 - driver::phase_3_run_analysis_passes::h5b5e8dbf669c7bf9NFa
  34:        0x106399a7c - driver::compile_input::h8e0204b8bb9ea44bBba
  35:        0x106462d3e - run_compiler::h322f9334a9a1d7e6l9b
  36:        0x10645fd7f - thunk::F.Invoke<A, R>::invoke::h14356237310415989951
  37:        0x10645ea10 - rt::unwind::try::try_fn::h798029436104877783
  38:        0x109da4329 - rust_try_inner
  39:        0x109da4316 - rust_try
  40:        0x10645f0d4 - thunk::F.Invoke<A, R>::invoke::h15628186897385384602
  41:        0x109d18553 - sys::thread::thread_start::h3f48e8c893476626A3w
  42:     0x7fff99505268 - _pthread_body
  43:     0x7fff995051e5 - _pthread_body

Could not compile `dsp-chain`.
