
  Compiling xxxx v0.1.0 (file:///home/lholden/Projects/xxxx)
src/systems/input.rs:28:17: 28:43 error: no associated item named `UserInput` found for type `protocol::Protocol` in the current scope
src/systems/input.rs:28                 Protocol::UserInput(input) => input,
                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~
note: in expansion of while let expansion
src/systems/input.rs:22:9: 42:10 note: expansion site
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'path not fully resolved: PathResolution { base_def: DefTy(DefId { krate: 0, node: 15 => protocol::Protocol }, true), last_private: LastMod(AllPublic), depth: 1 }', ../src/librustc/middle/def.rs:82

stack backtrace:
   1:     0x7f4d6de36ef9 - sys::backtrace::tracing::imp::write::h2dd719d6a90d1694REs
   2:     0x7f4d6de3dba6 - panicking::on_panic::hcdc970a599b2a4877ox
   3:     0x7f4d6de0167e - rt::unwind::begin_unwind_inner::h6793c7a97a7e3d7cDRw
   4:     0x7f4d6de023c7 - rt::unwind::begin_unwind_fmt::hf41ce6a5eab0f5e3JQw
   5:     0x7f4d6bb87af6 - middle::mem_categorization::MemCategorizationContext<'t, 'a, 'tcx>::cat_pattern_::h15751778615198713220
   6:     0x7f4d6bb86f72 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::determine_pat_move_mode::hbea6ec65ddad4bacEHr
   7:     0x7f4d6bb5b5f0 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_expr::he39be1fe2073ca9e6Rq
