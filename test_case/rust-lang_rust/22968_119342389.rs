
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: index.end <= self.len()', ../src/libcore\slice.rs:544

stack backtrace:
   1:         0x61dfa535 - sys::backtrace::write::h018e42c12f269f544Cs
   2:         0x61e038ac - rt::unwind::register::hc674956c97a716df1Cw
   3:         0x61dc57ff - rt::unwind::begin_unwind_inner::h662aa5119908cf48aAw
   4:         0x61dc617a - rt::unwind::begin_unwind_fmt::h01eb13496835025cgzw
   5:         0x61e032c3 - rust_begin_unwind
   6:         0x61e200a9 - panicking::panic_fmt::hbcf25944567f207319B
   7:         0x61e1a6f1 - panicking::panic::h93271eadbe8415b7y8B
   8:           0x795d3a - check::TupleArgumentsFlag...std..clone..Clone::clone::h605dce780ccf5dc9Ljq
   9:           0x755335 - check::FnCtxt<'a, 'tcx>::require_type_meets::h798960652a04997dgZo
  10:           0x7cc037 - collect::ast..Generics.GetTypeParameterBounds<'tcx>::get_type_parameter_bounds::ha042c13c7c6692a3xmx
  11:           0x7c3034 - collect::ast..Generics.GetTypeParameterBounds<'tcx>::get_type_parameter_bounds::ha042c13c7c6692a3xmx
  12:           0x7b4d08 - collect::AstConvRequest...std..clone..Clone::clone::h63a61bfae076fcfegWw
  13:           0x7bac23 - collect::AstConvRequest...std..clone..Clone::clone::h63a61bfae076fcfegWw
  14:           0x7b0332 - collect::collect_item_types::h8bacaf0c14501822JPw
  15:           0x7fca25 - check_crate::hac17c115800a24556WC
  16:         0x7106bcfb - driver::assign_node_ids_and_map::h80779d68a0b95aa24Da
  17:         0x7106a43d - driver::assign_node_ids_and_map::h80779d68a0b95aa24Da
  18:         0x71064ca7 - driver::assign_node_ids_and_map::h80779d68a0b95aa24Da
  19:         0x71042a66 - driver::compile_input::hdea87ef255adf358Tba
  20:         0x7112dc8d - run_compiler::h898d25887739e965y7b
  21:         0x7112b915 - run::he91bab0c68724632e7b
  22:         0x7112b259 - run::he91bab0c68724632e7b
  23:         0x61e3fedc - rust_try
  24:         0x61e3feb9 - rust_try
  25:         0x61ded605 - rt::unwind::try::inner_try::h75d613e781873d5a3vw
  26:         0x7112b417 - run::he91bab0c68724632e7b
  27:         0x61e0157e - sys::process::Command::cwd::hbe09b93cabba047cukv
  28:     0x7ffd5f6a13d2 - BaseThreadInitThunk
