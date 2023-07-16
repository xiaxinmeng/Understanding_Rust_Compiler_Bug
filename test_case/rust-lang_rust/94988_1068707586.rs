plain
   Compiling tracing-attributes v0.1.18
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.76.0
   Compiling chalk-ir v0.76.0
thread 'rustc' panicked at 'missing CrateMetadata in DecodeContext', compiler/rustc_metadata/src/rmeta/decoder.rs:404:9
   0:     0x7f7a0d063b5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f7a0d0d063e - core::fmt::write::h39191aa5431a5380
   1:     0x7f7a0d0d063e - core::fmt::write::h39191aa5431a5380
   2:     0x7f7a0d053a81 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f7a0d06398b - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f7a0d0681f4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f7a0d067dca - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f7a0dadf4b1 - rustc_driver[7387008bc487c86e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7a0d068906 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f7a0d0686d9 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7f7a0d064084 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7f7a0d0683e9 - rust_begin_unwind
  11:     0x7f7a0d01fa93 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7f7a0f8a2fe4 - <rustc_span[f78664e4d3610ddc]::hygiene::SyntaxContext as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  13:     0x7f7a0f8a357a - <rustc_span[f78664e4d3610ddc]::span_encoding::Span as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  14:     0x7f7a0f95c313 - <rustc_attr[827ba36fe9eb008c]::builtin::Unstability as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  15:     0x7f7a0f931deb - <alloc[75caa6313e26ebc1]::vec::Vec<rustc_attr[827ba36fe9eb008c]::builtin::Unstability> as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  16:     0x7f7a0f95c14b - <rustc_attr[827ba36fe9eb008c]::builtin::StabilityLevel as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  17:     0x7f7a0f947b8d - <core[8382fcd636289ab6]::option::Option<rustc_attr[827ba36fe9eb008c]::builtin::Stability> as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  18:     0x7f7a0f8b318b - <rustc_metadata[13636c83be9747c7]::rmeta::ProcMacroData as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  19:     0x7f7a0f94741c - <core[8382fcd636289ab6]::option::Option<rustc_metadata[13636c83be9747c7]::rmeta::ProcMacroData> as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  20:     0x7f7a0f8b39ae - <rustc_metadata[13636c83be9747c7]::rmeta::CrateRoot as rustc_serialize[ddcd976aa604b7da]::serialize::Decodable<rustc_metadata[13636c83be9747c7]::rmeta::decoder::DecodeContext>>::decode
  21:     0x7f7a0f8aa91c - <rustc_metadata[13636c83be9747c7]::rmeta::Lazy<rustc_metadata[13636c83be9747c7]::rmeta::CrateRoot, ()>>::decode::<&rustc_metadata[13636c83be9747c7]::rmeta::decoder::MetadataBlob>
  22:     0x7f7a0f8c0e59 - <rustc_metadata[13636c83be9747c7]::rmeta::decoder::MetadataBlob>::get_root
  23:     0x7f7a0f973c01 - <rustc_metadata[13636c83be9747c7]::locator::CrateLocator>::extract_one
  24:     0x7f7a0f97306f - <rustc_metadata[13636c83be9747c7]::locator::CrateLocator>::extract_lib
  25:     0x7f7a0f970c01 - <rustc_metadata[13636c83be9747c7]::locator::CrateLocator>::maybe_load_library_crate
  26:     0x7f7a0f909a8a - <rustc_metadata[13636c83be9747c7]::creader::CrateLoader>::load
  27:     0x7f7a0f905fe4 - <rustc_metadata[13636c83be9747c7]::creader::CrateLoader>::maybe_resolve_crate
  28:     0x7f7a0f90c8ef - <rustc_metadata[13636c83be9747c7]::creader::CrateLoader>::maybe_process_path_extern
  29:     0x7f7a0e779719 - <rustc_resolve[62267bb9da75df5a]::Resolver>::extern_prelude_get
  30:     0x7f7a0e7621aa - <rustc_resolve[62267bb9da75df5a]::Resolver>::early_resolve_ident_in_lexical_scope
  31:     0x7f7a0e775512 - <rustc_resolve[62267bb9da75df5a]::Resolver>::resolve_path_with_ribs::{closure#1}
  32:     0x7f7a0e77327e - <rustc_resolve[62267bb9da75df5a]::Resolver>::resolve_path_with_ribs
  33:     0x7f7a0e6f57bb - <rustc_resolve[62267bb9da75df5a]::imports::ImportResolver>::resolve_imports
  34:     0x7f7a0e75e2ce - <rustc_resolve[62267bb9da75df5a]::Resolver as rustc_expand[efa6bc5acd30c03b]::base::ResolverExpand>::resolve_imports
  35:     0x7f7a0fac4f84 - <rustc_expand[efa6bc5acd30c03b]::expand::MacroExpander>::fully_expand_fragment
  36:     0x7f7a0fac4bd4 - <rustc_expand[efa6bc5acd30c03b]::expand::MacroExpander>::expand_crate
  37:     0x7f7a0dc47d2b - <rustc_session[d8904b8db467f383]::session::Session>::time::<core[8382fcd636289ab6]::result::Result<rustc_ast[4c5af1b4f56e9b2b]::ast::Crate, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[7a474fd684de44d0]::passes::configure_and_expand::{closure#1}>
  38:     0x7f7a0dcddbd2 - rustc_interface[7a474fd684de44d0]::passes::configure_and_expand
  39:     0x7f7a0dceef3d - <rustc_interface[7a474fd684de44d0]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[7a474fd684de44d0]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[8382fcd636289ab6]::result::Result<rustc_ast[4c5af1b4f56e9b2b]::ast::Crate, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  40:     0x7f7a0dc7f6fd - <rustc_interface[7a474fd684de44d0]::queries::Queries>::expansion
  41:     0x7f7a0db4da12 - <rustc_interface[7a474fd684de44d0]::interface::Compiler>::enter::<rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}::{closure#2}, core[8382fcd636289ab6]::result::Result<core[8382fcd636289ab6]::option::Option<rustc_interface[7a474fd684de44d0]::queries::Linker>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  42:     0x7f7a0db5a829 - rustc_span[f78664e4d3610ddc]::with_source_map::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[7a474fd684de44d0]::interface::create_compiler_and_run<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f7a0db4b35d - <scoped_tls[6532d0c6e5128321]::ScopedKey<rustc_span[f78664e4d3610ddc]::SessionGlobals>>::set::<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  44:     0x7f7a0db05ce9 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  45:     0x7f7a0db641d1 - std[9f788053d5631f34]::panic::catch_unwind::<core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  46:     0x7f7a0db554bb - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[7a474fd684de44d0]::util::run_in_thread_pool_with_globals<rustc_interface[7a474fd684de44d0]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[7387008bc487c86e]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f7a0d077783 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  48:     0x7f7a075ca609 - start_thread
  49:     0x7f7a0cedd163 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (6587e80b9 2022-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
