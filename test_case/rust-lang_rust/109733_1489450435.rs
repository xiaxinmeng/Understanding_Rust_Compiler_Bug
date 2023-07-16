

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler\rustc_resolve\src\lib.rs:1876:67
stack backtrace:
   0:     0x7ffcab8f6c72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h496acad3e5333a14
   1:     0x7ffcab93540b - core::fmt::write::h069031b30e516187
   2:     0x7ffcab8ec34a - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h4a42a68c62a0da99     
   3:     0x7ffcab8f69bb - std::sys::common::alloc::realloc_fallback::h3369f43b6728265f
   4:     0x7ffcab8fa319 - std::panicking::default_hook::h47f798cc522c8194
   5:     0x7ffcab8f9f9b - std::panicking::default_hook::h47f798cc522c8194
   6:     0x7ffc5448087c - rustc_driver_impl[ad227f1719532c9c]::describe_lints
   7:     0x7ffcab8fac82 - std::panicking::rust_panic_with_hook::hd39650a597753593
   8:     0x7ffcab8fa99b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h7d7e5954cde42deb
   9:     0x7ffcab8f7939 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h496acad3e5333a14
  10:     0x7ffcab8fa690 - rust_begin_unwind
  11:     0x7ffcab968d45 - core::panicking::panic_fmt::h37efe0a188178441
  12:     0x7ffcab968dfc - core::panicking::panic::hb77962d753708ec9
  13:     0x7ffc51c6d1b0 - <rustc_resolve[a3e18f00d8624b92]::Resolver>::resolve_crate
  14:     0x7ffc532f4409 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  15:     0x7ffc532f4e05 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  16:     0x7ffc532d887f - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  17:     0x7ffc533621bc - <rustc_resolve[a3e18f00d8624b92]::Segment as core[663717899f0eee36]::convert::From<&rustc_ast[aff60dad45e2f618]::ast::PathSegment>>::from
  18:     0x7ffc532dedc4 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  19:     0x7ffc532c2eab - <rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_item
  20:     0x7ffc51c6523b - <rustc_resolve[a3e18f00d8624b92]::def_collector::DefCollector as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_param
  21:     0x7ffc533128ac - <rustc_resolve[a3e18f00d8624b92]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  22:     0x7ffc5187f1f0 - rustc_interface[50c04c391bbbe617]::passes::resolver_for_lowering      
  23:     0x7ffc51b280fa - rustc_privacy[57f72dae7a249f52]::provide
  24:     0x7ffc51bcd0f2 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  25:     0x7ffc51882228 - rustc_interface[50c04c391bbbe617]::passes::output_filenames
  26:     0x7ffc51ae65e7 - rustc_privacy[57f72dae7a249f52]::provide
  27:     0x7ffc51be4a72 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  28:     0x7ffc52373aa1 - rustc_ast_lowering[60b2977bf22bd781]::lower_to_hir
  29:     0x7ffc51b8e55c - rustc_privacy[57f72dae7a249f52]::provide
  30:     0x7ffc51bcd352 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  31:     0x7ffc531d9325 - <rustc_middle[f8d94cf90023dcea]::ty::Visibility as rustc_privacy[57f72dae7a249f52]::VisibilityLike>::new_min
  32:     0x7ffc5323719b - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::as_any
  33:     0x7ffc5196e6cb - <rustc_middle[f8d94cf90023dcea]::hir::map::Map>::get_module
  34:     0x7ffc51970758 - <rustc_middle[f8d94cf90023dcea]::hir::map::Map as rustc_hir[89e8a09b50c1156d]::intravisit::Map>::trait_item
  35:     0x7ffc51ad32e6 - rustc_privacy[57f72dae7a249f52]::provide
  36:     0x7ffc51bcd5b2 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  37:     0x7ff7dffdf41e - <unknown>
  38:     0x7ff7dfeb0899 - <unknown>
  39:     0x7ff7e00a9670 - <unknown>
  40:     0x7ff7dfeaff92 - <unknown>
  41:     0x7ff7e004434e - <unknown>
  42:     0x7ff7e00f6916 - <unknown>
  43:     0x7ff7dffe3aa3 - <unknown>
  44:     0x7ff7dfec3306 - <unknown>
  45:     0x7ff7e00207bb - <unknown>
  46:     0x7ffcab90cacc - std::sys::windows::thread::Thread::new::h22703d71819bc824
  47:     0x7ffcf77b26bd - BaseThreadInitThunk
  48:     0x7ffcf8aaa9f8 - RtlUserThreadStart

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (39f2657d1 2023-03-09) running on x86_64-pc-windows-msvc

