
[INFO] [stderr] thread 'rustc' panicked at 'Box<Any>', /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/panic.rs:59:5
[INFO] [stdout] error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:568:17: symbol_names: unsupported constant of type `[u32; 16]` (Const { ty: [u32; 16], val: Value(ByRef { alloc: Allocation { bytes: [8, 0, 0, 0, 24, 0, 0, 0, 9, 0, 0, 0, 25, 0, 0, 0, 10, 0, 0, 0, 26, 0, 0, 0, 11, 0, 0, 0, 27, 0, 0, 0, 12, 0, 0, 0, 28, 0, 0, 0, 13, 0, 0, 0, 29, 0, 0, 0, 14, 0, 0, 0, 30, 0, 0, 0, 15, 0, 0, 0, 31, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [18446744073709551615, 0], len: Size { raw: 64 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) })
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stderr] stack backtrace:
[INFO] [stderr]    0:     0x7f328cbf74c0 - std[77c0456e0eb29f0c]::backtrace_rs::backtrace::libunwind::trace
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
[INFO] [stderr]    1:     0x7f328cbf74c0 - std[77c0456e0eb29f0c]::backtrace_rs::backtrace::trace_unsynchronized::<std[77c0456e0eb29f0c]::sys_common::backtrace::_print_fmt::{closure#1}>
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
[INFO] [stderr]    2:     0x7f328cbf74c0 - std[77c0456e0eb29f0c]::sys_common::backtrace::_print_fmt
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/sys_common/backtrace.rs:67:5
[INFO] [stderr]    3:     0x7f328cbf74c0 - <std[77c0456e0eb29f0c]::sys_common::backtrace::_print::DisplayBacktrace as core[ee1ca6415511a9c4]::fmt::Display>::fmt
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/sys_common/backtrace.rs:46:22
[INFO] [stderr]    4:     0x7f328cc64e9c - core[ee1ca6415511a9c4]::fmt::write
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/core/src/fmt/mod.rs:1110:17
[INFO] [stderr]    5:     0x7f328cbe8e15 - <std[77c0456e0eb29f0c]::sys::unix::stdio::Stderr as std[77c0456e0eb29f0c]::io::Write>::write_fmt
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/io/mod.rs:1584:15
[INFO] [stderr]    6:     0x7f328cbfb20b - std[77c0456e0eb29f0c]::sys_common::backtrace::_print
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/sys_common/backtrace.rs:49:5
[INFO] [stderr]    7:     0x7f328cbfb20b - std[77c0456e0eb29f0c]::sys_common::backtrace::print
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/sys_common/backtrace.rs:36:9
[INFO] [stderr]    8:     0x7f328cbfb20b - std[77c0456e0eb29f0c]::panicking::default_hook::{closure#1}
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/panicking.rs:208:50
[INFO] [stderr]    9:     0x7f328cbface1 - std[77c0456e0eb29f0c]::panicking::default_hook
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/panicking.rs:225:9
[INFO] [stderr]   10:     0x7f328d3b0afd - rustc_driver[483abbcf9e72e7cf]::report_ice
[INFO] [stderr]   11:     0x7f328cbfba16 - std[77c0456e0eb29f0c]::panicking::rust_panic_with_hook
[INFO] [stderr]                                at /rustc/ce59db7a1b1b012fb2793c4641c1bdecad7a128b/library/std/src/panicking.rs:626:17
[INFO] [stderr]   12:     0x7f328e37234b - std[77c0456e0eb29f0c]::panicking::begin_panic::<rustc_errors[a6601eba5631ab8e]::ExplicitBug>::{closure#0}
[INFO] [stderr]   13:     0x7f328e372296 - std[77c0456e0eb29f0c]::sys_common::backtrace::__rust_end_short_backtrace::<std[77c0456e0eb29f0c]::panicking::begin_panic<rustc_errors[a6601eba5631ab8e]::ExplicitBug>::{closure#0}, !>
[INFO] [stderr]   14:     0x7f328e37361f - std[77c0456e0eb29f0c]::panicking::begin_panic::<rustc_errors[a6601eba5631ab8e]::ExplicitBug>
[INFO] [stderr]   15:     0x7f328e38b73d - std[77c0456e0eb29f0c]::panic::panic_any::<rustc_errors[a6601eba5631ab8e]::ExplicitBug>
[INFO] [stderr]   16:     0x7f328e38e5da - <rustc_errors[a6601eba5631ab8e]::HandlerInner>::bug
[INFO] [stderr]   17:     0x7f328e38e080 - <rustc_errors[a6601eba5631ab8e]::Handler>::bug
[INFO] [stderr]   18:     0x7f328e2c8450 - rustc_middle[4cd8eb4e66530e50]::ty::context::tls::with_opt::<rustc_middle[4cd8eb4e66530e50]::util::bug::opt_span_bug_fmt<rustc_span[41f5d8395d191526]::span_encoding::Span>::{closure#0}, ()>
[INFO] [stderr]   19:     0x7f328e2cd500 - rustc_middle[4cd8eb4e66530e50]::util::bug::opt_span_bug_fmt::<rustc_span[41f5d8395d191526]::span_encoding::Span>
[INFO] [stderr]   20:     0x7f328e2cd476 - rustc_middle[4cd8eb4e66530e50]::util::bug::bug_fmt
[INFO] [stderr]   21:     0x7f328f4f7957 - <rustc_symbol_mangling[e3ac7cad7aae7613]::v0::SymbolMangler as rustc_middle[4cd8eb4e66530e50]::ty::print::Printer>::print_const
[INFO] [stderr]   22:     0x7f328ead1f79 - <rustc_symbol_mangling[e3ac7cad7aae7613]::v0::SymbolMangler as rustc_middle[4cd8eb4e66530e50]::ty::print::Printer>::print_def_path
[INFO] [stderr]   23:     0x7f328eace1ac - rustc_symbol_mangling[e3ac7cad7aae7613]::v0::mangle
[INFO] [stderr]   24:     0x7f328eadafba - rustc_symbol_mangling[e3ac7cad7aae7613]::symbol_name_provider
[INFO] [stderr]   25:     0x7f328e7a7183 - <rustc_query_impl[73652419363b7eb3]::queries::symbol_name as rustc_query_system[195a98efe178d20a]::query::config::QueryAccessors<rustc_query_impl[73652419363b7eb3]::plumbing::QueryCtxt>>::compute
[INFO] [stderr]   26:     0x7f328e7ee766 - <rustc_query_system[195a98efe178d20a]::dep_graph::graph::DepGraph<rustc_middle[4cd8eb4e66530e50]::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_query_impl[73652419363b7eb3]::plumbing::QueryCtxt, rustc_middle[4cd8eb4e66530e50]::ty::instance::Instance, rustc_middle[4cd8eb4e66530e50]::ty::SymbolName, for<'a, 'b> fn(&'a mut rustc_middle[4cd8eb4e66530e50]::ich::hcx::StableHashingContext, &'b rustc_middle[4cd8eb4e66530e50]::ty::SymbolName) -> core[ee1ca6415511a9c4]::option::Option<rustc_data_structures[52e6d6087bd5b52c]::fingerprint::Fingerprint>>
[INFO] [stderr]   27:     0x7f328e7fa9af - rustc_data_structures[52e6d6087bd5b52c]::stack::ensure_sufficient_stack::<(rustc_middle[4cd8eb4e66530e50]::ty::SymbolName, rustc_query_system[195a98efe178d20a]::dep_graph::graph::DepNodeIndex), rustc_query_system[195a98efe178d20a]::query::plumbing::force_query_with_job<rustc_query_system[195a98efe178d20a]::query::caches::DefaultCache<rustc_middle[4cd8eb4e66530e50]::ty::instance::Instance, rustc_middle[4cd8eb4e66530e50]::ty::SymbolName>, rustc_query_impl[73652419363b7eb3]::plumbing::QueryCtxt>::{closure#0}::{closure#0}>
[...]
