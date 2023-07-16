
thread '<unnamed>' panicked at 'called `Option::unwrap()` on a `None` value', libcore\option.rs:345:21
stack backtrace:
   0: <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt
   1: <std::sys::windows::dynamic_lib::DynamicLibrary as core::ops::drop::Drop>::drop
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: rustc::ty::walk::walk_shallow
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::panicking::panic
  10: rustc_save_analysis::SaveContext::get_expr_data
  11: <unknown>
  12: <rustc_save_analysis::json_dumper::Access as core::fmt::Debug>::fmt
  13: <rustc_save_analysis::json_dumper::Access as core::fmt::Debug>::fmt
  14: <rustc_save_analysis::json_dumper::Access as core::fmt::Debug>::fmt
  15: <unknown>
  16: <unknown>
  17: <unknown>
  18: <rustc_save_analysis::CallbackHandler<'b> as rustc_save_analysis::SaveHandler>::save
  19: <unknown>
  20: <unknown>
  21: rustc_driver::profile::dump
  22: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  23: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_trait_item
  24: <unknown>
  25: rustc_driver::driver::compile_input
  26: rustc_driver::run_compiler
  27: rustc_driver::target_features::add_configuration
  28: <unknown>
  29: rustc_driver::run_compiler
  30: <unknown>
  31: <unknown>
  32: _rust_maybe_catch_panic
  33: <unknown>
  34: <unknown>
  35: _rust_maybe_catch_panic
  36: <unknown>
  37: <unknown>
  38: <unknown>
  39: <unknown>
  40: _rust_maybe_catch_panic
  41: <unknown>
  42: <std::sys::windows::handle::Handle as core::ops::drop::Drop>::drop
  43: std::sys::windows::thread::Thread::new
  44: BaseThreadInitThunk
  45: RtlUserThreadStart
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (4a9c58c6b 2018-06-05) running on x86_64-pc-windows-msvc
