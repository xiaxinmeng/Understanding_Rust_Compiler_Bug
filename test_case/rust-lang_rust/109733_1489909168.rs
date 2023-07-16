

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler\rustc_resolve\src\lib.rs:1870:67
stack backtrace:
   0:     0x7ffc77976a72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91f604f4ffa5801e
   1:     0x7ffc779b4eab - core::fmt::write::hab634a7da6eda80f
   2:     0x7ffc7796be1a - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hd4d7057f831cdf36
   3:     0x7ffc779767bb - std::sys::common::alloc::realloc_fallback::h0462ecd6e4895a2b
   4:     0x7ffc7797a14a - std::panicking::default_hook::h11dda33d25b1c293
   5:     0x7ffc77979db0 - std::panicking::default_hook::h11dda33d25b1c293
   6:     0x7ffc6d02b57e - rustc_driver_impl[80e6611556be2cef]::describe_lints
   7:     0x7ffc7797aa5f - std::panicking::rust_panic_with_hook::hbdafd2453062d848
   8:     0x7ffc7797a77b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h65d2be1af01a551c
   9:     0x7ffc77977719 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91f604f4ffa5801e
  10:     0x7ffc7797a4d0 - rust_begin_unwind
  11:     0x7ffc779e8ed5 - core::panicking::panic_fmt::hef645f8ef8eec178
  12:     0x7ffc779e8f8c - core::panicking::panic::h45fca6531abd18aa
  13:     0x7ffc6b15d970 - <rustc_resolve[eb7387a18a5833cf]::Resolver>::resolve_crate
  14:     0x7ffc6c5aefc1 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  15:     0x7ffc6c5af6a0 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  16:     0x7ffc6c5959d3 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  17:     0x7ffc6c5df64a - <rustc_resolve[eb7387a18a5833cf]::effective_visibilities::EffectiveVisibilitiesVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_item
  18:     0x7ffc6c59be1d - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  19:     0x7ffc6c58063b - <rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_item
  20:     0x7ffc6b155e8b - <rustc_builtin_macros[b8b40dbabaaf7e00]::deriving::generic::find_type_parameters::Visitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  21:     0x7ffc6b15e7cd - rustc_resolve[eb7387a18a5833cf]::provide
  22:     0x7ffc6b15d150 - <rustc_resolve[eb7387a18a5833cf]::Resolver>::resolve_crate
  23:     0x7ffc6a8a4e3e - rustc_interface[acb21b123d0ba37]::passes::resolver_for_lowering
  24:     0x7ffc6b40857d - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  25:     0x7ffc6b346941 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  26:     0x7ffc6a8a792b - rustc_interface[acb21b123d0ba37]::passes::output_filenames
  27:     0x7ffc6b30bf5f - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  28:     0x7ffc6b3e4e72 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  29:     0x7ffc6b355051 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  30:     0x7ffc6c4e5de2 - rustc_ast_lowering[81dcca6d70b57c42]::lower_to_hir
  31:     0x7ffc6b30d819 - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  32:     0x7ffc6b4404c2 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  33:     0x7ffc6b346aa1 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  34:     0x7ffc6ceed0ce - <rustc_middle[99c60881b72579c1]::ty::Predicate>::is_coinductive
  35:     0x7ffc6ca989d8 - <rustc_span[e1ca11e9f2e7f58d]::symbol::Symbol as rustc_serialize[d95a6b06eb86861e]::serialize::Encodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheEncoder>>::encode
  36:     0x7ffc6c9829a8 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::as_any
  37:     0x7ffc6b6bb89a - <rustc_middle[99c60881b72579c1]::hir::map::Map>::get_module
  38:     0x7ffc6b6bbfb9 - <rustc_middle[99c60881b72579c1]::hir::map::Map as rustc_hir[9ac1ca697d999d0a]::intravisit::Map>::trait_item
  39:     0x7ffc6b30b83e - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  40:     0x7ffc6b3dbe52 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  41:     0x7ffc6b346c01 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  42:     0x7ff7cb312d2e - <unknown>
  43:     0x7ff7cb2ce8f4 - <unknown>
  44:     0x7ff7cb456ee8 - <unknown>
  45:     0x7ff7cb2cdfef - <unknown>
  46:     0x7ff7cb47a583 - <unknown>
  47:     0x7ff7cb371459 - <unknown>
  48:     0x7ff7cb2cc567 - <unknown>
  49:     0x7ff7cb37412c - <unknown>
  50:     0x7ff7cb3e1236 - <unknown>
  51:     0x7ff7cb445062 - <unknown>
  52:     0x7ffc7798c91c - std::sys::windows::thread::Thread::new::hc271a03890a34556
  53:     0x7ffcd7a87614 - BaseThreadInitThunk
  54:     0x7ffcd88c26a1 - RtlUserThreadStart

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (17c116721 2023-03-29) running on x86_64-pc-windows-msvc

