`
   Compiling rustc-std-workspace-core v1.99.0 (/home/matthias/vcs/github/rust/src/tools/rustc-std-workspace-core)
thread 'rustc' panicked at 'internal error: entered unreachable code', <::std::macros::panic macros>:2:4
stack backtrace:
   0:     0x7f92e585f0c8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98cd3dee97c4cf65
   1:     0x7f92e58a60ec - core::fmt::write::h1f46f059eaf9219f
   2:     0x7f92e585e795 - std::io::Write::write_fmt::h36c86a593cbd1590
   3:     0x7f92e5836d85 - std::panicking::default_hook::{{closure}}::h9a3a41b67bb037fb
   4:     0x7f92e5836a76 - std::panicking::default_hook::h01aac4b9fe280ce2
   5:     0x7f92e6f11203 - rustc_driver::report_ice::hd2eba4ea9f27dbcc
   6:     0x7f92e58375c5 - std::panicking::rust_panic_with_hook::h6deaeefc56672e26
   7:     0x7f92e9a1ff9e - std::panicking::begin_panic::h880977661c4908d4
   8:     0x7f92e9b52b7e - <rustc_span::SourceFile as serialize::serialize::Decodable>::decode::hf67ea99b3e3a4dab
   9:     0x7f92e9a6b75d - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h22fb777da3d9a5a1
  10:     0x7f92e9b74e3c - rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::imported_source_files::h9946abe7c8f19abf
  11:     0x7f92e9b3a73a - <rustc_metadata::rmeta::decoder::DecodeContext as serialize::serialize::SpecializedDecoder<rustc_span::span_encoding::Span>>::specialized_decode::h169c8f045b63ef76
  12:     0x7f92e9b67d54 - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_metadata::creader::CStore>::item_children_untracked::h0032eb94dc3ab968
  13:     0x7f92e8f1e2cd - rustc_resolve::Resolver::resolutions::h79e8a0279a7176a7
  14:     0x7f92e8e5b459 - rustc_resolve::imports::ImportResolver::resolve_imports::hf8687998229af9f1
  15:     0x7f92e8f095be - rustc_resolve::macros::<impl rustc_expand::base::Resolver for rustc_resolve::Resolver>::resolve_imports::h71ad1801b43469a5
  16:     0x7f92e9cd96dc - rustc_expand::expand::MacroExpander::fully_expand_fragment::haf86c5ef2edf203a
  17:     0x7f92e9cd8ce4 - rustc_expand::expand::MacroExpander::expand_crate::h6eaf85d46e90c1c4
  18:     0x7f92e7105d68 - rustc_session::utils::<impl rustc_session::session::Session>::time::h079848db3bcfd3fb
  19:     0x7f92e71a1ac9 - rustc_interface::passes::configure_and_expand_inner::h7d9e62ef90bf2e65
  20:     0x7f92e70c83ef - rustc_interface::passes::configure_and_expand::{{closure}}::h4fff5724d27e7258
  21:     0x7f92e70ac6e1 - rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new::hb83bee823324c376
  22:     0x7f92e71a0b53 - rustc_interface::passes::configure_and_expand::ha157eb8331b33b71
  23:     0x7f92e70cb26e - rustc_interface::queries::Queries::expansion::h830b381995ac40d4
  24:     0x7f92e6f667f9 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h8aacb1822953387e
  25:     0x7f92e6f1a3ca - std::sys_common::backtrace::__rust_begin_short_backtrace::hd0bcef876e2e5413
  26:     0x7f92e5836ef4 - std::panicking::try::do_try::h68415d0eb0d1728a
  27:     0x7f92e6f6a25f - core::ops::function::FnOnce::call_once{{vtable.shim}}::h4916feb8f3d5ccf7
  28:     0x7f92e5835d1f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hebff0696e6b8e665
  29:     0x7f92e586422f - std::sys_common::thread::start_thread::h588d103079703071
  30:     0x7f92e5844716 - std::sys::unix::thread::Thread::new::thread_start::hc2107903fae320c0
  31:     0x7f92e54cb46f - start_thread
  32:     0x7f92e56873d3 - clone
  33:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C target-cpu=native -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `rustc-std-workspace-core`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/home/matthias/vcs/github/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /home/matthias/vcs/github/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:00:01
