
$ RUST_BACKTRACE=1 make
cfg: version 1.2.0-dev (b301e02f3 2015-05-19) (built 2015-05-19)
cfg: build triple x86_64-pc-windows-gnu
cfg: host triples x86_64-pc-windows-gnu
cfg: target triples x86_64-pc-windows-gnu
cfg: enabling debug assertions (CFG_ENABLE_DEBUG_ASSERTIONS)
cfg: enabling debuginfo (CFG_ENABLE_DEBUGINFO)
cfg: host for x86_64-pc-windows-gnu is x86_64
cfg: os for x86_64-pc-windows-gnu is pc-windows-gnu
cfg: good valgrind for x86_64-pc-windows-gnu is
cfg: using CC=gcc (CFG_CC)
cfg: disabling valgrind run-pass tests
cfg: no xelatex found, disabling LaTeX docs
cfg: no pandoc found, omitting PDF and EPUB docs
rustc: x86_64-pc-windows-gnu/stage1/bin/rustlib/x86_64-pc-windows-gnu/lib/libcore
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'attempted to negate with overflow', C:/msys64/home/we/rust/src/libcore\num/mod.rs:581

stack backtrace:
   1:         0x6d2b3fa1 - sys::backtrace::write::h410e8b1c02e6a4cb8fs
   2:         0x6d2bca98 - rt::unwind::register::h1ca1862cef28bdc0xNv
   3:         0x6d284e14 - rt::unwind::begin_unwind_inner::h6525e25cf712ad9fHKv
   4:         0x6d2857f9 - rt::unwind::begin_unwind_fmt::ha2ec3cbf348c0730NJv
   5:         0x6d2bc6a0 - rust_begin_unwind
   6:         0x6d2da032 - panicking::panic_fmt::h3bec9bbe2b23bab6pZB
   7:         0x6d2d2e2f - panicking::panic::hcc1bf6d590d7a6bfWXB
   8:         0x71201f22 - builtin::TypeLimits.LintPass::check_expr::h3cbdea1f95212ea22ea
   9:           0xac16b0 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_expr::ha294eff0288554d123s
  10:           0xac1ae6 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_expr::ha294eff0288554d123s
  11:           0xac190b - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_expr::ha294eff0288554d123s
  12:           0xac1ae6 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_expr::ha294eff0288554d123s
  13:           0xac18f6 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_expr::ha294eff0288554d123s
  14:           0xabd984 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_item::h97cd569f2a202a39k0s
  15:           0xac21c9 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_mod::ha15baa921fa04facaft
  16:           0xabda37 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_item::h97cd569f2a202a39k0s
  17:           0xac21c9 - lint::context::Context<'a, 'tcx>.Visitor<'v>::visit_mod::ha15baa921fa04facaft
  18:           0xacf212 - lint::context::check_crate::h008b1bd3f8cb5d749yt
  19:         0x65399d3d - driver::phase_3_run_analysis_passes::h4302356ee8fb62b5tGa
  20:         0x6538322f - driver::compile_input::h2cf0fc5773828de8Qba
  21:         0x6541babd - run_compiler::hce52d824a1c83d3675b
  22:         0x65419a70 - run::h315a1ba5be8f35fbN5b
  23:         0x6541921e - run::h315a1ba5be8f35fbN5b
  24:         0x6d2fc07c - rust_try
  25:         0x6d2fc059 - rust_try
  26:         0x654194dc - run::h315a1ba5be8f35fbN5b
  27:         0x6d2bac4d - sys::process::Command::cwd::ha96a7c671c94f9abquu
  28:     0x7ffbd1b113d2 - BaseThreadInitThunk
