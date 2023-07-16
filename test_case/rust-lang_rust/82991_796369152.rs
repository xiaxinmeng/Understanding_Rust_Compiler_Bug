
PS D:\work\trash\github\windows-rs> cargo +nightly build --target i686-pc-windows-msvc --verbose
       Fresh unicode-xid v0.2.1
       Fresh squote v0.1.2
       Fresh const-sha1 v0.2.0
       Fresh proc-macro2 v1.0.24
       Fresh quote v1.0.9
       Fresh syn v1.0.63
       Fresh windows_gen_macros v0.4.0 (D:\work\trash\github\windows-rs\crates\gen\macros)
   Compiling windows_gen v0.4.0 (D:\work\trash\github\windows-rs\crates\gen)
       Fresh windows_macros v0.4.0 (D:\work\trash\github\windows-rs\crates\macros)
     Running `rustc --crate-name windows_gen --edition=2018 crates\gen\src\lib.rs --error-format=json --json=diagnostic-
rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1e
a50951892df2d0 -C extra-filename=-1ea50951892df2d0 --out-dir D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps --target i686-pc-windows-msvc -C incremental=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\incremental -L dependency=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps -L dependency=D:\work\trash\github\windows-rs\target\debug\deps --extern proc_macro2=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libproc_macro2-38e33f2f3aac18a7.rmeta --extern quote=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libquote-3fdae8b6361c3c59.rmeta --extern squote=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libsquote-7e42ba303971a56d.rmeta --extern syn=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libsyn-b33358b6864d6692.rmeta --extern macros=D:\work\trash\github\windows-rs\target\debug\deps\windows_gen_macros-a1f290352570ce5d.dll`
thread 'rustc' panicked at 'Failed to get crate data for crate15', compiler\rustc_metadata\src\creader.rs:136:32
stack backtrace:
   0:      0x7fed9c3879e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29e3178dfca9172d
   1:      0x7fed9c6405c - core::fmt::write::h187a24ff0c56237d
   2:      0x7fed9c2bf38 - <std::io::IoSlice as core::fmt::Debug>::fmt::hf9bc4afd00f56a97
   3:      0x7fed9c3c94d - std::panicking::take_hook::he864250439773e97
   4:      0x7fed9c3c419 - std::panicking::take_hook::he864250439773e97
   5:      0x7feccec2457 - rustc_driver::report_ice::hbe9b74fa4ffbc8a3
   6:      0x7fed9c3d1b5 - std::panicking::rust_panic_with_hook::hfaab7d6c0aa47dfe
   7:      0x7fed9c3cd11 - rust_begin_unwind
   8:      0x7fed9c390ef - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29e3178dfca9172d
   9:      0x7fed9c3cc69 - rust_begin_unwind
  10:      0x7fed9c9584c - std::panicking::begin_panic_fmt::h827db613866cf127
  11:      0x7fed0667447 - <rustc_metadata::creader::CrateDump as core::fmt::Debug>::fmt::h5df8090f56b7e514
  12:      0x7fed066bb24 - rustc_metadata::creader::CrateLoader::into_cstore::h5cc49120d4022104
  13:      0x7fed0670313 - rustc_metadata::creader::CrateLoader::maybe_process_path_extern::hd535eb9109c96348
  14:      0x7fecf858b0e - rustc_resolve::Resolver::traits_in_scope::h260c452b1769e900
  15:      0x7fecf83ba77 - rustc_resolve::macros::<impl rustc_resolve::Resolver>::resolve_macro_path::h9733130447ef38a8
  16:      0x7fecf8538d9 - rustc_resolve::Resolver::traits_in_scope::h260c452b1769e900
  17:      0x7fecf851560 - rustc_resolve::Resolver::traits_in_scope::h260c452b1769e900
  18:      0x7fecf83a28e - rustc_resolve::macros::<impl rustc_resolve::Resolver>::resolve_macro_path::h9733130447ef38a8
  19:      0x7fecf836ada - rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::resolve_macro_invocation::h115bf92ce6620889
  20:      0x7fed082d673 - rustc_expand::expand::MacroExpander::fully_expand_fragment::h4189cd99156fd4cc
  21:      0x7fed082c60c - rustc_expand::expand::MacroExpander::expand_crate::h3dedfb87f63a1924
  22:      0x7fecd009bc0 - rustc_interface::passes::BoxedResolver::to_resolver_outputs::hcf4e5c823f6ab62b
  23:      0x7fecd0adb69 - rustc_interface::interface::try_print_query_stack::h390f27bc39ef53eb
  24:      0x7fecd0a2636 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h42fa963856279c22
  25:      0x7fecd007a67 - <rls_span::compiler::_::<impl serde::de::Deserialize for rls_span::compiler::DiagnosticSpanMacroExpansion>::deserialize::__Visitor as serde::de::Visitor>::expecting::hd5b46ca79c97c9e8
  26:      0x7fecd01b4d8 - rustc_interface::queries::Queries::expansion::h11dcf0fe3e54df0d
  27:      0x7feccedaa1d - <rustc_middle::ty::SymbolName as core::fmt::Display>::fmt::h17a09aa888927f04
  28:      0x7feccec4b8c - <rustc_driver::Compilation as core::fmt::Debug>::fmt::he3b75735c17c4dbb
  29:      0x7feccedd206 - <rustc_middle::ty::SymbolName as core::fmt::Display>::fmt::h17a09aa888927f04
  30:      0x7fecced77c4 - chalk_engine::table::AnswerIndex::increment::he965260fccad8761
  31:      0x7feccedd93b - <rustc_middle::ty::SymbolName as core::fmt::Display>::fmt::h17a09aa888927f04
  32:      0x7feccf013bd - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::he7f90af69710fa6b
  33:      0x7fed9c4c84a - std::sys::windows::thread::Thread::new::h41ec480efd53d216
  34:         0x7701556d - BaseThreadInitThunk
  35:         0x7717372d - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler
&template=ice.md

note: rustc 1.52.0-nightly (8f349be27 2021-03-08) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `windows_gen`

Caused by:
  process didn't exit successfully: `rustc --crate-name windows_gen --edition=2018 crates\gen\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1ea50951892df2d0 -C extra-filename=-1ea50951892df2d0 --out-dir D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps --target i686-pc-windows-msvc -C incremental=D:\work\trash\github\windows-rs\target\i
686-pc-windows-msvc\debug\incremental -L dependency=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps -L dependency=D:\work\trash\github\windows-rs\target\debug\deps --extern proc_macro2=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libproc_macro2-38e33f2f3aac18a7.rmeta --extern quote=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libquote-3fdae8b6361c3c59.rmeta --extern squote=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libsquote-7e42ba303971a56d.rmeta --extern syn=D:\work\trash\github\windows-rs\target\i686-pc-windows-msvc\debug\deps\libsyn-b33358b6864d6692.rmeta --extern macros=D:\work\trash\github\windows-rs\target\debug\deps\windows_gen_macros-a1f290352570ce5d.dll` (exit code: 101)
