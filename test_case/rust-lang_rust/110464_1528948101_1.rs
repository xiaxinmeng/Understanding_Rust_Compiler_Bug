
stack backtrace:
   0:     0x7f2dbb878331 - std::backtrace_rs::backtrace::libunwind::trace::h8355d489046eea9f
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2dbb878331 - std::backtrace_rs::backtrace::trace_unsynchronized::h5224130277357a26
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2dbb878331 - std::sys_common::backtrace::_print_fmt::hecc97c6ada6b37c5
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2dbb878331 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha40fb82141ca69c7
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2dbb8d8f1f - core::fmt::rt::Argument::fmt::h3abab6d788a756e3
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/core/src/fmt/rt.rs:138:9
   5:     0x7f2dbb8d8f1f - core::fmt::write::h0d9c37369cab45ae
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/core/src/fmt/mod.rs:1094:21
   6:     0x7f2dbb86b4a1 - std::io::Write::write_fmt::h48802cd26188edd4
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/io/mod.rs:1712:15
   7:     0x7f2dbb878145 - std::sys_common::backtrace::_print::hd06dedc75707c8aa
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7f2dbb878145 - std::sys_common::backtrace::print::he87158405a814203
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7f2dbb87ac87 - std::panicking::default_hook::{{closure}}::hed443a04a4ae9143
  10:     0x7f2dbb87aa74 - std::panicking::default_hook::hcb7845fe9b5159b6
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/panicking.rs:288:9
  11:     0x7f2dbeafb015 - rustc_driver_impl[738786893c5490bf]::DEFAULT_HOOK::{closure#0}::{closure#0}
  12:     0x7f2dbb87b3a7 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h1480dd017092d509
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/alloc/src/boxed.rs:1976:9
  13:     0x7f2dbb87b3a7 - std::panicking::rust_panic_with_hook::he4834f7686de7b98
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/panicking.rs:695:13
  14:     0x7f2dbb87b127 - std::panicking::begin_panic_handler::{{closure}}::ha2d182560a0fb060
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/panicking.rs:582:13
  15:     0x7f2dbb878776 - std::sys_common::backtrace::__rust_end_short_backtrace::h2df9d74a88462a94
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys_common/backtrace.rs:150:18
  16:     0x7f2dbb87ae92 - rust_begin_unwind
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/panicking.rs:578:5
  17:     0x7f2dbb8d51c3 - core::panicking::panic_fmt::hdf51123d3e5c1099
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/core/src/panicking.rs:67:14
  18:     0x7f2dbef37e82 - rustc_metadata[ae0e66a2a2c113fb]::rmeta::decoder::cstore_impl::provide_extern::def_span::{closure#2}
  19:     0x7f2dbe434f26 - rustc_metadata[ae0e66a2a2c113fb]::rmeta::decoder::cstore_impl::provide_extern::def_span
  20:     0x7f2dbd3c50ed - rustc_query_system[34a9c542073df]::query::plumbing::try_execute_query::<rustc_query_impl[5c8c89bc676e7b7e]::queries::def_span, rustc_query_impl[5c8c89bc676e7b7e]::plumbing::QueryCtxt>
  21:     0x7f2dbd3c296a - <rustc_query_impl[5c8c89bc676e7b7e]::Queries as rustc_middle[4a9d2aea1650cf38]::ty::query::QueryEngine>::def_span
  22:     0x7f2dbd0edefa - <rustc_infer[5d7b6307277651b3]::infer::InferCtxt>::infer_projection
  23:     0x7f2dbd0d890c - rustc_trait_selection[87bbd939b3c90e9c]::traits::project::normalize_projection_type
  24:     0x7f2dbd0d6365 - <rustc_trait_selection[87bbd939b3c90e9c]::traits::project::AssocTypeNormalizer as rustc_type_ir[1d3e9495482829d7]::fold::TypeFolder<rustc_middle[4a9d2aea1650cf38]::ty::context::TyCtxt>>::fold_ty
  25:     0x7f2dbdc992cc - rustc_middle[4a9d2aea1650cf38]::ty::util::fold_list::<rustc_trait_selection[87bbd939b3c90e9c]::traits::project::AssocTypeNormalizer, rustc_middle[4a9d2aea1650cf38]::ty::Ty, <&rustc_middle[4a9d2aea1650cf38]::ty::list::List<rustc_middle[4a9d2aea1650cf38]::ty::Ty> as rustc_type_ir[1d3e9495482829d7]::fold::TypeFoldable<rustc_middle[4a9d2aea1650cf38]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[87bbd939b3c90e9c]::traits::project::AssocTypeNormalizer>::{closure#0}>
  26:     0x7f2dbcce8c70 - <rustc_infer[5d7b6307277651b3]::infer::at::At as rustc_trait_selection[87bbd939b3c90e9c]::traits::project::NormalizeExt>::normalize::<rustc_middle[4a9d2aea1650cf38]::ty::sty::FnSig>
  27:     0x7f2dbcccd187 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_call
  28:     0x7f2dbd19d99a - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f2dbd19dfa1 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  30:     0x7f2dbd19dfa1 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7f2dbd1f7c0a - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_decl
  32:     0x7f2dbd1f373d - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_block_with_expected
  33:     0x7f2dbd19df24 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f2dbd20fcce - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_match::{closure#0}
  35:     0x7f2dbd19f460 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7f2dbd1f363b - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_block_with_expected
  37:     0x7f2dbd19df24 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7f2dbd376d86 - <rustc_hir_typeck[e3c70be22799a2fb]::fn_ctxt::FnCtxt>::check_return_expr
  39:     0x7f2dbd36e51e - rustc_hir_typeck[e3c70be22799a2fb]::check::check_fn
  40:     0x7f2dbd3574ea - rustc_hir_typeck[e3c70be22799a2fb]::typeck
  41:     0x7f2dbd34c9c9 - rustc_query_system[34a9c542073df]::query::plumbing::try_execute_query::<rustc_query_impl[5c8c89bc676e7b7e]::queries::typeck, rustc_query_impl[5c8c89bc676e7b7e]::plumbing::QueryCtxt>
  42:     0x7f2dbe40905a - rustc_hir_typeck[e3c70be22799a2fb]::used_trait_imports
  43:     0x7f2dbd731aac - rustc_query_system[34a9c542073df]::query::plumbing::try_execute_query::<rustc_query_impl[5c8c89bc676e7b7e]::queries::used_trait_imports, rustc_query_impl[5c8c89bc676e7b7e]::plumbing::QueryCtxt>
  44:     0x7f2dbdf738d3 - rustc_hir_analysis[505c17fd5083e6d3]::check_crate
  45:     0x7f2dbdf666fe - rustc_interface[1da25c7cb643f6df]::passes::analysis
  46:     0x7f2dbe277cef - rustc_query_system[34a9c542073df]::query::plumbing::try_execute_query::<rustc_query_impl[5c8c89bc676e7b7e]::queries::analysis, rustc_query_impl[5c8c89bc676e7b7e]::plumbing::QueryCtxt>
  47:     0x7f2dbe277820 - <rustc_query_impl[5c8c89bc676e7b7e]::Queries as rustc_middle[4a9d2aea1650cf38]::ty::query::QueryEngine>::analysis
  48:     0x7f2dbdd1c86a - <rustc_middle[4a9d2aea1650cf38]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[738786893c5490bf]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>>
  49:     0x7f2dbdd1bb6a - <rustc_interface[1da25c7cb643f6df]::interface::Compiler>::enter::<rustc_driver_impl[738786893c5490bf]::run_compiler::{closure#1}::{closure#2}, core[af2acebbc2b23591]::result::Result<core[af2acebbc2b23591]::option::Option<rustc_interface[1da25c7cb643f6df]::queries::Linker>, rustc_span[8cddaddefb760954]::ErrorGuaranteed>>
  50:     0x7f2dbdd19bc1 - rustc_span[8cddaddefb760954]::set_source_map::<core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>, rustc_interface[1da25c7cb643f6df]::interface::run_compiler<core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>, rustc_driver_impl[738786893c5490bf]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f2dbdd19267 - std[b167bfecd85c8427]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1da25c7cb643f6df]::util::run_in_thread_pool_with_globals<rustc_interface[1da25c7cb643f6df]::interface::run_compiler<core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>, rustc_driver_impl[738786893c5490bf]::run_compiler::{closure#1}>::{closure#0}, core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>>
  52:     0x7f2dbdd18b85 - <<std[b167bfecd85c8427]::thread::Builder>::spawn_unchecked_<rustc_interface[1da25c7cb643f6df]::util::run_in_thread_pool_with_globals<rustc_interface[1da25c7cb643f6df]::interface::run_compiler<core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>, rustc_driver_impl[738786893c5490bf]::run_compiler::{closure#1}>::{closure#0}, core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af2acebbc2b23591]::result::Result<(), rustc_span[8cddaddefb760954]::ErrorGuaranteed>>::{closure#1} as core[af2acebbc2b23591]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f2dbb8858c5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha80838fe840f5c99
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/alloc/src/boxed.rs:1962:9
  54:     0x7f2dbb8858c5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb7b8292ae0f012c1
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/alloc/src/boxed.rs:1962:9
  55:     0x7f2dbb8858c5 - std::sys::unix::thread::Thread::new::thread_start::h64b218fd2c840776
                               at /rustc/87b1f891ea76713462cfc5a15137a8fe2b24ecc2/library/std/src/sys/unix/thread.rs:108:17
  56:     0x7f2dbb5d2b43 - start_thread
                               at ./nptl/pthread_create.c:442:8
  57:     0x7f2dbb664a00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  58:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (87b1f891e 2023-04-29) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED] -C link-arg=-fuse-ld=lld

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [def_span] looking up span for `web_ws_handlers::handler::Handler::call::{opaque#0}`
#1 [typeck] type-checking `ws::spawn_request_handler`
#2 [used_trait_imports] finding used_trait_imports `ws::spawn_request_handler`
#3 [analysis] running analysis passes on this crate
end of query stack
