
failures:

---- [ui] ui/out_of_bounds_indexing/empty_array.rs stdout ----
normalized stderr:
error: index out of bounds: the len is 0 but the index is 0
  --> $DIR/empty_array.rs:6:5
   |
LL |     empty[0];
   |     ^^^^^^^^
   |
   = note: `#[deny(const_err)]` on by default

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size { raw: 0 }`,
 right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing -C prefer-dynamic

error: aborting due to previous error
