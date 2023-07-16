
error: internal compiler error: encountered incremental compilation error with def_span(all_is_cubes[1daa]::mesh::chunked_mesh::{impl#6}::{constant#0})
  |
  = help: This is a known issue with the compiler. Run `cargo clean -p all_is_cubes` or `cargo clean` to allow your project to compile
  = note: Please follow the instructions below to create a bug report with the provided information
  = note: See <https://github.com/rust-lang/rust/issues/84970> for more information

thread 'rustc' panicked at 'Found unstable fingerprints for ErrorGuaranteed(()): all-is-cubes/src/mesh/chunked_mesh.rs:40:71: 41:5 (#0)', /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/compiler/rustc_query_system/src/query/plumbing.rs:711:9
stack backtrace:
   0:     0x7f307ba99baa - std::backtrace_rs::backtrace::libunwind::trace::h9d4d46a1241d4149
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f307ba99baa - std::backtrace_rs::backtrace::trace_unsynchronized::h1d977e0ccdc95d1a
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f307ba99baa - std::sys_common::backtrace::_print_fmt::h2f16d6748c69c3cb
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f307ba99baa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23aff7f8c54e6645
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f307bafdb4f - core::fmt::write::h79726c2a902099f4
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f307ba8c7e5 - std::io::Write::write_fmt::h95eedad6917eaf86
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/io/mod.rs:1698:15
   6:     0x7f307ba99975 - std::sys_common::backtrace::_print::h24fa32acab53acd8
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f307ba99975 - std::sys_common::backtrace::print::h94b47d816d0ea14c
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f307ba9c61e - std::panicking::default_hook::{{closure}}::h3bfad4b8b9ac0389
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:271:22
   9:     0x7f307ba9c3c5 - std::panicking::default_hook::hd7b308d8bc779cf9
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:290:9
  10:     0x7f307ed41e85 - <rustc_driver_impl[2b818a379abe8476]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[4ada38e0d5cae8be]::ops::function::FnOnce<(&core[4ada38e0d5cae8be]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f307ba9ce14 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hca7afd504fdadbfc
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:2002:9
  12:     0x7f307ba9ce14 - std::panicking::rust_panic_with_hook::hcec224f40e508f49
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:696:13
  13:     0x7f307ba9cb89 - std::panicking::begin_panic_handler::{{closure}}::habd506011d7d1786
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:583:13
  14:     0x7f307ba9a016 - std::sys_common::backtrace::__rust_end_short_backtrace::h85b86f25ce9045f3
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f307ba9c8e2 - rust_begin_unwind
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:579:5
  16:     0x7f307baf9f53 - core::panicking::panic_fmt::hb2cfa4c0db12cc76
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/core/src/panicking.rs:67:14
  17:     0x7f307f5e0770 - rustc_query_system[131b14eb042ddf12]::query::plumbing::incremental_verify_ich_failed::<rustc_middle[344b7a5778ebfb41]::ty::context::TyCtxt>
  18:     0x7f307d56ced6 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::def_span, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  19:     0x7f307ccf82f6 - rustc_ty_utils[354017f06c94058c]::ty::param_env
  20:     0x7f307ccf6288 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::param_env, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  21:     0x7f307ccf5906 - <rustc_query_impl[11434efd0e0628d5]::Queries as rustc_middle[344b7a5778ebfb41]::ty::query::QueryEngine>::param_env
  22:     0x7f307e2b2040 - rustc_hir_typeck[66b9761b95fd1136]::typeck_const_arg
  23:     0x7f307e1989b8 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::typeck_const_arg, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  24:     0x7f307e1982e4 - <rustc_query_impl[11434efd0e0628d5]::Queries as rustc_middle[344b7a5778ebfb41]::ty::query::QueryEngine>::typeck_const_arg
  25:     0x7f307dde3d53 - rustc_hir_typeck[66b9761b95fd1136]::typeck
  26:     0x7f307dbcd79b - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::typeck, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  27:     0x7f307e49e14b - rustc_data_structures[3749598f95b8ed78]::sync::par_for_each_in::<&[rustc_span[d779338a88541528]::def_id::LocalDefId], <rustc_middle[344b7a5778ebfb41]::hir::map::Map>::par_body_owners<rustc_hir_typeck[66b9761b95fd1136]::typeck_item_bodies::{closure#0}>::{closure#0}>
  28:     0x7f307e49df1f - rustc_hir_typeck[66b9761b95fd1136]::typeck_item_bodies
  29:     0x7f307e50e2fd - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::typeck_item_bodies, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  30:     0x7f307e50dd5c - <rustc_query_impl[11434efd0e0628d5]::Queries as rustc_middle[344b7a5778ebfb41]::ty::query::QueryEngine>::typeck_item_bodies
  31:     0x7f307e1e591f - <rustc_session[53edf0328176f526]::session::Session>::time::<(), rustc_hir_analysis[1b62eb4aa9e52db3]::check_crate::{closure#7}>
  32:     0x7f307e1e22c7 - rustc_hir_analysis[1b62eb4aa9e52db3]::check_crate
  33:     0x7f307e1dc511 - rustc_interface[cc30d76fa8dae5fc]::passes::analysis
  34:     0x7f307e505810 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::analysis, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  35:     0x7f307e5052f0 - <rustc_query_impl[11434efd0e0628d5]::Queries as rustc_middle[344b7a5778ebfb41]::ty::query::QueryEngine>::analysis
  36:     0x7f307e389089 - <rustc_middle[344b7a5778ebfb41]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>
  37:     0x7f307deb1e01 - <rustc_interface[cc30d76fa8dae5fc]::interface::Compiler>::enter::<rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}::{closure#2}, core[4ada38e0d5cae8be]::result::Result<core[4ada38e0d5cae8be]::option::Option<rustc_interface[cc30d76fa8dae5fc]::queries::Linker>, rustc_span[d779338a88541528]::ErrorGuaranteed>>
  38:     0x7f307deaffb1 - rustc_span[d779338a88541528]::with_source_map::<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  39:     0x7f307deaf55f - std[d743b6fa88cd7a4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cc30d76fa8dae5fc]::util::run_in_thread_pool_with_globals<rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>
  40:     0x7f307e5ea52e - <<std[d743b6fa88cd7a4c]::thread::Builder>::spawn_unchecked_<rustc_interface[cc30d76fa8dae5fc]::util::run_in_thread_pool_with_globals<rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#1} as core[4ada38e0d5cae8be]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f307baa6f15 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h843c8319f93d5fbd
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:1988:9
  42:     0x7f307baa6f15 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9f6cdb354bbcea1a
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:1988:9
  43:     0x7f307baa6f15 - std::sys::unix::thread::Thread::new::thread_start::h53e32c182a23b75f
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys/unix/thread.rs:108:17
  44:     0x7f307b7e9b43 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  45:     0x7f307b87ba00 - clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  46:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (db0cbc48d 2023-03-26) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=2 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C linker=clang -C incremental=[REDACTED] -Z incremental-verify-ich=yes

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
