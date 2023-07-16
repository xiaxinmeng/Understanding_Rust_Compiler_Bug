
error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<<Q as Query>::V as std::fmt::Debug>, polarity:Positive), []), depth=0),Unimplemented)]` resolving bounds after type-checking
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:125:24

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1188:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
