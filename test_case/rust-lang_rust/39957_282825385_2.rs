\n"},"level":"error","spans":[{"file_name":"src/main.rs","byte_start":86,"byte_end":93,"line_start":7,"line_end":7,"column_start":6,"column_end":13,"is_primary":true,"text":[{"text":"impl TryFrom for Test {}","highlight_start":6,"highlight_end":13}],"label":"expected 1 type argument","suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}
{"message":"/checkout/src/librustc_save_analysis/lib.rs:390: Could not find container for method 19","code":null,"level":"error: internal compiler error","spans":[{"file_name":"src/main.rs","byte_start":59,"byte_end":64,"line_start":4,"line_end":4,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"#[derive(Debug)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":{"span":{"file_name":"src/main.rs","byte_start":59,"byte_end":64,"line_start":4,"line_end":4,"column_start":10,"column_end":15,"is_primary":false,"text":[{"text":"#[derive(Debug)]","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null},"macro_decl_name":"#[derive(Debug)]","def_site_span":null}}],"children":[],"rendered":null}
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:376
stack backtrace:
   1:     0x7f5379387f29 - std::sys::imp::backtrace::tracing::imp::write::hbb14611794d3841b
                        at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f53793965c9 - std::panicking::default_hook::{{closure}}::h6ed906c7818ac88c
                        at /checkout/src/libstd/panicking.rs:351
   3:     0x7f5379396166 - std::panicking::default_hook::h23eeafbf7c1c05c3
                        at /checkout/src/libstd/panicking.rs:361
   4:     0x7f5379396a2b - std::panicking::rust_panic_with_hook::hd0067971b6d1240e
                        at /checkout/src/libstd/panicking.rs:545
   5:     0x7f5376e3fbd8 - std::panicking::begin_panic::h366b5d250763eeeb
   6:     0x7f5376e4ed45 - rustc::session::opt_span_bug_fmt::{{closure}}::hf34d75eb1ccfc4b0
   7:     0x7f5376e4eb5a - rustc::session::span_bug_fmt::hb1e0ab0ab4f30a37
   8:     0x7f5376eb9be0 - rustc_save_analysis::SaveContext::get_method_data::h0d5a5564890fb4e1
   9:     0x7f5376e7ae7e - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D>>::process_method::h5ea37817d7e0ed44
  10:     0x7f5376e8d70c - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D>>::process_impl_item::h43f683221ffd0120
  11:     0x7f5376ea430a - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D> as syntax::visit::Visitor<'l>>::visit_item::hd8ca901740b4d338
  12:     0x7f5376ebef5a - rustc_save_analysis::process_crate::h5648bbe73ca8dc22
  13:     0x7f53797649f1 - <rustc_driver::RustcDefaultCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::{{closure}}::hb1dfa6e8a5994df2
  14:     0x7f5379764a8c - <rustc_driver::RustcDefaultCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::h15710042eb9bc2f0
  15:     0x7f5379719e4d - rustc_driver::driver::compile_input::{{closure}}::hc10e3fe5b6db0076
  16:     0x7f53797345cf - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h7d2a6da150c7eba7
  17:     0x7f53796a2a14 - rustc::ty::context::TyCtxt::create_and_enter::h825fbad7d24d1480
  18:     0x7f5379717e13 - rustc_driver::driver::compile_input::hf3e3aa4173908b86
  19:     0x7f537975f3dd - rustc_driver::run_compiler::h8f8d47f1d258a8a6
  20:     0x7f53796697db - std::panicking::try::do_call::h206b9daee04f4ea2
  21:     0x7f537939f8da - __rust_maybe_catch_panic
                        at /checkout/src/libpanic_unwind/lib.rs:98
  22:     0x7f5379691b92 - <F as alloc::boxed::FnBox<A>>::call_box::h5d196fbb3229f499
  23:     0x7f5379395404 - std::sys::imp::thread::Thread::new::thread_start::h2c901daa88f3cb32
                        at /checkout/src/liballoc/boxed.rs:648
                        at /checkout/src/libstd/sys_common/thread.rs:21
                        at /checkout/src/libstd/sys/unix/thread.rs:84
  24:     0x7f537116c453 - start_thread
  25:     0x7f53790557de - __GI___clone
  26:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:376
stack backtrace:
   1:     0x7fc9a2ce8f29 - std::sys::imp::backtrace::tracing::imp::write::hbb14611794d3841b
                        at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7fc9a2cf75c9 - std::panicking::default_hook::{{closure}}::h6ed906c7818ac88c
                        at /checkout/src/libstd/panicking.rs:351
   3:     0x7fc9a2cf7166 - std::panicking::default_hook::h23eeafbf7c1c05c3
                        at /checkout/src/libstd/panicking.rs:361
   4:     0x7fc9a2cf7a2b - std::panicking::rust_panic_with_hook::hd0067971b6d1240e
                        at /checkout/src/libstd/panicking.rs:545
   5:     0x7fc99eeb1bd8 - std::panicking::begin_panic::h366b5d250763eeeb
   6:     0x7fc99eec0d45 - rustc::session::opt_span_bug_fmt::{{closure}}::hf34d75eb1ccfc4b0
   7:     0x7fc99eec0b5a - rustc::session::span_bug_fmt::hb1e0ab0ab4f30a37
   8:     0x7fc99ef2bbe0 - rustc_save_analysis::SaveContext::get_method_data::h0d5a5564890fb4e1
   9:     0x7fc99eeece7e - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D>>::process_method::h5ea37817d7e0ed44
  10:     0x7fc99eeff70c - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D>>::process_impl_item::h43f683221ffd0120
  11:     0x7fc99ef1630a - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, D> as syntax::visit::Visitor<'l>>::visit_item::hd8ca901740b4d338
  12:     0x7fc99ef30f5a - rustc_save_analysis::process_crate::h5648bbe73ca8dc22
  13:     0x7fc9a41c19f1 - <rustc_driver::RustcDefaultCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::{{closure}}::hb1dfa6e8a5994df2
  14:     0x7fc9a41c1a8c - <rustc_driver::RustcDefaultCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::h15710042eb9bc2f0
  15:     0x7fc9a4176e4d - rustc_driver::driver::compile_input::{{closure}}::hc10e3fe5b6db0076
  16:     0x7fc9a41915cf - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h7d2a6da150c7eba7
  17:     0x7fc9a40ffa14 - rustc::ty::context::TyCtxt::create_and_enter::h825fbad7d24d1480
  18:     0x7fc9a4174e13 - rustc_driver::driver::compile_input::hf3e3aa4173908b86
  19:     0x7fc9a41bc3dd - rustc_driver::run_compiler::h8f8d47f1d258a8a6
  20:     0x560c6678e755 - rls::build::BuildQueue::rustc::{{closure}}::{{closure}}::h3ff3641e834cf225
                        at /home/sergey/projects/rls/src/build.rs:469
  21:     0x560c66540d69 - rustc_driver::run::{{closure}}::hb0424d7a686fbd03
                        at /checkout/src/librustc_driver/lib.rs:136
  22:     0x560c6654205b - rustc_driver::monitor::{{closure}}::hb33d44d753453479
                        at /checkout/src/librustc_driver/lib.rs:1063
  23:     0x560c66776ed7 - <std::panic::AssertUnwindSafe<F> as core::ops::FnOnce<()>>::call_once::hcc3f15d1bb8bc826
                        at /checkout/src/libstd/panic.rs:296
  24:     0x560c665daf7b - std::panicking::try::do_call::h8ae5e7c480d867d2
                        at /checkout/src/libstd/panicking.rs:450
  25:     0x7fc9a2d008da - __rust_maybe_catch_panic
                        at /checkout/src/libpanic_unwind/lib.rs:98
  26:     0x560c665d9e9c - std::panicking::try::ha1d7c2dc7cf57463
                        at /checkout/src/libstd/panicking.rs:429
  27:     0x560c665cef21 - std::panic::catch_unwind::h1db870df96f19257
                        at /checkout/src/libstd/panic.rs:361
  28:     0x560c665d68f0 - std::thread::Builder::spawn::{{closure}}::h0220c66d70d225eb
                        at /checkout/src/libstd/thread/mod.rs:360
  29:     0x560c6669adaf - <F as alloc::boxed::FnBox<A>>::call_box::h023b702dfc9ab81d
                        at /checkout/src/liballoc/boxed.rs:638
  30:     0x7fc9a2cf6404 - std::sys::imp::thread::Thread::new::thread_start::h2c901daa88f3cb32
                        at /checkout/src/liballoc/boxed.rs:648
                        at /checkout/src/libstd/sys_common/thread.rs:21
                        at /checkout/src/libstd/sys/unix/thread.rs:84
  31:     0x7fc9a189f453 - start_thread
  32:     0x7fc9a10c77de - __GI___clone
  33:                0x0 - <unknown>

DEBUG:rls::actions: build - Success
DEBUG:rls::server: response: "Content-Length: 664\r\n\r\n{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"uri\":\"file:///home/sergey/projects/test_bug/src/main.rs\",\"diagnostics\":[{\"range\":{\"start\":{\"line\":6,\"character\":5},\"end\":{\"line\":6,\"character\":12},\"label\":\"expected 1 type argument\"},\"secondaryRanges\":[],\"severity\":1,\"code\":\"E0243\",\"source\":\"rustc\",\"message\":\"wrong number of type arguments: expected 1, found 0\\nexpected 1 type argument\"},{\"range\":{\"start\":{\"line\":3,\"character\":9},\"end\":{\"line\":3,\"character\":14},\"label\":null},\"secondaryRanges\":[],\"severity\":2,\"code\":\"\",\"source\":\"rustc\",\"message\":\"/checkout/src/librustc_save_analysis/lib.rs:390: Could not find container for method 19\"}]}}"
INFO:rls_analysis::raw: Considering Listing { kind: File(SystemTime { tv_sec: 1488221817, tv_nsec: 460043764 }), name: "test_bug-2e2cb212f13566c9.json" }
INFO:rls_analysis::raw: read_crate_data "/home/sergey/projects/test_bug/target/rls/debug/deps/save-analysis/test_bug-2e2cb212f13566c9.json"
INFO:rls_analysis::lowering: Lowering /home/sergey/projects/test_bug/target/rls/debug/deps/save-analysis/test_bug-2e2cb212f13566c9.json in 0.00s
INFO:rls_analysis::lowering:     defs:  1
INFO:rls_analysis::lowering:     refs:  0
INFO:rls_analysis::lowering:     globs: 0
INFO:rls_analysis::lowering: Total lowering time: 0.00s
INFO:rls_analysis::lowering: Diff in rss: 0.00KB
DEBUG:rls::server: response: "Content-Length: 70\r\n\r\n{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"
