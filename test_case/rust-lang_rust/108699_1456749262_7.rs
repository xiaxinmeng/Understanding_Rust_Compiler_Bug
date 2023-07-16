text
---- [ui] tests/rustdoc-ui/intra-doc/issue-108653-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/var/tmp/rust-build/x86_64-unknown-linux-gnu/stage1/bin/rustdoc" "/home/michael/Development/rust/tests/rustdoc-ui/intra-doc/issue-108653-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/var/tmp/rust-build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/issue-108653-3" "-Cdebuginfo=0" "-Lnative=/var/tmp/rust-build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/var/tmp/rust-build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/issue-108653-3/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'DefId(1:11857 ~ std[b2d9]::prim_u32) does not have a "type_of"', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:204:1
stack backtrace:
   0:     0x7f238cf04849 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h78f6bced1ab9b952
   1:     0x7f238cf78a3e - core::fmt::write::h5341206de545b898
   2:     0x7f238cec56b5 - std::io::Write::write_fmt::hbeb8ef999f3c7868
   3:     0x7f238cf04605 - std::sys_common::backtrace::print::h0da51f2725c33842
   4:     0x7f238cecd55f - std::panicking::default_hook::{{closure}}::h619e143597771e26
   5:     0x7f238cecd21c - std::panicking::default_hook::h91aabafbf64f1a6b
   6:     0x7f238d880ad5 - rustc_driver_impl[1dcb38fcc6e7e77c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f238cecdf2d - std::panicking::rust_panic_with_hook::h3a745a637417bf43
   8:     0x7f238cf05a89 - std::panicking::begin_panic_handler::{{closure}}::h5d490f47745c1d00
   9:     0x7f238cf0498c - std::sys_common::backtrace::__rust_end_short_backtrace::hb67f613a530a3f9b
  10:     0x7f238cecda92 - rust_begin_unwind
  11:     0x7f238cea8de3 - core::panicking::panic_fmt::h497dcee45bc95959
  12:     0x7f238f8c7a70 - rustc_metadata[bb7e47e5e4c187ff]::rmeta::decoder::cstore_impl::provide_extern::type_of::{closure#2}
  13:     0x7f238f8c7972 - rustc_metadata[bb7e47e5e4c187ff]::rmeta::decoder::cstore_impl::provide_extern::type_of
  14:     0x7f238f4eb791 - rustc_query_system[b7c2d4a4947a6b57]::query::plumbing::try_execute_query::<rustc_query_impl[8b60fe27614f6a9d]::queries::type_of, rustc_query_impl[8b60fe27614f6a9d]::plumbing::QueryCtxt>
  15:     0x7f238f2a86f5 - <rustc_query_impl[8b60fe27614f6a9d]::Queries as rustc_middle[8f8cc58a38a08b32]::ty::query::QueryEngine>::type_of
  16:     0x5564e34a0c96 - rustdoc[3f0501292528f1e0]::passes::collect_intra_doc_links::ambiguity_error
  17:     0x5564e349bd90 - <rustdoc[3f0501292528f1e0]::passes::collect_intra_doc_links::LinkCollector as rustdoc[3f0501292528f1e0]::visit::DocVisitor>::visit_item
  18:     0x5564e34a472a - <rustdoc[3f0501292528f1e0]::passes::collect_intra_doc_links::LinkCollector as rustdoc[3f0501292528f1e0]::visit::DocVisitor>::visit_inner_recur
  19:     0x5564e349d1db - <rustdoc[3f0501292528f1e0]::passes::collect_intra_doc_links::LinkCollector as rustdoc[3f0501292528f1e0]::visit::DocVisitor>::visit_item
  20:     0x5564e3494f4f - rustdoc[3f0501292528f1e0]::passes::collect_intra_doc_links::collect_intra_doc_links
  21:     0x5564e34a683e - <rustc_session[5867f2a876facb7]::session::Session>::time::<rustdoc[3f0501292528f1e0]::clean::types::Crate, rustdoc[3f0501292528f1e0]::core::run_global_ctxt::{closure#7}>
  22:     0x5564e3541e67 - rustdoc[3f0501292528f1e0]::core::run_global_ctxt
  23:     0x5564e34a6ac9 - <rustc_session[5867f2a876facb7]::session::Session>::time::<(rustdoc[3f0501292528f1e0]::clean::types::Crate, rustdoc[3f0501292528f1e0]::config::RenderOptions, rustdoc[3f0501292528f1e0]::formats::cache::Cache), rustdoc[3f0501292528f1e0]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  24:     0x5564e36aca6d - rustc_span[4085bd95152d41da]::with_source_map::<core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>, rustc_interface[258a4538246304e0]::interface::run_compiler<core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>, rustdoc[3f0501292528f1e0]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  25:     0x5564e33e3684 - <scoped_tls[b966ac58386d5624]::ScopedKey<rustc_span[4085bd95152d41da]::SessionGlobals>>::set::<rustc_interface[258a4538246304e0]::interface::run_compiler<core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>, rustdoc[3f0501292528f1e0]::main_args::{closure#1}>::{closure#0}, core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>>
  26:     0x5564e346ef50 - std[b5fab123d48d12b9]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[258a4538246304e0]::util::run_in_thread_pool_with_globals<rustc_interface[258a4538246304e0]::interface::run_compiler<core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>, rustdoc[3f0501292528f1e0]::main_args::{closure#1}>::{closure#0}, core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>>
  27:     0x5564e355f88d - <<std[b5fab123d48d12b9]::thread::Builder>::spawn_unchecked_<rustc_interface[258a4538246304e0]::util::run_in_thread_pool_with_globals<rustc_interface[258a4538246304e0]::interface::run_compiler<core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>, rustdoc[3f0501292528f1e0]::main_args::{closure#1}>::{closure#0}, core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[eec4a6d8192687cb]::result::Result<(), rustc_span[4085bd95152d41da]::ErrorGuaranteed>>::{closure#1} as core[eec4a6d8192687cb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7f238cf0cf93 - std::sys::unix::thread::Thread::new::thread_start::h6a3611c28007b704
  29:     0x7f238cce412d - start_thread
  30:     0x7f238cd65bc0 - clone3
  31:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C debuginfo=0

query stack during panic:
#0 [type_of] computing type of `std::prim_u32`
end of query stack
------------------------------------------
