plain
    Checking toml v0.5.7
    Checking url v2.1.1
    Checking clippy_utils v0.1.52 (/checkout/src/tools/clippy/clippy_utils)
    Checking cargo_metadata v0.12.0
error[E0050]: method `fake_read` has 3 parameters but the declaration in trait `fake_read` has 4
  --> src/tools/clippy/clippy_utils/src/usage.rs:82:18
   |
82 |     fn fake_read(&mut self, _: rustc_typeck::expr_use_visitor::Place<'tcx>, _: FakeReadCause) { }
   |
   |
   = note: `fake_read` from trait: `fn(&mut Self, rustc_typeck::expr_use_visitor::Place<'tcx>, FakeReadCause, HirId)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0050`.
error: could not compile `clippy_utils`
