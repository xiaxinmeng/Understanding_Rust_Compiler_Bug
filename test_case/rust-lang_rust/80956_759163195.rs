
thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c5eae562935922f712edec56a45591bc2f8ded1c/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/c5eae562935922f712edec56a45591bc2f8ded1c/library/std/src/panicking.rs:435:5
   2: rustc_errors::HandlerInner::flush_delayed
   3: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
   4: core::ptr::drop_in_place<rustc_session::parse::ParseSess>
   5: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
   6: core::ptr::drop_in_place<rustc_interface::interface::Compiler>
   7: rustc_span::with_source_map
   8: rustc_interface::interface::create_compiler_and_run
   9: rustc_span::with_session_globals
