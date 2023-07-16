
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:12 ~ playground[dc12]::foo), const_param_did: None }) (end of phase transition to Optimized) at bb1[1]:
                                Field projection `_0.field[0]` specified type `T`, but actual type is <T as Trait>::Associated
  --> src/lib.rs:19:5
   |
14 |     bar()
   |     ----- in this inlined function call
...
19 |     Struct(baz())
   |     ^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:129:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
   3: core::ptr::drop_in_place::<rustc_session::parse::ParseSess>
   4: <alloc::rc::Rc<rustc_session::session::Session> as core::ops::drop::Drop>::drop
   5: core::ptr::drop_in_place::<rustc_interface::interface::Compiler>
   6: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
   7: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
   8: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-beta.2 (fb2194acc 2022-08-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `playground`
