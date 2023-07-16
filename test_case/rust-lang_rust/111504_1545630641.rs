plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/library/core/src/ops/function.rs:250:5
   0:     0x7fc583db30c1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8568fb9cf73ffe61
   1:     0x7fc583e1cb78 - core::fmt::write::h01a374123323c4c4
   2:     0x7fc583da7491 - std::io::Write::write_fmt::h9cf94f7ee0f74589
   3:     0x7fc583db2ed5 - std::sys_common::backtrace::print::hf66447743851f4fa
   3:     0x7fc583db2ed5 - std::sys_common::backtrace::print::hf66447743851f4fa
   4:     0x7fc583db6294 - std::panicking::default_hook::{{closure}}::hcce65c608141da85
   5:     0x7fc583db6048 - std::panicking::default_hook::hce2c3355d454fac7
   6:     0x7fc58482d18c - rustc_driver_impl[51ae620fdc395faf]::install_ice_hook::{closure#0}
   7:     0x7fc583db6a8d - std::panicking::rust_panic_with_hook::h95c29c29d0102c3f
   8:     0x7fc583db67c9 - std::panicking::begin_panic_handler::{{closure}}::h6fd2a6ff759266a6
   9:     0x7fc583db3586 - std::sys_common::backtrace::__rust_end_short_backtrace::hd45237ff9d0293c5
  10:     0x7fc583db6482 - rust_begin_unwind
  11:     0x7fc583d6de73 - core::panicking::panic_fmt::hba5dfd24d0a29bf3
  12:     0x7fc583d6e463 - core::result::unwrap_failed::hcc821d1e1d0855c6
  13:     0x7fc584d612bd - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  14:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  15:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  16:     0x7fc584d17fd7 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_stmt
  17:     0x7fc584d18534 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_block_with_expected
  18:     0x7fc584d5b97b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  19:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  20:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  21:     0x7fc584d5e067 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  22:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  24:     0x7fc584d17e9d - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_stmt
  25:     0x7fc584d18534 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_block_with_expected
  26:     0x7fc584d5b97b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7fc584cf6258 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_return_expr
  30:     0x7fc584f06eca - rustc_hir_typeck[9fee3b115ea4ded6]::check::check_fn
  31:     0x7fc584d55294 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_closure
  32:     0x7fc584d5bd8a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7fc584d1858b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_block_with_expected
  36:     0x7fc584d5b97b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  37:     0x7fc584cf493a - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7fc584d5aa52 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  39:     0x7fc584cf6258 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_return_expr
  40:     0x7fc584f06eca - rustc_hir_typeck[9fee3b115ea4ded6]::check::check_fn
  41:     0x7fc584e1077f - rustc_hir_typeck[9fee3b115ea4ded6]::typeck
  42:     0x7fc5862c9b4d - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::typeck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  43:     0x7fc5865cf877 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::typeck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  44:     0x7fc586434afb - rustc_query_impl[dccb39fc5ff2a194]::get_query::typeck
  45:     0x7fc585b20d42 - rustc_mir_build[9428368b6273a128]::thir::cx::thir_body
  46:     0x7fc5862caa71 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::thir_body, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>
  47:     0x7fc5865db1ac - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::thir_body, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  48:     0x7fc58641a05b - rustc_query_impl[dccb39fc5ff2a194]::get_query::thir_body
  49:     0x7fc585b687c2 - rustc_mir_build[9428368b6273a128]::thir::pattern::check_match::check_match
  50:     0x7fc5862bc281 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::check_match, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>
  51:     0x7fc5865221b8 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::check_match, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  52:     0x7fc58643cc7d - rustc_query_impl[dccb39fc5ff2a194]::get_query::check_match
  53:     0x7fc585a55118 - rustc_mir_build[9428368b6273a128]::build::mir_build
  54:     0x7fc585a54edd - rustc_mir_build[9428368b6273a128]::build::mir_built
  55:     0x7fc5862ca70d - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_built, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  56:     0x7fc5865d89c7 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_built, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  57:     0x7fc58641be5b - rustc_query_impl[dccb39fc5ff2a194]::get_query::mir_built
  58:     0x7fc58529fd4a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  59:     0x7fc5852ab77b - rustc_mir_transform[dec1380c0a37062f]::check_unsafety::unsafety_check_result
  60:     0x7fc5862c4c6d - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::unsafety_check_result, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  61:     0x7fc58658dd77 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::unsafety_check_result, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  62:     0x7fc58642dbab - rustc_query_impl[dccb39fc5ff2a194]::get_query::unsafety_check_result
  63:     0x7fc585319171 - rustc_middle[515e59ca886bf23d]::ty::query::query_ensure::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  64:     0x7fc58534a8bc - rustc_mir_transform[dec1380c0a37062f]::mir_const
  65:     0x7fc5862ca7ad - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_const, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  66:     0x7fc5865d933a - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_const, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  67:     0x7fc58641c45b - rustc_query_impl[dccb39fc5ff2a194]::get_query::mir_const
  68:     0x7fc58531992a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  69:     0x7fc58534acd5 - rustc_mir_transform[dec1380c0a37062f]::mir_promoted
  70:     0x7fc5862bd3d3 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_promoted, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>
  71:     0x7fc5865304bc - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_promoted, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  72:     0x7fc58641dc8b - rustc_query_impl[dccb39fc5ff2a194]::get_query::mir_promoted
  73:     0x7fc585bde528 - rustc_borrowck[d09f2adbfa40e690]::mir_borrowck
  74:     0x7fc5862bd27d - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_borrowck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  75:     0x7fc58652f1a7 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::mir_borrowck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  76:     0x7fc5864368db - rustc_query_impl[dccb39fc5ff2a194]::get_query::mir_borrowck
  77:     0x7fc58511a41a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  78:     0x7fc58512b34c - rustc_hir_analysis[7d349000a2ee8374]::collect::type_of::find_opaque_ty_constraints_for_rpit
  79:     0x7fc58512950e - rustc_hir_analysis[7d349000a2ee8374]::collect::type_of::type_of
  80:     0x7fc5862c9cad - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::type_of, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  81:     0x7fc5865d0bd4 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::type_of, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  82:     0x7fc586412ed3 - rustc_query_impl[dccb39fc5ff2a194]::get_query::type_of
  83:     0x7fc584f612f2 - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_span[22ba373f1707d53d]::def_id::DefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  84:     0x7fc584f67f44 - <rustc_privacy[ab924c155f6bf970]::ReachEverythingInTheInterfaceVisitor>::ty
  85:     0x7fc584f667de - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
  86:     0x7fc584f73df3 - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_ty::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  87:     0x7fc584f7371a - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_fn::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  88:     0x7fc584f77ee3 - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  89:     0x7fc584f6753c - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
  90:     0x7fc584f6d75d - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_block::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  91:     0x7fc584f67a08 - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_block
  92:     0x7fc584f768f1 - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_expr::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  93:     0x7fc584f737e2 - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_fn::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  94:     0x7fc584f7073f - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_trait_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  95:     0x7fc584f7826e - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  96:     0x7fc584f6753c - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
  97:     0x7fc584f77f3d - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
  98:     0x7fc584f6753c - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
  99:     0x7fc584f77f3d - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
 100:     0x7fc584f6753c - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
 101:     0x7fc584f77f3d - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_item::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
 102:     0x7fc584f6753c - <rustc_privacy[ab924c155f6bf970]::EmbargoVisitor as rustc_hir[d0eb13f6c3555e76]::intravisit::Visitor>::visit_item
 103:     0x7fc584f74e8a - rustc_hir[d0eb13f6c3555e76]::intravisit::walk_mod::<rustc_privacy[ab924c155f6bf970]::EmbargoVisitor>
 104:     0x7fc584f6c76b - rustc_privacy[ab924c155f6bf970]::effective_visibilities
 105:     0x7fc5862c5146 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::effective_visibilities, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
 106:     0x7fc586591fa6 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::effective_visibilities, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
 107:     0x7fc58643d251 - rustc_query_impl[dccb39fc5ff2a194]::get_query::effective_visibilities
 108:     0x7fc58595f48a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::SingleCache<rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
 109:     0x7fc58597049a - rustc_passes[ea39ca6a0565511e]::stability::check_unused_or_stable_features
 110:     0x7fc5849a4ed7 - <rustc_session[d31b66e86b01534e]::session::Session>::time::<(), rustc_interface[3675127c915af918]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
 111:     0x7fc5848e38c5 - std[4ccf602710e1e200]::panicking::try::<(), core[b17f3ca5674245f1]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3675127c915af918]::passes::analysis::{closure#0}::{closure#2}>>
 112:     0x7fc5849a5523 - <rustc_session[d31b66e86b01534e]::session::Session>::time::<(), rustc_interface[3675127c915af918]::passes::analysis::{closure#0}>
 113:     0x7fc58491016d - rustc_interface[3675127c915af918]::passes::analysis
 114:     0x7fc5862c9d44 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::analysis, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>
 115:     0x7fc5865d155d - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::analysis, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
 116:     0x7fc5864146c4 - rustc_query_impl[dccb39fc5ff2a194]::get_query::analysis
 117:     0x7fc5848498c0 - <rustc_interface[3675127c915af918]::queries::QueryResult<&rustc_middle[515e59ca886bf23d]::ty::context::GlobalCtxt>>::enter::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
 118:     0x7fc58487aeef - <rustc_interface[3675127c915af918]::interface::Compiler>::enter::<rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}::{closure#2}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_interface[3675127c915af918]::queries::Linker>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
 119:     0x7fc5848528bf - std[4ccf602710e1e200]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
 120:     0x7fc584897fc6 - std[4ccf602710e1e200]::panicking::try::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, core[b17f3ca5674245f1]::panic::unwind_safe::AssertUnwindSafe<<std[4ccf602710e1e200]::thread::Builder>::spawn_unchecked_<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 121:     0x7fc584846bb3 - <<std[4ccf602710e1e200]::thread::Builder>::spawn_unchecked_<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#1} as core[b17f3ca5674245f1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 122:     0x7fc583dc3a8e - std::sys::unix::thread::Thread::new::thread_start::hea5ef66f6d4d54ac
 123:     0x7fc583b62b43 - <unknown>
 124:     0x7fc583bf4a00 - <unknown>
 125:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (dbd611866 2023-05-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `iter::traits::iterator::Iterator::is_sorted_by::check`
#1 [thir_body] building THIR for `iter::traits::iterator::Iterator::is_sorted_by::check`
#2 [check_match] match-checking `iter::traits::iterator::Iterator::is_sorted_by::check`
#3 [mir_built] building MIR for `iter::traits::iterator::Iterator::is_sorted_by::check`
#4 [unsafety_check_result] unsafety-checking `iter::traits::iterator::Iterator::is_sorted_by::check`
#5 [mir_const] preparing `iter::traits::iterator::Iterator::is_sorted_by::check` for borrow checking
#6 [mir_promoted] promoting constants in MIR for `iter::traits::iterator::Iterator::is_sorted_by::check`
#7 [mir_borrowck] borrow-checking `iter::traits::iterator::Iterator::is_sorted_by::check`
#8 [type_of] computing type of `iter::traits::iterator::Iterator::is_sorted_by::check::{opaque#1}`
#9 [effective_visibilities] checking effective visibilities
#10 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: {OpaqueTypeKey { def_id: DefId(0:53195 ~ core[9bd4]::iter::traits::iterator::Iterator::is_sorted_by::check::{opaque#1}), substs: [ReEarlyBound(0, 'a), T, impl FnMut(&T, &T) -> Option<Ordering> + 'a, ReEarlyBound(0, 'a)] }: OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/iter/traits/iterator.rs:3903:14: 3903:40 (#30807), ty: _ }, origin: FnReturn(DefId(0:7514 ~ core[9bd4]::iter::traits::iterator::Iterator::is_sorted_by::check)) }}
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
             1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
             2: <rustc_infer::infer::opaque_types::table::OpaqueTypeStorage as core::ops::drop::Drop>::drop
             3: core::ptr::drop_in_place::<rustc_infer::infer::opaque_types::table::OpaqueTypeStorage>
             3: core::ptr::drop_in_place::<rustc_infer::infer::opaque_types::table::OpaqueTypeStorage>
             4: core::ptr::drop_in_place::<rustc_hir_typeck::inherited::Inherited>
             5: rustc_hir_typeck::typeck
             6: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
             8: rustc_query_impl::get_query::typeck
             8: rustc_query_impl::get_query::typeck
             9: rustc_mir_build::thir::cx::thir_body
            10: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::thir_body, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 16]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 16]>>
            11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::thir_body, rustc_query_impl::plumbing::QueryCtxt>
            12: rustc_query_impl::get_query::thir_body
            13: rustc_mir_build::thir::pattern::check_match::check_match
            14: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::check_match, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
            15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_match, rustc_query_impl::plumbing::QueryCtxt>
            16: rustc_query_impl::get_query::check_match
            18: rustc_mir_build::build::mir_built
            18: rustc_mir_build::build::mir_built
            19: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::mir_built, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_built, rustc_query_impl::plumbing::QueryCtxt>
            21: rustc_query_impl::get_query::mir_built
            22: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            23: rustc_mir_transform::check_unsafety::unsafety_check_result
            24: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::unsafety_check_result, rustc_query_impl::plumbing::QueryCtxt>
            26: rustc_query_impl::get_query::unsafety_check_result
            27: rustc_middle::ty::query::query_ensure::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            28: rustc_mir_transform::mir_const
            29: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_const, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            31: rustc_query_impl::get_query::mir_const
            31: rustc_query_impl::get_query::mir_const
            32: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            33: rustc_mir_transform::mir_promoted
            34: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_promoted, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 16]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 16]>>
            35: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_promoted, rustc_query_impl::plumbing::QueryCtxt>
            36: rustc_query_impl::get_query::mir_promoted
            37: rustc_borrowck::mir_borrowck
            38: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_borrowck, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            40: rustc_query_impl::get_query::mir_borrowck
            40: rustc_query_impl::get_query::mir_borrowck
            41: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            42: rustc_hir_analysis::collect::type_of::find_opaque_ty_constraints_for_rpit
            43: rustc_hir_analysis::collect::type_of::type_of
            44: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            45: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
            46: rustc_query_impl::get_query::type_of
            47: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            48: <rustc_privacy::ReachEverythingInTheInterfaceVisitor>::ty
            49: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            50: rustc_hir::intravisit::walk_ty::<rustc_privacy::EmbargoVisitor>
            51: rustc_hir::intravisit::walk_fn::<rustc_privacy::EmbargoVisitor>
            52: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            53: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            54: rustc_hir::intravisit::walk_block::<rustc_privacy::EmbargoVisitor>
            55: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_block
            56: rustc_hir::intravisit::walk_expr::<rustc_privacy::EmbargoVisitor>
            57: rustc_hir::intravisit::walk_fn::<rustc_privacy::EmbargoVisitor>
            58: rustc_hir::intravisit::walk_trait_item::<rustc_privacy::EmbargoVisitor>
            59: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            60: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            61: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            62: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            63: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            64: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            65: rustc_hir::intravisit::walk_item::<rustc_privacy::EmbargoVisitor>
            66: <rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
            67: rustc_hir::intravisit::walk_mod::<rustc_privacy::EmbargoVisitor>
            68: rustc_privacy::effective_visibilities
            69: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::effective_visibilities, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            70: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::effective_visibilities, rustc_query_impl::plumbing::QueryCtxt>
            71: rustc_query_impl::get_query::effective_visibilities
            72: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 8]>>>
            74: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
            75: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#0}::{closure#2}>>
            76: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#0}>
            77: rustc_interface::passes::analysis
            77: rustc_interface::passes::analysis
            78: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
            80: rustc_query_impl::get_query::analysis
            81: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
            82: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            82: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            83: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            84: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
            85: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            87: <unknown>
            88: <unknown>
          


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (dbd611866 2023-05-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=fa992565d2130c71 -C extra-filename=-fa992565d2130c71 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Zinline-mir -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
