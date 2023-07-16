
rustc: x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd
error: internal compiler error: no type for node 85759: expr buf.as_mut_ptr() (id=85759) in fcx 0x361d0b857e0
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/steb/Code/Projects/rust/src/libsyntax/diagnostic.rs:230

stack backtrace:
   1:      0x361d80d4819 - sys::backtrace::write::hdd4d62d456867831u6r
   2:      0x361d80dba7d - panicking::on_panic::hbb2188b8b35dcf6cdww
   3:      0x361d80a557c - rt::unwind::begin_unwind_inner::h0259abc6ecf88252lbw
   4:      0x361d573e749 - rt::unwind::begin_unwind::h17934595601296836584
   5:      0x361d573ed1b - diagnostic::Handler::bug::h57a278f965e936bb0JB
   6:      0x361d622568b - session::Session::bug::h3a23a9d8685d047fhYp
   7:      0x361d787b4ec - check::FnCtxt<'a, 'tcx>::node_ty::h61c97da1870f112fwrp
   8:      0x361d789b2db - check::regionck::type_of_node_must_outlive::hc215bcd8f34283beppe
   9:      0x361d7899fa3 - check::regionck::visit_expr::h6693b71679b2ea0aiGd
  10:      0x361d789a3bb - check::regionck::visit_expr::h6693b71679b2ea0aiGd
  11:      0x361d7897d7d - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h2708ba44632c99bcFid
  12:      0x361d78fdb4c - check::check_bare_fn::h4c12ac148dc4a251AUn
  13:      0x361d78fbbb4 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hc46ecad48c847aeeBRn
  14:      0x361d78fbd9f - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hc46ecad48c847aeeBRn
  15:      0x361d78fbd9f - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hc46ecad48c847aeeBRn
  16:      0x361d78fbd9f - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hc46ecad48c847aeeBRn
  17:      0x361d79a325e - check_crate::closure.36316
  18:      0x361d79a0700 - check_crate::h52c99a549b4c8975pDC
  19:      0x361d8610d22 - driver::phase_3_run_analysis_passes::h8e3fb093e4361425tGa
  20:      0x361d85f9084 - driver::compile_input::h00a02dc5812e10deQba
  21:      0x361d86992d0 - run_compiler::h8e40b347429203fbX4b
  22:      0x361d8696f2f - boxed::F.FnBox<A>::call_box::h11085863581512576413
  23:      0x361d8696739 - rt::unwind::try::try_fn::h17400314009759166474
  24:      0x361d81571a8 - rust_try_inner
  25:      0x361d8157195 - rust_try
  26:      0x361d86969b8 - boxed::F.FnBox<A>::call_box::h5995785377111646518
  27:      0x361d80da9cc - sys::thread::create::thread_start::h67b8f63f899fd3ecIvv
  28:      0x361d2e19373 - start_thread
  29:      0x361d7d3927c - clone
  30:                0x0 - <unknown>

/home/steb/Code/Projects/rust/mk/target.mk:167: recipe for target 'x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.std' failed
make: *** [x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.std] Error 101
