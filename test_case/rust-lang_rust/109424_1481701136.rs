
 Documenting rustix v0.36.10
thread 'rustc' panicked at 'no resolutions for a doc link', compiler/rustc_metadata/src/rmeta/encoder.rs:2253:18
stack backtrace:
   0:     0x7f900fff581a - std::backtrace_rs::backtrace::libunwind::trace::h26518014dbf31aba
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f900fff581a - std::backtrace_rs::backtrace::trace_unsynchronized::ha516581d0aef3757
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f900fff581a - std::sys_common::backtrace::_print_fmt::h9eca712360b21da0
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f900fff581a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h528fecd217131eb4
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f901005944e - core::fmt::write::h073da6791f3f2ff7
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/core/src/fmt/mod.rs:1254:17
   5:     0x7f900ffe8395 - std::io::Write::write_fmt::h51f8756996066b5a
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/io/mod.rs:1698:15
   6:     0x7f900fff55e5 - std::sys_common::backtrace::_print::h5b4ffde9ddd340d3
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f900fff55e5 - std::sys_common::backtrace::print::hde4ce191c0ed53d2
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f900fff835f - std::panicking::default_hook::{{closure}}::h23a2d3c2d62785b5
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/panicking.rs:271:22
   9:     0x7f900fff809b - std::panicking::default_hook::hfe56491cf86bf314
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/panicking.rs:290:9
  10:     0x7f901334df35 - <rustc_driver_impl[ad8fc07c03d45871]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[ee581b2e0272cb3e]::ops::function::FnOnce<(&core[ee581b2e0272cb3e]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f900fff8b9d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hfa9e4663303a5377
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/alloc/src/boxed.rs:2002:9
  12:     0x7f900fff8b9d - std::panicking::rust_panic_with_hook::hb3c1c1d27d072101
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/panicking.rs:696:13
  13:     0x7f900fff8919 - std::panicking::begin_panic_handler::{{closure}}::h52b8863815aadd2e
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/panicking.rs:583:13
  14:     0x7f900fff5c86 - std::sys_common::backtrace::__rust_end_short_backtrace::hc0cd8c631fc1c324
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys_common/backtrace.rs:150:18
  15:     0x7f900fff8622 - rust_begin_unwind
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/panicking.rs:579:5
  16:     0x7f9010055793 - core::panicking::panic_fmt::heb2bd7c92b03c49a
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/core/src/panicking.rs:64:14
  17:     0x7f9010055901 - core::panicking::panic_display::h86181fd327517fb2
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/core/src/panicking.rs:147:5
  18:     0x7f90100558ab - core::panicking::panic_str::h4810192062705dc7
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/core/src/panicking.rs:131:5
  19:     0x7f9010055516 - core::option::expect_failed::h72bd5f9c1943a9a2
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/core/src/option.rs:2055:5
  20:     0x7f90137db51f - <rustc_metadata[b99d73cb12f1c727]::rmeta::encoder::provide::{closure#0} as core[ee581b2e0272cb3e]::ops::function::FnOnce<(rustc_middle[267d604f42fb2b42]::ty::context::TyCtxt, rustc_span[48752dc4b679ffbb]::def_id::LocalDefId)>>::call_once
  21:     0x7f9013bd9258 - rustc_query_system[4e6a7ce19b64cf81]::query::plumbing::try_execute_query::<rustc_query_impl[5e505097f868cbdd]::queries::doc_link_resolutions, rustc_query_impl[5e505097f868cbdd]::plumbing::QueryCtxt>
  22:     0x7f9013c68ba7 - <rustc_query_impl[5e505097f868cbdd]::Queries as rustc_middle[267d604f42fb2b42]::ty::query::QueryEngine>::doc_link_resolutions
  23:     0x562c86a1bc4f - <rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::LinkCollector>::resolve_path
  24:     0x562c86a1be18 - <rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::LinkCollector>::resolve
  25:     0x562c86a1f44d - <rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::LinkCollector>::resolve_links
  26:     0x562c86a26f30 - <rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::LinkCollector as rustdoc[99b5189239f61b47]::visit::DocVisitor>::visit_inner_recur
  27:     0x562c86a26f18 - <rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::LinkCollector as rustdoc[99b5189239f61b47]::visit::DocVisitor>::visit_inner_recur
  28:     0x562c86a1a7de - rustdoc[99b5189239f61b47]::passes::collect_intra_doc_links::collect_intra_doc_links
  29:     0x562c86a2973e - <rustc_session[6aba769f1422c309]::session::Session>::time::<rustdoc[99b5189239f61b47]::clean::types::Crate, rustdoc[99b5189239f61b47]::core::run_global_ctxt::{closure#7}>
  30:     0x562c86adbdab - rustdoc[99b5189239f61b47]::core::run_global_ctxt
  31:     0x562c86a299c9 - <rustc_session[6aba769f1422c309]::session::Session>::time::<(rustdoc[99b5189239f61b47]::clean::types::Crate, rustdoc[99b5189239f61b47]::config::RenderOptions, rustdoc[99b5189239f61b47]::formats::cache::Cache), rustdoc[99b5189239f61b47]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  32:     0x562c868c7527 - <rustc_middle[267d604f42fb2b42]::ty::context::GlobalCtxt>::enter::<rustdoc[99b5189239f61b47]::main_args::{closure#1}::{closure#0}::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>
  33:     0x562c868c6500 - <rustc_interface[dce09aa29aba2b19]::interface::Compiler>::enter::<rustdoc[99b5189239f61b47]::main_args::{closure#1}::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>
  34:     0x562c86a5483b - rustc_span[48752dc4b679ffbb]::with_source_map::<core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>, rustc_interface[dce09aa29aba2b19]::interface::run_compiler<core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>, rustdoc[99b5189239f61b47]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x562c868c9bc4 - <scoped_tls[f64df8b0bc44aebd]::ScopedKey<rustc_span[48752dc4b679ffbb]::SessionGlobals>>::set::<rustc_interface[dce09aa29aba2b19]::interface::run_compiler<core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>, rustdoc[99b5189239f61b47]::main_args::{closure#1}>::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>
  36:     0x562c86a63500 - std[f9545795ea4997bf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dce09aa29aba2b19]::util::run_in_thread_pool_with_globals<rustc_interface[dce09aa29aba2b19]::interface::run_compiler<core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>, rustdoc[99b5189239f61b47]::main_args::{closure#1}>::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>
  37:     0x562c86b084fd - <<std[f9545795ea4997bf]::thread::Builder>::spawn_unchecked_<rustc_interface[dce09aa29aba2b19]::util::run_in_thread_pool_with_globals<rustc_interface[dce09aa29aba2b19]::interface::run_compiler<core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>, rustdoc[99b5189239f61b47]::main_args::{closure#1}>::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ee581b2e0272cb3e]::result::Result<(), rustc_span[48752dc4b679ffbb]::ErrorGuaranteed>>::{closure#1} as core[ee581b2e0272cb3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f9010002c13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h31d31ee934fae5d7
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/alloc/src/boxed.rs:1988:9
  39:     0x7f9010002c13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h235705d08d5be362
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/alloc/src/boxed.rs:1988:9
  40:     0x7f9010002c13 - std::sys::unix::thread::Thread::new::thread_start::h8f78f28fa2155287
                               at /rustc/a266f11990d9544ee408e213e1eec8cc9eb032b7/library/std/src/sys/unix/thread.rs:108:17
  41:     0x7f900fc571da - <unknown>
  42:     0x7f900fcdff44 - __clone
  43:                0x0 - <unknown>
 
error: the compiler unexpectedly panicked. this is a bug.
 
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
 
note: rustc 1.70.0-nightly (a266f1199 2023-03-22) running on x86_64-unknown-linux-gnu
 
note: compiler flags: --crate-type lib
 
note: some of the compiler flags provided by cargo are hidden
 
query stack during panic:
#0 [doc_link_resolutions] resolutions for documentation links for a module
end of query stack
error: could not document `rustix`
 
Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type lib --crate-name rustix /home/deck/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustix-0.36.10/src/lib.rs --cap-lints allow -o /home/deck/Documents/scratchpad/target/doc --cfg 'feature="all-apis"' --cfg 'feature="default"' --cfg 'feature="fs"' --cfg 'feature="io-lifetimes"' --cfg 'feature="io_uring"' --cfg 'feature="itoa"' --cfg 'feature="libc"' --cfg 'feature="mm"' --cfg 'feature="net"' --cfg 'feature="once_cell"' --cfg 'feature="param"' --cfg 'feature="process"' --cfg 'feature="procfs"' --cfg 'feature="rand"' --cfg 'feature="runtime"' --cfg 'feature="std"' --cfg 'feature="termios"' --cfg 'feature="thread"' --cfg 'feature="time"' --cfg 'feature="use-libc-auxv"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=125 -C metadata=c602430f872de611 -L dependency=/home/deck/Documents/scratchpad/target/debug/deps --extern bitflags=/home/deck/Documents/scratchpad/target/debug/deps/libbitflags-f75283d3dcd3daad.rmeta --extern io_lifetimes=/home/deck/Documents/scratchpad/target/debug/deps/libio_lifetimes-4c970ac99651d72e.rmeta --extern itoa=/home/deck/Documents/scratchpad/target/debug/deps/libitoa-85f7878b85724230.rmeta --extern libc=/home/deck/Documents/scratchpad/target/debug/deps/liblibc-8aab922602f2fc15.rmeta --extern linux_raw_sys=/home/deck/Documents/scratchpad/target/debug/deps/liblinux_raw_sys-a31463b73c270e13.rmeta --extern once_cell=/home/deck/Documents/scratchpad/target/debug/deps/libonce_cell-ca06155735a0ee9b.rmeta --crate-version 0.36.10 --cfg rustc_attrs --cfg linux_raw --cfg core_intrinsics --cfg asm` (exit status: 101)
