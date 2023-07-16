
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/compiler/rustc_errors/src/lib.rs:995:33
stack backtrace:
   0:     0x7fa9c242ebaa - std::backtrace_rs::backtrace::libunwind::trace::h9d4d46a1241d4149
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fa9c242ebaa - std::backtrace_rs::backtrace::trace_unsynchronized::h1d977e0ccdc95d1a
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fa9c242ebaa - std::sys_common::backtrace::_print_fmt::h2f16d6748c69c3cb
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fa9c242ebaa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h23aff7f8c54e6645
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fa9c2492b4f - core::fmt::write::h79726c2a902099f4
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/core/src/fmt/mod.rs:1254:17
   5:     0x7fa9c24217e5 - std::io::Write::write_fmt::h95eedad6917eaf86
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/io/mod.rs:1698:15
   6:     0x7fa9c242e975 - std::sys_common::backtrace::_print::h24fa32acab53acd8
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fa9c242e975 - std::sys_common::backtrace::print::h94b47d816d0ea14c
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fa9c243161e - std::panicking::default_hook::{{closure}}::h3bfad4b8b9ac0389
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:271:22
   9:     0x7fa9c24313c5 - std::panicking::default_hook::hd7b308d8bc779cf9
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:290:9
  10:     0x7fa9c56d6e85 - <rustc_driver_impl[2b818a379abe8476]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[4ada38e0d5cae8be]::ops::function::FnOnce<(&core[4ada38e0d5cae8be]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fa9c2431e14 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hca7afd504fdadbfc
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:2002:9
  12:     0x7fa9c2431e14 - std::panicking::rust_panic_with_hook::hcec224f40e508f49
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/panicking.rs:696:13
  13:     0x7fa9c5671641 - std[d743b6fa88cd7a4c]::panicking::begin_panic::<rustc_errors[a80cbd0c8751c720]::ExplicitBug>::{closure#0}
  14:     0x7fa9c566ebf6 - std[d743b6fa88cd7a4c]::sys_common::backtrace::__rust_end_short_backtrace::<std[d743b6fa88cd7a4c]::panicking::begin_panic<rustc_errors[a80cbd0c8751c720]::ExplicitBug>::{closure#0}, !>
  15:     0x7fa9c5686496 - std[d743b6fa88cd7a4c]::panicking::begin_panic::<rustc_errors[a80cbd0c8751c720]::ExplicitBug>
  16:     0x7fa9c56416e2 - <rustc_errors[a80cbd0c8751c720]::HandlerInner>::span_bug::<rustc_span[d779338a88541528]::span_encoding::Span, &alloc[8662f753d347c67a]::string::String>
  17:     0x7fa9c5641587 - <rustc_errors[a80cbd0c8751c720]::Handler>::span_bug::<rustc_span[d779338a88541528]::span_encoding::Span, &alloc[8662f753d347c67a]::string::String>
  18:     0x7fa9c56474bb - rustc_middle[344b7a5778ebfb41]::util::bug::opt_span_bug_fmt::<rustc_span[d779338a88541528]::span_encoding::Span>::{closure#0}
  19:     0x7fa9c564750a - rustc_middle[344b7a5778ebfb41]::ty::context::tls::with_opt::<rustc_middle[344b7a5778ebfb41]::util::bug::opt_span_bug_fmt<rustc_span[d779338a88541528]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  20:     0x7fa9c5646c96 - rustc_middle[344b7a5778ebfb41]::ty::context::tls::with_context_opt::<rustc_middle[344b7a5778ebfb41]::ty::context::tls::with_opt<rustc_middle[344b7a5778ebfb41]::util::bug::opt_span_bug_fmt<rustc_span[d779338a88541528]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  21:     0x7fa9c5646b66 - rustc_middle[344b7a5778ebfb41]::util::bug::opt_span_bug_fmt::<rustc_span[d779338a88541528]::span_encoding::Span>
  22:     0x7fa9c3c02894 - rustc_middle[344b7a5778ebfb41]::util::bug::span_bug_fmt::<rustc_span[d779338a88541528]::span_encoding::Span>
  23:     0x7fa9c568b29f - <rustc_const_eval[b7f6bcb39e5bd1ca]::transform::check_consts::ops::RawPtrComparison as rustc_const_eval[b7f6bcb39e5bd1ca]::transform::check_consts::ops::NonConstOp>::build_error
  24:     0x7fa9c49bc2d7 - <rustc_const_eval[b7f6bcb39e5bd1ca]::transform::check_consts::check::Checker as rustc_middle[344b7a5778ebfb41]::mir::visit::Visitor>::visit_basic_block_data
  25:     0x7fa9c49b91fe - <rustc_const_eval[b7f6bcb39e5bd1ca]::transform::check_consts::check::Checker>::check_body
  26:     0x7fa9c49b739c - <rustc_mir_transform[c5d3163ff4a25b61]::provide::{closure#0} as core[4ada38e0d5cae8be]::ops::function::FnOnce<(rustc_middle[344b7a5778ebfb41]::ty::context::TyCtxt, rustc_span[d779338a88541528]::def_id::LocalDefId)>>::call_once
  27:     0x7fa9c49b5dc3 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::mir_const_qualif, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  28:     0x7fa9c40bc6c5 - rustc_mir_transform[c5d3163ff4a25b61]::mir_promoted
  29:     0x7fa9c40bb03e - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::mir_promoted, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  30:     0x7fa9c45991f5 - <rustc_borrowck[8d13603367d57f6d]::provide::{closure#0} as core[4ada38e0d5cae8be]::ops::function::FnOnce<(rustc_middle[344b7a5778ebfb41]::ty::context::TyCtxt, rustc_span[d779338a88541528]::def_id::LocalDefId)>>::call_once
  31:     0x7fa9c4597a00 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::mir_borrowck, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  32:     0x7fa9c4b720f0 - rustc_data_structures[3749598f95b8ed78]::sync::par_for_each_in::<&[rustc_span[d779338a88541528]::def_id::LocalDefId], <rustc_middle[344b7a5778ebfb41]::hir::map::Map>::par_body_owners<rustc_interface[cc30d76fa8dae5fc]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  33:     0x7fa9c4b71d26 - <rustc_session[53edf0328176f526]::session::Session>::time::<(), rustc_interface[cc30d76fa8dae5fc]::passes::analysis::{closure#2}>
  34:     0x7fa9c4b7154f - rustc_interface[cc30d76fa8dae5fc]::passes::analysis
  35:     0x7fa9c4e9a5e2 - rustc_query_system[131b14eb042ddf12]::query::plumbing::try_execute_query::<rustc_query_impl[11434efd0e0628d5]::queries::analysis, rustc_query_impl[11434efd0e0628d5]::plumbing::QueryCtxt>
  36:     0x7fa9c4e9a2f0 - <rustc_query_impl[11434efd0e0628d5]::Queries as rustc_middle[344b7a5778ebfb41]::ty::query::QueryEngine>::analysis
  37:     0x7fa9c4d1e089 - <rustc_middle[344b7a5778ebfb41]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>
  38:     0x7fa9c4846e01 - <rustc_interface[cc30d76fa8dae5fc]::interface::Compiler>::enter::<rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}::{closure#2}, core[4ada38e0d5cae8be]::result::Result<core[4ada38e0d5cae8be]::option::Option<rustc_interface[cc30d76fa8dae5fc]::queries::Linker>, rustc_span[d779338a88541528]::ErrorGuaranteed>>
  39:     0x7fa9c4844fb1 - rustc_span[d779338a88541528]::with_source_map::<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7fa9c484455f - std[d743b6fa88cd7a4c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[cc30d76fa8dae5fc]::util::run_in_thread_pool_with_globals<rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>
  41:     0x7fa9c4f7f52e - <<std[d743b6fa88cd7a4c]::thread::Builder>::spawn_unchecked_<rustc_interface[cc30d76fa8dae5fc]::util::run_in_thread_pool_with_globals<rustc_interface[cc30d76fa8dae5fc]::interface::run_compiler<core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>, rustc_driver_impl[2b818a379abe8476]::run_compiler::{closure#1}>::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4ada38e0d5cae8be]::result::Result<(), rustc_span[d779338a88541528]::ErrorGuaranteed>>::{closure#1} as core[4ada38e0d5cae8be]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7fa9c243bf15 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h843c8319f93d5fbd
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:1988:9
  43:     0x7fa9c243bf15 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9f6cdb354bbcea1a
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/alloc/src/boxed.rs:1988:9
  44:     0x7fa9c243bf15 - std::sys::unix::thread::Thread::new::thread_start::h53e32c182a23b75f
                               at /rustc/db0cbc48d4aaa300713a95d9b317a365a474490c/library/std/src/sys/unix/thread.rs:108:17
  45:     0x7fa9c2309609 - start_thread
  46:     0x7fa9c222c133 - clone
  47:                0x0 - <unknown>