query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/17c11672167827b0dd92c88ef69f24346d1286dd\compiler\rustc_query_system\src\query\job.rs:139:44
stack backtrace:
   0:     0x7ffc77976a72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91f604f4ffa5801e
   1:     0x7ffc779b4eab - core::fmt::write::hab634a7da6eda80f
   2:     0x7ffc7796be1a - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hd4d7057f831cdf36
   3:     0x7ffc779767bb - std::sys::common::alloc::realloc_fallback::h0462ecd6e4895a2b
   4:     0x7ffc7797a14a - std::panicking::default_hook::h11dda33d25b1c293
   5:     0x7ffc77979db0 - std::panicking::default_hook::h11dda33d25b1c293
   6:     0x7ffc6d02b57e - rustc_driver_impl[80e6611556be2cef]::describe_lints
   7:     0x7ffc7797aa5f - std::panicking::rust_panic_with_hook::hbdafd2453062d848
   8:     0x7ffc7797a77b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h65d2be1af01a551c
   9:     0x7ffc77977719 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91f604f4ffa5801e
  10:     0x7ffc7797a4d0 - rust_begin_unwind
  11:     0x7ffc779e8ed5 - core::panicking::panic_fmt::hef645f8ef8eec178
  12:     0x7ffc779e8f8c - core::panicking::panic::h45fca6531abd18aa
  13:     0x7ffc6ee2e595 - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  14:     0x7ffc6ee694fd - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  15:     0x7ffc6b43b1f1 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7ffc6b35623d - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  17:     0x7ffc6b6e681a - <rustc_middle[99c60881b72579c1]::ty::print::pretty::FmtPrinter>::new
  18:     0x7ffc6f567cd1 - <rustc_middle[99c60881b72579c1]::ty::context::TyCtxt>::def_path_str_with_substs
  19:     0x7ffc6f550a24 - rustc_middle[99c60881b72579c1]::query::descs::hir_attrs
  20:     0x7ffc6ed674d5 - <rustc_query_impl[d3aba5db8cad4e9d]::plumbing::QueryCtxt>::try_print_query_stack
  21:     0x7ffc6ee08b35 - <alloc[573ce340f5291ee5]::vec::Vec<u8> as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7ffc6ee62fc7 - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  23:     0x7ffc6ee0fa55 - <alloc[573ce340f5291ee5]::vec::Vec<u8> as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  24:     0x7ffc6ed56c4d - <rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheEncoder as rustc_serialize[d95a6b06eb86861e]::serialize::Encoder>::emit_char     
  25:     0x7ffc6ee6610f - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  26:     0x7ffc6b440775 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  27:     0x7ffc6b346aa1 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  28:     0x7ffc6cee8782 - <rustc_middle[99c60881b72579c1]::ty::Predicate>::is_coinductive
  29:     0x7ffc6ca978f9 - <rustc_span[e1ca11e9f2e7f58d]::symbol::Symbol as rustc_serialize[d95a6b06eb86861e]::serialize::Encodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheEncoder>>::encode
  30:     0x7ffc6c98374a - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::as_any
  31:     0x7ffc6cf1fe4b - <rustc_middle[99c60881b72579c1]::hir::map::Map>::attrs
  32:     0x7ffc6b6b6bf1 - <rustc_middle[99c60881b72579c1]::mir::tcx::PlaceTy>::field_ty
  33:     0x7ffc6b43ad83 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  34:     0x7ffc6b35623d - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  35:     0x7ffc6b6e681a - <rustc_middle[99c60881b72579c1]::ty::print::pretty::FmtPrinter>::new
  36:     0x7ffc6f567cd1 - <rustc_middle[99c60881b72579c1]::ty::context::TyCtxt>::def_path_str_with_substs
  37:     0x7ffc6f5504e4 - rustc_middle[99c60881b72579c1]::query::descs::hir_owner
  38:     0x7ffc6ed674d5 - <rustc_query_impl[d3aba5db8cad4e9d]::plumbing::QueryCtxt>::try_print_query_stack
  39:     0x7ffc6ee08bb5 - <alloc[573ce340f5291ee5]::vec::Vec<u8> as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7ffc6ee62fc7 - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  41:     0x7ffc6ee0fb15 - <alloc[573ce340f5291ee5]::vec::Vec<u8> as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  42:     0x7ffc6ed56c4d - <rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheEncoder as rustc_serialize[d95a6b06eb86861e]::serialize::Encoder>::emit_char     
  43:     0x7ffc6ee302c1 - <&[rustc_ast[c03cc9997dba8755]::ast::Attribute] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  44:     0x7ffc6d088fdb - rustc_interface[acb21b123d0ba37]::interface::try_print_query_stack
  45:     0x7ffc6d02c0e0 - rustc_driver_impl[80e6611556be2cef]::report_ice
  46:     0x7ffc6d02b5c4 - rustc_driver_impl[80e6611556be2cef]::describe_lints
  47:     0x7ffc7797aa5f - std::panicking::rust_panic_with_hook::hbdafd2453062d848
  48:     0x7ffc7797a77b - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h65d2be1af01a551c
  49:     0x7ffc77977719 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h91f604f4ffa5801e
  50:     0x7ffc7797a4d0 - rust_begin_unwind
  51:     0x7ffc779e8ed5 - core::panicking::panic_fmt::hef645f8ef8eec178
  52:     0x7ffc779e8f8c - core::panicking::panic::h45fca6531abd18aa
  53:     0x7ffc6b15d970 - <rustc_resolve[eb7387a18a5833cf]::Resolver>::resolve_crate
  54:     0x7ffc6c5aefc1 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  55:     0x7ffc6c5af6a0 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  56:     0x7ffc6c5959d3 - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  57:     0x7ffc6c5df64a - <rustc_resolve[eb7387a18a5833cf]::effective_visibilities::EffectiveVisibilitiesVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_item
  58:     0x7ffc6c59be1d - <<rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor>::find_lifetime_for_self::SelfVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  59:     0x7ffc6c58063b - <rustc_resolve[eb7387a18a5833cf]::late::LateResolutionVisitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_item
  60:     0x7ffc6b155e8b - <rustc_builtin_macros[b8b40dbabaaf7e00]::deriving::generic::find_type_parameters::Visitor as rustc_ast[c03cc9997dba8755]::visit::Visitor>::visit_ty
  61:     0x7ffc6b15e7cd - rustc_resolve[eb7387a18a5833cf]::provide
  62:     0x7ffc6b15d150 - <rustc_resolve[eb7387a18a5833cf]::Resolver>::resolve_crate
  63:     0x7ffc6a8a4e3e - rustc_interface[acb21b123d0ba37]::passes::resolver_for_lowering
  64:     0x7ffc6b40857d - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  65:     0x7ffc6b346941 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  66:     0x7ffc6a8a792b - rustc_interface[acb21b123d0ba37]::passes::output_filenames
  67:     0x7ffc6b30bf5f - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  68:     0x7ffc6b3e4e72 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  69:     0x7ffc6b355051 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  70:     0x7ffc6c4e5de2 - rustc_ast_lowering[81dcca6d70b57c42]::lower_to_hir
  71:     0x7ffc6b30d819 - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  72:     0x7ffc6b4404c2 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  73:     0x7ffc6b346aa1 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  74:     0x7ffc6ceed0ce - <rustc_middle[99c60881b72579c1]::ty::Predicate>::is_coinductive
  75:     0x7ffc6ca989d8 - <rustc_span[e1ca11e9f2e7f58d]::symbol::Symbol as rustc_serialize[d95a6b06eb86861e]::serialize::Encodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheEncoder>>::encode
  76:     0x7ffc6c9829a8 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::as_any
  77:     0x7ffc6b6bb89a - <rustc_middle[99c60881b72579c1]::hir::map::Map>::get_module
  78:     0x7ffc6b6bbfb9 - <rustc_middle[99c60881b72579c1]::hir::map::Map as rustc_hir[9ac1ca697d999d0a]::intravisit::Map>::trait_item
  79:     0x7ffc6b30b83e - <rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::CustomEq as rustc_const_eval[5bc5628c60c7bc7b]::transform::check_consts::qualifs::Qualif>::in_adt_inherently
  80:     0x7ffc6b3dbe52 - <&[rustc_span[e1ca11e9f2e7f58d]::symbol::Ident] as rustc_serialize[d95a6b06eb86861e]::serialize::Decodable<rustc_query_impl[d3aba5db8cad4e9d]::on_disk_cache::CacheDecoder>>::decode
  81:     0x7ffc6b346c01 - <rustc_query_impl[d3aba5db8cad4e9d]::Queries as rustc_middle[99c60881b72579c1]::ty::query::QueryEngine>::try_mark_green
  82:     0x7ff7cb312d2e - <unknown>
  83:     0x7ff7cb2ce8f4 - <unknown>
  84:     0x7ff7cb456ee8 - <unknown>
  85:     0x7ff7cb2cdfef - <unknown>
  86:     0x7ff7cb47a583 - <unknown>
  87:     0x7ff7cb371459 - <unknown>
  88:     0x7ff7cb2cc567 - <unknown>
  89:     0x7ff7cb37412c - <unknown>
  90:     0x7ff7cb3e1236 - <unknown>
  91:     0x7ff7cb445062 - <unknown>
  92:     0x7ffc7798c91c - std::sys::windows::thread::Thread::new::hc271a03890a34556
  93:     0x7ffcd7a87614 - BaseThreadInitThunk
  94:     0x7ffcd88c26a1 - RtlUserThreadStart

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (17c116721 2023-03-29) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
