
error: internal compiler error: /checkout/src/librustc/traits/project.rs:1256: Failed to unify obligation `Obligation(predicate=ProjectionTy { trait_ref: <[closure@src/lib.rs:66:9: 66:23] as std::ops::FnOnce<(std::option::Option<_>, <AnyParser<'_> as Parse<'x>>::Output)>>, item_name: Output(87) },depth=1)` with poly_projection `Binder(ProjectionPredicate(ProjectionTy { trait_ref: <[closure@src/lib.rs:66:9: 66:23] as std::ops::FnOnce<(std::option::Option<_>, &str)>>, item_name: Output(87) }, std::option::Option<_>))`: Sorts(ExpectedFound { expected: &str, found: <AnyParser<'_> as Parse<'x>>::Output })
  --> src/lib.rs:70:17
   |
70 |         let _ = apply(&content, parser);
   |                 ^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace
