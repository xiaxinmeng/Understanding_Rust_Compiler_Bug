plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
thread 'rustc' panicked at 'unexpected `def_kind` in `codegen_fn_attrs`: Const', compiler/rustc_typeck/src/collect.rs:2696:9
   0:     0x7f366474553d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h11237028a75a1850
   1:     0x7f36647add78 - core::fmt::write::h75736d5168df1a59
   2:     0x7f3664736641 - std::io::Write::write_fmt::h933f9eb9dcce85a1
   2:     0x7f3664736641 - std::io::Write::write_fmt::h933f9eb9dcce85a1
   3:     0x7f366474870e - std::panicking::default_hook::{{closure}}::h1f2a2c824eeeb999
   4:     0x7f366474845e - std::panicking::default_hook::h2355862caad73260
   5:     0x7f3665253514 - rustc_driver[e8d3c5195d92d990]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3664748e81 - std::panicking::rust_panic_with_hook::h9876449947bbdc93
   7:     0x7f3664748ca7 - std::panicking::begin_panic_handler::{{closure}}::hc72773511a05eeab
   8:     0x7f3664745ae4 - std::sys_common::backtrace::__rust_end_short_backtrace::h1d84a51dfb3646b1
   9:     0x7f3664748989 - rust_begin_unwind
  10:     0x7f36646fc043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f3665d9c181 - rustc_typeck[c0f26da51bbb1d6]::collect::codegen_fn_attrs
  12:     0x7f3666d3fc0f - rustc_query_system[8525185271f8bf9]::query::plumbing::try_execute_query::<rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt, rustc_query_system[8525185271f8bf9]::query::caches::ArenaCache<rustc_span[61b6b3eff38f7b69]::def_id::DefId, rustc_middle[5d5fde75cb8d8c1d]::middle::codegen_fn_attrs::CodegenFnAttrs>>
  13:     0x7f3666e3fb03 - rustc_query_system[8525185271f8bf9]::query::plumbing::get_query::<rustc_query_impl[d599c90e6b17ac17]::queries::codegen_fn_attrs, rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt>
  14:     0x7f36669c8469 - <rustc_query_impl[d599c90e6b17ac17]::Queries as rustc_middle[5d5fde75cb8d8c1d]::ty::query::QueryEngine>::codegen_fn_attrs
  15:     0x7f3665d9bb41 - rustc_typeck[c0f26da51bbb1d6]::collect::codegen_fn_attrs
  16:     0x7f3666d3fc0f - rustc_query_system[8525185271f8bf9]::query::plumbing::try_execute_query::<rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt, rustc_query_system[8525185271f8bf9]::query::caches::ArenaCache<rustc_span[61b6b3eff38f7b69]::def_id::DefId, rustc_middle[5d5fde75cb8d8c1d]::middle::codegen_fn_attrs::CodegenFnAttrs>>
  17:     0x7f3666e3fb03 - rustc_query_system[8525185271f8bf9]::query::plumbing::get_query::<rustc_query_impl[d599c90e6b17ac17]::queries::codegen_fn_attrs, rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt>
  18:     0x7f36669c8469 - <rustc_query_impl[d599c90e6b17ac17]::Queries as rustc_middle[5d5fde75cb8d8c1d]::ty::query::QueryEngine>::codegen_fn_attrs
  19:     0x7f366619e9a0 - <rustc_passes[e00a1a91b4b62704]::check_attr::CheckAttrVisitor>::check_attributes
  20:     0x7f36661a1666 - <rustc_passes[e00a1a91b4b62704]::check_attr::CheckAttrVisitor as rustc_hir[73f5179f97ca72d1]::intravisit::Visitor>::visit_expr
  21:     0x7f36661c6929 - rustc_hir[73f5179f97ca72d1]::intravisit::walk_item::<rustc_passes[e00a1a91b4b62704]::check_attr::CheckAttrVisitor>
  22:     0x7f36661a132d - <rustc_passes[e00a1a91b4b62704]::check_attr::CheckAttrVisitor as rustc_hir[73f5179f97ca72d1]::intravisit::Visitor>::visit_item
  23:     0x7f366612ec55 - <rustc_middle[5d5fde75cb8d8c1d]::hir::map::Map>::visit_item_likes_in_module::<rustc_passes[e00a1a91b4b62704]::check_attr::CheckAttrVisitor>
  24:     0x7f36661a174b - rustc_passes[e00a1a91b4b62704]::check_attr::check_mod_attrs
  25:     0x7f3666d71b82 - rustc_query_system[8525185271f8bf9]::query::plumbing::try_execute_query::<rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt, rustc_query_system[8525185271f8bf9]::query::caches::DefaultCache<rustc_span[61b6b3eff38f7b69]::def_id::LocalDefId, ()>>
  26:     0x7f3666e3b534 - rustc_query_system[8525185271f8bf9]::query::plumbing::get_query::<rustc_query_impl[d599c90e6b17ac17]::queries::check_mod_attrs, rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt>
  27:     0x7f36669b5d14 - <rustc_query_impl[d599c90e6b17ac17]::Queries as rustc_middle[5d5fde75cb8d8c1d]::ty::query::QueryEngine>::check_mod_attrs
  28:     0x7f366540c8da - <rustc_middle[5d5fde75cb8d8c1d]::hir::map::Map>::for_each_module::<rustc_interface[5088e07ffcb5d140]::passes::analysis::{closure#0}::{closure#1}::{closure#0}>
  29:     0x7f3665376dac - std[45ea6e04d395d00d]::panicking::try::<(), core[25bfd9c2f7020e11]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5088e07ffcb5d140]::passes::analysis::{closure#0}::{closure#1}>>
  30:     0x7f366538c768 - <rustc_session[5b4d9c7979230222]::session::Session>::time::<(), rustc_interface[5088e07ffcb5d140]::passes::analysis::{closure#0}>
  31:     0x7f3665385a26 - rustc_interface[5088e07ffcb5d140]::passes::analysis
  32:     0x7f3666da8036 - rustc_query_system[8525185271f8bf9]::query::plumbing::try_execute_query::<rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt, rustc_query_system[8525185271f8bf9]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>>
  33:     0x7f3666e83c52 - rustc_query_system[8525185271f8bf9]::query::plumbing::get_query::<rustc_query_impl[d599c90e6b17ac17]::queries::analysis, rustc_query_impl[d599c90e6b17ac17]::plumbing::QueryCtxt>
  34:     0x7f366699e68e - <rustc_query_impl[d599c90e6b17ac17]::Queries as rustc_middle[5d5fde75cb8d8c1d]::ty::query::QueryEngine>::analysis
  35:     0x7f36652b6904 - <rustc_interface[5088e07ffcb5d140]::passes::QueryContext>::enter::<rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>
  36:     0x7f366526d180 - <rustc_interface[5088e07ffcb5d140]::interface::Compiler>::enter::<rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[5088e07ffcb5d140]::queries::Linker>, rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>
  37:     0x7f36652c3d0d - rustc_span[61b6b3eff38f7b69]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>, rustc_interface[5088e07ffcb5d140]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>, rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}>::{closure#1}>
  38:     0x7f366526dd3a - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[61b6b3eff38f7b69]::SessionGlobals>>::set::<rustc_interface[5088e07ffcb5d140]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>, rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>
  39:     0x7f36652c899f - std[45ea6e04d395d00d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5088e07ffcb5d140]::util::run_in_thread_pool_with_globals<rustc_interface[5088e07ffcb5d140]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>, rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>
  40:     0x7f36652bbc89 - <<std[45ea6e04d395d00d]::thread::Builder>::spawn_unchecked_<rustc_interface[5088e07ffcb5d140]::util::run_in_thread_pool_with_globals<rustc_interface[5088e07ffcb5d140]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>, rustc_driver[e8d3c5195d92d990]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[1b6d01a399cf82b6]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f36647551e5 - std::sys::unix::thread::Thread::new::thread_start::ha6ddf8fc8884e2d9
  42:     0x7f365eca4609 - start_thread
  43:     0x7f36645b7133 - clone
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (addc6d82d 2022-07-26) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [codegen_fn_attrs] computing codegen attributes of `io::error::repr_bitpacked::kind_from_prim::_`
#1 [codegen_fn_attrs] computing codegen attributes of `io::error::repr_bitpacked::kind_from_prim::_::{closure#0}`
#2 [check_mod_attrs] checking attributes in module `io::error::repr_bitpacked`
#3 [analysis] running analysis passes on this crate
error: could not compile `std`
Build completed unsuccessfully in 0:03:55
