
$ cargo +nightly doc -Z unstable-options -Z rustdoc-scrape-examples=examples
   Compiling pyo3 v0.14.5 (C:\Users\user\rust\pyo3)
thread 'rustc' panicked at 'assertion failed: enclosing_item_span.contains(expr_span)', src\librustdoc\scrape_examples.rs:93:9
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

error: Unrecognized option: 'scrape-examples-output-path'

error: could not document `pyo3
