
$ cargo test
   Compiling omicron-nexus v0.1.0 (/home/dap/omicron-auth/nexus)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/65c55bf931a55e6b1e5ed14ad8623814a7386424/compiler/rustc_hir/src/definitions.rs:452:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65c55bf93 2021-11-23) running on x86_64-unknown-illumos

note: compiler flags: -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-Wl,-R/opt/ooce/pgsql-13/lib/amd64 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `for<'c> fn(steno::saga_exec::ActionContext<sagas::SagaInstanceCreate>) -> impl Future<Output = core::result::Result<(), anyhow::Error>> {sagas::sic_delete_instance_record}: steno::saga_action_func::ActionFn<'c, sagas::SagaInstanceCreate>`
#1 [codegen_fulfill_obligation] checking if `core::ops::unsize::CoerceUnsized` fulfills its obligations
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/65c55bf931a55e6b1e5ed14ad8623814a7386424/compiler/rustc_hir/src/definitions.rs:452:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65c55bf93 2021-11-23) running on x86_64-unknown-illumos

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-Wl,-R/opt/ooce/pgsql-13/lib/amd64 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `for<'c> fn(steno::saga_exec::ActionContext<sagas::SagaInstanceCreate>) -> impl Future<Output = core::result::Result<(), anyhow::Error>> {sagas::sic_delete_instance_record}: steno::saga_action_func::ActionFn<'c, sagas::SagaInstanceCreate>`
#1 [codegen_fulfill_obligation] checking if `core::ops::unsize::CoerceUnsized` fulfills its obligations
end of query stack
error: could not compile `omicron-nexus`
warning: build failed, waiting for other jobs to finish...
error: build failed
