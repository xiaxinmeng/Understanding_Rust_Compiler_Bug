plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `DefId(0:705 ~ core[8a3d]::num::{misc#3})`,
 right: `DefId(0:251 ~ core[8a3d]::num): Different parents for DefId(0:40875 ~ core[8a3d]::num::{misc#3}::{misc#0})`', compiler/rustc_middle/src/hir/map/collector.rs:243:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (d87e6327a 2021-03-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:246:47
stack backtrace:
   0:     0x7f586abeac00 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf046c43099074cce
   1:     0x7f586ac5ff5c - core::fmt::write::h6aad43fd5adf14b8
   2:     0x7f586abddac2 - std::io::Write::write_fmt::h6fa577d60b88d8df
   3:     0x7f586abef091 - std::panicking::default_hook::{{closure}}::hb0655ba12c11fe4c
   4:     0x7f586abeea77 - std::panicking::default_hook::hbbe6945bbfd080e9
   5:     0x7f586b4d822d - rustc_driver::report_ice::hdc4cd1892873c9d2
   6:     0x7f586abef9c2 - std::panicking::rust_panic_with_hook::hceeb5ceca7e959c6
   7:     0x7f586abef4d7 - std::panicking::begin_panic_handler::{{closure}}::h57b4c99dd0511406
   8:     0x7f586abeb09c - std::sys_common::backtrace::__rust_end_short_backtrace::h54352bc295bb9e47
   9:     0x7f586abef469 - rust_begin_unwind
  10:     0x7f586ac5c161 - core::panicking::panic_fmt::h79f7782fa1a68289
  11:     0x7f586ac5c0ad - core::panicking::panic::h51525e468b384ed7
  12:     0x7f586c6e738c - rustc_data_structures::cold_path::h6f84c0ccf147cce4
  13:     0x7f586c203e13 - rustc_query_system::query::plumbing::get_query_impl::hb6c550801542e59c
  14:     0x7f586c2f5d53 - rustc_query_system::query::plumbing::get_query::h93aa123f6bcf4bc3
  15:     0x7f586d8ec49c - rustc_middle::hir::map::Map::find::h76f2301e4b2259fe
  16:     0x7f586d8ef945 - rustc_middle::hir::map::Map::opt_span::h0ec382ddf0416b0f
  17:     0x7f586d8efc6c - rustc_middle::hir::map::Map::span_if_local::hba779ddf990399b9
  18:     0x7f586d992e61 - core::ops::function::FnOnce::call_once::h7a48b38a5bfb7f76
  19:     0x7f586c61154d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf46d173ec36325ea
  20:     0x7f586c4c59a1 - rustc_data_structures::stack::ensure_sufficient_stack::h0ee63ef04e048381
  21:     0x7f586c261ced - rustc_query_system::query::plumbing::force_query_with_job::h0221fbac90857d8e
  22:     0x7f586c20857a - rustc_query_system::query::plumbing::get_query_impl::hbf52494bdeeef656
  23:     0x7f586c2debb1 - rustc_query_system::query::plumbing::get_query::h1d0ea59f1f5e650b
  24:     0x7f586c6b0392 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h56d77b6b8f9c2f7c
  25:     0x7f586c6b025a - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::h12743d3a6817060b
  26:     0x7f586c679466 - rustc_query_impl::make_query::index_hir::h76ed4e942246311e
  27:     0x7f586c4855ad - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::hcd818269cf4cc04f
  28:     0x7f586c3cf201 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::hb76127bd2baafd50
  29:     0x7f586c2adf7c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h66aafc5a88f5bf6f
  30:     0x7f586c659ffc - rustc_query_impl::Queries::try_collect_active_jobs::hb8f2476848e39b4d
  31:     0x7f586c4bd2f1 - rustc_query_system::query::job::print_query_stack::h87eab64cf956770a
  32:     0x7f586b62e8b4 - rustc_interface::interface::try_print_query_stack::h88e14a786a009fe9
  33:     0x7f586b4d8a8a - rustc_driver::report_ice::hdc4cd1892873c9d2
  34:     0x7f586abef9c2 - std::panicking::rust_panic_with_hook::hceeb5ceca7e959c6
  35:     0x7f586abef507 - std::panicking::begin_panic_handler::{{closure}}::h57b4c99dd0511406
  36:     0x7f586abeb09c - std::sys_common::backtrace::__rust_end_short_backtrace::h54352bc295bb9e47
  37:     0x7f586abef469 - rust_begin_unwind
  38:     0x7f586ac5c161 - core::panicking::panic_fmt::h79f7782fa1a68289
  39:     0x7f586ac5c422 - core::panicking::assert_failed_inner::h1b528e776c79839e
  40:     0x7f586da6c32b - core::panicking::assert_failed::h6473fe9f951b4f7b
  41:     0x7f586d951cc5 - rustc_middle::hir::map::collector::NodeCollector::insert_nested::hf632b65c3792bdec
  42:     0x7f586d951d26 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h0a8368a090be3699
  43:     0x7f586da95d2a - rustc_hir::intravisit::walk_item::h706fb49ece8499a3
  44:     0x7f586d9520a1 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h9f0c3eaa9b96fadc
  45:     0x7f586d9509c7 - rustc_middle::hir::map::collector::collect::h5853602e41417409
  46:     0x7f586d8f0f65 - rustc_middle::hir::map::index_hir::h95bb8a53c7c9213e
  47:     0x7f586c60e318 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hee51dc08ef14a361
  48:     0x7f586c4da720 - rustc_data_structures::stack::ensure_sufficient_stack::hcc43312043fc8ecd
  49:     0x7f586c277667 - rustc_query_system::query::plumbing::force_query_with_job::h7b0210776d9f493d
  50:     0x7f586c1bba85 - rustc_query_system::query::plumbing::get_query_impl::h1c95f04788612d49
  51:     0x7f586c2df7f3 - rustc_query_system::query::plumbing::get_query::h206b0af230b4b29f
  52:     0x7f586d993b97 - core::ops::function::FnOnce::call_once::he46a3b5da407521d
  53:     0x7f586c6075ba - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hdfcbccea0a237a9e
  54:     0x7f586c4d57b8 - rustc_data_structures::stack::ensure_sufficient_stack::h9ed94955e6fe6ee5
  55:     0x7f586c28640d - rustc_query_system::query::plumbing::force_query_with_job::hd0216974d583a2d0
  56:     0x7f586c2041cd - rustc_query_system::query::plumbing::get_query_impl::hb6c550801542e59c
  57:     0x7f586c2f5d53 - rustc_query_system::query::plumbing::get_query::h93aa123f6bcf4bc3
  58:     0x7f586d8ec49c - rustc_middle::hir::map::Map::find::h76f2301e4b2259fe
  59:     0x7f586d8eca1c - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::h4eeabac3397788cd
  60:     0x7f586c7bf058 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h8d851570b31da044
  61:     0x7f586c7cca19 - rustc_passes::hir_id_validator::check_crate::hf47c08250354450e
  62:     0x7f586b64c3cf - rustc_interface::passes::analysis::hbc7933b4dca50625
  63:     0x7f586c5d63b8 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h72efe417ada5012b
  64:     0x7f586c617d09 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h1b73b5ef437cec51
  65:     0x7f586c4c50f2 - rustc_data_structures::stack::ensure_sufficient_stack::h0aee334893614301
  66:     0x7f586c284247 - rustc_query_system::query::plumbing::force_query_with_job::hc5bae4d59f9d6039
  67:     0x7f586c1c2999 - rustc_query_system::query::plumbing::get_query_impl::h2910dbf82014012f
  68:     0x7f586c2dfe13 - rustc_query_system::query::plumbing::get_query::h20af5864ef7a4c2f
  69:     0x7f586b4c4934 - rustc_interface::passes::QueryContext::enter::h22294d697c6fe41b
  70:     0x7f586b4ef4dd - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hed1efd2cb9101523
  71:     0x7f586b4e0ee0 - rustc_span::with_source_map::h93438a34252cef61
  72:     0x7f586b4edecb - scoped_tls::ScopedKey<T>::set::hf0c7bacfb077c959
  73:     0x7f586b4e1437 - rustc_span::with_session_globals::h1472b819789f6dd3
  74:     0x7f586b4f100e - std::sys_common::backtrace::__rust_begin_short_backtrace::hc86037b5019ecc04
  75:     0x7f586b543e3a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h76932e130b3481a1
  76:     0x7f586ac00bfa - std::sys::unix::thread::Thread::new::thread_start::h8ce4bc150339d41c
  77:     0x7f5865d3f6db - start_thread
  78:     0x7f586a87f71f - __clone
  79:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (d87e6327a 2021-03-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4
error: could not compile `core`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=a4d41b08166a174e -C extra-filename=-a4d41b08166a174e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes -Z binary-dep-depinfo` (exit code: 254)
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
