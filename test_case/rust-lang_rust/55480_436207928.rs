
Initializing (look for `progress[done:true]` message)...
> 1: InitializeResult {
    capabilities: ServerCapabilities {
        text_document_sync: Some(
            Kind(
                Incremental
            )
        ),
        hover_provider: Some(
            true
        ),
        completion_provider: Some(
            CompletionOptions {
                resolve_provider: Some(
                    true
                ),
                trigger_characters: Some(
                    [
                        ".",
                        ":"
                    ]
                )
            }
        ),
        signature_help_provider: None,
        definition_provider: Some(
            true
        ),
        type_definition_provider: None,
        implementation_provider: Some(
            Simple(
                true
            )
        ),
        references_provider: Some(
            true
        ),
        document_highlight_provider: Some(
            true
        ),
        document_symbol_provider: Some(
            true
        ),
        workspace_symbol_provider: Some(
            true
        ),
        code_action_provider: Some(
            true
        ),
        code_lens_provider: Some(
            CodeLensOptions {
                resolve_provider: Some(
                    false
                )
            }
        ),
        document_formatting_provider: Some(
            true
        ),
        document_range_formatting_provider: Some(
            false
        ),
        document_on_type_formatting_provider: None,
        rename_provider: Some(
            true
        ),
        color_provider: None,
        execute_command_provider: Some(
            ExecuteCommandOptions {
                commands: [
                    "rls.applySuggestion-8803",
                    "rls.deglobImports-8803"
                ]
            }
        )
    }
}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"palette","title":"Building"}}
{"jsonrpc":"2.0","method":"window/progress","params":{"id":"progress_1","message":"gtk","title":"Building"}}
thread 'main' panicked at 'No span found for use glob', libcore/option.rs:1008:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::option::expect_failed
             at libcore/option.rs:1008
  10: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_use_tree
  11: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  12: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  13: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  14: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
  15: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  16: rustc::ty::context::tls::with_context
  17: rustc_save_analysis::process_crate
  18: rustc_driver::enable_save_analysis::{{closure}}::{{closure}}
  19: rustc::util::common::time
  20: rustc_driver::enable_save_analysis::{{closure}}
  21: rustc::ty::context::tls::with_context
  22: rustc_driver::driver::compile_input::{{closure}}
  23: rustc::ty::context::tls::enter_context
  24: <std::thread::local::LocalKey<T>>::with
  25: rustc::ty::context::TyCtxt::create_and_enter
  26: rustc_driver::driver::compile_input
  27: rustc_driver::run_compiler_with_pool
  28: rustc_driver::driver::spawn_thread_pool
  29: rustc_driver::run_compiler
  30: <scoped_tls::ScopedKey<T>>::set
  31: syntax::with_globals
  32: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  33: rustc_driver::run
  34: rls_rustc::run
  35: rls::main_inner
  36: rls::main
  37: std::rt::lang_start::{{closure}}
  38: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  39: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  40: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:398
             at libstd/rt.rs:58
  41: main
  42: __libc_start_main
  43: <unknown>
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (13dab66a6 2018-11-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
