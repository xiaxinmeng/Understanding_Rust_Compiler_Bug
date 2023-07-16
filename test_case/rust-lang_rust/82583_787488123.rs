
thread 'rustc' panicked at 'Unknown start of token: BytePos(9623887) BytePos(9623890) '♥'', compiler/rustc_parse/src/lexer/mod.rs:265:17
stack backtrace:
   0: rust_begin_unwind
             at /home/aaron/repos/rust/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /home/aaron/repos/rust/library/std/src/panicking.rs:435:5
   2: <rustc_parse::lexer::StringReader>::cook_lexer_token
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/mod.rs:265:17
   3: <rustc_parse::lexer::StringReader>::next_token
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/mod.rs:88:19
   4: <rustc_parse::lexer::tokentrees::TokenTreesReader>::bump
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/tokentrees.rs:272:32
   5: <rustc_parse::lexer::tokentrees::TokenTreesReader>::parse_all_token_trees
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/tokentrees.rs:54:9
   6: <rustc_parse::lexer::StringReader>::into_token_trees
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/tokentrees.rs:26:19
   7: rustc_parse::lexer::parse_token_trees
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lexer/mod.rs:34:5
   8: rustc_parse::maybe_file_to_stream
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lib.rs:195:9
   9: rustc_parse::source_file_to_stream
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lib.rs:179:45
  10: rustc_parse::parse_stream_from_source_str
             at /home/aaron/repos/rust/compiler/rustc_parse/src/lib.rs:93:9
  11: <rustc_expand::proc_macro_server::Rustc as proc_macro::bridge::server::TokenStream>::from_str
             at /home/aaron/repos/rust/compiler/rustc_expand/src/proc_macro_server.rs:408:9
  12: <proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc> as proc_macro::bridge::server::TokenStream>::from_str
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:72:34
  13: std::panicking::try::do_call::<std::panic::AssertUnwindSafe<<proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>> as proc_macro::bridge::server::DispatcherTrait>::dispatch::{closure#6}>, proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>>
             at /home/aaron/repos/rust/library/std/src/panicking.rs:379:40
  14: std::panicking::try::<proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>, std::panic::AssertUnwindSafe<<proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>> as proc_macro::bridge::server::DispatcherTrait>::dispatch::{closure#6}>>
             at /home/aaron/repos/rust/library/std/src/panicking.rs:343:19
  15: std::panic::catch_unwind::<std::panic::AssertUnwindSafe<<proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>> as proc_macro::bridge::server::DispatcherTrait>::dispatch::{closure#6}>, proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>>
             at /home/aaron/repos/rust/library/std/src/panic.rs:431:14
  16: <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>> as proc_macro::bridge::server::DispatcherTrait>::dispatch
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:115:33
  17: <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client::<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream, proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>>>::{closure#0}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:153:32
  18: <proc_macro::bridge::closure::Closure<_, _> as core::convert::From<&mut _>>::from::call::<proc_macro::bridge::buffer::Buffer<u8>, proc_macro::bridge::buffer::Buffer<u8>, <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream, proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>>>::{closure#0}>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/closure.rs:19:13
  19: <proc_macro::bridge::closure::Closure<proc_macro::bridge::buffer::Buffer<u8>, proc_macro::bridge::buffer::Buffer<u8>>>::call
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/closure.rs:27:18
  20: <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:244:25
  21: <proc_macro::bridge::Bridge>::with::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:336:47
  22: <proc_macro::bridge::client::BridgeState>::with::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:293:17
  23: <proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>::replace::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::BridgeState>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/scoped_cell.rs:75:9
  24: <proc_macro::bridge::client::BridgeState>::with::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:291:13
  25: <std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>>::try_with::<<proc_macro::bridge::client::BridgeState>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>::{closure#0}, proc_macro::bridge::client::TokenStream>
             at /home/aaron/repos/rust/library/std/src/thread/local.rs:272:16
  26: <std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>>::with::<<proc_macro::bridge::client::BridgeState>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>::{closure#0}, proc_macro::bridge::client::TokenStream>
             at /home/aaron/repos/rust/library/std/src/thread/local.rs:248:9
  27: <proc_macro::bridge::client::BridgeState>::with::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::Bridge>::with<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>::{closure#0}>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:290:9
  28: <proc_macro::bridge::Bridge>::with::<proc_macro::bridge::client::TokenStream, <proc_macro::bridge::client::TokenStream>::from_str::{closure#0}>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:329:9
  29: <proc_macro::bridge::client::TokenStream>::from_str
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:237:17
  30: <proc_macro::TokenStream as core::str::traits::FromStr>::from_str
             at /home/aaron/repos/rust/library/proc_macro/src/lib.rs:134:24
  31: core::str::<impl str>::parse
             at /home/aaron/repos/rust/library/core/src/str/mod.rs:2210:9
  32: proc_macro2::imp::proc_macro_parse::{{closure}}
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/src/wrapper.rs:110:28
  33: std::panicking::try::do_call
             at /home/aaron/repos/rust/library/std/src/panicking.rs:379:40
  34: __rust_try
  35: std::panicking::try
             at /home/aaron/repos/rust/library/std/src/panicking.rs:343:19
  36: std::panic::catch_unwind
             at /home/aaron/repos/rust/library/std/src/panic.rs:431:14
  37: proc_macro2::imp::proc_macro_parse
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/src/wrapper.rs:110:5
  38: <proc_macro2::imp::TokenStream as core::str::traits::FromStr>::from_str
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/src/wrapper.rs:100:17
  39: core::str::<impl str>::parse
             at /home/aaron/repos/rust/library/core/src/str/mod.rs:2210:9
  40: <proc_macro2::TokenStream as core::str::traits::FromStr>::from_str
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.24/src/lib.rs:182:17
  41: syn::parse::Parser::parse_str
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.60/src/parse.rs:1174:21
  42: syn::parse_str
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.60/src/lib.rs:926:5
  43: rocket_codegen::syn_ext::NameSource::new
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/syn_ext.rs:73:9
  44: rocket_codegen::attribute::segments::Segment::from
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/attribute/segments.rs:40:52
  45: rocket_codegen::attribute::segments::parse_segments
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/attribute/segments.rs:159:31
  46: <rocket_codegen::http_codegen::RoutePath as devise_core::from_meta::FromMeta>::from_meta
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/http_codegen.rs:198:20
  47: rocket_codegen::attribute::route::_::<impl devise_core::from_meta::FromMeta for rocket_codegen::attribute::route::MethodRouteAttribute>::from_meta
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/attribute/route.rs:31:5
  48: rocket_codegen::attribute::route::incomplete_route
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/attribute/route.rs:472:28
  49: rocket_codegen::attribute::route::route_attribute
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/attribute/route.rs:493:25
  50: rocket_codegen::get
             at /home/aaron/.cargo/git/checkouts/rocket-8bf16d9ca7e90bdc/66d2c17/core/codegen/src/lib.rs:277:19
  51: core::ops::function::FnOnce::call_once
             at /home/aaron/repos/rust/library/core/src/ops/function.rs:227:5
  52: proc_macro::bridge::client::Client<fn(proc_macro::TokenStream,proc_macro::TokenStream) .> proc_macro::TokenStream>::expand2::run::{{closure}}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:426:17
  53: proc_macro::bridge::client::run_client::{{closure}}::{{closure}}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:377:26
  54: proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/scoped_cell.rs:80:33
  55: proc_macro::bridge::scoped_cell::ScopedCell<T>::replace
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/scoped_cell.rs:75:9
  56: proc_macro::bridge::scoped_cell::ScopedCell<T>::set
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/scoped_cell.rs:80:9
  57: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:325:35
  58: std::thread::local::LocalKey<T>::try_with
             at /home/aaron/repos/rust/library/std/src/thread/local.rs:272:16
  59: std::thread::local::LocalKey<T>::with
             at /home/aaron/repos/rust/library/std/src/thread/local.rs:248:9
  60: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:325:9
  61: proc_macro::bridge::client::run_client::{{closure}}
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:370:9
  62: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/aaron/repos/rust/library/std/src/panic.rs:344:9
  63: std::panicking::try::do_call
             at /home/aaron/repos/rust/library/std/src/panicking.rs:379:40
  64: __rust_try
  65: std::panicking::try
             at /home/aaron/repos/rust/library/std/src/panicking.rs:343:19
  66: std::panic::catch_unwind
             at /home/aaron/repos/rust/library/std/src/panic.rs:431:14
  67: proc_macro::bridge::client::run_client
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:369:5
  68: proc_macro::bridge::client::Client<fn(proc_macro::TokenStream,proc_macro::TokenStream) .> proc_macro::TokenStream>::expand2::run
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/client.rs:425:13
  69: <proc_macro::bridge::server::SameThread as proc_macro::bridge::server::ExecutionStrategy>::run_bridge_and_client::<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream, proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<rustc_expand::proc_macro_server::Rustc>>>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:155:9
  70: proc_macro::bridge::server::run_server::<rustc_expand::proc_macro_server::Rustc, (proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>, proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>), proc_macro::bridge::Marked<rustc_ast::tokenstream::TokenStream, proc_macro::bridge::client::TokenStream>, fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream, proc_macro::bridge::server::SameThread>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:291:9
  71: <proc_macro::bridge::client::Client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream>>::run::<rustc_expand::proc_macro_server::Rustc, proc_macro::bridge::server::SameThread>
             at /home/aaron/repos/rust/library/proc_macro/src/bridge/server.rs:334:9
  72: <rustc_expand::proc_macro::AttrProcMacro as rustc_expand::base::AttrProcMacro>::expand
             at /home/aaron/repos/rust/compiler/rustc_expand/src/proc_macro.rs:52:9
  73: <rustc_expand::expand::MacroExpander>::expand_invoc
             at /home/aaron/repos/rust/compiler/rustc_expand/src/expand.rs:728:44
  74: <rustc_expand::expand::MacroExpander>::fully_expand_fragment
             at /home/aaron/repos/rust/compiler/rustc_expand/src/expand.rs:485:62
  75: <rustc_expand::expand::MacroExpander>::expand_crate
             at /home/aaron/repos/rust/compiler/rustc_expand/src/expand.rs:386:15
  76: rustc_interface::passes::configure_and_expand_inner::{closure#1}::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:311:50
  77: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<rustc_ast::ast::Crate, rustc_interface::passes::configure_and_expand_inner::{closure#1}::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:573:9
  78: <rustc_session::session::Session>::time::<rustc_ast::ast::Crate, rustc_interface::passes::configure_and_expand_inner::{closure#1}::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:10:9
  79: rustc_interface::passes::configure_and_expand_inner::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:311:21
  80: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, rustc_interface::passes::configure_and_expand_inner::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:573:9
  81: <rustc_session::session::Session>::time::<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, rustc_interface::passes::configure_and_expand_inner::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:10:9
  82: rustc_interface::passes::configure_and_expand_inner
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:261:13
  83: rustc_interface::passes::configure_and_expand::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:120:19
  84: <core::pin::Pin<alloc::boxed::Box<dyn core::ops::generator::Generator<rustc_data_structures::box_region::Action, Yield = rustc_data_structures::box_region::YieldType<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, for<'a, 'b> fn((&'a mut rustc_resolve::Resolver<'b>,))>, Return = rustc_middle::ty::ResolverOutputs>>> as core::ops::generator::Generator<rustc_data_structures::box_region::Action>>::resume
             at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1668:9
  85: <rustc_data_structures::box_region::PinnedGenerator<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorReported>, for<'a, 'b> fn((&'a mut rustc_resolve::Resolver<'b>,)), rustc_middle::ty::ResolverOutputs>>::new::<rustc_interface::passes::configure_and_expand::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/box_region.rs:44:26
  86: <rustc_interface::passes::BoxedResolver>::new::<rustc_interface::passes::configure_and_expand::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/box_region.rs:101:41
  87: rustc_interface::passes::configure_and_expand
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:116:30
  88: <rustc_interface::queries::Queries>::expansion::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:183:13
  89: <rustc_interface::queries::Query<(rustc_ast::ast::Crate, rustc_data_structures::steal::Steal<alloc::rc::Rc<core::cell::RefCell<rustc_interface::passes::BoxedResolver>>>, alloc::rc::Rc<rustc_lint::context::LintStore>)>>::compute::<<rustc_interface::queries::Queries>::expansion::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:40:28
  90: <rustc_interface::queries::Queries>::expansion
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:179:9
  91: rustc_driver::run_compiler::{closure#3}::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:384:13
  92: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#3}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:422:19
  93: rustc_driver::run_compiler::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:332:22
  94: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:197:13
  95: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:787:5
  96: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:191:5
  97: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:213:12
  98: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:155:13
  99: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 100: rustc_span::with_session_globals::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:104:5
 101: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:153:9
 102: rustc_interface::util::scoped_thread::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:128:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
    Finished dev [unoptimized + debuginfo] target(s) in 1m 18s