query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/39f2657d1101b50f9b71ae460b762d330cc8426b\compiler\rustc_query_system\src\query\job.rs:141:44
stack backtrace:
   0:     0x7ffcab8f6c72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h496acad3e5333a14
   1:     0x7ffcab93540b - core::fmt::write::h069031b30e516187
   2:     0x7ffcab8ec34a - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h4a42a68c62a0da99     
   3:     0x7ffcab8f69bb - std::sys::common::alloc::realloc_fallback::h3369f43b6728265f
   4:     0x7ffcab8fa319 - std::panicking::default_hook::h47f798cc522c8194
   5:     0x7ffcab8f9f9b - std::panicking::default_hook::h47f798cc522c8194
   6:     0x7ffc5448087c - rustc_driver_impl[ad227f1719532c9c]::describe_lints
   7:     0x7ffcab8fac82 - std::panicking::rust_panic_with_hook::hd39650a597753593
   8:     0x7ffcab8fa99b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h7d7e5954cde42deb
   9:     0x7ffcab8f7939 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h496acad3e5333a14
  10:     0x7ffcab8fa690 - rust_begin_unwind
  11:     0x7ffcab968d45 - core::panicking::panic_fmt::h37efe0a188178441
  12:     0x7ffcab968dfc - core::panicking::panic::hb77962d753708ec9
  13:     0x7ffc54e7d976 - <rustc_query_impl[94188344f151e92c]::plumbing::QueryCtxt>::try_print_query_stack
  14:     0x7ffc51b84941 - rustc_privacy[57f72dae7a249f52]::provide
  15:     0x7ffc51be668e - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  16:     0x7ffc5198383a - <rustc_middle[f8d94cf90023dcea]::ty::print::pretty::FmtPrinter>::new  
  17:     0x7ffc54983381 - <rustc_middle[f8d94cf90023dcea]::ty::context::TyCtxt>::def_path_str_with_substs
  18:     0x7ffc54a13797 - rustc_middle[f8d94cf90023dcea]::query::descs::hir_attrs
  19:     0x7ffc54e79c05 - <rustc_query_impl[94188344f151e92c]::plumbing::QueryCtxt>::try_print_query_stack
  20:     0x7ffc54ed14c5 - RNvXsf_NtCscIjetGtktaq_16rustc_query_impl13on_disk_cacheRINtNtNtNtCsgRVzOUNmIYX_3std11collections4hash3map7HashMapNtNtCse1AxD1PoIMD_10rustc_span6def_id5DefIdNtNtCslmCdM5tENny_12rustc_middle2ty2TyINtNtCs8M5wBmkPYS8_4core4hash18BuildHasherDefaultNtCsarH6rZ2
  21:     0x7ffc54d66ce0 - <rustc_privacy[57f72dae7a249f52]::errors::PrivateInPublicLint as rustc_errors[db358ed34dc1d3f1]::diagnostic::DecorateLint<()>>::msg
  22:     0x7ffc54ed8385 - RNvXsf_NtCscIjetGtktaq_16rustc_query_impl13on_disk_cacheRINtNtNtNtCsgRVzOUNmIYX_3std11collections4hash3map7HashMapNtNtCse1AxD1PoIMD_10rustc_span6def_id5DefIdNtNtCslmCdM5tENny_12rustc_middle2ty2TyINtNtCs8M5wBmkPYS8_4core4hash18BuildHasherDefaultNtCsarH6rZ2
  23:     0x7ffc54e21dcd - <&[(rustc_middle[f8d94cf90023dcea]::middle::exported_symbols::ExportedSymbol, rustc_middle[f8d94cf90023dcea]::middle::exported_symbols::SymbolExportInfo)] as rustc_serialize[d4891fa73bd6f232]::serialize::Decodable<rustc_query_impl[94188344f151e92c]::on_disk_cache::CacheDecoder>>::decode
  24:     0x7ffc51b8ebdf - rustc_privacy[57f72dae7a249f52]::provide
  25:     0x7ffc51bcd352 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  26:     0x7ffc531d6e9b - <rustc_middle[f8d94cf90023dcea]::ty::Visibility as rustc_privacy[57f72dae7a249f52]::VisibilityLike>::new_min
  27:     0x7ffc52cdcef5 - <rustc_middle[f8d94cf90023dcea]::hir::map::Map>::attrs
  28:     0x7ffc51956df0 - <rustc_middle[f8d94cf90023dcea]::ty::consts::Const>::from_opt_const_arg_anon_const
  29:     0x7ffc51b83faa - rustc_privacy[57f72dae7a249f52]::provide
  30:     0x7ffc51be668e - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  31:     0x7ffc5198383a - <rustc_middle[f8d94cf90023dcea]::ty::print::pretty::FmtPrinter>::new  
  32:     0x7ffc54983381 - <rustc_middle[f8d94cf90023dcea]::ty::context::TyCtxt>::def_path_str_with_substs
  33:     0x7ffc54a13257 - rustc_middle[f8d94cf90023dcea]::query::descs::hir_owner
  34:     0x7ffc54e79c05 - <rustc_query_impl[94188344f151e92c]::plumbing::QueryCtxt>::try_print_query_stack
  35:     0x7ffc54ed1545 - RNvXsf_NtCscIjetGtktaq_16rustc_query_impl13on_disk_cacheRINtNtNtNtCsgRVzOUNmIYX_3std11collections4hash3map7HashMapNtNtCse1AxD1PoIMD_10rustc_span6def_id5DefIdNtNtCslmCdM5tENny_12rustc_middle2ty2TyINtNtCs8M5wBmkPYS8_4core4hash18BuildHasherDefaultNtCsarH6rZ2
  36:     0x7ffc54d66ce0 - <rustc_privacy[57f72dae7a249f52]::errors::PrivateInPublicLint as rustc_errors[db358ed34dc1d3f1]::diagnostic::DecorateLint<()>>::msg
  37:     0x7ffc54ed8445 - RNvXsf_NtCscIjetGtktaq_16rustc_query_impl13on_disk_cacheRINtNtNtNtCsgRVzOUNmIYX_3std11collections4hash3map7HashMapNtNtCse1AxD1PoIMD_10rustc_span6def_id5DefIdNtNtCslmCdM5tENny_12rustc_middle2ty2TyINtNtCs8M5wBmkPYS8_4core4hash18BuildHasherDefaultNtCsarH6rZ2
  38:     0x7ffc54e21dcd - <&[(rustc_middle[f8d94cf90023dcea]::middle::exported_symbols::ExportedSymbol, rustc_middle[f8d94cf90023dcea]::middle::exported_symbols::SymbolExportInfo)] as rustc_serialize[d4891fa73bd6f232]::serialize::Decodable<rustc_query_impl[94188344f151e92c]::on_disk_cache::CacheDecoder>>::decode
  39:     0x7ffc54e7fbfc - <rustc_query_impl[94188344f151e92c]::plumbing::QueryCtxt>::try_print_query_stack
  40:     0x7ffc5485c646 - rustc_interface[50c04c391bbbe617]::interface::try_print_query_stack   
  41:     0x7ffc544813ba - rustc_driver_impl[ad227f1719532c9c]::report_ice
  42:     0x7ffc544808c2 - rustc_driver_impl[ad227f1719532c9c]::describe_lints
  43:     0x7ffcab8fac82 - std::panicking::rust_panic_with_hook::hd39650a597753593
  44:     0x7ffcab8fa99b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h7d7e5954cde42deb
  45:     0x7ffcab8f7939 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h496acad3e5333a14
  46:     0x7ffcab8fa690 - rust_begin_unwind
  47:     0x7ffcab968d45 - core::panicking::panic_fmt::h37efe0a188178441
  48:     0x7ffcab968dfc - core::panicking::panic::hb77962d753708ec9
  49:     0x7ffc51c6d1b0 - <rustc_resolve[a3e18f00d8624b92]::Resolver>::resolve_crate
  50:     0x7ffc532f4409 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  51:     0x7ffc532f4e05 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  52:     0x7ffc532d887f - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  53:     0x7ffc533621bc - <rustc_resolve[a3e18f00d8624b92]::Segment as core[663717899f0eee36]::convert::From<&rustc_ast[aff60dad45e2f618]::ast::PathSegment>>::from
  54:     0x7ffc532dedc4 - <<rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  55:     0x7ffc532c2eab - <rustc_resolve[a3e18f00d8624b92]::late::LateResolutionVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_item
  56:     0x7ffc51c6523b - <rustc_resolve[a3e18f00d8624b92]::def_collector::DefCollector as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_param
  57:     0x7ffc533128ac - <rustc_resolve[a3e18f00d8624b92]::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast[aff60dad45e2f618]::visit::Visitor>::visit_ty
  58:     0x7ffc5187f1f0 - rustc_interface[50c04c391bbbe617]::passes::resolver_for_lowering      
  59:     0x7ffc51b280fa - rustc_privacy[57f72dae7a249f52]::provide
  60:     0x7ffc51bcd0f2 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  61:     0x7ffc51882228 - rustc_interface[50c04c391bbbe617]::passes::output_filenames
  62:     0x7ffc51ae65e7 - rustc_privacy[57f72dae7a249f52]::provide
  63:     0x7ffc51be4a72 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  64:     0x7ffc52373aa1 - rustc_ast_lowering[60b2977bf22bd781]::lower_to_hir
  65:     0x7ffc51b8e55c - rustc_privacy[57f72dae7a249f52]::provide
  66:     0x7ffc51bcd352 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  67:     0x7ffc531d9325 - <rustc_middle[f8d94cf90023dcea]::ty::Visibility as rustc_privacy[57f72dae7a249f52]::VisibilityLike>::new_min
  68:     0x7ffc5323719b - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::as_any
  69:     0x7ffc5196e6cb - <rustc_middle[f8d94cf90023dcea]::hir::map::Map>::get_module
  70:     0x7ffc51970758 - <rustc_middle[f8d94cf90023dcea]::hir::map::Map as rustc_hir[89e8a09b50c1156d]::intravisit::Map>::trait_item
  71:     0x7ffc51ad32e6 - rustc_privacy[57f72dae7a249f52]::provide
  72:     0x7ffc51bcd5b2 - <rustc_query_impl[94188344f151e92c]::Queries as rustc_middle[f8d94cf90023dcea]::ty::query::QueryEngine>::try_mark_green
  73:     0x7ff7dffdf41e - <unknown>
  74:     0x7ff7dfeb0899 - <unknown>
  75:     0x7ff7e00a9670 - <unknown>
  76:     0x7ff7dfeaff92 - <unknown>
  77:     0x7ff7e004434e - <unknown>
  78:     0x7ff7e00f6916 - <unknown>
  79:     0x7ff7dffe3aa3 - <unknown>
  80:     0x7ff7dfec3306 - <unknown>
  81:     0x7ff7e00207bb - <unknown>
  82:     0x7ffcab90cacc - std::sys::windows::thread::Thread::new::h22703d71819bc824
  83:     0x7ffcf77b26bd - BaseThreadInitThunk
  84:     0x7ffcf8aaa9f8 - RtlUserThreadStart

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (39f2657d1 2023-03-09) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
