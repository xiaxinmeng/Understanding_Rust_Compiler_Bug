
thread 'rustc' panicked at 'index out of bounds: the len is 51321 but the index is 51321', /rust/compiler/rustc_serialize/src/opaque.rs:594:21
stack backtrace:
   0:     0x7f69faa6df29 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he5f97e1149b4b98d
   1:     0x7f69faae2dae - core::fmt::write::h4054f33d714f2e48
   2:     0x7f69faa4c955 - std::io::Write::write_fmt::h7033bd5c1b9da704
   3:     0x7f69faa6dce5 - std::sys_common::backtrace::print::ha1e8dfaf994e1661
   4:     0x7f69faa3ef8f - std::panicking::default_hook::{{closure}}::h6d8b1db20a2c55b5
   5:     0x7f69faa3ec55 - std::panicking::default_hook::h26618ab0f07fb3ee
   6:     0x7f69fb3303a5 - rustc_driver_impl[591904b1837e992b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f69faa3fd0d - std::panicking::rust_panic_with_hook::h51be139a6193b4d1
   8:     0x7f69faa6e259 - std::panicking::begin_panic_handler::{{closure}}::hd5264bde5e5b2f91
   9:     0x7f69faa6e036 - std::sys_common::backtrace::__rust_end_short_backtrace::h51c8ac09f2a0d06d
  10:     0x7f69faa3f862 - rust_begin_unwind
  11:     0x7f69faa14bf3 - core::panicking::panic_fmt::hae108289130a6835
  12:     0x7f69faa14d62 - core::panicking::panic_bounds_check::h7120c0eff84ec7f3
  13:     0x7f69fd11da5a - <rustc_metadata[79791ad705d4ab19]::rmeta::CrateRoot as rustc_serialize[28d70643afa36a2c]::serialize::Decodable<rustc_metadata[79791ad705d4ab19]::rmeta::decoder::DecodeContext>>::decode
  14:     0x7f69fd058132 - <rustc_metadata[79791ad705d4ab19]::locator::CrateLocator>::extract_one
  15:     0x7f69fd05709a - <rustc_metadata[79791ad705d4ab19]::locator::CrateLocator>::extract_lib
  16:     0x7f69fd055c57 - <rustc_metadata[79791ad705d4ab19]::locator::CrateLocator>::find_library_crate
  17:     0x7f69fd053e60 - <rustc_metadata[79791ad705d4ab19]::locator::CrateLocator>::maybe_load_library_crate
  18:     0x7f69fd0f565d - <rustc_metadata[79791ad705d4ab19]::creader::CrateLoader>::load
  19:     0x7f69fd0f19b3 - <rustc_metadata[79791ad705d4ab19]::creader::CrateLoader>::maybe_resolve_crate
  20:     0x7f69fd0f3a99 - <rustc_metadata[79791ad705d4ab19]::creader::CrateLoader>::maybe_resolve_crate
  21:     0x7f69fd0f81ad - <rustc_metadata[79791ad705d4ab19]::creader::CrateLoader>::maybe_process_path_extern
  22:     0x7f69fbf9303a - <rustc_resolve[39f173fbac3dd876]::Resolver>::extern_prelude_get
  23:     0x7f69fbf967a4 - <rustc_resolve[39f173fbac3dd876]::Resolver>::early_resolve_ident_in_lexical_scope
  24:     0x7f69fbf752bb - <rustc_resolve[39f173fbac3dd876]::Resolver>::resolve_path_with_ribs
  25:     0x7f69fbf80ef0 - <rustc_resolve[39f173fbac3dd876]::Resolver as rustc_expand[2c6503e5439025d1]::base::ResolverExpand>::resolve_imports
  26:     0x7f69fd2b37a5 - <rustc_expand[2c6503e5439025d1]::expand::MacroExpander>::fully_expand_fragment
  27:     0x7f69fd2b2ea0 - <rustc_expand[2c6503e5439025d1]::expand::MacroExpander>::expand_crate
  28:     0x7f69fb3bf2d1 - <rustc_session[4219d3d64f21f04b]::session::Session>::time::<rustc_ast[4aef29eb81a9ce2]::ast::Crate, rustc_interface[b270ec05995dc8d3]::passes::configure_and_expand::{closure#1}>
  29:     0x7f69fb397451 - rustc_interface[b270ec05995dc8d3]::passes::resolver_for_lowering
  30:     0x7f69fcc12f8c - rustc_query_system[827217707b92434c]::query::plumbing::try_execute_query::<rustc_query_impl[bb470f4c8b345078]::queries::resolver_for_lowering, rustc_query_impl[bb470f4c8b345078]::plumbing::QueryCtxt>
  31:     0x7f69fc93c9e5 - <rustc_query_impl[bb470f4c8b345078]::Queries as rustc_middle[ee4938beb2c48787]::ty::query::QueryEngine>::resolver_for_lowering
  32:     0x7f69fb338e5c - <rustc_middle[ee4938beb2c48787]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[591904b1837e992b]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[bd3456f1c2888d2b]::steal::Steal<(rustc_middle[ee4938beb2c48787]::ty::ResolverAstLowering, alloc[dc8a9bb7d5e1b453]::rc::Rc<rustc_ast[4aef29eb81a9ce2]::ast::Crate>)>>
  33:     0x7f69fb304a92 - rustc_span[5b47611f73d93c88]::with_source_map::<core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>, rustc_interface[b270ec05995dc8d3]::interface::run_compiler<core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>, rustc_driver_impl[591904b1837e992b]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7f69fb2d60f9 - std[91a00bd35404f355]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b270ec05995dc8d3]::util::run_in_thread_pool_with_globals<rustc_interface[b270ec05995dc8d3]::interface::run_compiler<core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>, rustc_driver_impl[591904b1837e992b]::run_compiler::{closure#1}>::{closure#0}, core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>>
  35:     0x7f69fb34f69d - <<std[91a00bd35404f355]::thread::Builder>::spawn_unchecked_<rustc_interface[b270ec05995dc8d3]::util::run_in_thread_pool_with_globals<rustc_interface[b270ec05995dc8d3]::interface::run_compiler<core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>, rustc_driver_impl[591904b1837e992b]::run_compiler::{closure#1}>::{closure#0}, core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6ea3fe8105c2c65d]::result::Result<(), rustc_span[5b47611f73d93c88]::ErrorGuaranteed>>::{closure#1} as core[6ea3fe8105c2c65d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f69faa78de3 - std::sys::unix::thread::Thread::new::thread_start::hc2566c2f0887626b
  37:     0x7f69f3a88609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  38:     0x7f69fa8b8133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  39:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
error: could not compile `tracing-subscriber`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:20:33
