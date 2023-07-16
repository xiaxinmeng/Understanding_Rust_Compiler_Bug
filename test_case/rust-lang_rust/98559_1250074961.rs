
error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:536:17: cannot relate region: LUB(ReEarlyBound(1, 'a), ReErased)
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/compiler/rustc_errors/src/lib.rs:1460:9
stack backtrace:
   0:     0x7fe0b51b8eb0 - std::backtrace_rs::backtrace::libunwind::trace::h1c0da60f68cd52d0
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7fe0b51b8eb0 - std::backtrace_rs::backtrace::trace_unsynchronized::he45e10f3230d4a33
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fe0b51b8eb0 - std::sys_common::backtrace::_print_fmt::hb7277fb151a2024e
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fe0b51b8eb0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb354ed7aeb9d4ccd
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fe0b5213f0e - core::fmt::write::hec67a20123752277
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/core/src/fmt/mod.rs:1202:17
   5:     0x7fe0b51a9595 - std::io::Write::write_fmt::h86bc6ecfe8b97adf
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/io/mod.rs:1679:15
   6:     0x7fe0b51bbb63 - std::sys_common::backtrace::_print::he9b60e75426b4e2c
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fe0b51bbb63 - std::sys_common::backtrace::print::h0094cd5e7dc5d2a3
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fe0b51bbb63 - std::panicking::default_hook::{{closure}}::he873f51ceb39b7ab
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/panicking.rs:295:22
   9:     0x7fe0b51bb84f - std::panicking::default_hook::hbd7e8dfbbdcbdd8d
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/panicking.rs:314:9
  10:     0x7fe0b7a27561 - rustc_driver[45772c003865a0c4]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fe0b51bc39d - std::panicking::rust_panic_with_hook::h440e574bebabd5f6
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/panicking.rs:702:17
  12:     0x7fe0b8a49871 - std[2c47335e2f8b42a0]::panicking::begin_panic::<rustc_errors[4c251b6c70f47263]::ExplicitBug>::{closure#0}
  13:     0x7fe0b8a48b96 - std[2c47335e2f8b42a0]::sys_common::backtrace::__rust_end_short_backtrace::<std[2c47335e2f8b42a0]::panicking::begin_panic<rustc_errors[4c251b6c70f47263]::ExplicitBug>::{closure#0}, !>
  14:     0x7fe0b89d46d6 - std[2c47335e2f8b42a0]::panicking::begin_panic::<rustc_errors[4c251b6c70f47263]::ExplicitBug>
  15:     0x7fe0b8a457e6 - std[2c47335e2f8b42a0]::panic::panic_any::<rustc_errors[4c251b6c70f47263]::ExplicitBug>
  16:     0x7fe0b8a4456d - <rustc_errors[4c251b6c70f47263]::HandlerInner>::bug::<&alloc[a4747db759a2a335]::string::String>
  17:     0x7fe0b8a43e20 - <rustc_errors[4c251b6c70f47263]::Handler>::bug::<&alloc[a4747db759a2a335]::string::String>
  18:     0x7fe0b8aac14d - rustc_middle[391f554ef2afaf2c]::ty::context::tls::with_context_opt::<rustc_middle[391f554ef2afaf2c]::ty::context::tls::with_opt<rustc_middle[391f554ef2afaf2c]::util::bug::opt_span_bug_fmt<rustc_span[df2b32850c624801]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7fe0b8aad5a6 - rustc_middle[391f554ef2afaf2c]::util::bug::opt_span_bug_fmt::<rustc_span[df2b32850c624801]::span_encoding::Span>
  20:     0x7fe0b63db8b3 - rustc_middle[391f554ef2afaf2c]::util::bug::bug_fmt
  21:     0x7fe0b654d177 - <rustc_infer[536d5a554b49bbbd]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  22:     0x7fe0b654a12b - rustc_infer[536d5a554b49bbbd]::infer::lexical_region_resolve::resolve
  23:     0x7fe0b715443b - <rustc_infer[536d5a554b49bbbd]::infer::InferCtxt>::resolve_regions_and_report_errors
  24:     0x7fe0b7141c09 - <rustc_infer[536d5a554b49bbbd]::infer::InferCtxt>::check_region_obligations_and_report_errors
  25:     0x7fe0b7394e11 - <rustc_infer[536d5a554b49bbbd]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[75df897160b4c8ff]::check::check::check_opaque_meets_bounds::{closure#0}>
  26:     0x7fe0b7374065 - rustc_typeck[75df897160b4c8ff]::check::check::check_item_type
  27:     0x7fe0b7370b2e - rustc_typeck[75df897160b4c8ff]::check::check::check_mod_item_types
  28:     0x7fe0b6959361 - <rustc_query_system[5706997ce835dcb4]::dep_graph::graph::DepGraph<rustc_middle[391f554ef2afaf2c]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[391f554ef2afaf2c]::ty::context::TyCtxt, rustc_span[df2b32850c624801]::def_id::LocalDefId, ()>
  29:     0x7fe0b695812d - rustc_query_system[5706997ce835dcb4]::query::plumbing::try_execute_query::<rustc_query_impl[2e6536e8c723df85]::plumbing::QueryCtxt, rustc_query_system[5706997ce835dcb4]::query::caches::DefaultCache<rustc_span[df2b32850c624801]::def_id::LocalDefId, ()>>
  30:     0x7fe0b749e3ce - rustc_query_system[5706997ce835dcb4]::query::plumbing::get_query::<rustc_query_impl[2e6536e8c723df85]::queries::check_mod_item_types, rustc_query_impl[2e6536e8c723df85]::plumbing::QueryCtxt>
  31:     0x7fe0b774367c - <rustc_middle[391f554ef2afaf2c]::hir::map::Map>::for_each_module::<rustc_typeck[75df897160b4c8ff]::check_crate::{closure#6}::{closure#0}>
  32:     0x7fe0b6d380d5 - rustc_typeck[75df897160b4c8ff]::check_crate
  33:     0x7fe0b6d376c7 - rustc_interface[2e430490b24d666b]::passes::analysis
  34:     0x7fe0b77f93e2 - <rustc_query_system[5706997ce835dcb4]::dep_graph::graph::DepGraph<rustc_middle[391f554ef2afaf2c]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[391f554ef2afaf2c]::ty::context::TyCtxt, (), core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>
  35:     0x7fe0b77f8aaa - rustc_query_system[5706997ce835dcb4]::query::plumbing::try_execute_query::<rustc_query_impl[2e6536e8c723df85]::plumbing::QueryCtxt, rustc_query_system[5706997ce835dcb4]::query::caches::DefaultCache<(), core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>>
  36:     0x7fe0b77f856c - rustc_query_system[5706997ce835dcb4]::query::plumbing::get_query::<rustc_query_impl[2e6536e8c723df85]::queries::analysis, rustc_query_impl[2e6536e8c723df85]::plumbing::QueryCtxt>
  37:     0x7fe0b67dd847 - <rustc_interface[2e430490b24d666b]::passes::QueryContext>::enter::<rustc_driver[45772c003865a0c4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>
  38:     0x7fe0b67d127c - rustc_interface[2e430490b24d666b]::interface::create_compiler_and_run::<core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>, rustc_driver[45772c003865a0c4]::run_compiler::{closure#1}>
  39:     0x7fe0b67cfbc1 - <scoped_tls[18b9aee780902605]::ScopedKey<rustc_span[df2b32850c624801]::SessionGlobals>>::set::<rustc_interface[2e430490b24d666b]::interface::run_compiler<core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>, rustc_driver[45772c003865a0c4]::run_compiler::{closure#1}>::{closure#0}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>
  40:     0x7fe0b67cf8af - std[2c47335e2f8b42a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e430490b24d666b]::util::run_in_thread_pool_with_globals<rustc_interface[2e430490b24d666b]::interface::run_compiler<core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>, rustc_driver[45772c003865a0c4]::run_compiler::{closure#1}>::{closure#0}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>::{closure#0}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>
  41:     0x7fe0b7893cc0 - <<std[2c47335e2f8b42a0]::thread::Builder>::spawn_unchecked_<rustc_interface[2e430490b24d666b]::util::run_in_thread_pool_with_globals<rustc_interface[2e430490b24d666b]::interface::run_compiler<core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>, rustc_driver[45772c003865a0c4]::run_compiler::{closure#1}>::{closure#0}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>::{closure#0}, core[5a729203526575bf]::result::Result<(), rustc_errors[4c251b6c70f47263]::ErrorGuaranteed>>::{closure#1} as core[5a729203526575bf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7fe0b51c6173 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h86a4b020a74606cb
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/alloc/src/boxed.rs:1940:9
  43:     0x7fe0b51c6173 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he22d4ad283374c0d
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/alloc/src/boxed.rs:1940:9
  44:     0x7fe0b51c6173 - std::sys::unix::thread::Thread::new::thread_start::ha509852e7590b369
                               at /rustc/2287107588d92889d282e6cd3c1ca5df34cd34a5/library/std/src/sys/unix/thread.rs:108:17
  45:     0x7fe0b4f136a3 - <unknown>
  46:     0x7fe0b4f9952c - <unknown>
  47:                0x0 - <unknown>
