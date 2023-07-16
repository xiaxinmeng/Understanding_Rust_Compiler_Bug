
---- [ui] tests/ui/typeck/bad-type-in-vec-contains.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/home/jistone/rust/rust-beta/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/jistone/rust/rust-beta/tests/ui/typeck/bad-type-in-vec-contains.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/jistone/rust/rust-beta/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/jistone/rust/rust-beta/build/x86_64-unknown-linux-gnu/test/ui/typeck/bad-type-in-vec-contains" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/jistone/rust/rust-beta/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/jistone/rust/rust-beta/build/x86_64-unknown-linux-gnu/test/ui/typeck/bad-type-in-vec-contains/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 12 but the index is 13', /home/jistone/.cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
stack backtrace:
   0:     0x7f3d5fad6421 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h48926b6600073216
   1:     0x7f3d5fb79dae - core::fmt::write::h4beb9d2e4cdda824
   2:     0x7f3d5fb102b1 - std::io::Write::write_fmt::hf931c18c4c61701f
   3:     0x7f3d5fad624a - std::sys_common::backtrace::print::hb4e1f30d04c52822
   4:     0x7f3d5fae70e7 - std::panicking::default_hook::{{closure}}::haeacb66d92ab5102
   5:     0x7f3d5fae6f15 - std::panicking::default_hook::h07e60f2fd94e0596
   6:     0x7f3d604795d5 - rustc_driver[56eea7eb949f9e3d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3d5fae75f3 - std::panicking::rust_panic_with_hook::h30c1b992798b358e
   8:     0x7f3d5fb13259 - std::panicking::begin_panic_handler::{{closure}}::h80c03d23cabf7aac
   9:     0x7f3d5fb1318c - std::sys_common::backtrace::__rust_end_short_backtrace::h079491746d554ce2
  10:     0x7f3d5fae71ba - rust_begin_unwind
  11:     0x7f3d5fac9293 - core::panicking::panic_fmt::he03fa4a98df4a1b7
  12:     0x7f3d5fac9402 - core::panicking::panic_bounds_check::h8734b56504755baf
  13:     0x7f3d626ee7ee - <ena[209fda735d60cbbe]::snapshot_vec::SnapshotVec<ena[209fda735d60cbbe]::unify::backing_vec::Delegate<rustc_infer[fef6918b819b6f5f]::infer::type_variable::TyVidEqKey>, &mut alloc[88dc4bacf9c5ecbf]::vec::Vec<ena[209fda735d60cbbe]::unify::VarValue<rustc_infer[fef6918b819b6f5f]::infer::type_variable::TyVidEqKey>>, &mut rustc_infer[fef6918b819b6f5f]::infer::undo_log::InferCtxtUndoLogs> as core[1dc6fc0132f421c8]::ops::index::Index<usize>>::index
  14:     0x7f3d6265fec1 - <ena[209fda735d60cbbe]::unify::UnificationTable<ena[209fda735d60cbbe]::unify::backing_vec::InPlace<rustc_infer[fef6918b819b6f5f]::infer::type_variable::TyVidEqKey, &mut alloc[88dc4bacf9c5ecbf]::vec::Vec<ena[209fda735d60cbbe]::unify::VarValue<rustc_infer[fef6918b819b6f5f]::infer::type_variable::TyVidEqKey>>, &mut rustc_infer[fef6918b819b6f5f]::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
  15:     0x7f3d62671c13 - <rustc_infer[fef6918b819b6f5f]::infer::type_variable::TypeVariableTable>::root_var
  16:     0x7f3d6260959f - <rustc_infer[fef6918b819b6f5f]::infer::InferCtxt>::root_var
  17:     0x7f3d60832925 - <rustc_middle[f0ae04c7979ecf21]::ty::fold::BottomUpFolder<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#2}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#0}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#1}> as rustc_middle[f0ae04c7979ecf21]::ty::fold::TypeFolder>::fold_ty
  18:     0x7f3d607f0aa5 - <rustc_middle[f0ae04c7979ecf21]::ty::Ty as rustc_middle[f0ae04c7979ecf21]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[f0ae04c7979ecf21]::ty::fold::BottomUpFolder<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#2}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#0}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#1}>>
  19:     0x7f3d608328e3 - <rustc_middle[f0ae04c7979ecf21]::ty::fold::BottomUpFolder<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#2}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#0}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#1}> as rustc_middle[f0ae04c7979ecf21]::ty::fold::TypeFolder>::fold_ty
  20:     0x7f3d607f0af8 - <rustc_middle[f0ae04c7979ecf21]::ty::Ty as rustc_middle[f0ae04c7979ecf21]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle[f0ae04c7979ecf21]::ty::fold::BottomUpFolder<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#2}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#0}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#1}>>
  21:     0x7f3d608328e3 - <rustc_middle[f0ae04c7979ecf21]::ty::fold::BottomUpFolder<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#2}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#0}, <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type::{closure#1}> as rustc_middle[f0ae04c7979ecf21]::ty::fold::TypeFolder>::fold_ty
  22:     0x7f3d60860809 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::point_at_expr_source_of_inferred_type
  23:     0x7f3d6087d148 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::report_arg_errors
  24:     0x7f3d60879cca - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_argument_types
  25:     0x7f3d60878691 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_method_argument_types
  26:     0x7f3d608bc140 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7f3d60866f1b - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7f3d608b7551 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7f3d6087f83d - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_stmt
  30:     0x7f3d6099dd0e - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
  31:     0x7f3d6087fa35 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_block_with_expected
  32:     0x7f3d608b880f - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7f3d60866f1b - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f3d608b7551 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7f3d608691d6 - <rustc_hir_typeck[f5d898eb3fc23593]::fn_ctxt::FnCtxt>::check_return_expr
  36:     0x7f3d60957c54 - rustc_hir_typeck[f5d898eb3fc23593]::check::check_fn
  37:     0x7f3d609d4d53 - <rustc_hir_typeck[f5d898eb3fc23593]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[f5d898eb3fc23593]::typeck_with_fallback<rustc_hir_typeck[f5d898eb3fc23593]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[f0ae04c7979ecf21]::ty::typeck_results::TypeckResults>
  38:     0x7f3d609e34aa - rustc_hir_typeck[f5d898eb3fc23593]::typeck_with_fallback::<rustc_hir_typeck[f5d898eb3fc23593]::typeck::{closure#0}>
  39:     0x7f3d60968054 - rustc_hir_typeck[f5d898eb3fc23593]::typeck
  40:     0x7f3d61b44144 - rustc_query_system[a32c053b8c20dd29]::query::plumbing::try_execute_query::<rustc_query_impl[875778a81ca0ae56]::queries::typeck, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt>
  41:     0x7f3d61bde910 - rustc_query_system[a32c053b8c20dd29]::query::plumbing::get_query::<rustc_query_impl[875778a81ca0ae56]::queries::typeck, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt, rustc_middle[f0ae04c7979ecf21]::dep_graph::dep_node::DepKind>
  42:     0x7f3d61856b28 - <rustc_query_impl[875778a81ca0ae56]::Queries as rustc_middle[f0ae04c7979ecf21]::ty::query::QueryEngine>::typeck
  43:     0x7f3d609bf706 - std[5dbba46818124626]::panicking::try::<(), core[1dc6fc0132f421c8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[dde98930a5d5f21]::sync::par_for_each_in<&[rustc_span[eeeb61687e933d22]::def_id::LocalDefId], <rustc_middle[f0ae04c7979ecf21]::hir::map::Map>::par_body_owners<rustc_hir_typeck[f5d898eb3fc23593]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  44:     0x7f3d609d8545 - rustc_data_structures[dde98930a5d5f21]::sync::par_for_each_in::<&[rustc_span[eeeb61687e933d22]::def_id::LocalDefId], <rustc_middle[f0ae04c7979ecf21]::hir::map::Map>::par_body_owners<rustc_hir_typeck[f5d898eb3fc23593]::typeck_item_bodies::{closure#0}>::{closure#0}>
  45:     0x7f3d60967f15 - rustc_hir_typeck[f5d898eb3fc23593]::typeck_item_bodies
  46:     0x7f3d61b0270a - rustc_query_system[a32c053b8c20dd29]::query::plumbing::try_execute_query::<rustc_query_impl[875778a81ca0ae56]::queries::typeck_item_bodies, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt>
  47:     0x7f3d61bb124f - rustc_query_system[a32c053b8c20dd29]::query::plumbing::get_query::<rustc_query_impl[875778a81ca0ae56]::queries::typeck_item_bodies, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt, rustc_middle[f0ae04c7979ecf21]::dep_graph::dep_node::DepKind>
  48:     0x7f3d618565ee - <rustc_query_impl[875778a81ca0ae56]::Queries as rustc_middle[f0ae04c7979ecf21]::ty::query::QueryEngine>::typeck_item_bodies
  49:     0x7f3d60a46e08 - <rustc_session[b9e04facfa5b9c76]::session::Session>::time::<(), rustc_hir_analysis[f696bd7baabdc5a6]::check_crate::{closure#7}>
  50:     0x7f3d60a8fe0e - rustc_hir_analysis[f696bd7baabdc5a6]::check_crate
  51:     0x7f3d6054e261 - rustc_interface[8ba98d2de33f2af1]::passes::analysis
  52:     0x7f3d61b4621a - rustc_query_system[a32c053b8c20dd29]::query::plumbing::try_execute_query::<rustc_query_impl[875778a81ca0ae56]::queries::analysis, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt>
  53:     0x7f3d61bdeba1 - rustc_query_system[a32c053b8c20dd29]::query::plumbing::get_query::<rustc_query_impl[875778a81ca0ae56]::queries::analysis, rustc_query_impl[875778a81ca0ae56]::plumbing::QueryCtxt, rustc_middle[f0ae04c7979ecf21]::dep_graph::dep_node::DepKind>
  54:     0x7f3d61838d4e - <rustc_query_impl[875778a81ca0ae56]::Queries as rustc_middle[f0ae04c7979ecf21]::ty::query::QueryEngine>::analysis
  55:     0x7f3d604581f5 - <rustc_interface[8ba98d2de33f2af1]::passes::QueryContext>::enter::<rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>
  56:     0x7f3d604d60c4 - <rustc_interface[8ba98d2de33f2af1]::queries::QueryResult<rustc_interface[8ba98d2de33f2af1]::passes::QueryContext>>::enter::<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  57:     0x7f3d604ca0a2 - <rustc_interface[8ba98d2de33f2af1]::interface::Compiler>::enter::<rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}::{closure#2}, core[1dc6fc0132f421c8]::result::Result<core[1dc6fc0132f421c8]::option::Option<rustc_interface[8ba98d2de33f2af1]::queries::Linker>, rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>
  58:     0x7f3d604cbc66 - rustc_span[eeeb61687e933d22]::with_source_map::<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_interface[8ba98d2de33f2af1]::interface::run_compiler<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  59:     0x7f3d604c235c - <scoped_tls[b798c2eb5b825363]::ScopedKey<rustc_span[eeeb61687e933d22]::SessionGlobals>>::set::<rustc_interface[8ba98d2de33f2af1]::interface::run_compiler<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}>::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>
  60:     0x7f3d604da03a - std[5dbba46818124626]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8ba98d2de33f2af1]::util::run_in_thread_pool_with_globals<rustc_interface[8ba98d2de33f2af1]::interface::run_compiler<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}>::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>
  61:     0x7f3d60471a2e - std[5dbba46818124626]::panicking::try::<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, core[1dc6fc0132f421c8]::panic::unwind_safe::AssertUnwindSafe<<std[5dbba46818124626]::thread::Builder>::spawn_unchecked_<rustc_interface[8ba98d2de33f2af1]::util::run_in_thread_pool_with_globals<rustc_interface[8ba98d2de33f2af1]::interface::run_compiler<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}>::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  62:     0x7f3d60459b74 - <<std[5dbba46818124626]::thread::Builder>::spawn_unchecked_<rustc_interface[8ba98d2de33f2af1]::util::run_in_thread_pool_with_globals<rustc_interface[8ba98d2de33f2af1]::interface::run_compiler<core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>, rustc_driver[56eea7eb949f9e3d]::run_compiler::{closure#1}>::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1dc6fc0132f421c8]::result::Result<(), rustc_errors[d60f5c61dce660fd]::ErrorGuaranteed>>::{closure#1} as core[1dc6fc0132f421c8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  63:     0x7f3d5fb1b408 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0a76c2e8d769b425
  64:     0x7f3d5fade8a7 - std::sys::unix::thread::Thread::new::thread_start::h23400d1ce2405b9a
  65:     0x7f3d5f90512d - start_thread
  66:     0x7f3d5f986bc0 - clone3
  67:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-beta.6 (467a9ce9e 2023-03-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: no errors reported for args
  --> fake-test-src-base/typeck/bad-type-in-vec-contains.rs:5:12
   |
LL |     primes.contains(3);
   |            ^^^^^^^^
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
              2: <rustc_hir_typeck::fn_ctxt::FnCtxt>::report_arg_errors
              3: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_argument_types
              4: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_method_argument_types
              5: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_kind
              6: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
              7: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation
              8: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_stmt
              9: <rustc_hir_typeck::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_hir_typeck::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
             10: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_block_with_expected
             11: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_kind
             12: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
             13: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation
             14: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_return_expr
             15: rustc_hir_typeck::check::check_fn
             16: <rustc_hir_typeck::inherited::InheritedBuilder>::enter::<rustc_hir_typeck::typeck_with_fallback<rustc_hir_typeck::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle::ty::typeck_results::TypeckResults>
             17: rustc_hir_typeck::typeck_with_fallback::<rustc_hir_typeck::typeck::{closure#0}>
             18: rustc_hir_typeck::typeck
             19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
             22: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_hir_typeck::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             23: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_hir_typeck::typeck_item_bodies::{closure#0}>::{closure#0}>
             24: rustc_hir_typeck::typeck_item_bodies
             25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
             26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies
             28: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#7}>
             29: rustc_hir_analysis::check_crate
             30: rustc_interface::passes::analysis
             31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             32: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             34: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             35: <rustc_interface::queries::QueryResult<rustc_interface::passes::QueryContext>>::enter::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}>
             36: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             37: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             38: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             39: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             40: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             41: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             42: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             43: std::sys::unix::thread::Thread::new::thread_start
             44: start_thread
             45: clone3


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-beta.6 (467a9ce9e 2023-03-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------

