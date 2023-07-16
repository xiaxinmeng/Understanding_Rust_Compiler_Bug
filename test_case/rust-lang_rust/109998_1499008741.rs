plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
 Documenting rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
 Documenting rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
 Documenting rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
thread 'rustc' panicked at 'Normalizing Binder(&rustc_ast::Attribute, []) without wrapping in a `Binder`', /checkout/compiler/rustc_trait_selection/src/traits/project.rs:437:9
   0:     0x7f56b0e42de5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8eea7fd4829d219a
   0:     0x7f56b0e42de5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8eea7fd4829d219a
   1:     0x7f56b0eafe9b - core::fmt::write::hf162ee32847f92f0
   2:     0x7f56b0e372c1 - std::io::Write::write_fmt::hd280d4b937f2d1b4
   3:     0x7f56b0e42bf5 - std::sys_common::backtrace::print::hd2f43a5f22d5c603
   4:     0x7f56b0e45e14 - std::panicking::default_hook::{{closure}}::hf91b7bf416a1c599
   5:     0x7f56b0e45b49 - std::panicking::default_hook::h9f52416b042e894d
   6:     0x7f56b18cb8c5 - rustc_driver_impl[f9a67899b9994f4e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f56b0e46500 - std::panicking::rust_panic_with_hook::hde73e282102451f0
   8:     0x7f56b0e46279 - std::panicking::begin_panic_handler::{{closure}}::hee595c2ca214e03c
   9:     0x7f56b0e432a6 - std::sys_common::backtrace::__rust_end_short_backtrace::h86aec12e0878d9c0
  10:     0x7f56b0e45f92 - rust_begin_unwind
  11:     0x7f56b0e00fc3 - core::panicking::panic_fmt::ha3c789dc8f63625a
  12:     0x55b93c3d8b50 - <rustc_trait_selection[6d16f1d881fadb4a]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[c662240a5abd7afb]::ty::sty::Binder<rustc_middle[c662240a5abd7afb]::ty::Ty>>
  13:     0x55b93c3e9b19 - rustc_trait_selection[6d16f1d881fadb4a]::traits::project::normalize_with_depth::<rustc_middle[c662240a5abd7afb]::ty::sty::Binder<rustc_middle[c662240a5abd7afb]::ty::Ty>>
  14:     0x55b93c537e96 - <rustc_trait_selection[6d16f1d881fadb4a]::traits::engine::ObligationCtxt>::normalize::<rustc_middle[c662240a5abd7afb]::ty::sty::Binder<rustc_middle[c662240a5abd7afb]::ty::Ty>>
  15:     0x55b93c412fdd - rustdoc[17f5ba4ad2af84f3]::clean::normalize
  16:     0x55b93c428f25 - rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_ty
  17:     0x55b93c409f5e - rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_term
  18:     0x55b93c3f5489 - <&mut rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_opaque_bounds::{closure#0} as core[8ef3115f986744cb]::ops::function::FnMut<(&rustc_middle[c662240a5abd7afb]::ty::Predicate,)>>::call_mut
  19:     0x55b93c5f118e - <alloc[940fa5d620bb22ec]::vec::Vec<rustdoc[17f5ba4ad2af84f3]::clean::types::GenericBound> as alloc[940fa5d620bb22ec]::vec::spec_from_iter::SpecFromIter<rustdoc[17f5ba4ad2af84f3]::clean::types::GenericBound, core[8ef3115f986744cb]::iter::adapters::filter_map::FilterMap<core[8ef3115f986744cb]::slice::iter::Iter<rustc_middle[c662240a5abd7afb]::ty::Predicate>, rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_opaque_bounds::{closure#0}>>>::from_iter
  20:     0x55b93c4138ad - rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_opaque_bounds
  21:     0x55b93c4293df - rustdoc[17f5ba4ad2af84f3]::clean::clean_middle_ty
  22:     0x55b93c40fa47 - rustdoc[17f5ba4ad2af84f3]::clean::clean_fn_decl_from_did_and_sig
  23:     0x55b93c59e1be - rustdoc[17f5ba4ad2af84f3]::clean::utils::enter_impl_trait::<rustdoc[17f5ba4ad2af84f3]::clean::inline::build_external_function::{closure#1}, (rustdoc[17f5ba4ad2af84f3]::clean::types::Generics, rustdoc[17f5ba4ad2af84f3]::clean::types::FnDecl)>
  24:     0x55b93c403cf0 - rustdoc[17f5ba4ad2af84f3]::clean::inline::build_external_function
  25:     0x55b93c402c96 - rustdoc[17f5ba4ad2af84f3]::clean::inline::try_inline
  26:     0x55b93c40774b - rustdoc[17f5ba4ad2af84f3]::clean::inline::build_module_items
  27:     0x55b93c416f6b - rustdoc[17f5ba4ad2af84f3]::clean::clean_use_statement
  28:     0x55b93c5e844c - <alloc[940fa5d620bb22ec]::vec::Vec<rustdoc[17f5ba4ad2af84f3]::clean::types::Item> as alloc[940fa5d620bb22ec]::vec::spec_extend::SpecExtend<rustdoc[17f5ba4ad2af84f3]::clean::types::Item, core[8ef3115f986744cb]::iter::adapters::flatten::FlatMap<indexmap[7d9220d4c04f2bfc]::map::Values<(rustc_span[72800ab0c6dfb8e0]::def_id::LocalDefId, core[8ef3115f986744cb]::option::Option<rustc_span[72800ab0c6dfb8e0]::symbol::Symbol>), (&rustc_hir[2a4109c7a44a3292]::hir::Item, core[8ef3115f986744cb]::option::Option<rustc_span[72800ab0c6dfb8e0]::symbol::Symbol>, core[8ef3115f986744cb]::option::Option<rustc_span[72800ab0c6dfb8e0]::def_id::LocalDefId>)>, alloc[940fa5d620bb22ec]::vec::Vec<rustdoc[17f5ba4ad2af84f3]::clean::types::Item>, rustdoc[17f5ba4ad2af84f3]::clean::clean_doc_module::{closure#3}>>>::spec_extend
  29:     0x55b93c4085a1 - rustdoc[17f5ba4ad2af84f3]::clean::clean_doc_module
  30:     0x55b93c595d42 - rustdoc[17f5ba4ad2af84f3]::clean::utils::krate
  31:     0x55b93c2e835a - <rustc_session[f718e86b03507a3c]::session::Session>::time::<rustdoc[17f5ba4ad2af84f3]::clean::types::Crate, rustdoc[17f5ba4ad2af84f3]::core::run_global_ctxt::{closure#5}>
  32:     0x55b93c3cae4d - rustdoc[17f5ba4ad2af84f3]::core::run_global_ctxt
  33:     0x55b93c2ea49e - <rustc_session[f718e86b03507a3c]::session::Session>::time::<(rustdoc[17f5ba4ad2af84f3]::clean::types::Crate, rustdoc[17f5ba4ad2af84f3]::config::RenderOptions, rustdoc[17f5ba4ad2af84f3]::formats::cache::Cache), rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  34:     0x55b93c48f02b - <std[cb18330984bee514]::thread::local::LocalKey<core[8ef3115f986744cb]::cell::Cell<*const ()>>>::with::<rustc_middle[c662240a5abd7afb]::ty::context::tls::enter_context<<rustc_middle[c662240a5abd7afb]::ty::context::GlobalCtxt>::enter<rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}::{closure#0}::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>
  35:     0x55b93c37efdb - <rustc_interface[ea221008a09ba587]::queries::QueryResult<&rustc_middle[c662240a5abd7afb]::ty::context::GlobalCtxt>>::enter::<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}::{closure#0}::{closure#0}>
  36:     0x55b93c536e55 - <rustc_interface[ea221008a09ba587]::interface::Compiler>::enter::<rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>
  37:     0x55b93c37e47b - rustc_span[72800ab0c6dfb8e0]::set_source_map::<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustc_interface[ea221008a09ba587]::interface::run_compiler<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x55b93c4929ac - <scoped_tls[31d6026428df6d6e]::ScopedKey<rustc_span[72800ab0c6dfb8e0]::SessionGlobals>>::set::<rustc_interface[ea221008a09ba587]::interface::run_compiler<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}>::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>
  39:     0x55b93c391a19 - std[cb18330984bee514]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ea221008a09ba587]::util::run_in_thread_pool_with_globals<rustc_interface[ea221008a09ba587]::interface::run_compiler<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}>::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>
  40:     0x55b93c44cfb1 - <<std[cb18330984bee514]::thread::Builder>::spawn_unchecked_<rustc_interface[ea221008a09ba587]::util::run_in_thread_pool_with_globals<rustc_interface[ea221008a09ba587]::interface::run_compiler<core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>, rustdoc[17f5ba4ad2af84f3]::main_args::{closure#1}>::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[8ef3115f986744cb]::result::Result<(), rustc_span[72800ab0c6dfb8e0]::ErrorGuaranteed>>::{closure#1} as core[8ef3115f986744cb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f56b0e52a1e - std::sys::unix::thread::Thread::new::thread_start::h093842d4cff5c527
  42:     0x7f56b0d4f609 - start_thread
  43:     0x7f56b0b23133 - clone
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (d957ef74d 2023-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z unstable-options -Z normalize-docs -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `rustc_attr`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_attr compiler/rustc_attr/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8e24c5cdd156c2b2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-b03e3cf1de591417.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-f78b299d4967ce0e.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3594b770f37815fd.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2a04c1cb0295af68.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-83fef33a537b4044.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-b3248e1f30bf40a6.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-d527f1ae35ed44ed.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-83a315f8a8d9062f.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-ac71c4b10cf8356f.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-96fd5490d6a89b15.rmeta --cfg=windows_raw_dylib -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' '--check-cfg=values(windows_raw_dylib)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.70.0-nightly
  (d957ef74d
  2023-04-06)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 101)
[RUSTC-TIMING] rustc_attr test:false 0.495
[RUSTC-TIMING] rustc_query_system test:false 1.040
[RUSTC-TIMING] rustc_parse test:false 3.500
Build completed unsuccessfully in 0:20:36
