

running 1 test
test src/sets.rs - sets::get_ascii_hex_digit (line 67) ... FAILED

failures:

---- src/sets.rs - sets::get_ascii_hex_digit (line 67) stdout ----
thread 'rustc' panicked at 'assertion failed: !new_self_ty.has_escaping_bound_vars()', compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1959:9
stack backtrace:
   0:     0x7faa172f2cb4 - std::backtrace_rs::backtrace::libunwind::trace::h5026dded2156dcaa
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7faa172f2cb4 - std::backtrace_rs::backtrace::trace_unsynchronized::h373d785316296b41
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7faa172f2cb4 - std::sys_common::backtrace::_print_fmt::h9a355d9766d46fef
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7faa172f2cb4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2379290fdac7fde5
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7faa173a9e39 - core::fmt::write::hf173fcea9973b855
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/fmt/mod.rs:1194:17
   5:     0x7faa172f4921 - std::io::Write::write_fmt::hb5a53cba968ebc19
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/io/mod.rs:1655:15
   6:     0x7faa172f2b09 - std::sys_common::backtrace::_print::h4f943e0388e61732
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7faa172f2b09 - std::sys_common::backtrace::print::h1928747851bf1be8
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7faa172e8377 - std::panicking::default_hook::{{closure}}::hd5e619ba4607f911
   9:     0x7faa172e8188 - std::panicking::default_hook::hdce419e5836c64ac
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:314:9
  10:     0x7faa17e1fd21 - <alloc[c9b3fe375b66920f]::boxed::Box<dyn for<'a, 'b> core[c13c27405bd03191]::ops::function::Fn<(&'a core[c13c27405bd03191]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[c13c27405bd03191]::marker::Send + core[c13c27405bd03191]::marker::Sync> as core[c13c27405bd03191]::ops::function::Fn<(&core[c13c27405bd03191]::panic::panic_info::PanicInfo,)>>::call
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1880:9
  11:     0x7faa17e1fd21 - rustc_driver[8a41892a12384e20]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:1157:13
  12:     0x7faa172e880f - std::panicking::rust_panic_with_hook::h3508c91d4eda9b15
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:702:17
  13:     0x7faa172ede59 - std::panicking::begin_panic_handler::{{closure}}::h44c734ec538bd78d
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:586:13
  14:     0x7faa172eddd4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbdf48f3291fba5ff
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:138:18
  15:     0x7faa172e8442 - rust_begin_unwind
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:584:5
  16:     0x7faa172ce013 - core::panicking::panic_fmt::ha448108a7240a776
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/panicking.rs:142:14
  17:     0x7faa172cdedd - core::panicking::panic::hbbad75009fa7050f
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/panicking.rs:48:5
  18:     0x7faa1a1a9dd7 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::InferCtxtPrivExt>::mk_trait_obligation_with_new_self_ty
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1959:9
  19:     0x7faa1a196eb0 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::suggestions::InferCtxtExt>::suggest_add_reference_to_arg::{closure#1}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:887:21
  20:     0x7faa1a196eb0 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::suggestions::InferCtxtExt>::suggest_add_reference_to_arg::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:890:30
  21:     0x7faa1a196c8c - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::suggestions::InferCtxtExt>::suggest_add_reference_to_arg
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:963:13
  22:     0x7faa1a1a45ad - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::InferCtxtExt>::report_selection_error
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:463:28
  23:     0x7faa1a1ad659 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1513:17
  24:     0x7faa1a1a1563 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxt as rustc_trait_selection[ea9235783e7ee907]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:206:17
  25:     0x7faa1896b115 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::select_obligations_where_possible::<<rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#2}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:651:13
  26:     0x7faa187d3f52 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_argument_types
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:343:17
  27:     0x7faa187b9b03 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/callee.rs:517:9
  28:     0x7faa187b7c10 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_call
  29:     0x7faa1880f527 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_kind
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:310:45
  30:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
  31:     0x7faa187c4cf8 - stacker[ddedec91c7f96370]::maybe_grow::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  32:     0x7faa187c4cf8 - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  33:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
  34:     0x7faa1880e549 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
  35:     0x7faa1880eeed - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:142:9
  36:     0x7faa1880eeed - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_method_call
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:1120:22
  37:     0x7faa1880eeed - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_kind
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:312:17
  38:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
  39:     0x7faa187c4cf8 - stacker[ddedec91c7f96370]::maybe_grow::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  40:     0x7faa187c4cf8 - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  41:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
  42:     0x7faa1880e549 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
  43:     0x7faa187c475f - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_hint
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:121:9
  44:     0x7faa187c475f - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_coercable_to_type
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:111:18
  45:     0x7faa187db7fe - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_decl
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1219:27
  46:     0x7faa187db9a3 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_decl_local
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1238:9
  47:     0x7faa187db9a3 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_stmt
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1256:17
  48:     0x7faa1896b354 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1336:17
  49:     0x7faa1896b354 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1552:22
  50:     0x7faa187dbc56 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_block_with_expected
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1334:26
  51:     0x7faa1880f83d - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_kind
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:309:41
  52:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
  53:     0x7faa187c4cf8 - stacker[ddedec91c7f96370]::maybe_grow::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  54:     0x7faa187c4cf8 - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<rustc_middle[41f76c3e63c1d9c7]::ty::Ty, <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  55:     0x7faa187c4cf8 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
  56:     0x7faa1880e549 - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
  57:     0x7faa187c632c - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_expr_with_hint
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:121:9
  58:     0x7faa187c632c - <rustc_typeck[ad9f69e232e66299]::check::fn_ctxt::FnCtxt>::check_return_expr
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:807:30
  59:     0x7faa1872224c - rustc_typeck[ad9f69e232e66299]::check::check::check_fn
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/check.rs:210:9
  60:     0x7faa18888176 - rustc_typeck[ad9f69e232e66299]::check::typeck_with_fallback::<rustc_typeck[ad9f69e232e66299]::check::typeck::{closure#0}>::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:405:23
  61:     0x7faa18888176 - <rustc_typeck[ad9f69e232e66299]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[ad9f69e232e66299]::check::typeck_with_fallback<rustc_typeck[ad9f69e232e66299]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/inherited.rs:96:34
  62:     0x7faa18888176 - <rustc_infer[9809a3f932fb06b2]::infer::InferCtxtBuilder>::enter::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, <rustc_typeck[ad9f69e232e66299]::check::inherited::InheritedBuilder>::enter<rustc_typeck[ad9f69e232e66299]::check::typeck_with_fallback<rustc_typeck[ad9f69e232e66299]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_infer/src/infer/mod.rs:628:9
  63:     0x7faa186798de - <rustc_typeck[ad9f69e232e66299]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[ad9f69e232e66299]::check::typeck_with_fallback<rustc_typeck[ad9f69e232e66299]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/inherited.rs:96:9
  64:     0x7faa186e83ce - rustc_typeck[ad9f69e232e66299]::check::typeck_with_fallback::<rustc_typeck[ad9f69e232e66299]::check::typeck::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:368:26
  65:     0x7faa186e9745 - rustc_typeck[ad9f69e232e66299]::check::typeck
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:332:9
  66:     0x7faa19663eac - <rustc_query_system[9ced47a84c6bfff0]::query::config::QueryVtable<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>>::compute
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
  67:     0x7faa19663eac - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
  68:     0x7faa19663eac - stacker[ddedec91c7f96370]::maybe_grow::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  69:     0x7faa19663eac - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  70:     0x7faa19663eac - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
  71:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
  72:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::set_tlv::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
  73:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
  74:     0x7faa19663eac - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
  75:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
  76:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
  77:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context_opt::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
  78:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
  79:     0x7faa19663eac - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
  80:     0x7faa19663eac - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<&rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
  81:     0x7faa19663eac - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
  82:     0x7faa19663eac - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::try_execute_query::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_query_system[9ced47a84c6bfff0]::query::caches::DefaultCache<rustc_span[df63c0ac7ff6c360]::def_id::LocalDefId, &rustc_middle[41f76c3e63c1d9c7]::ty::context::TypeckResults>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
  83:     0x7faa1978d6b0 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::get_query::<rustc_query_impl[6b38e94f2c6b48bc]::queries::typeck, rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
  84:     0x7faa1876ee9a - <rustc_middle[41f76c3e63c1d9c7]::ty::query::TyCtxtEnsure>::typeck
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:230:17
  85:     0x7faa1876ee9a - rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:956:51
  86:     0x7faa1876ee9a - <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners::<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:541:21
  87:     0x7faa1876ee9a - <core[c13c27405bd03191]::slice::iter::Iter<(rustc_hir[86f7332bc978f9f4]::hir_id::ItemLocalId, &rustc_hir[86f7332bc978f9f4]::hir::Body)> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::for_each::<<rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/slice/iter/macros.rs:211:21
  88:     0x7faa1876ee9a - <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners::<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:538:17
  89:     0x7faa1876ee9a - core[c13c27405bd03191]::iter::traits::iterator::Iterator::for_each::call::<(usize, &rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>), <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:780:29
  90:     0x7faa1876ee9a - <core[c13c27405bd03191]::iter::adapters::enumerate::Enumerate<_> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::fold::enumerate::<&rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>, (), core[c13c27405bd03191]::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>), <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/adapters/enumerate.rs:106:27
  91:     0x7faa1876ee9a - <core[c13c27405bd03191]::slice::iter::Iter<rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::fold::<(), <core[c13c27405bd03191]::iter::adapters::enumerate::Enumerate<_> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::fold::enumerate<&rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>, (), core[c13c27405bd03191]::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>), <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:2366:21
  92:     0x7faa1876ee9a - <core[c13c27405bd03191]::iter::adapters::enumerate::Enumerate<core[c13c27405bd03191]::slice::iter::Iter<rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>>> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::fold::<(), core[c13c27405bd03191]::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>), <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/adapters/enumerate.rs:112:9
  93:     0x7faa1876ee9a - <core[c13c27405bd03191]::iter::adapters::enumerate::Enumerate<core[c13c27405bd03191]::slice::iter::Iter<rustc_hir[86f7332bc978f9f4]::hir::MaybeOwner<&rustc_hir[86f7332bc978f9f4]::hir::OwnerInfo>>> as core[c13c27405bd03191]::iter::traits::iterator::Iterator>::for_each::<<rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:783:9
  94:     0x7faa1876ee9a - <rustc_middle[41f76c3e63c1d9c7]::hir::map::Map>::par_body_owners::<rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:535:9
  95:     0x7faa186ecfbd - rustc_typeck[ad9f69e232e66299]::check::typeck_item_bodies
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:956:5
  96:     0x7faa196a9a15 - <rustc_query_system[9ced47a84c6bfff0]::query::config::QueryVtable<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>>::compute
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
  97:     0x7faa196a9a15 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
  98:     0x7faa196a9a15 - stacker[ddedec91c7f96370]::maybe_grow::<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  99:     0x7faa196a9a15 - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
 100:     0x7faa196a9a15 - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
 101:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
 102:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::set_tlv::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
 103:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
 104:     0x7faa196a9a15 - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
 105:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
 106:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
 107:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context_opt::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
 108:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
 109:     0x7faa196a9a15 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
 110:     0x7faa196a9a15 - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<(), rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
 111:     0x7faa196a9a15 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), ()>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
 112:     0x7faa196a9a15 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::try_execute_query::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_query_system[9ced47a84c6bfff0]::query::caches::DefaultCache<(), ()>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
 113:     0x7faa19744128 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::get_query::<rustc_query_impl[6b38e94f2c6b48bc]::queries::typeck_item_bodies, rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
 114:     0x7faa18772b98 - <rustc_middle[41f76c3e63c1d9c7]::ty::query::TyCtxtAt>::typeck_item_bodies
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:259:17
 115:     0x7faa18772b98 - <rustc_middle[41f76c3e63c1d9c7]::ty::context::TyCtxt>::typeck_item_bodies
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:240:17
 116:     0x7faa18772b98 - rustc_typeck[ad9f69e232e66299]::check_crate::{closure#7}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/lib.rs:538:46
 117:     0x7faa18772b98 - <rustc_data_structures[6b8b6ffbf21226b0]::profiling::VerboseTimingGuard>::run::<(), rustc_typeck[ad9f69e232e66299]::check_crate::{closure#7}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/profiling.rs:732:9
 118:     0x7faa18772b98 - <rustc_session[4a3b4a0df331e32b]::session::Session>::time::<(), rustc_typeck[ad9f69e232e66299]::check_crate::{closure#7}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_session/src/utils.rs:16:9
 119:     0x7faa1891a208 - rustc_typeck[ad9f69e232e66299]::check_crate
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/lib.rs:538:5
 120:     0x7faa17f45081 - rustc_interface[66cde676f522cc2a]::passes::analysis
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:929:5
 121:     0x7faa1969e7da - <rustc_query_system[9ced47a84c6bfff0]::query::config::QueryVtable<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>>::compute
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
 122:     0x7faa1969e7da - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
 123:     0x7faa1969e7da - stacker[ddedec91c7f96370]::maybe_grow::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
 124:     0x7faa1969e7da - rustc_data_structures[6b8b6ffbf21226b0]::stack::ensure_sufficient_stack::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
 125:     0x7faa1969e7da - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
 126:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
 127:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::set_tlv::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
 128:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
 129:     0x7faa1969e7da - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
 130:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
 131:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
 132:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context_opt::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
 133:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_context::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
 134:     0x7faa1969e7da - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::with_related_context::<<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
 135:     0x7faa1969e7da - <rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt as rustc_query_system[9ced47a84c6bfff0]::query::QueryContext>::start_query::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
 136:     0x7faa1969e7da - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::execute_job::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, (), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
 137:     0x7faa1969e7da - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::try_execute_query::<rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt, rustc_query_system[9ced47a84c6bfff0]::query::caches::DefaultCache<(), core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
 138:     0x7faa1978e1f9 - rustc_query_system[9ced47a84c6bfff0]::query::plumbing::get_query::<rustc_query_impl[6b38e94f2c6b48bc]::queries::analysis, rustc_query_impl[6b38e94f2c6b48bc]::plumbing::QueryCtxt>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
 139:     0x7faa17e9e6f4 - <rustc_middle[41f76c3e63c1d9c7]::ty::query::TyCtxtAt>::analysis
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:259:17
 140:     0x7faa17e9e6f4 - <rustc_middle[41f76c3e63c1d9c7]::ty::context::TyCtxt>::analysis
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:240:17
 141:     0x7faa17e9e6f4 - rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:383:30
 142:     0x7faa17e9e6f4 - <rustc_interface[66cde676f522cc2a]::passes::QueryContext>::enter::<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:819:42
 143:     0x7faa17e9e6f4 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_interface[66cde676f522cc2a]::passes::QueryContext>::enter<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
 144:     0x7faa17e9e6f4 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::set_tlv::<rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context<<rustc_interface[66cde676f522cc2a]::passes::QueryContext>::enter<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
 145:     0x7faa17e9e6f4 - rustc_middle[41f76c3e63c1d9c7]::ty::context::tls::enter_context::<<rustc_interface[66cde676f522cc2a]::passes::QueryContext>::enter<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
 146:     0x7faa17e9e6f4 - <rustc_interface[66cde676f522cc2a]::passes::QueryContext>::enter::<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:819:9
 147:     0x7faa17eaca5f - rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:382:13
 148:     0x7faa17eaca5f - <rustc_interface[66cde676f522cc2a]::interface::Compiler>::enter::<rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}::{closure#2}, core[c13c27405bd03191]::result::Result<core[c13c27405bd03191]::option::Option<rustc_interface[66cde676f522cc2a]::queries::Linker>, rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/queries.rs:384:19
 149:     0x7faa17e2d8e7 - rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:311:22
 150:     0x7faa17e2d8e7 - rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:323:13
 151:     0x7faa17e2d8e7 - rustc_span[df63c0ac7ff6c360]::with_source_map::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#1}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:990:5
 152:     0x7faa17ead833 - rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:317:5
 153:     0x7faa17e35652 - rustc_interface[66cde676f522cc2a]::interface::run_compiler::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:337:12
 154:     0x7faa17e35652 - <scoped_tls[6f50be60c4a17844]::ScopedKey<rustc_span[df63c0ac7ff6c360]::SessionGlobals>>::set::<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 155:     0x7faa17e39d39 - rustc_span[df63c0ac7ff6c360]::create_session_globals_then::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:113:5
 156:     0x7faa17e39d39 - rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals::<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/util.rs:157:32
 157:     0x7faa17e39d39 - std[965c7f0aef32bf78]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:122:18
 158:     0x7faa17e952be - <std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:498:17
 159:     0x7faa17e952be - <core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[c13c27405bd03191]::ops::function::FnOnce<()>>::call_once
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/panic/unwind_safe.rs:271:9
 160:     0x7faa17e952be - std[965c7f0aef32bf78]::panicking::try::do_call::<core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:492:40
 161:     0x7faa17e952be - std[965c7f0aef32bf78]::panicking::try::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:456:19
 162:     0x7faa17eb2040 - std[965c7f0aef32bf78]::panic::catch_unwind::<core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panic.rs:137:14
 163:     0x7faa17eb2040 - <std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:497:30
 164:     0x7faa17eb2040 - <<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1} as core[c13c27405bd03191]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ops/function.rs:248:5
 165:     0x7faa17309948 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3f742a9ff51dd90a
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
 166:     0x7faa17309948 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hdd4bb9b480713a39
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
 167:     0x7faa172d04c7 - std::sys::unix::thread::Thread::new::thread_start::h62d77a231384ca95
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys/unix/thread.rs:108:17
 168:     0x7faa11e50d80 - start_thread
                               at ./nptl/./nptl/pthread_create.c:481:8
 169:     0x7faa1716a76f - clone
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
 170:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -C embed-bitcode=no

query stack during panic:
#0 [typeck] type-checking `main::_doctest_main_src_sets_rs_67_0`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::HandlerInner>::delay_good_path_bug::<&str>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_errors/src/lib.rs:1285:25
             1: <rustc_errors::Handler>::delay_good_path_bug::<&str>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_errors/src/lib.rs:859:9
             2: rustc_middle::ty::print::pretty::trimmed_def_paths
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2758:9
             3: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::compute
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
             4: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
             5: stacker::maybe_grow::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
             6: rustc_data_structures::stack::ensure_sufficient_stack::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
             7: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
             8: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
             9: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
            10: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
            11: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
            12: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
            13: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
            14: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
            15: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
            16: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
            17: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
            18: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
            19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
            20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
            21: <rustc_middle::ty::query::TyCtxtAt>::trimmed_def_paths
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:259:17
            22: <rustc_middle::ty::context::TyCtxt>::trimmed_def_paths
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:240:17
            23: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::try_print_trimmed_def_path
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:305:15
            24: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:1681:19
            25: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/mod.rs:146:42
            26: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::path_generic_args::<<rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:1844:16
            27: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/mod.rs:145:40
            28: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:617:17
            29: <rustc_middle::ty::Ty as rustc_middle::ty::print::Print<rustc_middle::ty::print::pretty::FmtPrinter>>::print
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/mod.rs:335:9
            30: <rustc_middle::ty::TraitPredicate as rustc_middle::ty::print::Print<rustc_middle::ty::print::pretty::FmtPrinter>>::print
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2600:9
            31: <rustc_middle::ty::print::pretty::FmtPrinter>::pretty_in_binder::<rustc_middle::ty::TraitPredicate>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2286:25
            32: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::in_binder::<rustc_middle::ty::TraitPredicate>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:1893:9
            33: <rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as rustc_middle::ty::print::Print<rustc_middle::ty::print::pretty::FmtPrinter>>::print
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2366:9
            34: <rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2390:30
            35: rustc_middle::ty::context::tls::with::<<rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1873:32
            36: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with<<rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
            37: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with<<rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
            38: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with<<rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
            39: rustc_middle::ty::context::tls::with::<<rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt::{closure#0}, core::result::Result<(), core::fmt::Error>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1873:9
            40: <rustc_middle::ty::sty::Binder<rustc_middle::ty::TraitPredicate> as core::fmt::Display>::fmt
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/print/pretty.rs:2389:17
            41: core::fmt::write
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/fmt/mod.rs:1194:17
            42: core::fmt::Write::write_fmt
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/fmt/mod.rs:186:9
            43: alloc::fmt::format
                       at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/fmt.rs:597:5
            44: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error::{closure#9}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:400:52
            45: <core::option::Option<alloc::string::String>>::unwrap_or_else::<<rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error::{closure#9}>
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/option.rs:805:21
            46: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:383:29
            47: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1513:17
            48: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:206:17
            49: <rustc_typeck::check::fn_ctxt::FnCtxt>::select_obligations_where_possible::<<rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#2}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:651:13
            50: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:343:17
            51: <rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/callee.rs:517:9
            52: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
            53: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:310:45
            54: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
            55: stacker::maybe_grow::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
            56: rustc_data_structures::stack::ensure_sufficient_stack::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
            57: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
            58: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
            59: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:142:9
            60: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_method_call
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:1120:22
            61: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:312:17
            62: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
            63: stacker::maybe_grow::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
            64: rustc_data_structures::stack::ensure_sufficient_stack::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
            65: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
            66: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
            67: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_hint
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:121:9
            68: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_coercable_to_type
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:111:18
            69: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_decl
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1219:27
            70: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_decl_local
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1238:9
            71: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1256:17
            72: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1336:17
            73: <rustc_typeck::check::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1552:22
            74: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1334:26
            75: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:309:41
            76: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:220:18
            77: stacker::maybe_grow::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
            78: rustc_data_structures::stack::ensure_sufficient_stack::<rustc_middle::ty::Ty, <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
            79: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:216:18
            80: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:169:9
            81: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_hint
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:121:9
            82: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/expr.rs:807:30
            83: rustc_typeck::check::check::check_fn
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/check.rs:210:9
            84: rustc_typeck::check::typeck_with_fallback::<rustc_typeck::check::typeck::{closure#0}>::{closure#1}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:405:23
            85: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/inherited.rs:96:34
            86: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_infer/src/infer/mod.rs:628:9
            87: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/inherited.rs:96:9
            88: rustc_typeck::check::typeck_with_fallback::<rustc_typeck::check::typeck::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:368:26
            89: rustc_typeck::check::typeck
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:332:9
            90: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>::compute
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
            91: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
            92: stacker::maybe_grow::<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
            93: rustc_data_structures::stack::ensure_sufficient_stack::<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
            94: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
            95: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
            96: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
            97: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
            98: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
            99: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
           100: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
           101: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
           102: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
           103: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
           104: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<&rustc_middle::ty::context::TypeckResults, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
           105: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
           106: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
           107: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
           108: <rustc_middle::ty::query::TyCtxtEnsure>::typeck
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:230:17
           109: rustc_typeck::check::typeck_item_bodies::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:956:51
           110: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:541:21
           111: <core::slice::iter::Iter<(rustc_hir::hir_id::ItemLocalId, &rustc_hir::hir::Body)> as core::iter::traits::iterator::Iterator>::for_each::<<rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/slice/iter/macros.rs:211:21
           112: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:538:17
           113: core::iter::traits::iterator::Iterator::for_each::call::<(usize, &rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:780:29
           114: <core::iter::adapters::enumerate::Enumerate<_> as core::iter::traits::iterator::Iterator>::fold::enumerate::<&rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>, (), core::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/adapters/enumerate.rs:106:27
           115: <core::slice::iter::Iter<rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>> as core::iter::traits::iterator::Iterator>::fold::<(), <core::iter::adapters::enumerate::Enumerate<_> as core::iter::traits::iterator::Iterator>::fold::enumerate<&rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>, (), core::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:2366:21
           116: <core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<(usize, &rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/adapters/enumerate.rs:112:9
           117: <core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>>> as core::iter::traits::iterator::Iterator>::for_each::<<rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/iter/traits/iterator.rs:783:9
           118: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/hir/map/mod.rs:535:9
           119: rustc_typeck::check::typeck_item_bodies
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/check/mod.rs:956:5
           120: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), ()>>::compute
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
           121: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
           122: stacker::maybe_grow::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
           123: rustc_data_structures::stack::ensure_sufficient_stack::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
           124: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
           125: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
           126: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
           127: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
           128: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
           129: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
           130: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
           131: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
           132: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
           133: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>::{closure#0}, ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
           134: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
           135: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
           136: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
           137: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
           138: <rustc_middle::ty::query::TyCtxtAt>::typeck_item_bodies
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:259:17
           139: <rustc_middle::ty::context::TyCtxt>::typeck_item_bodies
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:240:17
           140: rustc_typeck::check_crate::{closure#7}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/lib.rs:538:46
           141: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_typeck::check_crate::{closure#7}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/profiling.rs:732:9
           142: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_session/src/utils.rs:16:9
           143: rustc_typeck::check_crate
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_typeck/src/lib.rs:538:5
           144: rustc_interface::passes::analysis
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:929:5
           145: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>::compute
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/config.rs:43:9
           146: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:55
           147: stacker::maybe_grow::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
           148: rustc_data_structures::stack::ensure_sufficient_stack::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_data_structures/src/stack.rs:17:5
           149: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:113:17
           150: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
           151: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
           152: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
           153: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:112:13
           154: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1862:13
           155: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:40
           156: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1835:22
           157: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1846:9
           158: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1859:9
           159: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_impl/src/plumbing.rs:101:9
           160: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:384:22
           161: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:343:44
           162: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_query_system/src/query/plumbing.rs:701:36
           163: <rustc_middle::ty::query::TyCtxtAt>::analysis
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:259:17
           164: <rustc_middle::ty::context::TyCtxt>::analysis
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/query.rs:240:17
           165: rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:383:30
           166: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:819:42
           167: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:50
           168: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1802:9
           169: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_middle/src/ty/context.rs:1818:9
           170: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/passes.rs:819:9
           171: rustc_driver::run_compiler::{closure#1}::{closure#2}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:382:13
           172: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/queries.rs:384:19
           173: rustc_driver::run_compiler::{closure#1}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:311:22
           174: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:323:13
           175: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:990:5
           176: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:317:5
           177: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:337:12
           178: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
           179: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:113:5
           180: rustc_interface::util::run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/util.rs:157:32
           181: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:122:18
           182: <std::thread::Builder>::spawn_unchecked_::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:498:17
           183: <core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/panic/unwind_safe.rs:271:9
           184: std::panicking::try::do_call::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:492:40
           185: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:456:19
           186: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/panic.rs:137:14
           187: <std::thread::Builder>::spawn_unchecked_::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:497:30
           188: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                       at /usr/local/google/home/manishearth/dev/rust/library/core/src/ops/function.rs:248:5
           189: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
           190: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
           191: std::sys::unix::thread::Thread::new::thread_start
                       at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys/unix/thread.rs:108:17
           192: start_thread
                       at ./nptl/./nptl/pthread_create.c:481:8
           193: clone
                       at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
          

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1346:13
stack backtrace:
   0:     0x7faa172f2cb4 - std::backtrace_rs::backtrace::libunwind::trace::h5026dded2156dcaa
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7faa172f2cb4 - std::backtrace_rs::backtrace::trace_unsynchronized::h373d785316296b41
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7faa172f2cb4 - std::sys_common::backtrace::_print_fmt::h9a355d9766d46fef
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7faa172f2cb4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2379290fdac7fde5
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7faa173a9e39 - core::fmt::write::hf173fcea9973b855
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/fmt/mod.rs:1194:17
   5:     0x7faa172f4921 - std::io::Write::write_fmt::hb5a53cba968ebc19
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/io/mod.rs:1655:15
   6:     0x7faa172f2b09 - std::sys_common::backtrace::_print::h4f943e0388e61732
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7faa172f2b09 - std::sys_common::backtrace::print::h1928747851bf1be8
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7faa172e8377 - std::panicking::default_hook::{{closure}}::hd5e619ba4607f911
   9:     0x7faa172e8188 - std::panicking::default_hook::hdce419e5836c64ac
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:314:9
  10:     0x7faa17e1fd21 - <alloc[c9b3fe375b66920f]::boxed::Box<dyn for<'a, 'b> core[c13c27405bd03191]::ops::function::Fn<(&'a core[c13c27405bd03191]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[c13c27405bd03191]::marker::Send + core[c13c27405bd03191]::marker::Sync> as core[c13c27405bd03191]::ops::function::Fn<(&core[c13c27405bd03191]::panic::panic_info::PanicInfo,)>>::call
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1880:9
  11:     0x7faa17e1fd21 - rustc_driver[8a41892a12384e20]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_driver/src/lib.rs:1157:13
  12:     0x7faa172e880f - std::panicking::rust_panic_with_hook::h3508c91d4eda9b15
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:702:17
  13:     0x7faa1a6c4bd3 - std[965c7f0aef32bf78]::panicking::begin_panic::<rustc_errors[a532696fef9d9b53]::ExplicitBug>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:617:9
  14:     0x7faa1a6c4b86 - std[965c7f0aef32bf78]::sys_common::backtrace::__rust_end_short_backtrace::<std[965c7f0aef32bf78]::panicking::begin_panic<rustc_errors[a532696fef9d9b53]::ExplicitBug>::{closure#0}, !>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:138:18
  15:     0x7faa17debfbf - std[965c7f0aef32bf78]::panicking::begin_panic::<rustc_errors[a532696fef9d9b53]::ExplicitBug>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:616:12
  16:     0x7faa1a6b1166 - std[965c7f0aef32bf78]::panic::panic_any::<rustc_errors[a532696fef9d9b53]::ExplicitBug>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panic.rs:61:5
  17:     0x7faa1a6b4538 - <rustc_errors[a532696fef9d9b53]::HandlerInner>::flush_delayed::<core[c13c27405bd03191]::iter::adapters::map::Map<alloc[c9b3fe375b66920f]::vec::into_iter::IntoIter<rustc_errors[a532696fef9d9b53]::DelayedDiagnostic>, <rustc_errors[a532696fef9d9b53]::DelayedDiagnostic>::decorate>, &str>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_errors/src/lib.rs:1346:13
  18:     0x7faa1a692c43 - <rustc_errors[a532696fef9d9b53]::HandlerInner as core[c13c27405bd03191]::ops::drop::Drop>::drop
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_errors/src/lib.rs:493:13
  19:     0x7faa17e7bc62 - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_errors[a532696fef9d9b53]::HandlerInner>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  20:     0x7faa17e7bc62 - core[c13c27405bd03191]::ptr::drop_in_place::<core[c13c27405bd03191]::cell::UnsafeCell<rustc_errors[a532696fef9d9b53]::HandlerInner>>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  21:     0x7faa17e7bc62 - core[c13c27405bd03191]::ptr::drop_in_place::<core[c13c27405bd03191]::cell::RefCell<rustc_errors[a532696fef9d9b53]::HandlerInner>>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  22:     0x7faa17e7bc62 - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_data_structures[6b8b6ffbf21226b0]::sync::Lock<rustc_errors[a532696fef9d9b53]::HandlerInner>>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  23:     0x7faa17e7bc62 - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_errors[a532696fef9d9b53]::Handler>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  24:     0x7faa17e83961 - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_session[4a3b4a0df331e32b]::parse::ParseSess>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  25:     0x7faa17e6045a - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_session[4a3b4a0df331e32b]::session::Session>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  26:     0x7faa17e6045a - <alloc[c9b3fe375b66920f]::rc::Rc<rustc_session[4a3b4a0df331e32b]::session::Session> as core[c13c27405bd03191]::ops::drop::Drop>::drop
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/rc.rs:1511:17
  27:     0x7faa17e6045a - core[c13c27405bd03191]::ptr::drop_in_place::<alloc[c9b3fe375b66920f]::rc::Rc<rustc_session[4a3b4a0df331e32b]::session::Session>>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  28:     0x7faa17e862e1 - core[c13c27405bd03191]::ptr::drop_in_place::<rustc_interface[66cde676f522cc2a]::interface::Compiler>
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ptr/mod.rs:486:1
  29:     0x7faa17e2dc52 - rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:329:5
  30:     0x7faa17e2dc52 - rustc_span[df63c0ac7ff6c360]::with_source_map::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#1}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:990:5
  31:     0x7faa17ead833 - rustc_interface[66cde676f522cc2a]::interface::create_compiler_and_run::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:317:5
  32:     0x7faa17e35652 - rustc_interface[66cde676f522cc2a]::interface::run_compiler::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/interface.rs:337:12
  33:     0x7faa17e35652 - <scoped_tls[6f50be60c4a17844]::ScopedKey<rustc_span[df63c0ac7ff6c360]::SessionGlobals>>::set::<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  34:     0x7faa17e39d39 - rustc_span[df63c0ac7ff6c360]::create_session_globals_then::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}>
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_span/src/lib.rs:113:5
  35:     0x7faa17e39d39 - rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals::<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/compiler/rustc_interface/src/util.rs:157:32
  36:     0x7faa17e39d39 - std[965c7f0aef32bf78]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys_common/backtrace.rs:122:18
  37:     0x7faa17e952be - <std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:498:17
  38:     0x7faa17e952be - <core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[c13c27405bd03191]::ops::function::FnOnce<()>>::call_once
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/panic/unwind_safe.rs:271:9
  39:     0x7faa17e952be - std[965c7f0aef32bf78]::panicking::try::do_call::<core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:492:40
  40:     0x7faa17e952be - std[965c7f0aef32bf78]::panicking::try::<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panicking.rs:456:19
  41:     0x7faa17eb2040 - std[965c7f0aef32bf78]::panic::catch_unwind::<core[c13c27405bd03191]::panic::unwind_safe::AssertUnwindSafe<<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/panic.rs:137:14
  42:     0x7faa17eb2040 - <std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_::<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1}
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/thread/mod.rs:497:30
  43:     0x7faa17eb2040 - <<std[965c7f0aef32bf78]::thread::Builder>::spawn_unchecked_<rustc_interface[66cde676f522cc2a]::util::run_in_thread_pool_with_globals<rustc_interface[66cde676f522cc2a]::interface::run_compiler<core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>, rustc_driver[8a41892a12384e20]::run_compiler::{closure#1}>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#0}, core[c13c27405bd03191]::result::Result<(), rustc_errors[a532696fef9d9b53]::ErrorGuaranteed>>::{closure#1} as core[c13c27405bd03191]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /usr/local/google/home/manishearth/dev/rust/library/core/src/ops/function.rs:248:5
  44:     0x7faa17309948 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3f742a9ff51dd90a
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
  45:     0x7faa17309948 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hdd4bb9b480713a39
                               at /usr/local/google/home/manishearth/dev/rust/library/alloc/src/boxed.rs:1866:9
  46:     0x7faa172d04c7 - std::sys::unix::thread::Thread::new::thread_start::h62d77a231384ca95
                               at /usr/local/google/home/manishearth/dev/rust/library/std/src/sys/unix/thread.rs:108:17
  47:     0x7faa11e50d80 - start_thread
                               at ./nptl/./nptl/pthread_create.c:481:8
  48:     0x7faa1716a76f - clone
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  49:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C codegen-units=1 -C embed-bitcode=no

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
Couldn't compile the test.

failures:
    src/sets.rs - sets::get_ascii_hex_digit (line 67)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 70 filtered out; finished in 0.53s


