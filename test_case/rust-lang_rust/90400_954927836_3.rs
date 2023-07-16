
error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<u32 as Bar>, polarity:Positive), []), depth=1),Unimplemented)]` resolving bounds after type-checking
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:124:24

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1167:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/std/src/panicking.rs:495:5
   1: core::panicking::panic_fmt
             at /rustc/c390d69a615f095208ac94841f3310268521b2ee/library/core/src/panicking.rs:106:14
   2: core::panicking::panic_display::<&str>
   3: <rustc_errors::HandlerInner>::flush_delayed
   4: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
   5: core::ptr::drop_in_place::<rustc_session::parse::ParseSess>
   6: <alloc::rc::Rc<rustc_session::session::Session> as core::ops::drop::Drop>::drop
   7: core::ptr::drop_in_place::<rustc_interface::interface::Compiler>
   8: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
   9: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (c390d69a6 2021-10-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
