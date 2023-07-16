
cargo check --color=always
warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
    Checking dracula v0.1.0 (/Users/xlange/code/rust/dracula)
thread 'main' panicked at 'assertion failed: current_depth > ty::INNERMOST', librustc/infer/higher_ranked/mod.rs:757:21
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `impl std::future::Future: std::marker::Send`
#1 [typeck_tables_of] processing `main`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (fddcd316a 2018-10-05) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `dracula`.

To learn more, run the command again with --verbose.

Process finished with exit code 101
