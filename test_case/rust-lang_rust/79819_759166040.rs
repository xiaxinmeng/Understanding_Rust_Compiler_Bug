plain
    Checking semver v0.10.0
    Checking toml v0.5.8
    Checking crates-io v0.31.1 (/tmp/cargo/crates/crates-io)
    Checking rustfix v0.5.1
warning: Error finalizing incremental compilation session directory `/tmp/cargo/target/debug/incremental/cargo-19wuvm4n7ahyw/s-fuvk2g7uon-s9wyg3-working`: No such file or directory (os error 2)
warning: 1 warning emitted


error: internal compiler error: failed to process buffered lint here
   --> src/cargo/ops/cargo_new.rs:772:27
    |
772 |                 Err(e) => log::warn!("failed to call rustfmt: {}", e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/ops/fix.rs:435:22
    |
435 |         .inspect(|y| trace!("line: {}", y))
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/sources/git/utils.rs:275:31
    |
275 |                     Err(e) => debug!("failed reset after fetch {:?}", e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/sources/git/utils.rs:879:19
    |
879 |         Err(e) => debug!("failed to check github {:?}", e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
    --> src/cargo/sources/git/utils.rs:1063:19
     |
1063 |         Err(e) => debug!("git-gc failed to spawn: {}", e),
     |
     |
     = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
     = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/sources/registry/index.rs:511:27
    |
511 |                 Err(e) => log::debug!("cache missing for {:?} error: {}", relative, e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/util/diagnostic_server.rs:272:31
    |
272 |                     Err(e) => warn!("invalid diagnostics message: {}", e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
    |
    |
511 |         Ok(()) => log::debug!("set file mtime {} to {}", path.display(), time),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
    |
    |
512 |           Err(e) => log::warn!(
    |  ___________________^
513 | |             "could not set mtime of {} to {}: {:?}",
514 | |             path.display(),
516 | |             e
517 | |         ),
    | |_________^
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/util/rustc.rs:215:27
    |
215 |                 Ok(()) => info!("updated rustc info cache"),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: failed to process buffered lint here
   --> src/cargo/util/rustc.rs:216:27
    |
216 |                 Err(e) => warn!("failed to update rustc info cache: {}", e),
    |
    |
    = note: delayed at /rustc/50473808b4098f3d96d962b3940d38962305e408/compiler/rustc_lint/src/early.rs:391:22
    = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (50473808b 2021-01-13) running on x86_64-unknown-linux-gnu


note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
