
we@we-pc MINGW64 ~
$ rustc --version --verbose
rustc 1.3.0 (9a92aaf19 2015-09-15)
binary: rustc
commit-hash: 9a92aaf19a64603b02b4130fe52958cc12488900
commit-date: 2015-09-15
host: x86_64-pc-windows-gnu
release: 1.3.0

we@we-pc MINGW64 ~
$ RUST_BACKTRACE=1 rustc test.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'No def'n found for DefId { krate: 0, node: 11 }Z in tcx.item_variance_map', ../src/librustc\middle\ty.rs:5152

stack backtrace:
   1:         0x6c6bbc58 - sys::backtrace::write::h9192fb96d06a04279Hs
   2:         0x6c6c582d - rt::unwind::register::hf2debafc128a8bbcxWw
   3:         0x6c685a6f - rt::unwind::begin_unwind_inner::hcb96b94c1db5c440GTw
   4:         0x6c6863f6 - rt::unwind::begin_unwind_fmt::h28f0332c3e8f2752MSw
   5:         0x70f40b75 - middle::ty::ctxt<'tcx>::item_variances::ha5d9a4cd6cc6326f2tb
   6:           0xca4f1c - check::FnCtxt<'a, 'tcx>::register_builtin_bound::h943db64912a2171cN2o
   7:           0xca27b2 - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>::new::hd82ea0334ef8c589c5j
   8:           0xca976e - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>.Visitor<'v>::visit_item::hc1d90458249ea5fdiCk
   9:           0xca9588 - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>.Visitor<'v>::visit_item::hc1d90458249ea5fdiCk
  10:           0xca8abb - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>.Visitor<'v>::visit_item::hc1d90458249ea5fdiCk
  11:           0xcc6fd9 - check::check_item_types::h3739f0f18758ac32ewn
  12:           0xd83344 - check_crate::h8ef65f4638074ba9laD
  13:         0x64b2651b - driver::assign_node_ids_and_map::hae564fdeb2ac85907Da
  14:         0x64b24c34 - driver::assign_node_ids_and_map::hae564fdeb2ac85907Da
  15:         0x64b1f3b7 - driver::assign_node_ids_and_map::hae564fdeb2ac85907Da
  16:         0x64b02766 - driver::compile_input::h23ee8d2f30f41c3bTba
  17:         0x64beb53c - run_compiler::h13fbc9ea0e4d6074A7b
  18:         0x64be92b7 - run::h6cfcc1625a0298d9g7b
  19:         0x64be8c59 - run::h6cfcc1625a0298d9g7b
  20:         0x6c6af014 - rt::unwind::try::inner_try::hac50b220aa363696zPw
  21:         0x64be8e27 - run::h6cfcc1625a0298d9g7b
  22:         0x6c6c344e - sys::process::Command::cwd::h809ce520f020ef4eyzv
  23:     0x7ffa1fa313d2 - BaseThreadInitThunk
