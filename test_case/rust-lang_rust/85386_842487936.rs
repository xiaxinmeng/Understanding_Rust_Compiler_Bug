
    Checking graphite-editor-core v0.1.0 (/home/dennis/Projects/rust/Graphite/core/editor)
thread 'rustc' panicked at 'Failed to get crate data for crate20', compiler/rustc_metadata/src/creader.rs:136:32
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.1 (9bc8c42bb 2021-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: internal compiler error: expansion entered force mode without producing any errors
 --> core/editor/src/communication/document_action_handler.rs:7:1
  |
7 | #[impl_message(Message, Document)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_expand/src/expand.rs:450:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1013:13
stack backtrace:
   0:     0x7f314d29b990 - std::backtrace_rs::backtrace::libunwind::trace::h63b7a90188ab5fb3
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f314d29b990 - std::backtrace_rs::backtrace::trace_unsynchronized::h80aefbf9b851eca7
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f314d29b990 - std::sys_common::backtrace::_print_fmt::hbef05ae4237a4d72
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f314d29b990 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h28abce2fdb9884c2
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f314d309f1f - core::fmt::write::h3b84512577ca38a8
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/fmt/mod.rs:1092:17
   5:     0x7f314d28fae2 - std::io::Write::write_fmt::h465f8feea02e2aa1
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/io/mod.rs:1572:15
   6:     0x7f314d29f7d5 - std::sys_common::backtrace::_print::h525280ee0d29bdde
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f314d29f7d5 - std::sys_common::backtrace::print::h1f0f5b9f3ef8fb78
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f314d29f7d5 - std::panicking::default_hook::{{closure}}::ha5838f6faa4a5a8f
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:208:50
   9:     0x7f314d29f283 - std::panicking::default_hook::hfb9fe98acb0dcb3b
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:225:9
  10:     0x7f314da705eb - rustc_driver::report_ice::hfd0c5c387cfb7249
  11:     0x7f3142c33413 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hbf80f92e2ef22df5
                               at /home/dennis/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:1560:9
  12:     0x7f3142cd45f6 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h49290f5a3dfb1191
                               at /home/dennis/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7f314d29ff40 - std::panicking::rust_panic_with_hook::hb89f5f19036e6af8
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:595:17
  14:     0x7f314d29fab7 - std::panicking::begin_panic_handler::{{closure}}::h119e7951427f41da
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:497:13
  15:     0x7f314d29be4c - std::sys_common::backtrace::__rust_end_short_backtrace::hce386c44bf47a128
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys_common/backtrace.rs:141:18
  16:     0x7f314d29fa19 - rust_begin_unwind
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:493:5
  17:     0x7f314d26416b - std::panicking::begin_panic_fmt::h400b8e9dca200408
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:435:5
  18:     0x7f314ffa340e - rustc_errors::HandlerInner::flush_delayed::h263755ef17509b2a
  19:     0x7f314ffa1d7b - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h37a2b959f25c1a7e
  20:     0x7f314f714506 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::hf3df97dfbfd1ff07
  21:     0x7f314f71713d - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hed6de597fa439e98
  22:     0x7f314f71340d - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h6861ead4ffd5d1e1
  23:     0x7f314f712f48 - rustc_span::with_source_map::h2cd9a6094ef7af0e
  24:     0x7f314f71904e - rustc_interface::interface::create_compiler_and_run::h5877f5f12804db55
  25:     0x7f314f7138f8 - scoped_tls::ScopedKey<T>::set::hd60de2a62f3fb82b
  26:     0x7f314f7193fb - std::sys_common::backtrace::__rust_begin_short_backtrace::hf09e28c4d01a4892
  27:     0x7f314f730a35 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hff3afb7d3b239db0
  28:     0x7f314d2af72a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc444a77f8dd8d825
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/alloc/src/boxed.rs:1546:9
  29:     0x7f314d2af72a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8b68a0a9a2093dfc
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/alloc/src/boxed.rs:1546:9
  30:     0x7f314d2af72a - std::sys::unix::thread::Thread::new::thread_start::hb95464447f61f48d
                               at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/sys/unix/thread.rs:71:17
  31:     0x7f314d1b4259 - start_thread
  32:     0x7f314d0c95e3 - __GI___clone
  33:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.1 (9bc8c42bb 2021-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `graphite-editor-core`

Caused by:
  process didn't exit successfully: `rustc --crate-name graphite_editor_core --edition=2018 core/editor/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=10a1f6dc65bf6e11 -C extra-filename=-10a1f6dc65bf6e11 --out-dir /home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps --target wasm32-unknown-unknown -C incremental=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/incremental -L dependency=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps -L dependency=/home/dennis/Projects/rust/Graphite/target/debug/deps --extern bitflags=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps/libbitflags-32b8b9be07796006.rmeta --extern document_core=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps/libgraphite_document_core-753b9e7c4d48b2df.rmeta --extern proc_macros=/home/dennis/Projects/rust/Graphite/target/debug/deps/libgraphite_proc_macros-81b09e1ce2e4acb2.so --extern log=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps/liblog-b4e782aaa47756a7.rmeta --extern serde=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps/libserde-da223e3e6520aad4.rmeta --extern thiserror=/home/dennis/Projects/rust/Graphite/target/wasm32-unknown-unknown/debug/deps/libthiserror-9cc1f778e48efd39.rmeta` (signal: 4, SIGILL: illegal instruction)
