plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
 Documenting rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
 Documenting rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
 Documenting rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
thread 'rustc' panicked at 'Normalizing Binder(&rustc_ast::Attribute, []) without wrapping in a `Binder`', /checkout/compiler/rustc_trait_selection/src/traits/project.rs:437:9
   0:     0x7efe56f7ade5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c47460db0b1731
   0:     0x7efe56f7ade5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3c47460db0b1731
   1:     0x7efe56fe7e9b - core::fmt::write::h43180f6091a88d45
   2:     0x7efe56f6f221 - std::io::Write::write_fmt::h6624c2423e67eda6
   3:     0x7efe56f7abf5 - std::sys_common::backtrace::print::h39ae6eb6c7dad170
   4:     0x7efe56f7de14 - std::panicking::default_hook::{{closure}}::h99071314fd10fff2
   5:     0x7efe56f7db49 - std::panicking::default_hook::had6237dabbb0aa58
   6:     0x7efe57a02215 - rustc_driver_impl[5ba596da2f7eeb2f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efe56f7e500 - std::panicking::rust_panic_with_hook::hcdb18fd1b8cf4f67
   8:     0x7efe56f7e279 - std::panicking::begin_panic_handler::{{closure}}::h59defd92e5732ef5
   9:     0x7efe56f7b2a6 - std::sys_common::backtrace::__rust_end_short_backtrace::h9ab17d514c30a5de
  10:     0x7efe56f7df92 - rust_begin_unwind
  11:     0x7efe56f38fc3 - core::panicking::panic_fmt::h5b86afdf3c41e70b
  12:     0x564376eec885 - <rustc_trait_selection[9768cc72ef662258]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[5a8556b6a3465e]::ty::sty::Binder<rustc_middle[5a8556b6a3465e]::ty::Ty>>
  13:     0x564376efc419 - rustc_trait_selection[9768cc72ef662258]::traits::project::normalize_with_depth::<rustc_middle[5a8556b6a3465e]::ty::sty::Binder<rustc_middle[5a8556b6a3465e]::ty::Ty>>
  14:     0x5643770733fc - <rustc_trait_selection[9768cc72ef662258]::traits::engine::ObligationCtxt>::normalize::<rustc_middle[5a8556b6a3465e]::ty::sty::Binder<rustc_middle[5a8556b6a3465e]::ty::Ty>>
  15:     0x564376f2e5ad - rustdoc[61a59996b81154e6]::clean::normalize
  16:     0x564376f3c585 - rustdoc[61a59996b81154e6]::clean::clean_middle_ty
  17:     0x564376f2558e - rustdoc[61a59996b81154e6]::clean::clean_middle_term
  18:     0x564376f13b99 - <&mut rustdoc[61a59996b81154e6]::clean::clean_middle_opaque_bounds::{closure#0} as core[d8af018d09f3032c]::ops::function::FnMut<(&rustc_middle[5a8556b6a3465e]::ty::Predicate,)>>::call_mut
  19:     0x5643771016de - <alloc[11653a588ca9e597]::vec::Vec<rustdoc[61a59996b81154e6]::clean::types::GenericBound> as alloc[11653a588ca9e597]::vec::spec_from_iter::SpecFromIter<rustdoc[61a59996b81154e6]::clean::types::GenericBound, core[d8af018d09f3032c]::iter::adapters::filter_map::FilterMap<core[d8af018d09f3032c]::slice::iter::Iter<rustc_middle[5a8556b6a3465e]::ty::Predicate>, rustdoc[61a59996b81154e6]::clean::clean_middle_opaque_bounds::{closure#0}>>>::from_iter
  20:     0x564376f2ee7d - rustdoc[61a59996b81154e6]::clean::clean_middle_opaque_bounds
  21:     0x564376f3ca3f - rustdoc[61a59996b81154e6]::clean::clean_middle_ty
  22:     0x564376f2b066 - rustdoc[61a59996b81154e6]::clean::clean_fn_decl_from_did_and_sig
  23:     0x5643770ae94e - rustdoc[61a59996b81154e6]::clean::utils::enter_impl_trait::<rustdoc[61a59996b81154e6]::clean::inline::build_external_function::{closure#1}, (rustdoc[61a59996b81154e6]::clean::types::Generics, rustdoc[61a59996b81154e6]::clean::types::FnDecl)>
  24:     0x564376f1f4a0 - rustdoc[61a59996b81154e6]::clean::inline::build_external_function
  25:     0x564376f1e446 - rustdoc[61a59996b81154e6]::clean::inline::try_inline
  26:     0x564376f22d8b - rustdoc[61a59996b81154e6]::clean::inline::build_module_items
  27:     0x564376f322cb - rustdoc[61a59996b81154e6]::clean::clean_use_statement
  28:     0x5643770f840c - <alloc[11653a588ca9e597]::vec::Vec<rustdoc[61a59996b81154e6]::clean::types::Item> as alloc[11653a588ca9e597]::vec::spec_extend::SpecExtend<rustdoc[61a59996b81154e6]::clean::types::Item, core[d8af018d09f3032c]::iter::adapters::flatten::FlatMap<indexmap[1faa4a10b8767960]::map::Values<(rustc_span[914dbaa788e854e9]::def_id::LocalDefId, core[d8af018d09f3032c]::option::Option<rustc_span[914dbaa788e854e9]::symbol::Symbol>), (&rustc_hir[b0ec0b1e60ce2431]::hir::Item, core[d8af018d09f3032c]::option::Option<rustc_span[914dbaa788e854e9]::symbol::Symbol>, core[d8af018d09f3032c]::option::Option<rustc_span[914dbaa788e854e9]::def_id::LocalDefId>)>, alloc[11653a588ca9e597]::vec::Vec<rustdoc[61a59996b81154e6]::clean::types::Item>, rustdoc[61a59996b81154e6]::clean::clean_doc_module::{closure#3}>>>::spec_extend
  29:     0x564376f23be1 - rustdoc[61a59996b81154e6]::clean::clean_doc_module
  30:     0x5643770a64f2 - rustdoc[61a59996b81154e6]::clean::utils::krate
  31:     0x564376e44aca - <rustc_session[fc1150e1707fe055]::session::Session>::time::<rustdoc[61a59996b81154e6]::clean::types::Crate, rustdoc[61a59996b81154e6]::core::run_global_ctxt::{closure#5}>
  32:     0x564376ed805d - rustdoc[61a59996b81154e6]::core::run_global_ctxt
  33:     0x564376e44e7e - <rustc_session[fc1150e1707fe055]::session::Session>::time::<(rustdoc[61a59996b81154e6]::clean::types::Crate, rustdoc[61a59996b81154e6]::config::RenderOptions, rustdoc[61a59996b81154e6]::formats::cache::Cache), rustdoc[61a59996b81154e6]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  34:     0x564376f9e1cb - <std[13aae9ad84224e5e]::thread::local::LocalKey<core[d8af018d09f3032c]::cell::Cell<*const ()>>>::with::<rustc_middle[5a8556b6a3465e]::ty::context::tls::enter_context<<rustc_middle[5a8556b6a3465e]::ty::context::GlobalCtxt>::enter<rustdoc[61a59996b81154e6]::main_args::{closure#1}::{closure#0}::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>
  35:     0x564376e46e2b - <rustc_interface[f7a18016aee5ac55]::queries::QueryResult<&rustc_middle[5a8556b6a3465e]::ty::context::GlobalCtxt>>::enter::<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustdoc[61a59996b81154e6]::main_args::{closure#1}::{closure#0}::{closure#0}>
  36:     0x56437700bd63 - <rustc_interface[f7a18016aee5ac55]::interface::Compiler>::enter::<rustdoc[61a59996b81154e6]::main_args::{closure#1}::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>
  37:     0x564376e4422b - rustc_span[914dbaa788e854e9]::with_source_map::<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustc_interface[f7a18016aee5ac55]::interface::run_compiler<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustdoc[61a59996b81154e6]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x564376fa1eac - <scoped_tls[a7a51cb469a8fe7a]::ScopedKey<rustc_span[914dbaa788e854e9]::SessionGlobals>>::set::<rustc_interface[f7a18016aee5ac55]::interface::run_compiler<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustdoc[61a59996b81154e6]::main_args::{closure#1}>::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>
  39:     0x564376eabb69 - std[13aae9ad84224e5e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7a18016aee5ac55]::util::run_in_thread_pool_with_globals<rustc_interface[f7a18016aee5ac55]::interface::run_compiler<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustdoc[61a59996b81154e6]::main_args::{closure#1}>::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>
  40:     0x564376f1128e - <<std[13aae9ad84224e5e]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a18016aee5ac55]::util::run_in_thread_pool_with_globals<rustc_interface[f7a18016aee5ac55]::interface::run_compiler<core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>, rustdoc[61a59996b81154e6]::main_args::{closure#1}>::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[d8af018d09f3032c]::result::Result<(), rustc_span[914dbaa788e854e9]::ErrorGuaranteed>>::{closure#1} as core[d8af018d09f3032c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7efe56f8aa1e - std::sys::unix::thread::Thread::new::thread_start::hc13e8dde5baa769e
  42:     0x7efe56e87609 - start_thread
  43:     0x7efe56c5b133 - clone
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c992531b0 2023-03-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z unstable-options -Z normalize-docs -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `rustc_attr`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_attr compiler/rustc_attr/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=f6265d549db2cc35 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-438033e2342a03e4.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-b512da0e0672bce1.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-371c916390f2781b.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-0648b71f3e9ffa00.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-4472eb92cba87b31.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-b3248e1f30bf40a6.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d527f1ae35ed44ed.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-83a315f8a8d9062f.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-09b2c76f59d7a7d6.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-321ece9b35ebbe5d.rmeta --cfg=windows_raw_dylib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' '--check-cfg=values(windows_raw_dylib)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (c992531b0
  2023-03-31)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 101)
[RUSTC-TIMING] rustc_attr test:false 0.493
[RUSTC-TIMING] rustc_query_system test:false 1.039
[RUSTC-TIMING] rustc_parse test:false 3.511
Build completed unsuccessfully in 0:20:36
