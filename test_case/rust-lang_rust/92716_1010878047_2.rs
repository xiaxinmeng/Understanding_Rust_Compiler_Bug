
$ cargo xtask install
    Finished dev [unoptimized] target(s) in 0.02s
     Running `target/debug/xtask install`
$ cargo install --path crates/rust-analyzer --locked --force --features force-always-assert
  Installing rust-analyzer v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/rust-analyzer)
    Updating crates.io index
   Compiling ide_assists v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/ide_assists)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [unsafety_check_result] unsafety-checking `handlers::generate_documentation_template::generate_documentation_template::{closure#0}`
#1 [unsafety_check_result] unsafety-checking `handlers::generate_documentation_template::generate_documentation_template`
end of query stack
error: failed to compile `rust-analyzer v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/rust-analyzer)`, intermediate artifacts can be found at `/home/my_username_redacted/temp/rust-analyzer/target`

Caused by:
  could not compile `ide_assists`
Error: install server

Caused by:
    command `cargo install --path crates/rust-analyzer --locked --force --features force-always-assert` failed, exit status: 101
