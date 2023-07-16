
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-dev (d69e9cb15 2017-08-22) running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'byte index 41 is not a char boundary; it is inside 'ი' (bytes 40..43) of `fn საჭმელად_გემრიელი_სადილი ( ) -> isize {
`', src\libcore\str\mod.rs:2188:4
stack backtrace:
   0:     0x7ff9b0dfa507 - std::sys::imp::backtrace::unwind_backtrace
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys\windows\backtrace\mod.rs:65
   1:     0x7ff9b0de65a5 - std::sys_common::backtrace::_print
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys_common\backtrace.rs:71
   2:     0x7ff9b0de5b25 - std::sys_common::backtrace::print
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys_common\backtrace.rs:60
   3:     0x7ff9b0e1cad5 - std::panicking::default_hook::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:381
   4:     0x7ff9b0e1c42c - std::panicking::default_hook
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:391
   5:     0x7ff9b0e1d610 - std::panicking::rust_panic_with_hook
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:611
   6:     0x7ff9b0e1d472 - std::panicking::begin_panic<alloc::string::String>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:572
   7:     0x7ff9b0e1d22e - std::panicking::begin_panic_fmt
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:522
   8:     0x7ff9b0e1d18a - std::panicking::rust_begin_panic
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:498
   9:     0x7ff9b0e7e917 - core::panicking::panic_fmt
                               at C:\Users\matth\Code\Rust\rust\src\libcore\panicking.rs:71
  10:     0x7ff9b0e8d4f4 - core::str::slice_error_fail
                               at C:\Users\matth\Code\Rust\rust\src\libcore\str\mod.rs:2188
  11:     0x7ff9bcef5ca4 - core::str::traits::{{impl}}::index::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\libcore\str\mod.rs:1846
  12:     0x7ff9bcec0bce - core::option::Option<&str>::unwrap_or_else<&str,closure>
                               at C:\Users\matth\Code\Rust\rust\src\libcore\option.rs:370
  13:     0x7ff9bcef5c38 - core::str::traits::{{impl}}::index
                               at C:\Users\matth\Code\Rust\rust\src\libcore\str\mod.rs:1846
  14:     0x7ff9bcef5960 - core::str::traits::{{impl}}::index
                               at C:\Users\matth\Code\Rust\rust\src\libcore\str\mod.rs:1620
  15:     0x7ff9bcf4f4d5 - rustc_errors::emitter::EmitterWriter::render_source_line
                               at C:\Users\matth\Code\Rust\rust\src\librustc_errors\emitter.rs:314
  16:     0x7ff9bcf56d11 - rustc_errors::emitter::EmitterWriter::emit_message_default
                               at C:\Users\matth\Code\Rust\rust\src\librustc_errors\emitter.rs:987
  17:     0x7ff9bcf5ad98 - rustc_errors::emitter::EmitterWriter::emit_messages_default
                               at C:\Users\matth\Code\Rust\rust\src\librustc_errors\emitter.rs:1155
  18:     0x7ff9bcf4ba91 - rustc_errors::emitter::{{impl}}::emit
                               at C:\Users\matth\Code\Rust\rust\src\librustc_errors\emitter.rs:75
  19:     0x7ff9bcf4adda - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
                               at C:\Users\matth\Code\Rust\rust\src\librustc_errors\diagnostic_builder.rs:101
  20:     0x7ff99e1dbfed - rustc_typeck::check::coercion::CoerceMany<syntax::ptr::P<rustc::hir::Expr>>::coerce_inner<syntax::ptr::P<rustc::hir::Expr>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\coercion.rs:1211
  21:     0x7ff99e1d9ee9 - rustc_typeck::check::coercion::CoerceMany<syntax::ptr::P<rustc::hir::Expr>>::coerce_forced_unit<syntax::ptr::P<rustc::hir::Expr>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\coercion.rs:1066
  22:     0x7ff99e290bc4 - rustc_typeck::check::{{impl}}::check_block_with_expected::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:4251
  23:     0x7ff99e2995d7 - rustc_typeck::check::FnCtxt::with_breakable_ctxt<closure,()>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:4887
  24:     0x7ff99e290216 - rustc_typeck::check::FnCtxt::check_block_with_expected
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:4218
  25:     0x7ff99e2891f8 - rustc_typeck::check::FnCtxt::check_expr_kind
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:3813
  26:     0x7ff99e287d27 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:3436
  27:     0x7ff99e27a9b3 - rustc_typeck::check::FnCtxt::check_expr_with_expectation
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:2717
  28:     0x7ff99e27a931 - rustc_typeck::check::FnCtxt::check_expr_with_hint
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:2711
  29:     0x7ff99e27c81b - rustc_typeck::check::FnCtxt::check_return_expr
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:2851
  30:     0x7ff99e25d87e - rustc_typeck::check::check_fn
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:1034
  31:     0x7ff99e25b841 - rustc_typeck::check::typeck_tables_of::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:868
  32:     0x7ff99e253a30 - rustc_typeck::check::{{impl}}::enter::{{closure}}<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:595
  33:     0x7ff99e02bebc - rustc::infer::{{impl}}::enter::{{closure}}<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\infer\mod.rs:375
  34:     0x7ff99dfeb9a8 - rustc::ty::context::tls::enter::{{closure}}<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1262
  35:     0x7ff99debaca6 - std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>>::try_with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:365
  36:     0x7ff99deb77de - std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>>::with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:279
  37:     0x7ff99dfea93b - rustc::ty::context::tls::enter<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1259
  38:     0x7ff99dfe8b54 - rustc::ty::context::GlobalCtxt::enter_local<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1073
  39:     0x7ff99e02b581 - rustc::infer::InferCtxtBuilder::enter<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\infer\mod.rs:375
  40:     0x7ff99e252d19 - rustc_typeck::check::InheritedBuilder::enter<closure,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:595
  41:     0x7ff99e25b1a0 - rustc_typeck::check::typeck_tables_of
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:852
  42:     0x7ff99b968c21 - rustc::ty::maps::{{impl}}::try_get_with::{{closure}}::run_provider
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:609
  43:     0x7ff99b40b87f - rustc::dep_graph::graph::DepGraph::with_task<rustc::ty::context::TyCtxt,rustc::hir::def_id::DefId,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\dep_graph\graph.rs:125
  44:     0x7ff99b968f0e - rustc::ty::maps::{{impl}}::try_get_with::{{closure}}<fn(&&rustc::ty::context::TypeckTables) -> &rustc::ty::context::TypeckTables,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:612
  45:     0x7ff99b7c8d5a - rustc::ty::context::TyCtxt::cycle_check<closure,(&rustc::ty::context::TypeckTables, rustc::dep_graph::edges::DepNodeIndex)>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:257
  46:     0x7ff99b96826f - rustc::ty::maps::queries::typeck_tables_of::try_get_with<fn(&&rustc::ty::context::TypeckTables) -> &rustc::ty::context::TypeckTables,&rustc::ty::context::TypeckTables>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:596
  47:     0x7ff99b968fc4 - rustc::ty::maps::queries::typeck_tables_of::try_get
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:629
  48:     0x7ff99b9c9df9 - rustc::ty::maps::TyCtxtAt::typeck_tables_of
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:675
  49:     0x7ff99b9c2ce4 - rustc::ty::context::TyCtxt::typeck_tables_of
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:668
  50:     0x7ff99e25a104 - rustc_typeck::check::typeck_item_bodies::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:726
  51:     0x7ff99e054a0f - rustc::session::Session::track_errors<closure,()>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\session\mod.rs:270
  52:     0x7ff99e259fa6 - rustc_typeck::check::typeck_item_bodies
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:724
  53:     0x7ff99b966969 - rustc::ty::maps::{{impl}}::try_get_with::{{closure}}::run_provider
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:609
  54:     0x7ff99b418b6d - rustc::dep_graph::graph::DepGraph::with_task<rustc::ty::context::TyCtxt,rustc::hir::def_id::CrateNum,core::result::Result<(), rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\dep_graph\graph.rs:125
  55:     0x7ff99b966aca - rustc::ty::maps::{{impl}}::try_get_with::{{closure}}<fn(&core::result::Result<(), rustc::session::CompileIncomplete>) -> core::result::Result<(), rustc::session::CompileIncomplete>,core::result::Result<(), rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:612
  56:     0x7ff99b7a8ceb - rustc::ty::context::TyCtxt::cycle_check<closure,(core::result::Result<(), rustc::session::CompileIncomplete>, rustc::dep_graph::edges::DepNodeIndex)>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:257
  57:     0x7ff99b9652e7 - rustc::ty::maps::queries::typeck_item_bodies::try_get_with<fn(&core::result::Result<(), rustc::session::CompileIncomplete>) -> core::result::Result<(), rustc::session::CompileIncomplete>,core::result::Result<(), rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:596
  58:     0x7ff99b966cf8 - rustc::ty::maps::queries::typeck_item_bodies::try_get
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:629
  59:     0x7ff99b9c9b05 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:675
  60:     0x7ff99b9c2c3c - rustc::ty::context::TyCtxt::typeck_item_bodies
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\maps.rs:668
  61:     0x7ff99e259eda - rustc_typeck::check::check_item_bodies
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\check\mod.rs:717
  62:     0x7ff99e2e2b8c - rustc_typeck::check_crate::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\lib.rs:326
  63:     0x7ff99e0275ca - rustc::util::common::time<core::result::Result<(), rustc::session::CompileIncomplete>,closure>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\util\common.rs:48
  64:     0x7ff99e2e2698 - rustc_typeck::check_crate
                               at C:\Users\matth\Code\Rust\rust\src\librustc_typeck\lib.rs:326
  65:     0x7ff9a603eb66 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\driver.rs:1041
  66:     0x7ff9a5d962c4 - rustc::ty::context::tls::enter::{{closure}}<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1262
  67:     0x7ff9a5c75066 - std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>>::try_with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:365
  68:     0x7ff9a5c55866 - std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>>::with<core::cell::Cell<core::option::Option<(*const rustc::ty::context::tls::ThreadLocalGlobalCtxt, *const rustc::ty::context::tls::ThreadLocalInterners)>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:279
  69:     0x7ff9a5d95a39 - rustc::ty::context::tls::enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1259
  70:     0x7ff9a5d94fa0 - rustc::ty::context::tls::enter_global::{{closure}}<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1246
  71:     0x7ff9a5c64ac2 - std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>>::try_with<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:365
  72:     0x7ff9a5c4e4b5 - std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\local.rs:279
  73:     0x7ff9a5d946d0 - rustc::ty::context::tls::enter_global<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1243
  74:     0x7ff9a5d9c417 - rustc::ty::context::TyCtxt::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc\ty\context.rs:1024
  75:     0x7ff9a60383b3 - rustc_driver::driver::phase_3_run_analysis_passes<closure,core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation), rustc::session::CompileIncomplete>>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\driver.rs:1010
  76:     0x7ff9a602627d - rustc_driver::driver::compile_input
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\driver.rs:196
  77:     0x7ff9a6056151 - rustc_driver::run_compiler
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\lib.rs:316
  78:     0x7ff9a6065d27 - rustc_driver::main::{{closure}}
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\lib.rs:1338
  79:     0x7ff9a6054f12 - rustc_driver::run::{{closure}}<closure>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\lib.rs:132
  80:     0x7ff9a60653d8 - rustc_driver::monitor::{{closure}}<closure>
                               at C:\Users\matth\Code\Rust\rust\src\librustc_driver\lib.rs:1255
  81:     0x7ff9a5c38b1e - std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys_common\backtrace.rs:136
  82:     0x7ff9a5c3f57e - std::thread::{{impl}}::spawn::{{closure}}::{{closure}}<closure,()>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\mod.rs:394
  83:     0x7ff9a5bc338e - std::panic::{{impl}}::call_once<(),closure>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panic.rs:296
  84:     0x7ff9a5c3fb6b - std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:480
  85:     0x7ff9b0e26272 - panic_unwind::__rust_start_panic
  86:     0x7ff9b0e2614f - panic_unwind::__rust_maybe_catch_panic
                               at C:\Users\matth\Code\Rust\rust\src\libpanic_unwind\lib.rs:98
  87:     0x7ff9a5c3f9da - std::panicking::try<(),std::panic::AssertUnwindSafe<closure>>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panicking.rs:459
  88:     0x7ff9a5c3ebe5 - std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\panic.rs:361
  89:     0x7ff9a5c3f38e - std::thread::{{impl}}::spawn::{{closure}}<closure,()>
                               at C:\Users\matth\Code\Rust\rust\src\libstd\thread\mod.rs:393
  90:     0x7ff9a5d449d9 - alloc::boxed::{{impl}}::call_box<(),closure>
                               at C:\Users\matth\Code\Rust\rust\src\liballoc\boxed.rs:682
  91:     0x7ff9b0d43d6f - alloc::boxed::{{impl}}::call_once<(),()>
                               at C:\Users\matth\Code\Rust\rust\src\liballoc\boxed.rs:692
  92:     0x7ff9b0dee6ef - std::sys_common::thread::start_thread
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys_common\thread.rs:21
  93:     0x7ff9b0e1592e - std::sys::imp::thread::{{impl}}::new::thread_start
                               at C:\Users\matth\Code\Rust\rust\src\libstd\sys\windows\thread.rs:51
  94:     0x7ff9ff7e2773 - BaseThreadInitThunk

