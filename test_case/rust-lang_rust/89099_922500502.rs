
➜  ~ rustup default nightly
info: using existing install for 'nightly-aarch64-apple-darwin'
info: default toolchain set to 'nightly-aarch64-apple-darwin'

  nightly-aarch64-apple-darwin unchanged - rustc 1.57.0-nightly (aa8f2d432 2021-09-18)
➜  unm-server-rust git:(main) cargo init 
     Created binary (application) package
➜  unm-server-rust git:(main) ✗ RUST_BACKTRACE=full cargo run
   Compiling unm-server-rust v0.1.0 (/Users/pan93412/Projects/unm-server-rust)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: "Provided allocation has wrong size for slot count 131072"', compiler/rustc_metadata/src/rmeta/decoder.rs:263:29
stack backtrace:
   0:        0x100a3ef4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha0c23f46485c95e1
   1:        0x100a89314 - core::fmt::write::h5640551a9f41f06f
   2:        0x100a31434 - std::io::Write::write_fmt::hbeca16f559aa2017
   3:        0x100a41ffc - std::panicking::default_hook::{{closure}}::h48db6443cc1d76a7
   4:        0x100a41bc8 - std::panicking::default_hook::h08cafb1fd4b9edfc
   5:        0x1073ed9ec - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hf27d8a7076fb0fb5
   6:        0x100a427d0 - std::panicking::rust_panic_with_hook::h785bbc075dd03245
   7:        0x100a422f0 - std::panicking::begin_panic_handler::{{closure}}::hee40acbf37d05ae7
   8:        0x100a3f40c - std::sys_common::backtrace::__rust_end_short_backtrace::hbf9df9a363e7e39e
   9:        0x100a42258 - _rust_begin_unwind
  10:        0x100ab2ac0 - core::panicking::panic_fmt::heba7d60a4ad9cbd6
  11:        0x100ab2b64 - core::result::unwrap_failed::hd448f154fa370e8c
  12:        0x10a588094 - rustc_metadata::rmeta::decoder::CrateMetadata::new::h318ae4b00517b39c
  13:        0x10a53e928 - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h05c1ea113f2b4abd
  14:        0x10a53e16c - rustc_metadata::creader::CrateLoader::maybe_resolve_crate::h05c1ea113f2b4abd
  15:        0x10a541b10 - rustc_metadata::creader::CrateLoader::process_extern_crate::h9ed66d5f11a35407
  16:        0x1098b62c0 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item::h8ca819317c3b668a
  17:        0x109928d0c - rustc_ast::visit::walk_item::hee4b1a290149d26b
  18:        0x1098b7894 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_item::h8ca819317c3b668a
  19:        0x1098bce64 - rustc_expand::expand::AstFragment::visit_with::h95e77cace9477e8d
  20:        0x1098fcff8 - rustc_resolve::macros::<impl rustc_expand::base::ResolverExpand for rustc_resolve::Resolver>::visit_ast_fragment_with_placeholders::h069eb027fb44ad9f
  21:        0x10a6ad374 - rustc_expand::expand::MacroExpander::collect_invocations::haaa9200685e248fc
  22:        0x10a6a980c - rustc_expand::expand::MacroExpander::fully_expand_fragment::h804e3fd2ba6e6676
  23:        0x10a6a9024 - rustc_expand::expand::MacroExpander::expand_crate::h6fd1435d664fd8ba
  24:        0x1075546cc - rustc_session::utils::<impl rustc_session::session::Session>::time::h740eba2c43371c7d
  25:        0x1074df420 - rustc_interface::passes::configure_and_expand::h309f36a265d9cb51
  26:        0x1074e2ae4 - rustc_interface::queries::Queries::expansion::h3a3f9860778bd758
  27:        0x10742366c - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hf6baf8ac33727c7c
  28:        0x1073f8260 - rustc_span::with_source_map::h3459f8d2bd5b3e80
  29:        0x10742218c - scoped_tls::ScopedKey<T>::set::h06f8b74c0766ee27
  30:        0x1073fd700 - std::sys_common::backtrace::__rust_begin_short_backtrace::h96e536b9748c9e30
  31:        0x1073f044c - core::ops::function::FnOnce::call_once{{vtable.shim}}::h27dca340fbc11bdd
  32:        0x100a4d38c - std::sys::unix::thread::Thread::new::thread_start::h7583570679bee9a1
  33:        0x18302d5a8 - _pthread_from_mach_thread_np

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (aa8f2d432 2021-09-18) running on aarch64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `unm-server-rust`
