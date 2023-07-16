
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/ty/query/mod.rs:235:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (1700ca07c 2020-12-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-Tutil/x86_64.ld

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
 right: `Codegenning`', /rustc/1700ca07c6dd7becff85678409a5df6ad4cf4f47/compiler/rustc_codegen_ssa/src/back/write.rs:1425:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (1700ca07c 2020-12-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-Tutil/x86_64.ld

note: some of the compiler flags provided by cargo are hidden
