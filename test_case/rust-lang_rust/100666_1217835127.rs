rust
   Compiling std v0.0.0 (/home/jess/src/rust/library/std)
thread 'rustc' panicked at 'oh no!!', /home/jess/src/rust/library/core/src/slice/index.rs:225:13
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at ./library/core/src/panicking.rs:142:14
   2: <alloc::vec::Vec<rustc_ast::ptr::P<rustc_ast::ast::Expr>> as rustc_data_structures::map_in_place::MapInPlace<rustc_ast::ptr::P<rustc_ast::ast::Expr>>>::flat_map_in_place::<rustc_ast::mut_visit::visit_exprs<rustc_expand::expand::InvocationCollector>::{closure#0}, core::option::Option<rustc_ast::ptr::P<rustc_ast::ast::Expr>>>
   3: <rustc_ast::ast::Crate as rustc_expand::expand::InvocationCollectorNode>::noop_visit::<rustc_expand::expand::InvocationCollector>
             at ./compiler/rustc_expand/src/expand.rs:1371:9
   4: <rustc_expand::expand::InvocationCollector>::visit_node::<rustc_ast::ast::Crate>::{closure#2}
             at ./compiler/rustc_expand/src/expand.rs:1737:61
   5: <rustc_expand::expand::InvocationCollector>::visit_node::<rustc_ast::ast::Crate>
             at ./compiler/rustc_expand/src/expand.rs:1737:21
   6: <rustc_expand::expand::MacroExpander>::collect_invocations
             at ./compiler/rustc_expand/src/expand.rs:557:13
   7: <rustc_expand::expand::MacroExpander>::fully_expand_fragment
             at ./compiler/rustc_expand/src/expand.rs:392:13
   8: <rustc_expand::expand::MacroExpander>::expand_crate
             at ./compiler/rustc_expand/src/expand.rs:379:21
   9: rustc_interface::passes::configure_and_expand::{closure#1}::{closure#1}
             at ./compiler/rustc_interface/src/passes.rs:336:50
  10: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<rustc_ast::ast::Crate, rustc_interface::passes::configure_and_expand::{closure#1}::{closure#1}>
             at ./compiler/rustc_data_structures/src/profiling.rs:739:9
  11: <rustc_session::session::Session>::time::<rustc_ast::ast::Crate, rustc_interface::passes::configure_and_expand::{closure#1}::{closure#1}>
             at ./compiler/rustc_session/src/utils.rs:10:9
  12: rustc_interface::passes::configure_and_expand::{closure#1}
             at ./compiler/rustc_interface/src/passes.rs:336:21
  13: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorGuaranteed>, rustc_interface::passes::configure_and_expand::{closure#1}>
             at ./compiler/rustc_data_structures/src/profiling.rs:739:9
  14: <rustc_session::session::Session>::time::<core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorGuaranteed>, rustc_interface::passes::configure_and_expand::{closure#1}>
             at ./compiler/rustc_session/src/utils.rs:10:9
  15: rustc_interface::passes::configure_and_expand
             at ./compiler/rustc_interface/src/passes.rs:288:13
  16: <rustc_interface::queries::Queries>::expansion::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/queries.rs:181:17
  17: <rustc_interface::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface::queries::Queries>::expansion::{closure#0}::{closure#0}, core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorGuaranteed>>
             at ./compiler/rustc_interface/src/passes.rs:132:13
  18: <rustc_interface::queries::Queries>::expansion::{closure#0}
             at ./compiler/rustc_interface/src/queries.rs:180:25
  19: <rustc_interface::queries::Query<(alloc::rc::Rc<rustc_ast::ast::Crate>, alloc::rc::Rc<core::cell::RefCell<rustc_interface::passes::boxed_resolver::BoxedResolver>>, alloc::rc::Rc<rustc_lint::context::LintStore>)>>::compute::<<rustc_interface::queries::Queries>::expansion::{closure#0}>
             at ./compiler/rustc_interface/src/queries.rs:37:28
  20: <rustc_interface::queries::Queries>::expansion
             at ./compiler/rustc_interface/src/queries.rs:169:9
  21: rustc_driver::run_compiler::{closure#1}::{closure#2}
             at ./compiler/rustc_driver/src/lib.rs:358:13
  22: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             at ./compiler/rustc_interface/src/queries.rs:381:19
  23: rustc_driver::run_compiler::{closure#1}
             at ./compiler/rustc_driver/src/lib.rs:309:22
  24: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
             at ./compiler/rustc_interface/src/interface.rs:323:13
  25: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             at ./compiler/rustc_span/src/lib.rs:986:5
  26: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
             at ./compiler/rustc_interface/src/interface.rs:317:5
  27: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:339:12
  28: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/jess/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  29: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:112:5
  30: rustc_interface::util::run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:159:32
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
