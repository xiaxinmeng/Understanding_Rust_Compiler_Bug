
+ rustc +nightly --edition=2018 --crate-type lib --out-dir ./target -L dependency=./target child.rs --crate-name rust_issue_51798_example_child -C metadata=arbitrary_child_id -C extra-filename=-arbitrary_child_id
+ rustc +nightly --edition=2018 --crate-type lib --out-dir ./target -L dependency=./target parent.rs --crate-name rust_issue_51798_example_parent -C metadata=arbitrary_parent_id -C extra-filename=-arbitrary_parent_id --extern rust_issue_51798_example_child=./target/librust_issue_51798_example_child-arbitrary_child_id.rlib
error: internal compiler error: no type-dependent def for method call
 --> parent.rs:8:9
  |
8 |         v.clear();
  |         ^^^^^^^^^

thread 'main' panicked at 'LocalTableInContext: key not found', libcore/option.rs:960:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (e3bf634e0 2018-06-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

note: some of the compiler flags provided by cargo are hidden
