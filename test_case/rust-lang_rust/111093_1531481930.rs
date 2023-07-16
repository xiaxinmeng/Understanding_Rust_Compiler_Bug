plain
   = note: `#[warn(dead_code)]` implied by `#[warn(warnings)]`

error[E0275]: overflow evaluating the requirement `_: Sized`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`diesel`)
note: required for `Source` to implement `filter_dsl::FilterDsl<_>`
   |
   |
20 | impl<T, Predicate> FilterDsl<Predicate> for T
   |         ---------  ^^^^^^^^^^^^^^^^^^^^     ^
   |         unsatisfied trait bound introduced here

For more information about this error, try `rustc --explain E0275`.
warning: `diesel` (lib) generated 5 warnings
warning: `diesel` (lib) generated 5 warnings
error: could not compile `diesel` (lib) due to previous error; 5 warnings emitted
warning: build failed, waiting for other jobs to finish...
warning: `diesel` (lib test) generated 5 warnings (5 duplicates)
error: could not compile `diesel` (lib test) due to previous error; 5 warnings emitted
thread 'main' panicked at 'tests failed for https://github.com/diesel-rs/diesel', src/tools/cargotest/main.rs:124:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:21:35
make: *** [Makefile:44: check-aux] Error 1
