plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 343889b7234bf786e2bc673029467052f22fca08 and 1d73242194120fa881b83ccfc8cbc1ea0691ea24
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:107:26: DefId(0:0 ~ core[7b18]) doesn't have a parent
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1324:9
stack backtrace:
   0:     0x7fb4c312edd2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7fb4c3196798 - core::fmt::write::h42234c3e51154f4c
   1:     0x7fb4c3196798 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fb4c311f311 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7fb4c3132116 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7fb4c3131d0d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7fb4c457c21a - rustc_driver[33a470c284e524fb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb4c3132ab0 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7fb4ca2ebc63 - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[98000b9ad8036217]::ExplicitBug>::{closure#0}
   8:     0x7fb4ca2e9a06 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_end_short_backtrace::<std[38ff3720b7fd637]::panicking::begin_panic<rustc_errors[98000b9ad8036217]::ExplicitBug>::{closure#0}, !>
   9:     0x7fb4c43a8b76 - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[98000b9ad8036217]::ExplicitBug>
  10:     0x7fb4ca412596 - std[38ff3720b7fd637]::panic::panic_any::<rustc_errors[98000b9ad8036217]::ExplicitBug>
  11:     0x7fb4ca403810 - <rustc_errors[98000b9ad8036217]::HandlerInner>::bug::<&alloc[24f448636cd10183]::string::String>
  12:     0x7fb4ca403400 - <rustc_errors[98000b9ad8036217]::Handler>::bug::<&alloc[24f448636cd10183]::string::String>
  13:     0x7fb4ca51b47a - rustc_middle[a8b2aabb07d91460]::ty::context::tls::with_opt::<rustc_middle[a8b2aabb07d91460]::util::bug::opt_span_bug_fmt<rustc_span[188b5a9c74ea64c9]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7fb4ca51b759 - rustc_middle[a8b2aabb07d91460]::util::bug::opt_span_bug_fmt::<rustc_span[188b5a9c74ea64c9]::span_encoding::Span>
  15:     0x7fb4c43af5f5 - rustc_middle[a8b2aabb07d91460]::util::bug::bug_fmt
  16:     0x7fb4ca2f43b5 - <rustc_middle[a8b2aabb07d91460]::hir::provide::{closure#4} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[a8b2aabb07d91460]::ty::context::TyCtxt, rustc_span[188b5a9c74ea64c9]::def_id::LocalDefId)>>::call_once
  17:     0x7fb4c902c89e - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::try_execute_query::<rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt, rustc_query_system[f43e70d9f21dfba0]::query::caches::DefaultCache<rustc_span[188b5a9c74ea64c9]::def_id::LocalDefId, rustc_hir[a6a99886045e0ce5]::hir_id::HirId>>
  18:     0x7fb4c9123eca - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::get_query::<rustc_query_impl[49b8a40da55c2e5]::queries::hir_owner_parent, rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt>
  19:     0x7fb4ca4835d0 - <rustc_middle[a8b2aabb07d91460]::hir::map::Map>::find_parent_node
  20:     0x7fb4ca483767 - <rustc_middle[a8b2aabb07d91460]::hir::map::Map>::get_parent_node
  21:     0x7fb4c7a963ff - rustc_privacy[49d25f53ceb9c08e]::check_private_in_public
  22:     0x7fb4c907eafa - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::try_execute_query::<rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt, rustc_query_system[f43e70d9f21dfba0]::query::caches::DefaultCache<(), ()>>
  23:     0x7fb4c9150d48 - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::get_query::<rustc_query_impl[49b8a40da55c2e5]::queries::check_private_in_public, rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt>
  24:     0x7fb4c4670939 - <core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[33eecf3c47264298]::passes::analysis::{closure#5}::{closure#0}::{closure#0}> as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once
  25:     0x7fb4c4671798 - <core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[33eecf3c47264298]::passes::analysis::{closure#5}::{closure#0}> as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once
  26:     0x7fb4c46d5c4c - <rustc_session[45aeb5e225888b76]::session::Session>::time::<(), rustc_interface[33eecf3c47264298]::passes::analysis::{closure#5}>
  27:     0x7fb4c46aa61c - rustc_interface[33eecf3c47264298]::passes::analysis
  28:     0x7fb4c9072d8e - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::try_execute_query::<rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt, rustc_query_system[f43e70d9f21dfba0]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>>
  29:     0x7fb4c9183219 - rustc_query_system[f43e70d9f21dfba0]::query::plumbing::get_query::<rustc_query_impl[49b8a40da55c2e5]::queries::analysis, rustc_query_impl[49b8a40da55c2e5]::plumbing::QueryCtxt>
  30:     0x7fb4c4567dbd - <rustc_interface[33eecf3c47264298]::passes::QueryContext>::enter::<rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>
  31:     0x7fb4c44e94e5 - <rustc_interface[33eecf3c47264298]::interface::Compiler>::enter::<rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[33eecf3c47264298]::queries::Linker>, rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>
  32:     0x7fb4c44bf093 - rustc_span[188b5a9c74ea64c9]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>, rustc_interface[33eecf3c47264298]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>, rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}>::{closure#1}>
  33:     0x7fb4c44ea934 - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[188b5a9c74ea64c9]::SessionGlobals>>::set::<rustc_interface[33eecf3c47264298]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>, rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>
  34:     0x7fb4c45636bf - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[33eecf3c47264298]::util::run_in_thread_pool_with_globals<rustc_interface[33eecf3c47264298]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>, rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>
  35:     0x7fb4c45657d9 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[33eecf3c47264298]::util::run_in_thread_pool_with_globals<rustc_interface[33eecf3c47264298]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>, rustc_driver[33a470c284e524fb]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[98000b9ad8036217]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fb4c313f4b3 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  37:     0x7fb4c26906ba - start_thread
  38:     0x7fb4c2db951d - clone
  39:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (1d7324219 2022-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [hir_owner_parent] HIR parent of ``
#1 [check_private_in_public] checking for private elements in public interfaces
#2 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:05:40
cat: /tmp/toolstate/toolstates.json: No such file or directory
