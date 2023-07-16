
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2812 ~ combine[2263]::stream::{impl#47}::reset), const_param_did: None }) (after phase change to runtime-optimized) at bb0[3]:
                                encountered `Assign((_0, const Result::<(), StringStreamError>::Ok(())))` with incompatible types:
                                left-hand side has type: Result<(), <&str as StreamOnce>::Error>
                                right-hand side has type: Result<(), StringStreamError>
   --> /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/combine-4.6.6/src/stream/mod.rs:44:17
    |
44  |                 Ok(())
    |                 ^^^^^^
...
153 | clone_resetable! {('a) &'a str}
    | ------------------------------- in this macro invocation
    |
    = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
               1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
               2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
               3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
               4: rustc_mir_transform::optimized_mir
               5: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
               6: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
               7: rustc_metadata::rmeta::encoder::encode_metadata_impl
               8: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
               9: rustc_metadata::rmeta::encoder::encode_metadata
              10: rustc_metadata::fs::encode_and_write_metadata
              11: rustc_interface::passes::start_codegen
              12: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
              13: <rustc_interface::queries::Queries>::ongoing_codegen
              14: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
              15: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
              16: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
              17: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
              18: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
              19: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                         at /rustc/bd39bbb4bb92df439bf6d85470e296cc6a47ffbd/library/alloc/src/boxed.rs:1988:9
              20: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                         at /rustc/bd39bbb4bb92df439bf6d85470e296cc6a47ffbd/library/alloc/src/boxed.rs:1988:9
              21: std::sys::unix::thread::Thread::new::thread_start
                         at /rustc/bd39bbb4bb92df439bf6d85470e296cc6a47ffbd/library/std/src/sys/unix/thread.rs:108:17
              22: <unknown>
              23: <unknown>
            
    = note: this error: internal compiler error originates in the macro `clone_resetable` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (bd39bbb4b 2023-02-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C instrument-coverage

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
