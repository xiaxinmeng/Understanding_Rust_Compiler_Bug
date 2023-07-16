plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'missing CrateMetadata in DecodeContext', compiler/rustc_metadata/src/rmeta/decoder.rs:337:9
   0:     0x7f62e995ce22 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   0:     0x7f62e995ce22 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   1:     0x7f62e99c4a58 - core::fmt::write::hddd81543d4c11162
   2:     0x7f62e994d231 - std::io::Write::write_fmt::hb10c7c89d9d85e51
   3:     0x7f62e9960139 - std::panicking::default_hook::{{closure}}::hf26c6e572bb06902
   4:     0x7f62e995fdda - std::panicking::default_hook::h5e275631e30996f3
   5:     0x7f62ea45e3b4 - rustc_driver[49a19c9f287af04e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f62e996099f - std::panicking::rust_panic_with_hook::h88a63d00b2da51a4
   7:     0x7f62e99607a9 - std::panicking::begin_panic_handler::{{closure}}::h896476355c1c691e
   8:     0x7f62e995d3c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd0faa25817600199
   9:     0x7f62e99604d9 - rust_begin_unwind
  10:     0x7f62e9914203 - core::panicking::panic_fmt::h8ee02022fd7b4670
  11:     0x7f62ec48d9a5 - <rustc_span[4929ef0fa1883a40]::symbol::Symbol as rustc_serialize[2bae28866586257a]::serialize::Decodable<rustc_metadata[9c6581cb287475aa]::rmeta::decoder::DecodeContext>>::decode
  12:     0x7f62ec4a9310 - <rustc_metadata[9c6581cb287475aa]::rmeta::CrateRoot as rustc_serialize[2bae28866586257a]::serialize::Decodable<rustc_metadata[9c6581cb287475aa]::rmeta::decoder::DecodeContext>>::decode
  13:     0x7f62ec4a4f5c - <rustc_metadata[9c6581cb287475aa]::rmeta::LazyValue<rustc_metadata[9c6581cb287475aa]::rmeta::CrateRoot>>::decode::<&rustc_metadata[9c6581cb287475aa]::rmeta::decoder::MetadataBlob>
  14:     0x7f62ec4c6d29 - <rustc_metadata[9c6581cb287475aa]::rmeta::decoder::MetadataBlob>::get_root
  15:     0x7f62ec3e4272 - <rustc_metadata[9c6581cb287475aa]::locator::CrateLocator>::extract_one
  16:     0x7f62ec3e35ce - <rustc_metadata[9c6581cb287475aa]::locator::CrateLocator>::extract_lib
  17:     0x7f62ec3e141e - <rustc_metadata[9c6581cb287475aa]::locator::CrateLocator>::maybe_load_library_crate
  18:     0x7f62ec42efda - <rustc_metadata[9c6581cb287475aa]::creader::CrateLoader>::load
  19:     0x7f62ec42b462 - <rustc_metadata[9c6581cb287475aa]::creader::CrateLoader>::maybe_resolve_crate
  20:     0x7f62ec431b2f - <rustc_metadata[9c6581cb287475aa]::creader::CrateLoader>::maybe_process_path_extern
  21:     0x7f62eb243611 - <rustc_resolve[3209967f85b855c6]::Resolver>::extern_prelude_get
  22:     0x7f62eb248a44 - <rustc_resolve[3209967f85b855c6]::Resolver>::early_resolve_ident_in_lexical_scope
  23:     0x7f62eb22da99 - <rustc_resolve[3209967f85b855c6]::Resolver>::resolve_path_with_ribs
  24:     0x7f62eb24da9f - <rustc_resolve[3209967f85b855c6]::Resolver>::maybe_resolve_path
  25:     0x7f62eb1afec0 - <rustc_resolve[3209967f85b855c6]::imports::ImportResolver>::resolve_imports
  26:     0x7f62eb23229d - <rustc_resolve[3209967f85b855c6]::Resolver as rustc_expand[5a75f015a7e3b478]::base::ResolverExpand>::resolve_imports
  27:     0x7f62ec5c6b3d - <rustc_expand[5a75f015a7e3b478]::expand::MacroExpander>::fully_expand_fragment
  28:     0x7f62ec5c67ac - <rustc_expand[5a75f015a7e3b478]::expand::MacroExpander>::expand_crate
  29:     0x7f62ea5a3a6c - <rustc_session[4b5caf6a889f8616]::session::Session>::time::<core[990e9aac2c65c03a]::result::Result<rustc_ast[a9f07511ff3ea350]::ast::Crate, rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_interface[b0a9b6e49911083a]::passes::configure_and_expand::{closure#1}>
  30:     0x7f62ea585440 - rustc_interface[b0a9b6e49911083a]::passes::configure_and_expand
  31:     0x7f62ea617ef3 - <rustc_interface[b0a9b6e49911083a]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[b0a9b6e49911083a]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[990e9aac2c65c03a]::result::Result<rustc_ast[a9f07511ff3ea350]::ast::Crate, rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  32:     0x7f62ea57b755 - <rustc_interface[b0a9b6e49911083a]::queries::Queries>::expansion
  33:     0x7f62ea47b654 - <rustc_interface[b0a9b6e49911083a]::interface::Compiler>::enter::<rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}::{closure#2}, core[990e9aac2c65c03a]::result::Result<core[990e9aac2c65c03a]::option::Option<rustc_interface[b0a9b6e49911083a]::queries::Linker>, rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  34:     0x7f62ea4d7849 - rustc_span[4929ef0fa1883a40]::with_source_map::<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_interface[b0a9b6e49911083a]::interface::create_compiler_and_run<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#1}>
  35:     0x7f62ea47c9ce - <scoped_tls[56ec63818e29d8c5]::ScopedKey<rustc_span[4929ef0fa1883a40]::SessionGlobals>>::set::<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  36:     0x7f62ea4d4349 - std[cf5d045d49351115]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b0a9b6e49911083a]::util::run_in_thread_pool_with_globals<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  37:     0x7f62ea4cc0b9 - <<std[cf5d045d49351115]::thread::Builder>::spawn_unchecked_<rustc_interface[b0a9b6e49911083a]::util::run_in_thread_pool_with_globals<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#1} as core[990e9aac2c65c03a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f62e996d363 - std::sys::unix::thread::Thread::new::thread_start::hfdb91e9761b815b8
  39:     0x7f62e3ebd609 - start_thread
  40:     0x7f62e97d0133 - clone
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (5c5c10b11 2022-07-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
