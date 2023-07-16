plain
[RUSTC-TIMING] build_script_build test:false 0.127
[RUSTC-TIMING] build_script_build test:false 0.276
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs:1939:21
stack backtrace:
   0:     0x7fbe1dd79513 - std::backtrace_rs::backtrace::libunwind::trace::hd081781edc49e891
   1:     0x7fbe1dd79513 - std::backtrace_rs::backtrace::trace_unsynchronized::h3ce4ced55392c1ac
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fbe1dd79513 - std::sys_common::backtrace::_print_fmt::h0a0bbfebb5f69998
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:65:5
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fbe1dd79513 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb8c959a5e826396f
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fbe1ddda97f - core::fmt::rt::Argument::fmt::hb007c02558a7e048
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/core/src/fmt/rt.rs:138:9
   5:     0x7fbe1ddda97f - core::fmt::write::h889e7642f4b05be0
   6:     0x7fbe1dd6c1c5 - std::io::Write::write_fmt::h59c26e1d07aa7c7b
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/io/mod.rs:1712:15
   7:     0x7fbe1dd79325 - std::sys_common::backtrace::_print::h03e71c0958a9bbe2
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:47:5
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7fbe1dd79325 - std::sys_common::backtrace::print::hcfe7fe22b3c44705
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7fbe1dd7bf5e - std::panicking::default_hook::{{closure}}::h4b7c5ffbc284f739
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:269:22
  10:     0x7fbe1dd7bd05 - std::panicking::default_hook::h900f3f29125ad1e7
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:288:9
  11:     0x7fbe1b042e75 - <rustc_driver_impl[fe5952c18557c769]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[51e91c5fbe619c17]::ops::function::FnOnce<(&core[51e91c5fbe619c17]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  12:     0x7fbe1dd7c754 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h087d68bf78533a1d
  13:     0x7fbe1dd7c754 - std::panicking::rust_panic_with_hook::h6d50a10012f6e0c1
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:695:13
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:695:13
  14:     0x7fbe1dd7c4c9 - std::panicking::begin_panic_handler::{{closure}}::hb3150934e3440b92
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:582:13
  15:     0x7fbe1dd79956 - std::sys_common::backtrace::__rust_end_short_backtrace::h064f67b3cecabc0b
  16:     0x7fbe1dd7c222 - rust_begin_unwind
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/panicking.rs:578:5
  17:     0x7fbe1ddd6c23 - core::panicking::panic_fmt::h41ad0bd809fc2fed
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/core/src/panicking.rs:67:14
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/core/src/panicking.rs:67:14
  18:     0x7fbe1ddd6d92 - core::panicking::panic_bounds_check::hdca34d7fac69b4bb
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/core/src/panicking.rs:162:5
  19:     0x7fbe1b5f01ad - <rustc_middle[656b4ffd0288b9c0]::ty::Ty as rustc_type_ir[76aad0907a1d994e]::visit::TypeSuperVisitable<rustc_middle[656b4ffd0288b9c0]::ty::context::TyCtxt>>::super_visit_with::<rustc_hir_analysis[1f5b656e3510fddd]::collect::resolve_bound_vars::is_late_bound_map::ConstrainedCollectorPostAstConv>
  20:     0x7fbe1b6df731 - <rustc_hir_analysis[1f5b656e3510fddd]::collect::resolve_bound_vars::is_late_bound_map::ConstrainedCollectorPostAstConv as rustc_type_ir[76aad0907a1d994e]::visit::TypeVisitor<rustc_middle[656b4ffd0288b9c0]::ty::context::TyCtxt>>::visit_ty
  21:     0x7fbe1b6df88d - <rustc_hir_analysis[1f5b656e3510fddd]::collect::resolve_bound_vars::is_late_bound_map::ConstrainedCollector as rustc_hir[79964dc31275128e]::intravisit::Visitor>::visit_ty
  22:     0x7fbe1b6df172 - rustc_hir_analysis[1f5b656e3510fddd]::collect::resolve_bound_vars::is_late_bound_map
  23:     0x7fbe1c6bfd5a - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::is_late_bound_map, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  24:     0x7fbe1c48a4a1 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::is_late_bound_map
  25:     0x7fbe1d441d2f - <rustc_middle[656b4ffd0288b9c0]::ty::context::TyCtxt>::is_late_bound
  26:     0x7fbe1b63fcfe - <alloc[30b2db67739fbdd3]::vec::Vec<rustc_middle[656b4ffd0288b9c0]::ty::generics::GenericParamDef> as alloc[30b2db67739fbdd3]::vec::spec_extend::SpecExtend<rustc_middle[656b4ffd0288b9c0]::ty::generics::GenericParamDef, core[51e91c5fbe619c17]::iter::adapters::map::Map<core[51e91c5fbe619c17]::iter::adapters::enumerate::Enumerate<core[51e91c5fbe619c17]::iter::adapters::filter::Filter<core[51e91c5fbe619c17]::slice::iter::Iter<rustc_hir[79964dc31275128e]::hir::GenericParam>, rustc_hir_analysis[1f5b656e3510fddd]::collect::early_bound_lifetimes_from_generics::{closure#0}>>, rustc_hir_analysis[1f5b656e3510fddd]::collect::generics_of::generics_of::{closure#3}>>>::spec_extend
  27:     0x7fbe1b6a8b0d - rustc_hir_analysis[1f5b656e3510fddd]::collect::generics_of::generics_of
  28:     0x7fbe1c57297f - <std[814b7871d468b4a6]::thread::local::LocalKey<core[51e91c5fbe619c17]::cell::Cell<*const ()>>>::with::<rustc_middle[656b4ffd0288b9c0]::ty::context::tls::enter_context<rustc_query_system[6a5038b4564cd35]::query::plumbing::execute_job_non_incr<rustc_query_impl[2e42aa2eedb1dddf]::queries::generics_of, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>
  29:     0x7fbe1c68b95c - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::generics_of, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  30:     0x7fbe1c4730cb - rustc_query_impl[2e42aa2eedb1dddf]::get_query::generics_of
  31:     0x7fbe1b6101cf - rustc_middle[656b4ffd0288b9c0]::ty::query::query_get_at::<rustc_query_system[6a5038b4564cd35]::query::caches::DefaultCache<rustc_span[b12595b0d901fa0e]::def_id::DefId, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>>
  32:     0x7fbe1b636b75 - rustc_hir_analysis[1f5b656e3510fddd]::variance::terms::determine_parameters_to_be_inferred
  33:     0x7fbe1b7094dc - rustc_hir_analysis[1f5b656e3510fddd]::variance::crate_variances
  34:     0x7fbe1c574da8 - <std[814b7871d468b4a6]::thread::local::LocalKey<core[51e91c5fbe619c17]::cell::Cell<*const ()>>>::with::<rustc_middle[656b4ffd0288b9c0]::ty::context::tls::enter_context<rustc_query_system[6a5038b4564cd35]::query::plumbing::execute_job_non_incr<rustc_query_impl[2e42aa2eedb1dddf]::queries::crate_variances, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>
  35:     0x7fbe1c6a8262 - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::crate_variances, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  36:     0x7fbe1c4795e6 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::crate_variances
  37:     0x7fbe1b7096fc - rustc_hir_analysis[1f5b656e3510fddd]::variance::variances_of
  38:     0x7fbe1c697de7 - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::variances_of, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  39:     0x7fbe1c4797ce - rustc_query_impl[2e42aa2eedb1dddf]::get_query::variances_of
  40:     0x7fbe1d361ad6 - rustc_middle[656b4ffd0288b9c0]::ty::query::query_get_at::<rustc_query_system[6a5038b4564cd35]::query::caches::DefaultCache<rustc_span[b12595b0d901fa0e]::def_id::DefId, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 16usize]>>>
  41:     0x7fbe1d367579 - <rustc_infer[eca2343a41c467cc]::infer::combine::Generalizer as rustc_middle[656b4ffd0288b9c0]::ty::relate::TypeRelation>::relate_item_substs
  42:     0x7fbe1d3e485a - rustc_middle[656b4ffd0288b9c0]::ty::relate::super_relate_tys::<rustc_infer[eca2343a41c467cc]::infer::combine::Generalizer>
  43:     0x7fbe1d367b47 - <rustc_infer[eca2343a41c467cc]::infer::combine::Generalizer as rustc_middle[656b4ffd0288b9c0]::ty::relate::TypeRelation>::tys
  44:     0x7fbe1d37053d - <rustc_infer[eca2343a41c467cc]::infer::combine::CombineFields>::instantiate
  45:     0x7fbe1d35d9a5 - <rustc_infer[eca2343a41c467cc]::infer::sub::Sub as rustc_middle[656b4ffd0288b9c0]::ty::relate::TypeRelation>::tys
  46:     0x7fbe1b460741 - <rustc_infer[eca2343a41c467cc]::infer::InferCtxt>::commit_if_ok::<rustc_infer[eca2343a41c467cc]::infer::InferOk<()>, rustc_middle[656b4ffd0288b9c0]::ty::error::TypeError, <rustc_infer[eca2343a41c467cc]::infer::at::Trace>::sub<rustc_middle[656b4ffd0288b9c0]::ty::Ty>::{closure#0}>
  47:     0x7fbe1b45aa59 - <rustc_infer[eca2343a41c467cc]::infer::InferCtxt>::fudge_inference_if_ok::<core[51e91c5fbe619c17]::option::Option<alloc[30b2db67739fbdd3]::vec::Vec<rustc_middle[656b4ffd0288b9c0]::ty::Ty>>, rustc_middle[656b4ffd0288b9c0]::ty::error::TypeError, <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::expected_inputs_for_expected_output::{closure#0}>
  48:     0x7fbe1b435b07 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::expected_inputs_for_expected_output
  49:     0x7fbe1b3c1351 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::confirm_builtin_call
  50:     0x7fbe1b3be207 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_call
  51:     0x7fbe1b42f5bb - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_kind
  52:     0x7fbe1b3d2fa4 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  53:     0x7fbe1b3d43a0 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_return_expr
  54:     0x7fbe1b4e5999 - rustc_hir_typeck[676e8a2b8f0c9bcb]::check::check_fn
  55:     0x7fbe1b42d1cd - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_closure
  56:     0x7fbe1b42f5e0 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_kind
  57:     0x7fbe1b3d2fa4 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  58:     0x7fbe1b3f1a5f - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_block_with_expected
  59:     0x7fbe1b42f1eb - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_kind
  60:     0x7fbe1b3d2fa4 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  61:     0x7fbe1b3d43a0 - <rustc_hir_typeck[676e8a2b8f0c9bcb]::fn_ctxt::FnCtxt>::check_return_expr
  62:     0x7fbe1b4e5999 - rustc_hir_typeck[676e8a2b8f0c9bcb]::check::check_fn
  63:     0x7fbe1b49d1b2 - rustc_hir_typeck[676e8a2b8f0c9bcb]::typeck
  64:     0x7fbe1c71571a - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::typeck, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  65:     0x7fbe1c47d341 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::typeck
  66:     0x7fbe1bf5e4c7 - rustc_mir_build[deb97b10ccb4b0ef]::thir::cx::thir_body
  67:     0x7fbe1c71f3fe - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::thir_body, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  68:     0x7fbe1c4749a4 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::thir_body
  69:     0x7fbe1bfce3b3 - rustc_mir_build[deb97b10ccb4b0ef]::thir::pattern::check_match::check_match
  70:     0x7fbe1c68b0ea - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::check_match, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  71:     0x7fbe1c480001 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::check_match
  72:     0x7fbe1bee24fc - rustc_mir_build[deb97b10ccb4b0ef]::build::mir_built
  73:     0x7fbe1c71d1da - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::mir_built, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  74:     0x7fbe1c475331 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::mir_built
  75:     0x7fbe1b85f4b6 - rustc_mir_transform[dc0dc56283c2edb3]::check_unsafety::unsafety_check_result
  76:     0x7fbe1c6e283a - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::unsafety_check_result, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  77:     0x7fbe1c47b1e1 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::unsafety_check_result
  78:     0x7fbe1b8f2034 - rustc_mir_transform[dc0dc56283c2edb3]::mir_const
  79:     0x7fbe1c71d9fa - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::mir_const, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  80:     0x7fbe1c475511 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::mir_const
  81:     0x7fbe1b8f254e - rustc_mir_transform[dc0dc56283c2edb3]::mir_promoted
  82:     0x7fbe1c69610e - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::mir_promoted, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  83:     0x7fbe1c475cd4 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::mir_promoted
  84:     0x7fbe1c01f549 - rustc_borrowck[a7d7b4539086846a]::mir_borrowck
  85:     0x7fbe1c69505a - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::mir_borrowck, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  86:     0x7fbe1c47dc91 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::mir_borrowck
  87:     0x7fbe1b73e7f7 - rustc_hir_analysis[1f5b656e3510fddd]::collect::type_of::find_opaque_ty_constraints_for_rpit
  88:     0x7fbe1b73d6f8 - rustc_hir_analysis[1f5b656e3510fddd]::collect::type_of::type_of
  89:     0x7fbe1c7169bc - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::type_of, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
  90:     0x7fbe1c47259b - rustc_query_impl[2e42aa2eedb1dddf]::get_query::type_of
  91:     0x7fbe1b5b48a8 - rustc_middle[656b4ffd0288b9c0]::ty::query::query_get_at::<rustc_query_system[6a5038b4564cd35]::query::caches::DefaultCache<rustc_span[b12595b0d901fa0e]::def_id::DefId, rustc_middle[656b4ffd0288b9c0]::query::erase::Erased<[u8; 8usize]>>>
  92:     0x7fbe1b5be128 - <rustc_privacy[67b53a90fa3e8944]::ReachEverythingInTheInterfaceVisitor>::ty
  93:     0x7fbe1b5bd4c3 - <rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor as rustc_hir[79964dc31275128e]::intravisit::Visitor>::visit_item
  94:     0x7fbe1b5cbf63 - rustc_hir[79964dc31275128e]::intravisit::walk_ty::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
  95:     0x7fbe1b5cb94a - rustc_hir[79964dc31275128e]::intravisit::walk_fn::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
  96:     0x7fbe1b5c6513 - rustc_hir[79964dc31275128e]::intravisit::walk_impl_item::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
  97:     0x7fbe1b5d0a8d - rustc_hir[79964dc31275128e]::intravisit::walk_item::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
  98:     0x7fbe1b5bdba4 - <rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor as rustc_hir[79964dc31275128e]::intravisit::Visitor>::visit_item
  99:     0x7fbe1b5d06da - rustc_hir[79964dc31275128e]::intravisit::walk_item::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
 100:     0x7fbe1b5bdba4 - <rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor as rustc_hir[79964dc31275128e]::intravisit::Visitor>::visit_item
 101:     0x7fbe1b5d06da - rustc_hir[79964dc31275128e]::intravisit::walk_item::<rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor>
 102:     0x7fbe1b5bdba4 - <rustc_privacy[67b53a90fa3e8944]::EmbargoVisitor as rustc_hir[79964dc31275128e]::intravisit::Visitor>::visit_item
 103:     0x7fbe1b5c24cf - rustc_privacy[67b53a90fa3e8944]::effective_visibilities
 104:     0x7fbe1c6e5c1f - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::effective_visibilities, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
 105:     0x7fbe1c4801b6 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::effective_visibilities
 106:     0x7fbe1be77cfd - rustc_passes[5be7ee49f713a8d2]::stability::check_unused_or_stable_features
 107:     0x7fbe1b0c1747 - <rustc_session[433d7c5b5c3aada6]::session::Session>::time::<(), rustc_interface[ff94db832d6922ac]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
 108:     0x7fbe1b0c1d86 - <rustc_session[433d7c5b5c3aada6]::session::Session>::time::<(), rustc_interface[ff94db832d6922ac]::passes::analysis::{closure#0}>
 109:     0x7fbe1b087e36 - rustc_interface[ff94db832d6922ac]::passes::analysis
 110:     0x7fbe1c717053 - rustc_query_system[6a5038b4564cd35]::query::plumbing::try_execute_query::<rustc_query_impl[2e42aa2eedb1dddf]::queries::analysis, rustc_query_impl[2e42aa2eedb1dddf]::plumbing::QueryCtxt>
 111:     0x7fbe1c472d37 - rustc_query_impl[2e42aa2eedb1dddf]::get_query::analysis
 112:     0x7fbe1b055143 - <std[814b7871d468b4a6]::thread::local::LocalKey<core[51e91c5fbe619c17]::cell::Cell<*const ()>>>::with::<rustc_middle[656b4ffd0288b9c0]::ty::context::tls::enter_context<<rustc_middle[656b4ffd0288b9c0]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[fe5952c18557c769]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>
 113:     0x7fbe1afc7b41 - <rustc_interface[ff94db832d6922ac]::queries::QueryResult<&rustc_middle[656b4ffd0288b9c0]::ty::context::GlobalCtxt>>::enter::<core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>, rustc_driver_impl[fe5952c18557c769]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
 114:     0x7fbe1afd5f5f - <rustc_interface[ff94db832d6922ac]::interface::Compiler>::enter::<rustc_driver_impl[fe5952c18557c769]::run_compiler::{closure#1}::{closure#2}, core[51e91c5fbe619c17]::result::Result<core[51e91c5fbe619c17]::option::Option<rustc_interface[ff94db832d6922ac]::queries::Linker>, rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>
 115:     0x7fbe1afcf6c9 - std[814b7871d468b4a6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ff94db832d6922ac]::util::run_in_thread_pool_with_globals<rustc_interface[ff94db832d6922ac]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>, rustc_driver_impl[fe5952c18557c769]::run_compiler::{closure#1}>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>
 116:     0x7fbe1afd13ad - <<std[814b7871d468b4a6]::thread::Builder>::spawn_unchecked_<rustc_interface[ff94db832d6922ac]::util::run_in_thread_pool_with_globals<rustc_interface[ff94db832d6922ac]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>, rustc_driver_impl[fe5952c18557c769]::run_compiler::{closure#1}>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[b12595b0d901fa0e]::ErrorGuaranteed>>::{closure#1} as core[51e91c5fbe619c17]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 117:     0x7fbe1dd86cb5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha9401e9fd2712839
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/alloc/src/boxed.rs:1985:9
 118:     0x7fbe1dd86cb5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4cacc8d96aadda9a
 119:     0x7fbe1dd86cb5 - std::sys::unix::thread::Thread::new::thread_start::h45c0fe595737d0da
                               at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys/unix/thread.rs:108:17
 120:     0x7fbe19876ea5 - start_thread
 121:     0x7fbe1959fb0d - clone
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (59c53d269 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -C prefer-dynamic -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [is_late_bound_map] testing if a region is late bound
#1 [generics_of] computing generics of `fmt::rt::<impl at library/core/src/fmt/rt.rs:81:1: 81:22>::new`
#2 [crate_variances] computing the variances for items in this crate
#3 [variances_of] computing the variances of `ops::try_trait::NeverShortCircuit`
#4 [typeck] type-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#5 [thir_body] building THIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#6 [check_match] match-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#7 [mir_built] building MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#8 [unsafety_check_result] unsafety-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#9 [mir_const] preparing `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1` for borrow checking
#10 [mir_promoted] promoting constants in MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#11 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1`
#12 [type_of] computing type of `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_1::{opaque#1}`
#13 [effective_visibilities] checking effective visibilities
#14 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: {OpaqueTypeKey { def_id: DefId(0:51122 ~ core[ee1c]::ops::try_trait::{impl#0}::wrap_mut_1::{opaque#1}), substs: [T, A, impl FnMut(A) -> T] }: OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/ops/try_trait.rs:384:56: 384:93 (#29442), ty: _ }, origin: FnReturn(DefId(0:3186 ~ core[ee1c]::ops::try_trait::{impl#0}::wrap_mut_1)) }}
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
             1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
             2: <rustc_infer::infer::opaque_types::table::OpaqueTypeStorage as core::ops::drop::Drop>::drop
             3: core::ptr::drop_in_place::<rustc_hir_typeck::inherited::Inherited>
             3: core::ptr::drop_in_place::<rustc_hir_typeck::inherited::Inherited>
             4: rustc_hir_typeck::typeck
             5: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             6: rustc_query_impl::get_query::typeck
             7: rustc_mir_build::thir::cx::thir_body
             8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::thir_body, rustc_query_impl::plumbing::QueryCtxt>
             9: rustc_query_impl::get_query::thir_body
            10: rustc_mir_build::thir::pattern::check_match::check_match
            11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_match, rustc_query_impl::plumbing::QueryCtxt>
            12: rustc_query_impl::get_query::check_match
            14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_built, rustc_query_impl::plumbing::QueryCtxt>
            15: rustc_query_impl::get_query::mir_built
            16: rustc_mir_transform::check_unsafety::unsafety_check_result
            17: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
---
            23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_promoted, rustc_query_impl::plumbing::QueryCtxt>
            24: rustc_query_impl::get_query::mir_promoted
            25: rustc_borrowck::mir_borrowck
            26: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_borrowck, rustc_query_impl::plumbing::QueryCtxt>
            27: rustc_query_impl::get_query::mir_borrowck
            28: rustc_hir_analysis::collect::type_of::find_opaque_ty_constraints_for_rpit
            29: rustc_hir_analysis::collect::type_of::type_of
            30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
            31: rustc_query_impl::get_query::type_of
            32: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            33: <rustc_privacy::ReachEverythingInTheInterfaceVisitor>::ty
            34: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            35: rustc_hir::intravisit::walk_ty::<rustc_privacy::EmbargoVisitor>
            36: rustc_hir::intravisit::walk_fn::<rustc_privacy::EmbargoVisitor>
            37: rustc_hir::intravisit::walk_impl_item::<rustc_privacy::EmbargoVisitor>
            38: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            39: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            40: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            41: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            42: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            43: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            44: rustc_privacy::effective_visibilities
            45: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::effective_visibilities, rustc_query_impl::plumbing::QueryCtxt>
            46: rustc_query_impl::get_query::effective_visibilities
            48: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
            49: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}>
            50: rustc_interface::passes::analysis
            51: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            51: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            52: rustc_query_impl::get_query::analysis
            53: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            54: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
            55: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            56: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            57: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            58: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/alloc/src/boxed.rs:1985:9
            59: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
            60: std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/59c53d26935d5456e5382c154e036003b451af12/library/std/src/sys/unix/thread.rs:108:17
            61: start_thread
            62: clone
            62: clone
          

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (59c53d269 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -C prefer-dynamic -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
[RUSTC-TIMING] core test:false 3.359
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=05898138a596088a -C extra-filename=-05898138a596088a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Csplit-debuginfo=off -Cprefer-dynamic -Zinline-mir -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:11:43
stage-build INFO: Section `Stage 1 (LLVM PGO) > Build rustc and LLVM` ended: FAIL (704.03s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (704.04s)
stage-build ERROR: The multi-stage build has failed
---
Total duration:                          11m 44s
------------------------------------------------
root INFO: Free disk space: 510.74 GiB out of total 581.32 GiB (12.14% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
