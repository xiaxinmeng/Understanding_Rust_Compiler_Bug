
thread 'rustc' panicked at 'no entry found for key', src/librustc_mir_build/build/mod.rs:354:9
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at ./src/libstd/../backtrace/src/backtrace/libunwind.rs:96
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at ./src/libstd/../backtrace/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at ./src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at ./src/libstd/sys_common/backtrace.rs:58
   4: core::fmt::write
             at ./src/libcore/fmt/mod.rs:1117
   5: std::io::Write::write_fmt
             at ./src/libstd/io/mod.rs:1508
   6: std::sys_common::backtrace::_print
             at ./src/libstd/sys_common/backtrace.rs:61
   7: std::sys_common::backtrace::print
             at ./src/libstd/sys_common/backtrace.rs:48
   8: std::panicking::default_hook::{{closure}}
             at ./src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at ./src/libstd/panicking.rs:217
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at ./src/liballoc/boxed.rs:1095
  11: rustc_driver::report_ice
             at ./src/librustc_driver/lib.rs:1158
  12: std::panicking::rust_panic_with_hook
             at ./src/libstd/panicking.rs:530
  13: rust_begin_unwind
             at ./src/libstd/panicking.rs:437
  14: core::panicking::panic_fmt
             at ./src/libcore/panicking.rs:85
  15: core::option::expect_failed
             at ./src/libcore/option.rs:1265
  16: core::option::Option<T>::expect
             at ./src/libcore/option.rs:347
  17: <std::collections::hash::map::HashMap<K,V,S> as core::ops::index::Index<&Q>>::index
             at ./src/libstd/collections/hash/map.rs:1004
  18: rustc_mir_build::build::Builder::var_local_id
             at ./src/librustc_mir_build/build/mod.rs:354
  19: rustc_mir_build::build::expr::as_place::<impl rustc_mir_build::build::Builder>::expr_as_place
             at ./src/librustc_mir_build/build/expr/as_place.rs:169
  20: rustc_mir_build::build::expr::as_place::<impl rustc_mir_build::build::Builder>::as_place_builder
             at ./src/librustc_mir_build/build/expr/as_place.rs:84
  21: rustc_mir_build::build::expr::as_place::<impl rustc_mir_build::build::Builder>::as_place
             at ./src/librustc_mir_build/build/expr/as_place.rs:73
  22: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:408
  23: <rustc_mir_build::hair::Expr as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:53
  24: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  25: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::expr_as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:112
  26: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp::{{closure}}
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  27: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  28: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  29: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  30: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:143
  31: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:196
  32: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:116
  33: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand::{{closure}}
             at ./src/librustc_mir_build/build/expr/as_operand.rs:162
  34: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
  35: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:161
  36: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:116
  37: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_local_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:82
  38: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:200
  39: core::iter::adapters::map_fold::{{closure}}
             at ./src/libcore/iter/adapters/mod.rs:833
  40: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2003
  41: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at ./src/libcore/iter/adapters/mod.rs:873
  42: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:649
  43: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./src/liballoc/vec.rs:2140
  44: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
             at ./src/liballoc/vec.rs:2120
  45: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./src/liballoc/vec.rs:1995
  46: core::iter::traits::iterator::Iterator::collect
             at ./src/libcore/iter/traits/iterator.rs:1652
  47: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:198
  48: <rustc_mir_build::hair::Expr as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:53
  49: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  50: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::expr_as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:112
  51: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp::{{closure}}
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  52: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  53: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  54: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  55: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:143
  56: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:196
  57: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:116
  58: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand::{{closure}}
             at ./src/librustc_mir_build/build/expr/as_operand.rs:162
  59: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
  60: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::expr_as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:161
  61: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:116
  62: rustc_mir_build::build::expr::as_operand::<impl rustc_mir_build::build::Builder>::as_local_call_operand
             at ./src/librustc_mir_build/build/expr/as_operand.rs:82
  63: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:200
  64: core::iter::adapters::map_fold::{{closure}}
             at ./src/libcore/iter/adapters/mod.rs:833
  65: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2003
  66: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at ./src/libcore/iter/adapters/mod.rs:873
  67: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:649
  68: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./src/liballoc/vec.rs:2140
  69: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
             at ./src/liballoc/vec.rs:2120
  70: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./src/liballoc/vec.rs:1995
  71: core::iter::traits::iterator::Iterator::collect
             at ./src/libcore/iter/traits/iterator.rs:1652
  72: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:198
  73: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
  74: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  75: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
  76: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
  77: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
  78: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
  79: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  80: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::break_scope
             at ./src/librustc_mir_build/build/scope.rs:523
  81: rustc_mir_build::build::expr::stmt::<impl rustc_mir_build::build::Builder>::stmt_expr
             at ./src/librustc_mir_build/build/expr/stmt.rs:97
  82: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:396
  83: <rustc_mir_build::hair::Expr as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:53
  84: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  85: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::expr_as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:112
  86: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp::{{closure}}
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  87: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  88: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  89: rustc_mir_build::build::expr::as_temp::<impl rustc_mir_build::build::Builder>::as_temp
             at ./src/librustc_mir_build/build/expr/as_temp.rs:29
  90: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:63
  91: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
  92: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  93: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
  94: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
  95: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
  96: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
  97: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
  98: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
  99: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 100: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 101: <rustc_mir_build::hair::Expr as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:53
 102: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 103: rustc_mir_build::build::matches::<impl rustc_mir_build::build::Builder>::lower_match_arms::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/matches/mod.rs:260
 104: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 105: rustc_mir_build::build::matches::<impl rustc_mir_build::build::Builder>::lower_match_arms::{{closure}}
             at ./src/librustc_mir_build/build/matches/mod.rs:237
 106: core::iter::adapters::map_fold::{{closure}}
             at ./src/libcore/iter/adapters/mod.rs:833
 107: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2003
 108: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at ./src/libcore/iter/adapters/mod.rs:873
 109: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:649
 110: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
             at ./src/liballoc/vec.rs:2140
 111: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
             at ./src/liballoc/vec.rs:2120
 112: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./src/liballoc/vec.rs:1995
 113: core::iter::traits::iterator::Iterator::collect
             at ./src/libcore/iter/traits/iterator.rs:1652
 114: rustc_mir_build::build::matches::<impl rustc_mir_build::build::Builder>::lower_match_arms
             at ./src/librustc_mir_build/build/matches/mod.rs:230
 115: rustc_mir_build::build::matches::<impl rustc_mir_build::build::Builder>::match_expr
             at ./src/librustc_mir_build/build/matches/mod.rs:105
 116: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:52
 117: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 118: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 119: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 120: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 121: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 122: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 123: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 124: rustc_mir_build::build::matches::<impl rustc_mir_build::build::Builder>::expr_into_pattern
             at ./src/librustc_mir_build/build/matches/mod.rs:402
 125: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block_stmts::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:131
 126: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 127: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block_stmts::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:123
 128: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_opt_scope
             at ./src/librustc_mir_build/build/scope.rs:411
 129: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block_stmts
             at ./src/librustc_mir_build/build/block.rs:119
 130: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:40
 131: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 132: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:29
 133: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_opt_scope
             at ./src/librustc_mir_build/build/scope.rs:411
 134: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block
             at ./src/librustc_mir_build/build/block.rs:28
 135: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:49
 136: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 137: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 138: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 139: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 140: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 141: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 142: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 143: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 144: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 145: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 146: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 147: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 148: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:232
 149: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 150: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 151: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 152: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 153: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 154: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 155: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 156: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block_stmts
             at ./src/librustc_mir_build/build/block.rs:184
 157: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:40
 158: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 159: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block::{{closure}}
             at ./src/librustc_mir_build/build/block.rs:29
 160: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_opt_scope
             at ./src/librustc_mir_build/build/scope.rs:411
 161: rustc_mir_build::build::block::<impl rustc_mir_build::build::Builder>::ast_block
             at ./src/librustc_mir_build/build/block.rs:28
 162: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:49
 163: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 164: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 165: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 166: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 167: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 168: <rustc_mir_build::hair::ExprRef as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:42
 169: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 170: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr::{{closure}}
             at ./src/librustc_mir_build/build/expr/into.rs:46
 171: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 172: rustc_mir_build::build::expr::into::<impl rustc_mir_build::build::Builder>::into_expr
             at ./src/librustc_mir_build/build/expr/into.rs:46
 173: <rustc_mir_build::hair::Expr as rustc_mir_build::build::into::EvalInto>::eval_into
             at ./src/librustc_mir_build/build/into.rs:53
 174: rustc_mir_build::build::into::<impl rustc_mir_build::build::Builder>::into
             at ./src/librustc_mir_build/build/into.rs:30
 175: rustc_mir_build::build::Builder::args_and_body
             at ./src/librustc_mir_build/build/mod.rs:959
 176: rustc_mir_build::build::construct_fn::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/mod.rs:621
 177: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 178: rustc_mir_build::build::construct_fn::{{closure}}::{{closure}}
             at ./src/librustc_mir_build/build/mod.rs:620
 179: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_breakable_scope
             at ./src/librustc_mir_build/build/scope.rs:392
 180: rustc_mir_build::build::construct_fn::{{closure}}
             at ./src/librustc_mir_build/build/mod.rs:615
 181: rustc_mir_build::build::scope::<impl rustc_mir_build::build::Builder>::in_scope
             at ./src/librustc_mir_build/build/scope.rs:455
 182: rustc_mir_build::build::construct_fn
             at ./src/librustc_mir_build/build/mod.rs:606
 183: rustc_mir_build::build::mir_build::{{closure}}
             at ./src/librustc_mir_build/build/mod.rs:163
 184: rustc_infer::infer::InferCtxtBuilder::enter
             at ./src/librustc_infer/infer/mod.rs:621
 185: rustc_mir_build::build::mir_build
             at ./src/librustc_mir_build/build/mod.rs:72
 186: rustc_mir_build::build::mir_built
             at ./src/librustc_mir_build/build/mod.rs:34
 187: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:381
 188: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 189: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./src/librustc_query_system/dep_graph/graph.rs:200
 190: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:599
 191: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 192: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
 193: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
 194: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 195: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1721
 196: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1705
 197: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1721
 198: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 199: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1809
 200: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1793
 201: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1782
 202: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1793
 203: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1806
 204: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 205: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
 206: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
 207: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
 208: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
 209: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
 210: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
 211: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
 212: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
#0 [mir_built] building MIR for `connect::{{closure}}#0`
#1 [unsafety_check_result] unsafety-checking `connect::{{closure}}#0`
#2 [unsafety_check_result] unsafety-checking `connect`
#3 [mir_const] processing MIR for `connect`
#4 [mir_validated] processing `connect`
#5 [mir_borrowck] borrow-checking `connect`
#6 [type_of] computing type of `connect::{{opaque}}#0`
#7 [check_mod_item_types] checking item types in top-level module
#8 [analysis] running analysis passes on this crate
end of query stack
