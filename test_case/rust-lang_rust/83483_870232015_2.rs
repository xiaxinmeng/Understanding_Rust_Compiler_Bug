
error: internal compiler error: compiler/rustc_typeck/src/collect/type_of.rs:407:21: compute_type_of_item: unexpected item type: Trait(No, Normal, Generics { params: [], where_clause: WhereClause { predicates: [], span: mutant.rs:3:21: 3:21 (#0) }, span: mutant.rs:3:21: 3:21 (#0) }, [], [])
 --> mutant.rs:3:1
  |
3 | trait PrinterSupport {}
  | ^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/a435b49e86d16e98dcc6595dd471f95e823f41aa/compiler/rustc_errors/src/lib.rs:953:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (a435b49e8 2021-06-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type staticlib

query stack during panic:
#0 [type_of] computing type of `PrinterSupport`
#1 [symbol_name] computing the symbol for `PrinterSupport`
end of query stack
error: aborting due to previous error
