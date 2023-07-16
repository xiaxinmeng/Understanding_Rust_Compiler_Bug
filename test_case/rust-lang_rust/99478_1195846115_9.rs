sh
william@xubuntu-dtrain:~/Projects/MCVE/proc-macro-ice$ cargo +msp430-fix build
   Compiling proc-macro-ice v0.1.0 (/home/william/Projects/MCVE/proc-macro-ice)
thread 'rustc' panicked at 'explicit panic', macros/src/lib.rs:5:5
stack backtrace:
   0:     0x7f8640a52ced - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hca0a546125a68a55
   1:     0x7f8640ab144e - core::fmt::write::h9cec344321a406cd
   2:     0x7f8640a20fd1 - std::io::Write::write_fmt::h855d135186f11baa
   3:     0x7f8640a30725 - std::panicking::default_hook::{{closure}}::h1a617a00a6e8763c
   4:     0x7f8640a303aa - std::panicking::default_hook::h667877bc124347d5
   5:     0x7f86411d0fd6 - <alloc[2ceaa1994d41407e]::boxed::Box<dyn for<'a, 'b> core[cc9983ef6bac7021]::ops::function::Fn<(&'a core[cc9983ef6bac7021]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[cc9983ef6bac7021]::marker::Send + core[cc9983ef6bac7021]::marker::Sync> as core[cc9983ef6bac7021]::ops::function::Fn<(&core[cc9983ef6bac7021]::panic::panic_info::PanicInfo,)>>::call
                               at /home/william/Projects/toolchains/rust/library/alloc/src/boxed.rs:1886:9
   6:     0x7f86411d0fd6 - rustc_driver[b85c4f7d2727bf1d]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_driver/src/lib.rs:1157:13
   7:     0x7f8640a30eb0 - std::panicking::rust_panic_with_hook::hdfb12e49d681025e
   8:     0x7f8640a53289 - std::panicking::begin_panic_handler::{{closure}}::hbba127a075656c02
   9:     0x7f8640a52e04 - std::sys_common::backtrace::__rust_end_short_backtrace::hb1f1ef31aa5f0b5b
  10:     0x7f8640a309e2 - rust_begin_unwind
  11:     0x7f8638401dc3 - core::panicking::panic_fmt::h3eca5924847e4995
  12:     0x7f8638401c8d - core::panicking::panic::h47ffa04e8aa7901b
  13:     0x7f863840afd7 - macros::tester::h2f4fd7f773278a55
                               at /home/william/Projects/MCVE/proc-macro-ice/macros/src/lib.rs:5:5
  14:     0x7f8638403ecc - core::ops::function::FnOnce::call_once::h6c77c84ae82f0009
                               at /home/william/Projects/toolchains/rust/library/core/src/ops/function.rs:248:5
  15:     0x7f8638408cd3 - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream,proc_macro::TokenStream) .> proc_macro::TokenStream>::expand2::run::{{closure}}::h906a635d2f1ff21e
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:439:17
  16:     0x7f86384089f8 - proc_macro::bridge::client::run_client::{{closure}}::{{closure}}::h979725278eb39ec6
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:392:26
  17:     0x7f8638404d47 - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}}::h702fe46e770b0509
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/scoped_cell.rs:79:33
  18:     0x7f8638404fd9 - proc_macro::bridge::scoped_cell::ScopedCell<T>::replace::h3d3933e39bbb87fd
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/scoped_cell.rs:74:9
  19:     0x7f8638404d24 - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::h741f8eb626b3a331
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/scoped_cell.rs:79:9
  20:     0x7f863840936c - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::h1770263e17654209
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:340:35
  21:     0x7f863840a284 - std::thread::local::LocalKey<T>::try_with::ha39fac2277c1aa01
                               at /home/william/Projects/toolchains/rust/library/std/src/thread/local.rs:445:16
  22:     0x7f8638409eda - std::thread::local::LocalKey<T>::with::h43b0ca55e2953a5e
                               at /home/william/Projects/toolchains/rust/library/std/src/thread/local.rs:421:9
  23:     0x7f86384074f4 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::hf857ae18d63bb7df
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:340:9
  24:     0x7f86384087e4 - proc_macro::bridge::client::run_client::{{closure}}::h121962a067179fcd
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:385:9
  25:     0x7f8638404c60 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hfc725cf76302d4d9
                               at /home/william/Projects/toolchains/rust/library/core/src/panic/unwind_safe.rs:271:9
  26:     0x7f863840b36a - std::panicking::try::do_call::ha6f00fa07b9b66e0
                               at /home/william/Projects/toolchains/rust/library/std/src/panicking.rs:492:40
  27:     0x7f863840b43b - __rust_try
  28:     0x7f863840b2a1 - std::panicking::try::he105b047cc1ef314
                               at /home/william/Projects/toolchains/rust/library/std/src/panicking.rs:456:19
  29:     0x7f86384045b0 - std::panic::catch_unwind::h81b911a3b167055c
                               at /home/william/Projects/toolchains/rust/library/std/src/panic.rs:137:14
  30:     0x7f86384086f5 - proc_macro::bridge::client::run_client::ha9ef62ee19afa5b2
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:384:5
  31:     0x7f8638408c5c - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream,proc_macro::TokenStream) .> proc_macro::TokenStream>::expand2::run::hefe2cdb549436143
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/client.rs:438:13
  32:     0x7f8642ff275e - <proc_macro[9fcfd34ce4baea21]::bridge::server::SameThread as proc_macro[9fcfd34ce4baea21]::bridge::server::ExecutionStrategy>::run_bridge_and_client::<fn(proc_macro[9fcfd34ce4baea21]::TokenStream, proc_macro[9fcfd34ce4baea21]::TokenStream) -> proc_macro[9fcfd34ce4baea21]::TokenStream, proc_macro[9fcfd34ce4baea21]::bridge::server::Dispatcher<proc_macro[9fcfd34ce4baea21]::bridge::server::MarkedTypes<rustc_expand[e257aa43bb6ff17f]::proc_macro_server::Rustc>>>
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/server.rs:155:9
  33:     0x7f8642ff275e - proc_macro[9fcfd34ce4baea21]::bridge::server::run_server::<rustc_expand[e257aa43bb6ff17f]::proc_macro_server::Rustc, (proc_macro[9fcfd34ce4baea21]::bridge::Marked<rustc_ast[139fc0288610aa91]::tokenstream::TokenStream, proc_macro[9fcfd34ce4baea21]::bridge::client::TokenStream>, proc_macro[9fcfd34ce4baea21]::bridge::Marked<rustc_ast[139fc0288610aa91]::tokenstream::TokenStream, proc_macro[9fcfd34ce4baea21]::bridge::client::TokenStream>), proc_macro[9fcfd34ce4baea21]::bridge::Marked<rustc_ast[139fc0288610aa91]::tokenstream::TokenStream, proc_macro[9fcfd34ce4baea21]::bridge::client::TokenStream>, fn(proc_macro[9fcfd34ce4baea21]::TokenStream, proc_macro[9fcfd34ce4baea21]::TokenStream) -> proc_macro[9fcfd34ce4baea21]::TokenStream, proc_macro[9fcfd34ce4baea21]::bridge::server::SameThread>
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/server.rs:298:9
  34:     0x7f8642ff275e - <proc_macro[9fcfd34ce4baea21]::bridge::client::Client<fn(proc_macro[9fcfd34ce4baea21]::TokenStream, proc_macro[9fcfd34ce4baea21]::TokenStream) -> proc_macro[9fcfd34ce4baea21]::TokenStream>>::run::<rustc_expand[e257aa43bb6ff17f]::proc_macro_server::Rustc, proc_macro[9fcfd34ce4baea21]::bridge::server::SameThread>
                               at /home/william/Projects/toolchains/rust/library/proc_macro/src/bridge/server.rs:341:9
  35:     0x7f8642feec80 - <rustc_expand[e257aa43bb6ff17f]::proc_macro::AttrProcMacro as rustc_expand[e257aa43bb6ff17f]::base::AttrProcMacro>::expand
                               at /home/william/Projects/toolchains/rust/compiler/rustc_expand/src/proc_macro.rs:63:9
  36:     0x7f864305fddf - <rustc_expand[e257aa43bb6ff17f]::expand::MacroExpander>::expand_invoc
                               at /home/william/Projects/toolchains/rust/compiler/rustc_expand/src/expand.rs:691:42
  37:     0x7f864305fddf - <rustc_expand[e257aa43bb6ff17f]::expand::MacroExpander>::fully_expand_fragment
                               at /home/william/Projects/toolchains/rust/compiler/rustc_expand/src/expand.rs:452:62
  38:     0x7f864305db97 - <rustc_expand[e257aa43bb6ff17f]::expand::MacroExpander>::expand_crate
                               at /home/william/Projects/toolchains/rust/compiler/rustc_expand/src/expand.rs:379:21
  39:     0x7f86412ec6f4 - rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}::{closure#1}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/passes.rs:343:50
  40:     0x7f86412ec6f4 - <rustc_data_structures[8c14bfce71735677]::profiling::VerboseTimingGuard>::run::<rustc_ast[139fc0288610aa91]::ast::Crate, rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_data_structures/src/profiling.rs:732:9
  41:     0x7f86412ec6f4 - <rustc_session[1cf76bb48e9dfb34]::session::Session>::time::<rustc_ast[139fc0288610aa91]::ast::Crate, rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_session/src/utils.rs:16:9
  42:     0x7f86412ec6f4 - rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/passes.rs:343:21
  43:     0x7f86412ec6f4 - <rustc_data_structures[8c14bfce71735677]::profiling::VerboseTimingGuard>::run::<core[cc9983ef6bac7021]::result::Result<rustc_ast[139fc0288610aa91]::ast::Crate, rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_data_structures/src/profiling.rs:732:9
  44:     0x7f86412ec6f4 - <rustc_session[1cf76bb48e9dfb34]::session::Session>::time::<core[cc9983ef6bac7021]::result::Result<rustc_ast[139fc0288610aa91]::ast::Crate, rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_interface[e5838306fb163009]::passes::configure_and_expand::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_session/src/utils.rs:16:9
  45:     0x7f86412e2e30 - rustc_interface[e5838306fb163009]::passes::configure_and_expand
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/passes.rs:294:13
  46:     0x7f86413845a4 - <rustc_interface[e5838306fb163009]::queries::Queries>::expansion::{closure#0}::{closure#0}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/queries.rs:181:17
  47:     0x7f86413845a4 - <rustc_interface[e5838306fb163009]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[e5838306fb163009]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[cc9983ef6bac7021]::result::Result<rustc_ast[139fc0288610aa91]::ast::Crate, rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/passes.rs:141:13
  48:     0x7f86413845a4 - <rustc_interface[e5838306fb163009]::queries::Queries>::expansion::{closure#0}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/queries.rs:180:25
  49:     0x7f86413845a4 - <rustc_interface[e5838306fb163009]::queries::Query<(alloc[2ceaa1994d41407e]::rc::Rc<rustc_ast[139fc0288610aa91]::ast::Crate>, alloc[2ceaa1994d41407e]::rc::Rc<core[cc9983ef6bac7021]::cell::RefCell<rustc_interface[e5838306fb163009]::passes::boxed_resolver::BoxedResolver>>, alloc[2ceaa1994d41407e]::rc::Rc<rustc_lint[19500922967b03ed]::context::LintStore>)>>::compute::<<rustc_interface[e5838306fb163009]::queries::Queries>::expansion::{closure#0}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/queries.rs:37:28
  50:     0x7f86413845a4 - <rustc_interface[e5838306fb163009]::queries::Queries>::expansion
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/queries.rs:169:9
  51:     0x7f86411645c8 - rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}::{closure#2}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_driver/src/lib.rs:363:13
  52:     0x7f86411645c8 - <rustc_interface[e5838306fb163009]::interface::Compiler>::enter::<rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}::{closure#2}, core[cc9983ef6bac7021]::result::Result<core[cc9983ef6bac7021]::option::Option<rustc_interface[e5838306fb163009]::queries::Linker>, rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/queries.rs:381:19
  53:     0x7f86411d8e63 - rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_driver/src/lib.rs:311:22
  54:     0x7f86411d8e63 - rustc_interface[e5838306fb163009]::interface::create_compiler_and_run::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#1}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/interface.rs:323:13
  55:     0x7f86411d8e63 - rustc_span[b415182799051819]::with_source_map::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_interface[e5838306fb163009]::interface::create_compiler_and_run<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_span/src/lib.rs:992:5
  56:     0x7f864117f7c1 - rustc_interface[e5838306fb163009]::interface::create_compiler_and_run::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/interface.rs:317:5
  57:     0x7f8641184102 - rustc_interface[e5838306fb163009]::interface::run_compiler::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/interface.rs:337:12
  58:     0x7f8641184102 - <scoped_tls[122d63f92661ccab]::ScopedKey<rustc_span[b415182799051819]::SessionGlobals>>::set::<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  59:     0x7f864118d2ff - rustc_span[b415182799051819]::create_session_globals_then::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/william/Projects/toolchains/rust/compiler/rustc_span/src/lib.rs:115:5
  60:     0x7f864118d2ff - rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals::<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}
                               at /home/william/Projects/toolchains/rust/compiler/rustc_interface/src/util.rs:157:32
  61:     0x7f864118d2ff - std[5ffc2b1b9faf8eaf]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/Projects/toolchains/rust/library/std/src/sys_common/backtrace.rs:122:18
  62:     0x7f864118e759 - <std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_::<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/william/Projects/toolchains/rust/library/std/src/thread/mod.rs:501:17
  63:     0x7f864118e759 - <core[cc9983ef6bac7021]::panic::unwind_safe::AssertUnwindSafe<<std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[cc9983ef6bac7021]::ops::function::FnOnce<()>>::call_once
                               at /home/william/Projects/toolchains/rust/library/core/src/panic/unwind_safe.rs:271:9
  64:     0x7f864118e759 - std[5ffc2b1b9faf8eaf]::panicking::try::do_call::<core[cc9983ef6bac7021]::panic::unwind_safe::AssertUnwindSafe<<std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/Projects/toolchains/rust/library/std/src/panicking.rs:492:40
  65:     0x7f864118e759 - std[5ffc2b1b9faf8eaf]::panicking::try::<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, core[cc9983ef6bac7021]::panic::unwind_safe::AssertUnwindSafe<<std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/william/Projects/toolchains/rust/library/std/src/panicking.rs:456:19
  66:     0x7f864118e759 - std[5ffc2b1b9faf8eaf]::panic::catch_unwind::<core[cc9983ef6bac7021]::panic::unwind_safe::AssertUnwindSafe<<std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>
                               at /home/william/Projects/toolchains/rust/library/std/src/panic.rs:137:14
  67:     0x7f864118e759 - <std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_::<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1}
                               at /home/william/Projects/toolchains/rust/library/std/src/thread/mod.rs:500:30
  68:     0x7f864118e759 - <<std[5ffc2b1b9faf8eaf]::thread::Builder>::spawn_unchecked_<rustc_interface[e5838306fb163009]::util::run_in_thread_pool_with_globals<rustc_interface[e5838306fb163009]::interface::run_compiler<core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>, rustc_driver[b85c4f7d2727bf1d]::run_compiler::{closure#1}>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#0}, core[cc9983ef6bac7021]::result::Result<(), rustc_errors[c4e752902790a04d]::ErrorGuaranteed>>::{closure#1} as core[cc9983ef6bac7021]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/william/Projects/toolchains/rust/library/core/src/ops/function.rs:248:5
  69:     0x7f8640a40133 - std::sys::unix::thread::Thread::new::thread_start::he85bb848cde7d24e
  70:     0x7f863cdb1609 - start_thread
                               at /build/glibc-SzIz7B/glibc-2.31/nptl/pthread_create.c:477:8
  71:     0x7f86408b0133 - clone
                               at /build/glibc-SzIz7B/glibc-2.31/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  72:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: custom attribute panicked
 --> src/main.rs:3:1
  |
3 | #[tester]
  | ^^^^^^^^^
  |
  = help: message: explicit panic

error: could not compile `proc-macro-ice` due to previous error
