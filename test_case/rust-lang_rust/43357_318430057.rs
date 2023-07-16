
=>cargo bench
   Compiling ethcore v1.7.0 (file:///Users/timsiwula/Desktop/metropolis_testing/testing_hub/clients/parity/ethcore)
error: internal compiler error: src/librustc/infer/mod.rs:573: Encountered errors [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<K as verification::queue::kind::Kind>)),depth=0),Unimplemented)]  resolving bounds after type-checking
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.21.0-nightly (599be0d18 2017-07-26) running on x86_64-apple-darwin
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:438:8
note: Run with RUST_BACKTRACE=1 for a backtrace.
error: Could not compile ethcore.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: src/librustc/infer/mod.rs:573: Encountered errors [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<K as verification::queue::kind::Kind>)),depth=0),Unimplemented)] resolving bounds after type-checking
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.21.0-nightly (599be0d18 2017-07-26) running on x86_64-apple-darwin
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:438:8
note: Run with RUST_BACKTRACE=1 for a backtrace.
error: Could not compile `ethcore`.
To learn more, run the command again with --verbose.
=>
