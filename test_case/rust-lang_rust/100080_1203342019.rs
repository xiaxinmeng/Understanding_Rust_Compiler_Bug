plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'missing tokens for node at library/core/src/sync/atomic.rs:2732:5: 2732:6 (#0): Expr { id: NodeId(4294967040), kind: Lit(Lit { token: Lit { kind: Integer, symbol: "1", suffix: None }, kind: Int(1, Unsuffixed), span: library/core/src/sync/atomic.rs:2732:5: 2732:6 (#0) }), span: library/core/src/sync/atomic.rs:2732:5: 2732:6 (#0), attrs: ThinVec(None) }', compiler/rustc_ast/src/tokenstream.rs:455:32
   0:     0x7f65a32dc5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09508ff2bf609df
   1:     0x7f65a3344bd8 - core::fmt::write::h75736d5168df1a59
   1:     0x7f65a3344bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f65a32cd761 - std::io::Write::write_fmt::hd1a6120e10e78abb
   3:     0x7f65a32df77e - std::panicking::default_hook::{{closure}}::hb816096665b89bb7
   4:     0x7f65a32df4e1 - std::panicking::default_hook::h435d8268c11c2801
   5:     0x7f65a3de4624 - rustc_driver[87078a9a48453e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f65a32dfef1 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
   7:     0x7f65a32dfd17 - std::panicking::begin_panic_handler::{{closure}}::h1e42c3eac86bbd5d
   8:     0x7f65a32dcb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h870dca56d02560c5
   9:     0x7f65a32df9f9 - rust_begin_unwind
  10:     0x7f65a3293043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f65a6bff448 - <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::from_nonterminal_ast
  12:     0x7f65a6bfd66c - <&mut <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(&rustc_ast[4936451f046522e8]::tokenstream::TokenTree,)>>::call_once
  13:     0x7f65a6bf4efe - <alloc[11297c88fab3d63e]::vec::Vec<rustc_ast[4936451f046522e8]::tokenstream::TokenTree> as alloc[11297c88fab3d63e]::vec::spec_from_iter::SpecFromIter<rustc_ast[4936451f046522e8]::tokenstream::TokenTree, core[25bfd9c2f7020e11]::iter::adapters::map::Map<rustc_ast[4936451f046522e8]::tokenstream::CursorRef, <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0}>>>::from_iter
  14:     0x7f65a6bfd5cb - <&mut <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(&rustc_ast[4936451f046522e8]::tokenstream::TokenTree,)>>::call_once
  15:     0x7f65a6bf4fdd - <alloc[11297c88fab3d63e]::vec::Vec<rustc_ast[4936451f046522e8]::tokenstream::TokenTree> as alloc[11297c88fab3d63e]::vec::spec_from_iter::SpecFromIter<rustc_ast[4936451f046522e8]::tokenstream::TokenTree, core[25bfd9c2f7020e11]::iter::adapters::map::Map<rustc_ast[4936451f046522e8]::tokenstream::CursorRef, <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0}>>>::from_iter
  16:     0x7f65a6bff560 - <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened
  17:     0x7f65a450fcd6 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_mac_args
  18:     0x7f65a450f915 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attr
  19:     0x7f65a45320f0 - <smallvec[5437718c57b79d5c]::SmallVec<[rustc_ast[4936451f046522e8]::ast::Attribute; 8usize]> as core[25bfd9c2f7020e11]::iter::traits::collect::Extend<rustc_ast[4936451f046522e8]::ast::Attribute>>::extend::<core[25bfd9c2f7020e11]::iter::adapters::map::Map<core[25bfd9c2f7020e11]::slice::iter::Iter<rustc_ast[4936451f046522e8]::ast::Attribute>, <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs::{closure#0}>>
  20:     0x7f65a45a3d39 - <rustc_hir[8506301c70002e1b]::Arena>::alloc_from_iter::<rustc_ast[4936451f046522e8]::ast::Attribute, rustc_arena[afa1c0fda458c523]::IsNotCopy, core[25bfd9c2f7020e11]::iter::adapters::map::Map<core[25bfd9c2f7020e11]::slice::iter::Iter<rustc_ast[4936451f046522e8]::ast::Attribute>, <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs::{closure#0}>>
  21:     0x7f65a450f757 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs
  22:     0x7f65a451814c - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  23:     0x7f65a455ef57 - <rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::lower_node
  24:     0x7f65a450d2c8 - rustc_ast_lowering[35d76ede3ddcbb91]::lower_to_hir
  25:     0x7f65a591920d - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::ArenaCache<(), rustc_hir[8506301c70002e1b]::hir::Crate>>
  26:     0x7f65a5a44962 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_crate, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  27:     0x7f65a5561fde - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_crate
  28:     0x7f65a6896165 - <rustc_middle[7750eefd62072c8f]::hir::provide::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7750eefd62072c8f]::ty::context::TyCtxt, rustc_span[58f4f46925086679]::def_id::LocalDefId)>>::call_once
  29:     0x7f65a5922179 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<rustc_span[58f4f46925086679]::def_id::LocalDefId, core[25bfd9c2f7020e11]::option::Option<rustc_middle[7750eefd62072c8f]::hir::Owner>>>
  30:     0x7f65a5a44a9f - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_owner, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  31:     0x7f65a5562fee - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_owner
  32:     0x7f65a68a6ca3 - <rustc_middle[7750eefd62072c8f]::hir::map::Map>::get_module
  33:     0x7f65a68ad514 - rustc_middle[7750eefd62072c8f]::hir::map::hir_crate_items
  34:     0x7f65a59151f2 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::ArenaCache<(), rustc_middle[7750eefd62072c8f]::hir::ModuleItems>>
  35:     0x7f65a59fe232 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_crate_items, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  36:     0x7f65a556252e - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_crate_items
  37:     0x7f65a4cb9bc6 - <rustc_middle[7750eefd62072c8f]::hir::map::Map>::for_each_module::<rustc_passes[f5442a3b902b3385]::hir_id_validator::check_crate::{closure#0}>
  38:     0x7f65a4cfc9f5 - rustc_passes[f5442a3b902b3385]::hir_id_validator::check_crate
  39:     0x7f65a3f15115 - rustc_interface[16a315bebaa0a951]::passes::analysis
  40:     0x7f65a5968466 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>>
  41:     0x7f65a5a43de2 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::analysis, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  42:     0x7f65a5565b1e - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::analysis
  43:     0x7f65a3e47a74 - <rustc_interface[16a315bebaa0a951]::passes::QueryContext>::enter::<rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  44:     0x7f65a3dfeb20 - <rustc_interface[16a315bebaa0a951]::interface::Compiler>::enter::<rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[16a315bebaa0a951]::queries::Linker>, rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  45:     0x7f65a3e5628d - rustc_span[58f4f46925086679]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_interface[16a315bebaa0a951]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#1}>
  46:     0x7f65a3dff6da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[58f4f46925086679]::SessionGlobals>>::set::<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  47:     0x7f65a3e550a9 - std[51639afae0382935]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  48:     0x7f65a3e4cdf9 - <<std[51639afae0382935]::thread::Builder>::spawn_unchecked_<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f65a32ec185 - std::sys::unix::thread::Thread::new::thread_start::h634e4e323cdffe8d
  50:     0x7f659d83b609 - start_thread
  51:     0x7f65a314e133 - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (e2be83185 2022-08-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:184:59
   0:     0x7f65a32dc5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09508ff2bf609df
   1:     0x7f65a3344bd8 - core::fmt::write::h75736d5168df1a59
   1:     0x7f65a3344bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f65a32cd761 - std::io::Write::write_fmt::hd1a6120e10e78abb
   3:     0x7f65a32df77e - std::panicking::default_hook::{{closure}}::hb816096665b89bb7
   4:     0x7f65a32df4e1 - std::panicking::default_hook::h435d8268c11c2801
   5:     0x7f65a3de4624 - rustc_driver[87078a9a48453e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f65a32dfef1 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
   7:     0x7f65a32dfcd9 - std::panicking::begin_panic_handler::{{closure}}::h1e42c3eac86bbd5d
   8:     0x7f65a32dcb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h870dca56d02560c5
   9:     0x7f65a32df9f9 - rust_begin_unwind
  10:     0x7f65a3293043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f65a3292f0d - core::panicking::panic::hd3154c66a81cd989
  12:     0x7f65a5919a51 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::ArenaCache<(), rustc_hir[8506301c70002e1b]::hir::Crate>>
  13:     0x7f65a5a44962 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_crate, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  14:     0x7f65a5561fde - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_crate
  15:     0x7f65a6893f85 - <rustc_middle[7750eefd62072c8f]::hir::provide::{closure#2} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7750eefd62072c8f]::ty::context::TyCtxt, rustc_span[58f4f46925086679]::def_id::LocalDefId)>>::call_once
  16:     0x7f65a592a82f - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<rustc_span[58f4f46925086679]::def_id::LocalDefId, rustc_hir[8506301c70002e1b]::hir_id::HirId>>
  17:     0x7f65a5a1c70e - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::local_def_id_to_hir_id, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  18:     0x7f65a5563554 - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::local_def_id_to_hir_id
  19:     0x7f65a6895335 - <rustc_middle[7750eefd62072c8f]::hir::provide::{closure#7} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7750eefd62072c8f]::ty::context::TyCtxt, rustc_span[58f4f46925086679]::def_id::DefId)>>::call_once
  20:     0x7f65a59416a7 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<rustc_span[58f4f46925086679]::def_id::DefId, rustc_span[58f4f46925086679]::span_encoding::Span>>
  21:     0x7f65a5a43f4e - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::def_span, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  22:     0x7f65a558cd2b - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::def_span
  23:     0x7f65a56b08c9 - <rustc_span[58f4f46925086679]::def_id::DefId as rustc_query_impl[acf13bc999fd2152]::keys::Key>::default_span
  24:     0x7f65a56b0737 - <rustc_span[58f4f46925086679]::def_id::LocalDefId as rustc_query_impl[acf13bc999fd2152]::keys::Key>::default_span
  25:     0x7f65a58624a7 - rustc_query_impl[acf13bc999fd2152]::make_query::hir_owner
  26:     0x7f65a58d0f0d - <rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::QueryState<rustc_span[58f4f46925086679]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  27:     0x7f65a555e3d6 - <rustc_query_impl[acf13bc999fd2152]::Queries>::try_collect_active_jobs
  28:     0x7f65a553eff9 - rustc_query_system[74a8b8d0ddb9e374]::query::job::print_query_stack::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  29:     0x7f65a3f8fef4 - rustc_interface[16a315bebaa0a951]::interface::try_print_query_stack
  30:     0x7f65a3de51d5 - rustc_driver[87078a9a48453e3]::report_ice
  31:     0x7f65a32dfef1 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
  32:     0x7f65a32dfd17 - std::panicking::begin_panic_handler::{{closure}}::h1e42c3eac86bbd5d
  33:     0x7f65a32dcb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h870dca56d02560c5
  34:     0x7f65a32df9f9 - rust_begin_unwind
  35:     0x7f65a3293043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  36:     0x7f65a6bff448 - <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::from_nonterminal_ast
  37:     0x7f65a6bfd66c - <&mut <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(&rustc_ast[4936451f046522e8]::tokenstream::TokenTree,)>>::call_once
  38:     0x7f65a6bf4efe - <alloc[11297c88fab3d63e]::vec::Vec<rustc_ast[4936451f046522e8]::tokenstream::TokenTree> as alloc[11297c88fab3d63e]::vec::spec_from_iter::SpecFromIter<rustc_ast[4936451f046522e8]::tokenstream::TokenTree, core[25bfd9c2f7020e11]::iter::adapters::map::Map<rustc_ast[4936451f046522e8]::tokenstream::CursorRef, <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0}>>>::from_iter
  39:     0x7f65a6bfd5cb - <&mut <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(&rustc_ast[4936451f046522e8]::tokenstream::TokenTree,)>>::call_once
  40:     0x7f65a6bf4fdd - <alloc[11297c88fab3d63e]::vec::Vec<rustc_ast[4936451f046522e8]::tokenstream::TokenTree> as alloc[11297c88fab3d63e]::vec::spec_from_iter::SpecFromIter<rustc_ast[4936451f046522e8]::tokenstream::TokenTree, core[25bfd9c2f7020e11]::iter::adapters::map::Map<rustc_ast[4936451f046522e8]::tokenstream::CursorRef, <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened::{closure#0}>>>::from_iter
  41:     0x7f65a6bff560 - <rustc_ast[4936451f046522e8]::tokenstream::TokenStream>::flattened
  42:     0x7f65a450fcd6 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_mac_args
  43:     0x7f65a450f915 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attr
  44:     0x7f65a45320f0 - <smallvec[5437718c57b79d5c]::SmallVec<[rustc_ast[4936451f046522e8]::ast::Attribute; 8usize]> as core[25bfd9c2f7020e11]::iter::traits::collect::Extend<rustc_ast[4936451f046522e8]::ast::Attribute>>::extend::<core[25bfd9c2f7020e11]::iter::adapters::map::Map<core[25bfd9c2f7020e11]::slice::iter::Iter<rustc_ast[4936451f046522e8]::ast::Attribute>, <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs::{closure#0}>>
  45:     0x7f65a45a3d39 - <rustc_hir[8506301c70002e1b]::Arena>::alloc_from_iter::<rustc_ast[4936451f046522e8]::ast::Attribute, rustc_arena[afa1c0fda458c523]::IsNotCopy, core[25bfd9c2f7020e11]::iter::adapters::map::Map<core[25bfd9c2f7020e11]::slice::iter::Iter<rustc_ast[4936451f046522e8]::ast::Attribute>, <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs::{closure#0}>>
  46:     0x7f65a450f757 - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::lower_attrs
  47:     0x7f65a451814c - <rustc_ast_lowering[35d76ede3ddcbb91]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  48:     0x7f65a455ef57 - <rustc_ast_lowering[35d76ede3ddcbb91]::item::ItemLowerer>::lower_node
  49:     0x7f65a450d2c8 - rustc_ast_lowering[35d76ede3ddcbb91]::lower_to_hir
  50:     0x7f65a591920d - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::ArenaCache<(), rustc_hir[8506301c70002e1b]::hir::Crate>>
  51:     0x7f65a5a44962 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_crate, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  52:     0x7f65a5561fde - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_crate
  53:     0x7f65a6896165 - <rustc_middle[7750eefd62072c8f]::hir::provide::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7750eefd62072c8f]::ty::context::TyCtxt, rustc_span[58f4f46925086679]::def_id::LocalDefId)>>::call_once
  54:     0x7f65a5922179 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<rustc_span[58f4f46925086679]::def_id::LocalDefId, core[25bfd9c2f7020e11]::option::Option<rustc_middle[7750eefd62072c8f]::hir::Owner>>>
  55:     0x7f65a5a44a9f - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_owner, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  56:     0x7f65a5562fee - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_owner
  57:     0x7f65a68a6ca3 - <rustc_middle[7750eefd62072c8f]::hir::map::Map>::get_module
  58:     0x7f65a68ad514 - rustc_middle[7750eefd62072c8f]::hir::map::hir_crate_items
  59:     0x7f65a59151f2 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::ArenaCache<(), rustc_middle[7750eefd62072c8f]::hir::ModuleItems>>
  60:     0x7f65a59fe232 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::hir_crate_items, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  61:     0x7f65a556252e - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::hir_crate_items
  62:     0x7f65a4cb9bc6 - <rustc_middle[7750eefd62072c8f]::hir::map::Map>::for_each_module::<rustc_passes[f5442a3b902b3385]::hir_id_validator::check_crate::{closure#0}>
  63:     0x7f65a4cfc9f5 - rustc_passes[f5442a3b902b3385]::hir_id_validator::check_crate
  64:     0x7f65a3f15115 - rustc_interface[16a315bebaa0a951]::passes::analysis
  65:     0x7f65a5968466 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>>
  66:     0x7f65a5a43de2 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::analysis, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  67:     0x7f65a5565b1e - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::analysis
  68:     0x7f65a3e47a74 - <rustc_interface[16a315bebaa0a951]::passes::QueryContext>::enter::<rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  69:     0x7f65a3dfeb20 - <rustc_interface[16a315bebaa0a951]::interface::Compiler>::enter::<rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[16a315bebaa0a951]::queries::Linker>, rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  70:     0x7f65a3e5628d - rustc_span[58f4f46925086679]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_interface[16a315bebaa0a951]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#1}>
  71:     0x7f65a3dff6da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[58f4f46925086679]::SessionGlobals>>::set::<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  72:     0x7f65a3e550a9 - std[51639afae0382935]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  73:     0x7f65a3e4cdf9 - <<std[51639afae0382935]::thread::Builder>::spawn_unchecked_<rustc_interface[16a315bebaa0a951]::util::run_in_thread_pool_with_globals<rustc_interface[16a315bebaa0a951]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[87078a9a48453e3]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:     0x7f65a32ec185 - std::sys::unix::thread::Thread::new::thread_start::h634e4e323cdffe8d
  75:     0x7f659d83b609 - start_thread
  76:     0x7f65a314e133 - clone
  77:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (e2be83185 2022-08-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=fa992565d2130c71 -C extra-filename=-fa992565d2130c71 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
