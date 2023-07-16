plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: self.reachable_blocks.contains(block)', /checkout/compiler/rustc_mir_dataflow/src/framework/cursor.rs:98:9
stack backtrace:
   0:     0x7f8503b594cd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09508ff2bf609df
   1:     0x7f8503bc1c38 - core::fmt::write::h75736d5168df1a59
   2:     0x7f8503b4a5d1 - std::io::Write::write_fmt::hd1a6120e10e78abb
   3:     0x7f8503b5c69e - std::panicking::default_hook::{{closure}}::hb816096665b89bb7
   4:     0x7f8503b5c3ee - std::panicking::default_hook::h435d8268c11c2801
   5:     0x7f850465ea84 - rustc_driver[afa06a6af71f3840]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8503b5ce11 - std::panicking::rust_panic_with_hook::h0e39e196f7138250
   7:     0x7f8503b5cbf9 - std::panicking::begin_panic_handler::{{closure}}::h1e42c3eac86bbd5d
   8:     0x7f8503b59a74 - std::sys_common::backtrace::__rust_end_short_backtrace::h870dca56d02560c5
   9:     0x7f8503b5c919 - rust_begin_unwind
  10:     0x7f8503b10043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f8503b0ff0d - core::panicking::panic::hd3154c66a81cd989
  12:     0x7f8504cc0c05 - <rustc_mir_dataflow[75584eda9b70f339]::framework::cursor::ResultsCursor<rustc_mir_dataflow[75584eda9b70f339]::impls::MaybeInitializedPlaces>>::seek_before_primary_effect
  13:     0x7f8504bc3a4d - <rustc_mir_transform[1ecf054b39b0d55a]::elaborate_drops::ElaborateDrops as rustc_middle[7750eefd62072c8f]::mir::MirPass>::run_pass
  14:     0x7f8504b965c4 - rustc_mir_transform[1ecf054b39b0d55a]::pass_manager::run_passes_inner
  15:     0x7f8504c79f8e - rustc_mir_transform[1ecf054b39b0d55a]::run_analysis_to_runtime_passes
  16:     0x7f8504c79b8f - rustc_mir_transform[1ecf054b39b0d55a]::mir_drops_elaborated_and_const_checked
  17:     0x7f85061e3336 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<rustc_middle[7750eefd62072c8f]::ty::WithOptConstParam<rustc_span[58f4f46925086679]::def_id::LocalDefId>, &rustc_data_structures[34efcb7e6a915a3a]::steal::Steal<rustc_middle[7750eefd62072c8f]::mir::Body>>>
  18:     0x7f8506303953 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  19:     0x7f8505e2ae97 - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  20:     0x7f8504798b65 - <rustc_session[aa9f798f87b09fd5]::session::Session>::time::<(), rustc_interface[bb8a930d718260cc]::passes::analysis::{closure#3}>
  21:     0x7f8504791722 - rustc_interface[bb8a930d718260cc]::passes::analysis
  22:     0x7f850622cb96 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::try_execute_query::<rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt, rustc_query_system[74a8b8d0ddb9e374]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>>
  23:     0x7f85063087b2 - rustc_query_system[74a8b8d0ddb9e374]::query::plumbing::get_query::<rustc_query_impl[acf13bc999fd2152]::queries::analysis, rustc_query_impl[acf13bc999fd2152]::plumbing::QueryCtxt>
  24:     0x7f8505e23cde - <rustc_query_impl[acf13bc999fd2152]::Queries as rustc_middle[7750eefd62072c8f]::ty::query::QueryEngine>::analysis
  25:     0x7f85046c1eb4 - <rustc_interface[bb8a930d718260cc]::passes::QueryContext>::enter::<rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  26:     0x7f8504678720 - <rustc_interface[bb8a930d718260cc]::interface::Compiler>::enter::<rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[bb8a930d718260cc]::queries::Linker>, rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  27:     0x7f85046d06dd - rustc_span[58f4f46925086679]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_interface[bb8a930d718260cc]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}>::{closure#1}>
  28:     0x7f85046792da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[58f4f46925086679]::SessionGlobals>>::set::<rustc_interface[bb8a930d718260cc]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  29:     0x7f85046cf4f9 - std[51639afae0382935]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bb8a930d718260cc]::util::run_in_thread_pool_with_globals<rustc_interface[bb8a930d718260cc]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>
  30:     0x7f85046c7239 - <<std[51639afae0382935]::thread::Builder>::spawn_unchecked_<rustc_interface[bb8a930d718260cc]::util::run_in_thread_pool_with_globals<rustc_interface[bb8a930d718260cc]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>, rustc_driver[afa06a6af71f3840]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2e9488e57e3b41c0]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f8503b69175 - std::sys::unix::thread::Thread::new::thread_start::h634e4e323cdffe8d
  32:     0x7f84fe0b8609 - start_thread
  33:     0x7f85039cb133 - clone
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (4b939548b 2022-07-30) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `cell::<impl at library/core/src/cell.rs:2042:1: 2042:26>::new`
#1 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:04:11
