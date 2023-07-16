
$ RUST_BACKTRACE=1 rustc bug.rs
error: internal compiler error: src/librustc_trans/_match.rs:943: only string and byte strings supported in compare_values
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:584
stack backtrace:
   1:     0x7f19a933bcdf - <unknown>
   2:     0x7f19a934978b - <unknown>
   3:     0x7f19a934932c - <unknown>
   4:     0x7f19a930f85d - std::panicking::rust_panic_with_hook::h5dd7da6bb3d06020
   5:     0x7f19a4882037 - <unknown>
   6:     0x7f19a488199e - syntax::errors::Handler::bug::hb67a449bfdd4b525
   7:     0x7f19a58ff704 - <unknown>
   8:     0x7f19a58ff50d - <unknown>
   9:     0x7f19a5918ce6 - rustc::session::bug_fmt::h3d079546792f3a77
  10:     0x7f19a6ea855c - <unknown>
  11:     0x7f19a6ea25c0 - <unknown>
  12:     0x7f19a6eaab99 - <unknown>
  13:     0x7f19a6e72c0d - <unknown>
  14:     0x7f19a6dc596e - <unknown>
  15:     0x7f19a6dbd251 - <unknown>
  16:     0x7f19a6dbbaa8 - <unknown>
  17:     0x7f19a6dbda16 - <unknown>
  18:     0x7f19a6dc826e - <unknown>
  19:     0x7f19a6de436d - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::hf2a5fda0a4c4bb41
  20:     0x7f19a6dd1837 - rustc_trans::base::trans_crate::hdb9f0761c2bb8e02
  21:     0x7f19a989e493 - rustc_driver::driver::phase_4_translate_to_llvm::he77984ef06ab7894
  22:     0x7f19a989c308 - <unknown>
  23:     0x7f19a9898b1d - <unknown>
  24:     0x7f19a989223b - <unknown>
  25:     0x7f19a9859dcc - rustc_driver::driver::compile_input::hda370d330171d8d7
  26:     0x7f19a9844fc4 - rustc_driver::run_compiler::ha942b7e1d33fe553
  27:     0x7f19a984209e - <unknown>
  28:     0x7f19a9357f0b - <unknown>
  29:     0x7f19a9357eae - __rust_maybe_catch_panic
  30:     0x7f19a9842b84 - <unknown>
  31:     0x7f19a9347a04 - <unknown>
  32:     0x7f19a3cd9463 - start_thread
  33:     0x7f19a8f9e30c - clone
  34:                0x0 - <unknown>

