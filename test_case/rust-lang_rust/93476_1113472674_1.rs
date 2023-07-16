
[INFO] [stderr]  Documenting crash-handler v0.1.0 (/opt/rustwide/workdir)
[INFO] [stderr] thread 'rustc' panicked at 'no entry found for key', src/librustdoc/passes/collect_intra_doc_links.rs:986:16
[INFO] [stderr] stack backtrace:
[INFO] [stderr]    0:     0x7f8e7f064f8d - std::backtrace_rs::backtrace::libunwind::trace::he0e523aecaf9b69a
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
[INFO] [stderr]    1:     0x7f8e7f064f8d - std::backtrace_rs::backtrace::trace_unsynchronized::h0b3d5356c458deb9
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
[INFO] [stderr]    2:     0x7f8e7f064f8d - std::sys_common::backtrace::_print_fmt::h532142ee1f4c3cce
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys_common/backtrace.rs:66:5
[INFO] [stderr]    3:     0x7f8e7f064f8d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0e80dc5906b67229
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys_common/backtrace.rs:45:22
[INFO] [stderr]    4:     0x7f8e7f0c0d0c - core::fmt::write::h513fcc964bdd71c0
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/core/src/fmt/mod.rs:1194:17
[INFO] [stderr]    5:     0x7f8e7f056681 - std::io::Write::write_fmt::h3cddf21fb8a62a54
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/io/mod.rs:1655:15
[INFO] [stderr]    6:     0x7f8e7f067ca5 - std::sys_common::backtrace::_print::h0c926e8785159afb
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys_common/backtrace.rs:48:5
[INFO] [stderr]    7:     0x7f8e7f067ca5 - std::sys_common::backtrace::print::hd5783f5f7c42c4da
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys_common/backtrace.rs:35:9
[INFO] [stderr]    8:     0x7f8e7f067ca5 - std::panicking::default_hook::{{closure}}::h315d25e8d5d703cd
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/panicking.rs:295:22
[INFO] [stderr]    9:     0x7f8e7f067919 - std::panicking::default_hook::h4301451e8ab677e5
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/panicking.rs:314:9
[INFO] [stderr]   10:     0x7f8e7f80a981 - rustc_driver[7ec1e2549f75ff84]::DEFAULT_HOOK::{closure#0}::{closure#0}
[INFO] [stderr]   11:     0x7f8e7f068476 - std::panicking::rust_panic_with_hook::h9ade200275992b5b
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/panicking.rs:702:17
[INFO] [stderr]   12:     0x7f8e7f068277 - std::panicking::begin_panic_handler::{{closure}}::h3460cd0d1874a7f9
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/panicking.rs:588:13
[INFO] [stderr]   13:     0x7f8e7f065444 - std::sys_common::backtrace::__rust_end_short_backtrace::h01570b830acaf147
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys_common/backtrace.rs:138:18
[INFO] [stderr]   14:     0x7f8e7f067fa9 - rust_begin_unwind
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/panicking.rs:584:5
[INFO] [stderr]   15:     0x7f8e7f02d2c3 - core::panicking::panic_fmt::h188393b449504c74
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/core/src/panicking.rs:142:14
[INFO] [stderr]   16:     0x7f8e7f0bd971 - core::panicking::panic_display::h20a3f045e4a98ba7
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/core/src/panicking.rs:72:5
[INFO] [stderr]   17:     0x7f8e7f0bd91b - core::panicking::panic_str::h4b1f9ab4ea89367e
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/core/src/panicking.rs:56:5
[INFO] [stderr]   18:     0x7f8e7f02d136 - core::option::expect_failed::h1ece3d7c55836f36
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/core/src/option.rs:1855:5
[INFO] [stderr]   19:     0x55647826a979 - rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::resolve_associated_trait_item
[INFO] [stderr]   20:     0x55647826911e - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector>::resolve_associated_item
[INFO] [stderr]   21:     0x5564782662b5 - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector>::resolve
[INFO] [stderr]   22:     0x55647826c410 - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector as rustdoc[61d2f2338f9af60b]::visit::DocVisitor>::visit_item
[INFO] [stderr]   23:     0x55647827e94a - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector as rustdoc[61d2f2338f9af60b]::visit::DocVisitor>::visit_inner_recur
[INFO] [stderr]   24:     0x55647826fa1e - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector as rustdoc[61d2f2338f9af60b]::visit::DocVisitor>::visit_item
[INFO] [stderr]   25:     0x55647827e9ea - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector as rustdoc[61d2f2338f9af60b]::visit::DocVisitor>::visit_inner_recur
[INFO] [stderr]   26:     0x55647826fa03 - <rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::LinkCollector as rustdoc[61d2f2338f9af60b]::visit::DocVisitor>::visit_item
[INFO] [stderr]   27:     0x55647826464c - rustdoc[61d2f2338f9af60b]::passes::collect_intra_doc_links::collect_intra_doc_links
[INFO] [stderr]   28:     0x5564782d9313 - <rustc_interface[8b8682ff349448b4]::passes::QueryContext>::enter::<rustdoc[61d2f2338f9af60b]::main_options::{closure#0}::{closure#0}::{closure#1}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>::{closure#0}
[INFO] [stderr]   29:     0x5564782b9480 - <rustc_interface[8b8682ff349448b4]::passes::QueryContext>::enter::<rustdoc[61d2f2338f9af60b]::main_options::{closure#0}::{closure#0}::{closure#1}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>
[INFO] [stderr]   30:     0x556478203eef - <rustc_interface[8b8682ff349448b4]::interface::Compiler>::enter::<rustdoc[61d2f2338f9af60b]::main_options::{closure#0}::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>
[INFO] [stderr]   31:     0x55647808db8a - rustc_span[c6bedfd0c66fb146]::with_source_map::<core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>, rustc_interface[8b8682ff349448b4]::interface::create_compiler_and_run<core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>, rustdoc[61d2f2338f9af60b]::main_options::{closure#0}>::{closure#1}>
[INFO] [stderr]   32:     0x55647822bc84 - rustc_interface[8b8682ff349448b4]::interface::create_compiler_and_run::<core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>, rustdoc[61d2f2338f9af60b]::main_options::{closure#0}>
[INFO] [stderr]   33:     0x5564780909d7 - <scoped_tls[3e040dd67153ed8a]::ScopedKey<rustc_span[c6bedfd0c66fb146]::SessionGlobals>>::set::<rustdoc[61d2f2338f9af60b]::main_args::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>
[INFO] [stderr]   34:     0x55647822c59f - std[9bd8fc2679550d2b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8b8682ff349448b4]::util::run_in_thread_pool_with_globals<rustdoc[61d2f2338f9af60b]::main_args::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>
[INFO] [stderr]   35:     0x5564782db949 - <<std[9bd8fc2679550d2b]::thread::Builder>::spawn_unchecked_<rustc_interface[8b8682ff349448b4]::util::run_in_thread_pool_with_globals<rustdoc[61d2f2338f9af60b]::main_args::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>::{closure#0}, core[649e541f68e0944c]::result::Result<(), rustc_errors[94fb9cd2dcbb52d6]::ErrorGuaranteed>>::{closure#1} as core[649e541f68e0944c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
[INFO] [stderr]   36:     0x7f8e7f0723c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1e4cdc05936d75d5
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/alloc/src/boxed.rs:1866:9
[INFO] [stderr]   37:     0x7f8e7f0723c3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h627ceec73be0ad94
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/alloc/src/boxed.rs:1866:9
[INFO] [stderr]   38:     0x7f8e7f0723c3 - std::sys::unix::thread::Thread::new::thread_start::hff6fe6753bb13e69
[INFO] [stderr]                                at /rustc/e85edd9a844b523a02dbd89f3c02cd13e4c9fe46/library/std/src/sys/unix/thread.rs:108:17
[INFO] [stderr]   39:     0x7f8e7ef85609 - start_thread
[INFO] [stderr]   40:     0x7f8e7ed2e163 - clone
[INFO] [stderr]   41:                0x0 - <unknown>
[INFO] [stderr] 
[INFO] [stderr] error: internal compiler error: unexpected panic
[INFO] [stderr] 
[INFO] [stderr] note: the compiler unexpectedly panicked. this is a bug.
[INFO] [stderr] 
[INFO] [stderr] note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
[INFO] [stderr] 
[INFO] [stderr] note: rustc 1.62.0-nightly (e85edd9a8 2022-04-28) running on x86_64-unknown-linux-gnu
[INFO] [stderr] 
[INFO] [stderr] note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options
[INFO] [stderr] 
[INFO] [stderr] note: some of the compiler flags provided by cargo are hidden
[INFO] [stderr] 
[INFO] [stderr] query stack during panic:
[INFO] [stderr] end of query stack
[INFO] [stderr] error: could not document `crash-handler`
[INFO] [stderr] 
[INFO] [stderr] Caused by:
[INFO] [stderr]   process didn't exit successfully: `rustdoc --edition=2021 --crate-type lib --crate-name crash_handler src/lib.rs --target x86_64-unknown-linux-gnu -o /opt/rustwide/target/x86_64-unknown-linux-gnu/doc --cfg 'feature="default"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=9a9f4e4ad82d43dd -L dependency=/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/deps -L dependency=/opt/rustwide/target/debug/deps --extern cfg_if=/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/deps/libcfg_if-c778d32636046772.rmeta --extern crash_context=/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/deps/libcrash_context-eea757ad0eca6db0.rmeta --extern libc=/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/deps/liblibc-f439e9e9c367ace9.rmeta --extern parking_lot=/opt/rustwide/target/x86_64-unknown-linux-gnu/debug/deps/libparking_lot-c437846bba4449be.rmeta --extern-html-root-url 'cfg_if=https://docs.rs/cfg-if/1.0.0/x86_64-unknown-linux-gnu' --extern-html-root-url 'crash_context=https://docs.rs/crash-context/0.2.0/x86_64-unknown-linux-gnu' --extern-html-root-url 'libc=https://docs.rs/libc/0.2.125/x86_64-unknown-linux-gnu' --extern-html-root-url 'parking_lot=https://docs.rs/parking_lot/0.12.0/x86_64-unknown-linux-gnu' -Zunstable-options -Z unstable-options --emit=invocation-specific --resource-suffix -20220428-1.62.0-nightly-e85edd9a8 --static-root-path / --cap-lints warn --disable-per-crate-search --crate-version 0.1.0` (exit status: 101)
